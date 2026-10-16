//! Kernel threads.
//!
//! Titles that use the standard library use `std::thread`. This module is
//! for `no_std` code. A thread runs either an `extern "C" fn(u32) -> u32`
//! through an entry that [`thread_entry!`](crate::thread_entry) generates,
//! or, with the `alloc` feature, a boxed closure.
//!
//! The kernel enters a thread with the console calling convention, so each
//! start routine runs behind `xbox360_sys::entry_bridge`. Returning from
//! the routine ends the thread with its result as the exit code. [`exit`]
//! ends the thread early.

use core::num::NonZeroU32;
use core::time::Duration;

use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;
use xbox360_sys::xboxkrnl::ThreadCreationFlags;

use crate::status::KernelError;
use crate::sync::NOT_ALERTABLE;
use crate::sync::ProcessorMode;
use crate::sync::WaitError;
use crate::sync::WaitOutcome;
use crate::sync::wait_outcome;
use crate::sync::wait_signaled;

/// `ExCreateThread` stack size that selects the executable's default.
const DEFAULT_STACK_SIZE: u32 = 0;
/// `ExCreateThread` `xapi_thread_startup`: enter the start routine
/// directly with the context in r3.
const NO_XAPI_STARTUP: u32 = 0;
/// `ExCreateThread` creation flags that start the thread at once.
const START_AT_ONCE: ThreadCreationFlags = ThreadCreationFlags::empty();

pub use crate::time::sleep;

/// A thread start routine behind the console-ABI bridge.
#[derive(Clone, Copy)]
pub struct ThreadEntry(unsafe extern "C" fn());

impl ThreadEntry {
    /// # Safety
    ///
    /// `entry` must be a [`xbox360_sys::bridged!`] entry for an
    /// `extern "C" fn(u32) -> u32`. Returning from that function ends the
    /// thread with its result as the exit code.
    /// [`thread_entry!`](crate::thread_entry) generates such entries.
    pub const unsafe fn from_bridged(entry: unsafe extern "C" fn()) -> Self {
        Self(entry)
    }

    fn address(self) -> u32 {
        self.0 as usize as u32
    }
}

/// Define a [`ThreadEntry`] constant that runs `function`, an
/// `extern "C" fn(u32) -> u32`, and ends the thread with its result.
///
/// ```ignore
/// extern "C" fn worker(argument: u32) -> u32 {
///     argument + 1
/// }
///
/// xbox360::thread_entry!(WORKER = worker);
///
/// let thread = xbox360::thread::spawn_entry(WORKER, 41)?;
/// thread.join()?;
/// ```
#[macro_export]
macro_rules! thread_entry {
    ($(#[$meta:meta])* $vis:vis $name:ident = $function:path) => {
        $(#[$meta])*
        $vis const $name: $crate::thread::ThreadEntry = {
            extern "C" fn run(argument: u32) -> u32 {
                let function: extern "C" fn(u32) -> u32 = $function;
                function(argument)
            }
            $crate::sys::bridged!(entry, run);
            // SAFETY: `entry` runs `run`, an `extern "C" fn(u32) -> u32`,
            // behind the bridge.
            unsafe { $crate::thread::ThreadEntry::from_bridged(entry) }
        };
    };
}

/// Kernel thread id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(u32);

impl ThreadId {
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Address of a kernel thread object (`KTHREAD`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadObject(u32);

impl ThreadObject {
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Id of the calling thread (`GetCurrentThreadId`, which Xenia does not
/// implement).
pub fn current_id() -> ThreadId {
    ThreadId(xam::GetCurrentThreadId())
}

/// Thread object of the calling thread.
pub fn current_object() -> ThreadObject {
    ThreadObject(xbox360_sys::processor::current_thread())
}

/// Give up the rest of the time slice.
pub fn yield_now() {
    xboxkrnl::NtYieldExecution();
}

/// End the calling thread with `code` as its exit code.
///
/// # Safety
///
/// The thread's stack is released without running destructors. No frame
/// of the thread may own a value that must be dropped, such as a lock
/// guard, or data that is pinned or lent to another thread.
pub unsafe fn exit(code: u32) -> ! {
    // SAFETY: the caller guarantees that no frame of the thread owns a value
    // that must be dropped.
    unsafe { xboxkrnl::ExTerminateThread(code) }
}

/// Suspend the calling thread for good, after a kernel call that does not
/// return.
pub(crate) fn park() -> ! {
    loop {
        sleep(Duration::from_secs(3600));
    }
}

/// Options for a new thread.
#[derive(Debug, Clone, Copy, Default)]
pub struct Builder {
    stack_size: Option<NonZeroU32>,
}

impl Builder {
    pub const fn new() -> Self {
        Self { stack_size: None }
    }

    /// Stack size in bytes. Without it the kernel uses the stack size of
    /// the executable's header.
    pub const fn stack_size(self, bytes: NonZeroU32) -> Self {
        Self {
            stack_size: Some(bytes),
        }
    }

    fn create(self, start: u32, context: u32) -> Result<JoinHandle, KernelError> {
        let stack_size = self.stack_size.map_or(DEFAULT_STACK_SIZE, NonZeroU32::get);
        let mut handle = 0u32;
        let mut id = 0u32;
        // SAFETY: `start` is a bridged entry for an `extern "C" fn(u32) ->
        // u32` (the `ThreadEntry` contract or `boxed_entry`), and `handle`
        // and `id` are live locals.
        unsafe {
            xboxkrnl::ExCreateThread(
                &mut handle,
                stack_size,
                &mut id,
                NO_XAPI_STARTUP,
                start,
                context,
                START_AT_ONCE,
            )
        }
        .result()?;
        // Without RETURN_THREAD_OBJECT the slot holds a handle.
        Ok(JoinHandle {
            handle: Handle::from_raw(handle),
            id: ThreadId(id),
        })
    }

    /// Start a thread that runs `entry` with `argument`.
    pub fn spawn_entry(self, entry: ThreadEntry, argument: u32) -> Result<JoinHandle, KernelError> {
        self.create(entry.address(), argument)
    }

    /// Start a thread that runs `function`. Its exit code is 0.
    #[cfg(feature = "alloc")]
    pub fn spawn<F>(self, function: F) -> Result<JoinHandle, KernelError>
    where
        F: FnOnce() + Send + 'static,
    {
        let closure: alloc::boxed::Box<BoxedClosure> =
            alloc::boxed::Box::new(alloc::boxed::Box::new(function));
        let context = alloc::boxed::Box::into_raw(closure);
        let result = self.create(
            boxed_entry as *const () as usize as u32,
            context as usize as u32,
        );
        if result.is_err() {
            // SAFETY: the thread was not created, so this is the only owner.
            drop(unsafe { alloc::boxed::Box::from_raw(context) });
        }
        result
    }
}

#[cfg(feature = "alloc")]
type BoxedClosure = alloc::boxed::Box<dyn FnOnce() + Send + 'static>;

#[cfg(feature = "alloc")]
extern "C" fn boxed_main(context: u32) -> u32 {
    // SAFETY: `Builder::spawn` passes a leaked `Box<BoxedClosure>` that only
    // this thread uses.
    let closure = unsafe { alloc::boxed::Box::from_raw(context as usize as *mut BoxedClosure) };
    closure();
    0
}

#[cfg(feature = "alloc")]
xbox360_sys::bridged!(boxed_entry, boxed_main);

/// Start a thread that runs `entry` with `argument`.
pub fn spawn_entry(entry: ThreadEntry, argument: u32) -> Result<JoinHandle, KernelError> {
    Builder::new().spawn_entry(entry, argument)
}

/// Start a thread that runs `function`.
#[cfg(feature = "alloc")]
pub fn spawn<F>(function: F) -> Result<JoinHandle, KernelError>
where
    F: FnOnce() + Send + 'static,
{
    Builder::new().spawn(function)
}

/// A started thread. Dropping it closes the handle and leaves the thread
/// running.
#[derive(Debug)]
pub struct JoinHandle {
    handle: Handle,
    id: ThreadId,
}

impl JoinHandle {
    pub const fn id(&self) -> ThreadId {
        self.id
    }

    /// Wait for the thread to end or `timeout` to elapse.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<WaitOutcome, WaitError> {
        let interval = crate::time::relative_interval(timeout);
        // SAFETY: `self` owns the open thread handle; `interval` is a live
        // local.
        wait_outcome(unsafe {
            xboxkrnl::NtWaitForSingleObjectEx(
                self.handle,
                ProcessorMode::User,
                NOT_ALERTABLE,
                &interval,
            )
        })
    }

    /// Wait for the thread to end and close the handle.
    pub fn join(self) -> Result<(), WaitError> {
        // SAFETY: `self` owns the open thread handle; a null timeout waits
        // without limit.
        wait_signaled(unsafe {
            xboxkrnl::NtWaitForSingleObjectEx(
                self.handle,
                ProcessorMode::User,
                NOT_ALERTABLE,
                core::ptr::null(),
            )
        })
    }
}

impl Drop for JoinHandle {
    fn drop(&mut self) {
        // SAFETY: `self` owns the handle and closes it once.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}
