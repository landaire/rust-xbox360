use core::cell::UnsafeCell;
use core::sync::atomic::AtomicU32;
use core::time::Duration;

use xbox360_sys::xboxkrnl;

use super::EventType;
use super::NOT_ALERTABLE;
use super::ProcessorMode;
use super::WaitError;
use super::WaitOutcome;
use super::WaitReason;
use super::anchor::anchor_at;
use super::wait_outcome;
use super::wait_signaled;
use xbox360_time::relative_interval;

/// A kernel event (`KEVENT`) in static or owned storage.
pub struct Event {
    /// Dispatcher header: type, signal state, wait list head. Kept as words
    /// so that `new` can set the type and state in a const context.
    header: UnsafeCell<[u32; 4]>,
    anchor: AtomicU32,
}

// SAFETY: the kernel synchronizes access to dispatcher objects, and
// `anchor_at` serializes the setup of the wait list head.
unsafe impl Sync for Event {}

impl Event {
    pub const fn new(kind: EventType, signaled: bool) -> Self {
        // The type is the first byte of the header word.
        Self {
            header: UnsafeCell::new([kind.raw() << 24, signaled as u32, 0, 0]),
            anchor: AtomicU32::new(0),
        }
    }

    /// The `KEVENT` for kernel calls that take one. It is valid while
    /// `self` is borrowed.
    pub fn as_ptr(&self) -> *mut xboxkrnl::Kevent {
        let header = self.header.get().cast::<u32>();

        anchor_at(&self.anchor, header as usize as u32, || {
            let list = header.wrapping_add(2) as usize as u32;
            // SAFETY: no other thread uses the event while the anchor is
            // busy, and the kernel sees the header only after the anchor
            // store that publishes it; the signal state is kept.
            unsafe {
                header.add(2).write(list);
                header.add(3).write(list);
            }
        });

        header.cast()
    }

    pub fn set(&self) {
        // `KeSetEvent` `wait` argument: no wait call follows at raised IRQL.
        const NO_WAIT_FOLLOWS: bool = false;

        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the call.
        unsafe { xboxkrnl::KeSetEvent(self.as_ptr(), xboxkrnl::EVENT_INCREMENT, NO_WAIT_FOLLOWS) };
    }

    pub fn reset(&self) {
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the call.
        unsafe { xboxkrnl::KeResetEvent(self.as_ptr()) };
    }

    /// Wait until the event is signaled.
    pub fn wait(&self) -> Result<(), WaitError> {
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the wait; a null timeout waits
        // without limit.
        wait_signaled(unsafe {
            xboxkrnl::KeWaitForSingleObject(
                self.as_ptr().cast(),
                WaitReason::Executive,
                ProcessorMode::Kernel,
                NOT_ALERTABLE,
                core::ptr::null(),
            )
        })
    }

    /// Wait until the event is signaled or `timeout` elapses. A zero
    /// timeout tests the state without waiting.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<WaitOutcome, WaitError> {
        let interval = relative_interval(timeout);

        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the wait; `interval` is a live
        // local.
        wait_outcome(unsafe {
            xboxkrnl::KeWaitForSingleObject(
                self.as_ptr().cast(),
                WaitReason::Executive,
                ProcessorMode::Kernel,
                NOT_ALERTABLE,
                &interval,
            )
        })
    }
}
