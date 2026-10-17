use crate::xboxkrnl::Handle;

value_enum! {
    /// Win32 creation disposition, as `XamContentCreate` and `CreateFileA`
    /// take it: what to do when the target exists or does not.
    pub enum CreationDisposition: u32 {
        /// Create; fail if it exists.
        CreateNew = 1,
        /// Create; replace an existing one.
        CreateAlways = 2,
        /// Open; fail if it does not exist.
        OpenExisting = 3,
        /// Open, or create if it does not exist.
        OpenAlways = 4,
        /// Replace; fail if it does not exist.
        TruncateExisting = 5,
    }
}

/// `WaitForSingleObject` timeout that waits without limit.
pub const INFINITE: u32 = u32::MAX;

flags! {
    /// `HEAP_*` flags of the heap calls.
    pub struct HeapFlags(u32) {
        ZERO_MEMORY = 0x8,
    }
}

/// Result of `WaitForSingleObject`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaitResult(pub u32);

impl WaitResult {
    pub const OBJECT_0: Self = Self(0);
    pub const ABANDONED_0: Self = Self(0x80);
    pub const TIMEOUT: Self = Self(0x102);
    pub const FAILED: Self = Self(u32::MAX);
}

/// `CreateFileA` result of a failed call.
pub const INVALID_HANDLE_VALUE: Handle = Handle::from_raw(u32::MAX);

flags! {
    /// `DUPLICATE_*` options of `DuplicateHandle`.
    pub struct DuplicateOptions(u32) {
        CLOSE_SOURCE = 0x1,
        SAME_ACCESS = 0x2,
    }
}

/// `SYSTEMTIME`, as `GetLocalTime` stores it.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SystemTime {
    pub year: u16,
    pub month: u16,
    pub day_of_week: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub milliseconds: u16,
}
