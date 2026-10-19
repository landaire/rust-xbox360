use core::cell::UnsafeCell;
use core::marker::PhantomPinned;
use core::pin::Pin;

use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

use xbox360_status::KernelError;
use xbox360_status::Win32Error;
use xbox360_status::Win32ErrorCode;
use xbox360_sync::EventType;
use xbox360_sync::NOT_ALERTABLE;
use xbox360_sync::ProcessorMode;
use xbox360_sync::wait_signaled;

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

    /// Prepares the block for another call: clears the event and the
    /// fields of the previous call. Call only when no request is pending.
    pub(super) fn reset(&self) {
        // SAFETY: `self` owns the open event handle, and no request uses
        // the block, so nothing else accesses it.
        unsafe {
            let _ = xboxkrnl::NtClearEvent(self.event);
            *self.block.get() = xam::Overlapped { event: self.event, ..xam::Overlapped::default() };
        }
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

/// State of an [`AsyncOperation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsyncState {
    /// XAM still owns the block and the buffers of the call.
    Pending,
    /// The Win32 result of the call. A block that never started a call
    /// reads as `Ok`.
    Complete(Result<(), Win32Error>),
}

/// An `XOVERLAPPED` without an event, polled by its owner as the 2.0.6717
/// dashboard polls its calls from timers. XAM writes the block until the
/// call completes, so it is pinned. Dropping a pending operation cancels
/// the call and waits for it, so buffers declared after it in a struct
/// outlive the call.
pub struct AsyncOperation {
    block: UnsafeCell<xam::Overlapped>,
    _pinned: PhantomPinned,
}

impl AsyncOperation {
    pub const fn new() -> Self {
        Self {
            block: UnsafeCell::new(idle_block()),
            _pinned: PhantomPinned,
        }
    }

    pub fn as_ptr(self: Pin<&Self>) -> *mut xam::Overlapped {
        self.block.get()
    }

    pub fn is_pending(&self) -> bool {
        // SAFETY: the block is live; XAM writes the result field last and
        // the volatile read observes its stores.
        let result = unsafe { core::ptr::read_volatile(&raw const (*self.block.get()).result) };
        result.error() == Some(Win32Error::IO_PENDING)
    }

    /// The state of the call, without waiting.
    pub fn state(self: Pin<&Self>) -> AsyncState {
        if self.is_pending() {
            return AsyncState::Pending;
        }

        // Orders reads of the call's buffers after the result XAM wrote last.
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        let dont_wait = false;
        // SAFETY: the block is pinned and the call completed.
        let result = unsafe { xam::XamGetOverlappedResult(self.as_ptr(), core::ptr::null_mut(), dont_wait) };

        AsyncState::Complete(result.result())
    }

    /// The length field of a completed call, such as the record count of
    /// `XamEnumerate`. `None` while the call is pending.
    pub fn length(self: Pin<&Self>) -> Option<u32> {
        if self.is_pending() {
            return None;
        }

        // Orders the read after the result XAM wrote last.
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        // SAFETY: the block is live and the call no longer writes it.
        Some(unsafe { core::ptr::read_volatile(&raw const (*self.block.get()).length) })
    }

    /// `XMsgCancelIORequest` of a pending call, waiting for it to complete
    /// as the dashboard does. The call then completes with
    /// `ERROR_CANCELLED` unless it finished first.
    pub fn cancel(self: Pin<&Self>) {
        if !self.is_pending() {
            return;
        }

        let wait = true;
        // SAFETY: the block is pinned and names the pending call.
        unsafe { xam::XMsgCancelIORequest(self.as_ptr(), wait) };
    }

    /// Prepares the block for another call: cancels a pending one and
    /// clears the fields, as the dashboard zeroes its blocks before a call.
    pub fn reset(self: Pin<&Self>) {
        self.cancel();

        // SAFETY: no call uses the block, so nothing else accesses it.
        unsafe { *self.block.get() = idle_block() };
    }
}

impl Default for AsyncOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AsyncOperation {
    fn drop(&mut self) {
        // SAFETY: a value is only dropped in place, so a pinned operation
        // is still at the address XAM writes.
        unsafe { Pin::new_unchecked(&*self) }.cancel();
    }
}

const fn idle_block() -> xam::Overlapped {
    xam::Overlapped {
        result: Win32ErrorCode::SUCCESS,
        length: 0,
        context: 0,
        event: Handle::NULL,
        completion_routine: 0,
        completion_context: 0,
        extended_error: 0,
    }
}
