//! Threads for the Xbox 360 devkit, over the kernel's `ExCreateThread`.

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::sys::pal::abi;
use crate::thread::ThreadInit;
use crate::time::Duration;

pub const DEFAULT_MIN_STACK_SIZE: usize = 256 * 1024;

pub struct Thread {
    handle: u32,
}

// The handle is an opaque kernel object id; moving it across threads is fine.
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

/// Kernel thread entry: reconstruct the boxed init, run the closure, terminate.
extern "C" fn thread_start(param: usize) -> u32 {
    let init = unsafe { Box::from_raw(param as *mut ThreadInit) };
    (init.init())();
    unsafe { abi::ExTerminateThread(0) };
    0
}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        let raw = Box::into_raw(init);
        let mut handle: u32 = 0;
        // ExCreateThread(handle, stack, tid=null, apiThreadStartup=0, start, ctx, flags=0).
        let status = unsafe {
            abi::ExCreateThread(
                &mut handle,
                stack as u32,
                crate::ptr::null_mut(),
                0,
                thread_start as usize,
                raw as usize,
                0,
            )
        };
        if status != 0 {
            drop(unsafe { Box::from_raw(raw) });
            return Err(io::Error::from_raw_os_error(status as i32));
        }
        Ok(Thread { handle })
    }

    pub fn join(self) {
        unsafe {
            // Wait forever (null timeout), then close the handle.
            abi::NtWaitForSingleObjectEx(self.handle, 1, 0, crate::ptr::null());
            abi::NtClose(self.handle);
        }
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
    unsafe { abi::NtYieldExecution() };
}

pub fn set_name(_name: &CStr) {}

pub fn sleep(dur: Duration) {
    // KeDelayExecutionThread: negative interval = relative, 100ns ticks.
    let ticks = (dur.as_nanos() / 100) as i64;
    let interval = -ticks.max(1);
    unsafe { abi::KeDelayExecutionThread(1, 0, &interval) };
}
