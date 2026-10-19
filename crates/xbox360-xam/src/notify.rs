//! Notification listeners.
//!
//! Checked against the call sites of the 2.0.6717 dashboard and Xenia: the
//! ordinals (0x291 `XamNotifyCreateListenerInternal`, 0x28B
//! `XNotifyGetNext`) and the argument order. Not run on hardware.

use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

/// A queued notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Notification {
    pub id: u32,
    pub parameter: u32,
}

/// A notification listener. Closed on drop.
pub struct Listener {
    handle: Handle,
}

impl Listener {
    /// `XamNotifyCreateListenerInternal(mask, is_system, max_version)`. The
    /// dashboard passes `(0x23, 1, 3)`. `None` when XAM returns no handle.
    pub fn create_internal(mask: u64, is_system: bool, max_version: u32) -> Option<Self> {
        // SAFETY: the export takes only values.
        let handle = unsafe { xam::XamNotifyCreateListenerInternal(mask, u32::from(is_system), max_version) };
        (handle != Handle::NULL).then_some(Self { handle })
    }

    /// `XNotifyGetNext`: the next queued notification of any id.
    pub fn next(&self) -> Option<Notification> {
        let mut id = 0;
        let mut parameter = 0;
        // SAFETY: `handle` is open and both outputs are live locals.
        let taken = unsafe { xam::XNotifyGetNext(self.handle, 0, &mut id, &mut parameter) };
        taken.get().then_some(Notification { id, parameter })
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        // SAFETY: `handle` is open and not used after this.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}
