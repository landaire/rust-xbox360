//! Kernel version and clock times.

use super::field::KernelU32;

/// A kernel version, as `XboxKrnlVersion` and `XboxKrnlBaseVersion` hold
/// it.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub qfe: u16,
}

impl core::fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.qfe
        )
    }
}

const _: () = assert!(core::mem::size_of::<KernelVersion>() == 8);

/// `KeTimeStampBundle`: times that the kernel updates on each clock tick.
#[repr(C)]
pub struct TimeStampBundle {
    _times: [KernelU32; 4],
    /// Milliseconds since boot.
    pub tick_count: KernelU32,
    _reserved_14: KernelU32,
}

const _: () = {
    assert!(core::mem::offset_of!(TimeStampBundle, tick_count) == 0x10);
    assert!(core::mem::size_of::<TimeStampBundle>() == 0x18);
};
