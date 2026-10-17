//! `BOOL` and `BOOLEAN` results.

/// A `BOOL` result: nonzero is true.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boolean(pub u32);

impl Boolean {
    pub const fn get(self) -> bool {
        self.0 != 0
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> bool {
        value.get()
    }
}

impl From<bool> for Boolean {
    /// `TRUE` is 1.
    fn from(value: bool) -> Self {
        Self(u32::from(value))
    }
}

/// A `BOOLEAN` result. The export returns one byte in a full register, so
/// only the low byte is the value: nonzero is true.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boolean8(pub u32);

impl Boolean8 {
    pub const fn get(self) -> bool {
        self.0 as u8 != 0
    }
}

impl From<Boolean8> for bool {
    fn from(value: Boolean8) -> bool {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean8_tests_the_low_byte() {
        assert!(!Boolean8(0xdead_be00).get());
        assert!(Boolean8(0x0000_0001).get());
        assert!(Boolean(0x0000_0100).get());
    }
}
