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

#![no_std]
#![cfg(any(target_os = "xbox360", xbox360_docs))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// The raw bindings, for the expansion of [`thread_entry!`].
#[doc(hidden)]
pub use xbox360_sys as sys;

use core::num::NonZeroU32;
use core::sync::atomic::Ordering;
use core::time::Duration;

use xbox360_sys::processor::HardwareThread;
use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;
use xbox360_sys::xboxkrnl::Kthread;
use xbox360_sys::xboxkrnl::ProcessType;
use xbox360_sys::xboxkrnl::ThreadCreationFlags;

use xbox360_status::KernelError;
use xbox360_sync::NOT_ALERTABLE;
use xbox360_sync::ProcessorMode;
use xbox360_sync::WaitError;
use xbox360_sync::WaitOutcome;
use xbox360_sync::wait_outcome;
use xbox360_sync::wait_signaled;
use xbox360_time::relative_interval;

pub use xbox360_time::sleep;

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

/// Kernel thread id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(u32);

impl ThreadId {
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// A kernel thread object (`KTHREAD`) that stays allocated for `'a`. Like
/// a reference to [`Kthread`], it is not `Send`.
#[derive(Clone, Copy)]
pub struct ThreadObject<'a>(&'a Kthread);

impl<'a> ThreadObject<'a> {
    /// The thread object at `object`.
    ///
    /// # Safety
    ///
    /// `object` must be the address of an initialized thread object that
    /// stays allocated for `'a`, such as one that a reference from
    /// `ObReferenceObjectByHandle` keeps.
    pub unsafe fn from_raw(object: u32) -> Self {
        // SAFETY: the caller guarantees the object for `'a`.
        Self(unsafe { &*core::ptr::with_exposed_provenance::<Kthread>(object as usize) })
    }

    /// The address of the object, as kernel calls take it.
    pub fn raw(self) -> u32 {
        core::ptr::from_ref(self.0).expose_provenance() as u32
    }

    /// The fields of the object.
    pub const fn kthread(self) -> &'a Kthread {
        self.0
    }

    pub fn id(self) -> ThreadId {
        ThreadId(self.0.id)
    }

    /// Process type of the process that owns the thread.
    pub fn process_type(self) -> ProcessType {
        ProcessType(u32::from(self.0.process_type))
    }

    /// Process type that `KeGetCurrentProcessType` reports for the thread.
    pub fn current_process_type(self) -> ProcessType {
        ProcessType(u32::from(
            self.0.current_process_type.load(Ordering::Relaxed),
        ))
    }

    /// Hardware thread the thread runs or last ran on.
    pub fn hardware_thread(self) -> HardwareThread {
        self.0.processor()
    }
}

impl core::fmt::Debug for ThreadObject<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ThreadObject({:#010x})", self.raw())
    }
}

impl PartialEq for ThreadObject<'_> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.0, other.0)
    }
}

impl Eq for ThreadObject<'_> {}

impl core::hash::Hash for ThreadObject<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::ptr::hash(self.0, state);
    }
}

/// Options for a new thread.
#[derive(Debug, Clone, Copy, Default)]
pub struct Builder {
    stack_size: Option<NonZeroU32>,
    processor: Option<HardwareThread>,
}

impl Builder {
    pub const fn new() -> Self {
        Self { stack_size: None, processor: None }
    }

    /// Stack size in bytes. Without it the kernel uses the stack size of
    /// the executable's header.
    pub const fn stack_size(self, bytes: NonZeroU32) -> Self {
        Self {
            stack_size: Some(bytes),
            processor: self.processor,
        }
    }

    /// Run the thread on hardware thread `processor` only. Without it the
    /// kernel chooses.
    pub const fn processor(self, processor: HardwareThread) -> Self {
        Self {
            stack_size: self.stack_size,
            processor: Some(processor),
        }
    }

    fn create(self, start: u32, context: u32) -> Result<JoinHandle, KernelError> {
        // `ExCreateThread` stack size that selects the executable's default.
        const DEFAULT_STACK_SIZE: u32 = 0;
        // `ExCreateThread` `xapi_thread_startup`: enter the start routine
        // directly with the context in r3.
        const NO_XAPI_STARTUP: u32 = 0;

        let stack_size = self.stack_size.map_or(DEFAULT_STACK_SIZE, NonZeroU32::get);
        // Empty flags start the thread at once on a processor of the kernel's choice.
        let flags = self.processor.map_or(ThreadCreationFlags::empty(), |processor| {
            ThreadCreationFlags::processors(1u8 << processor.index())
        });
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
                flags,
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
        let interval = relative_interval(timeout);

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

#[cfg(feature = "alloc")]
type BoxedClosure = alloc::boxed::Box<dyn FnOnce() + Send + 'static>;

/// Id of the calling thread (`GetCurrentThreadId`, which Xenia does not
/// implement).
pub fn current_id() -> ThreadId {
    ThreadId(xam::GetCurrentThreadId())
}

/// Thread object of the calling thread.
pub fn current_object() -> ThreadObject<'static> {
    // SAFETY: the processor control region names the running thread's
    // object, which stays allocated while the thread runs, and a
    // `ThreadObject` cannot leave the thread.
    unsafe { ThreadObject::from_raw(xbox360_sys::processor::current_thread()) }
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
pub fn park() -> ! {
    loop {
        sleep(Duration::from_secs(3600));
    }
}

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
        $vis const $name: $crate::ThreadEntry = {
            extern "C" fn run(argument: u32) -> u32 {
                let function: extern "C" fn(u32) -> u32 = $function;
                function(argument)
            }
            $crate::sys::bridged!(entry, run);
            // SAFETY: `entry` runs `run`, an `extern "C" fn(u32) -> u32`,
            // behind the bridge.
            unsafe { $crate::ThreadEntry::from_bridged(entry) }
        };
    };
}
