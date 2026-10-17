//! Counted strings.

/// Counted ANSI string.
#[repr(C)]
pub struct AnsiString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *const u8,
}

impl AnsiString {
    /// String that refers to `bytes`. `None` if `bytes` is longer than
    /// `u16::MAX`.
    pub const fn new(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > u16::MAX as usize {
            return None;
        }

        Some(Self {
            length: bytes.len() as u16,
            maximum_length: bytes.len() as u16,
            buffer: bytes.as_ptr(),
        })
    }
}

/// `UNICODE_STRING` in a kernel structure: a counted UTF-16 string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnicodeString {
    /// Length in bytes.
    pub length: u16,
    pub maximum_length: u16,
    /// Address of the first code unit.
    pub buffer: u32,
}
