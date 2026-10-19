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
    /// The user enters a four-key passcode, returned in
    /// [`MessageBoxResult::passcode`]. The box takes no buttons.
    pub const PASSCODE_ENTRY: Self = Self(0x1_0000);
    /// XAM compares the entered passcode with the one the result block
    /// holds and completes successfully only on a match.
    pub const PASSCODE_VERIFY: Self = Self(0x2_0000);

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

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// `MESSAGEBOX_RESULT`: the index of the chosen button, or the four virtual
/// key codes of a passcode.
#[repr(C)]
#[derive(Clone, Copy)]
pub union MessageBoxResult {
    pub button: u32,
    pub passcode: [u16; 4],
}

const _: () = assert!(core::mem::size_of::<MessageBoxResult>() == 8);

/// `XamShowKeyboardUI` flags, passed on unchanged.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardFlags(u32);

impl KeyboardFlags {
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// `XamShowDeviceSelectorUI` flags, passed on unchanged.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceSelectorFlags(u32);

impl DeviceSelectorFlags {
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// `XamShowSigninUI` flags, passed on unchanged.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigninFlags(u32);

impl SigninFlags {
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}
