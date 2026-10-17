use core::cell::UnsafeCell;

use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

use crate::status::KernelError;
use crate::status::Win32Error;
use crate::status::Win32ErrorCode;
use crate::sync::EventType;
use crate::sync::NOT_ALERTABLE;
use crate::sync::ProcessorMode;
use crate::sync::wait_signaled;

/// An `XOVERLAPPED` with its own notification event, for one
/// asynchronous XAM call that is waited for before the block is released.
pub(super) struct OverlappedCall {
    block: UnsafeCell<xam::Overlapped>,
    event: Handle,
}

impl OverlappedCall {
    pub(super) fn new() -> Result<Self, KernelError> {
        let mut event = Handle::NULL;
        let initially_signaled = false;

        // SAFETY: `event` is a live local, and a null attribute pointer
        // creates an unnamed event.
        unsafe {
            xboxkrnl::NtCreateEvent(
                &mut event,
                core::ptr::null(),
                EventType::Notification,
                initially_signaled,
            )
        }
        .result()?;

        let block = xam::Overlapped {
            event,
            ..xam::Overlapped::default()
        };

        Ok(Self {
            block: UnsafeCell::new(block),
            event,
        })
    }

    pub(super) fn as_ptr(&self) -> *mut xam::Overlapped {
        self.block.get()
    }

    /// The Win32 result of the call that returned `code` with this block.
    /// Waits for completion when the call is pending.
    ///
    /// # Panics
    ///
    /// If the wait fails. XAM may still write the block then, so it cannot
    /// be released.
    pub(super) fn complete(&self, code: Win32ErrorCode) -> Result<(), Win32Error> {
        if code.error() != Some(Win32Error::IO_PENDING) {
            return code.result();
        }

        // SAFETY: `self` owns the open event handle; a null timeout waits
        // without limit.
        let wait = unsafe {
            xboxkrnl::NtWaitForSingleObjectEx(
                self.event,
                ProcessorMode::User,
                NOT_ALERTABLE,
                core::ptr::null(),
            )
        };

        if let Err(error) = wait_signaled(wait) {
            panic!("waiting for an XAM operation failed: {error}");
        }

        // SAFETY: XAM finished with the block before setting the event, and
        // the wait orders this read after its writes.
        unsafe { (*self.block.get()).result }.result()
    }
}

impl Drop for OverlappedCall {
    fn drop(&mut self) {
        // SAFETY: `self` owns the event handle and closes it once.
        unsafe { xboxkrnl::NtClose(self.event) };
    }
}
