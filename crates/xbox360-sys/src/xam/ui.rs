value_enum! {
    /// Icon of a message box.
    pub enum MessageBoxIcon: u32 {
        None = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
    }
}

/// `XamShowMessageBoxUI` flags. The low four bits are the
/// [`MessageBoxIcon`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageBoxFlags(u32);

impl MessageBoxFlags {
    pub const fn new(icon: MessageBoxIcon) -> Self {
        Self(icon as u32)
    }

    /// `bits` including bits without a name, for values passed on unchanged.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}
