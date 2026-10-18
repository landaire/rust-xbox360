//! Threads for the Xbox 360 devkit, over the kernel's `ExCreateThread`.

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::ptr;
use crate::sys::pal::WaitInterval;
use crate::thread::ThreadInit;
use crate::time::{Duration, Instant};

use xbox360_sys::xboxkrnl::{self, Handle, ProcessorMode, ThreadCreationFlags};

pub const DEFAULT_MIN_STACK_SIZE: usize = 256 * 1024;

pub struct Thread {
    handle: Handle,
}

// The handle is an opaque kernel object id; moving it across threads is fine.
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

// The kernel enters a thread with the console calling convention, which
// expects r14-r31 preserved in full 64 bits when the start routine returns.
xbox360_sys::bridged!(thread_entry, thread_start);

// ExCreateThread is called with xapi_thread_startup = 0, so returning from the
// start routine ends the thread with the returned value as its exit code.
extern "C" fn thread_start(init: u32) -> u32 {
    // SAFETY: `init` is the `Box<ThreadInit>` that `Thread::new` leaked.
    let init = unsafe { Box::from_raw(ptr::with_exposed_provenance_mut::<ThreadInit>(init as usize)) };
    let rust_start = init.init();
    rust_start();
    crate::rt::thread_cleanup();
    0
}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        let init = Box::into_raw(init);
        let mut handle: u32 = 0;
        // SAFETY: `handle` is a live local, and `thread_entry` is the bridged start
        // routine that takes ownership of `init`.
        let status = unsafe {
            xboxkrnl::ExCreateThread(
                &mut handle,
                stack as u32,
                ptr::null_mut(),
                0,
                thread_entry as usize as u32,
                init.expose_provenance() as u32,
                ThreadCreationFlags::empty(),
            )
        };
        if !status.is_success() {
            // SAFETY: the thread was not created, so `init` was not consumed.
            drop(unsafe { Box::from_raw(init) });
            return Err(io::Error::from_raw_os_error(status.0 as i32));
        }
        // Without RETURN_THREAD_OBJECT the slot holds a handle.
        Ok(Thread { handle: Handle::from_raw(handle) })
    }

    pub fn join(self) {
        // A null timeout waits without limit. Dropping `self` closes the handle.
        // SAFETY: `self.handle` is the thread handle owned by this `Thread`.
        unsafe {
            xboxkrnl::NtWaitForSingleObjectEx(self.handle, ProcessorMode::User, false, ptr::null());
        }
    }
}

// Closing the handle of a detached thread does not stop it. Without the close,
// the handle stays open until its process exits.
impl Drop for Thread {
    fn drop(&mut self) {
        // SAFETY: `self.handle` is the thread handle owned by this `Thread`, closed
        // only here.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    // Xenon: 3 cores x 2 hardware threads.
    Ok(NonZero::new(6).unwrap())
}

pub fn current_os_id() -> Option<u64> {
    None
}

pub fn yield_now() {
    xboxkrnl::NtYieldExecution();
}

pub fn set_name(_name: &CStr) {}

pub fn sleep(dur: Duration) {
    // A kernel delay can end up to a clock tick early, so delay again until
    // the full duration has passed.
    let start = Instant::now();
    let mut remaining = dur;
    loop {
        // A duration beyond the interval range sleeps for the longest interval.
        let interval = WaitInterval::relative(remaining).unwrap_or(WaitInterval::LONGEST);
        // SAFETY: `interval` is a live local.
        unsafe { xboxkrnl::KeDelayExecutionThread(ProcessorMode::User, false, interval.as_ptr()) };
        match dur.checked_sub(start.elapsed()) {
            Some(rest) if !rest.is_zero() => remaining = rest,
            _ => return,
        }
    }
}
