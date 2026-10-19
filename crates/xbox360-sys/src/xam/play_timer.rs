/// Record of [`XamPlayTimerGetData`](super::XamPlayTimerGetData) and
/// [`XamPlayTimerSetData`](super::XamPlayTimerSetData) (0x14 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayTimerData {
    /// +0x00, written by
    /// [`XamPlayTimerGetNextResetDate`](super::XamPlayTimerGetNextResetDate).
    pub next_reset: [u32; 2],
    /// +0x08: period flags; 0 is off, 2 daily, 4 weekly.
    pub period: u32,
    /// +0x0C: play time allowed per period, in milliseconds.
    pub allowance: u32,
    /// +0x10: play time remaining in the period, in milliseconds.
    pub remaining: u32,
}

const _: () = assert!(core::mem::size_of::<PlayTimerData>() == 0x14);
