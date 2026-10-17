//! Kernel object handles.

/// A kernel object handle as the kernel passes it. [`Handle::NULL`] is the
/// kernel's own "no handle": optional handle arguments take it, and calls
/// that fail may store it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Handle(u32);

impl Handle {
    pub const NULL: Self = Self(0);
    /// Pseudo handle of the current process.
    pub const CURRENT_PROCESS: Self = Self(0xffff_ffff);
    /// Pseudo handle of the current thread.
    pub const CURRENT_THREAD: Self = Self(0xffff_fffe);
    /// Pseudo handle of the `\??` object directory (Xenia's
    /// `ObDosDevices`): the root directory of an object name that does not
    /// start with a backslash.
    pub const DOS_DEVICES: Self = Self(0xffff_fffd);

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}
