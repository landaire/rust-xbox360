/// One profile of [`XamProfileEnumerate`](super::XamProfileEnumerate). The
/// 2.0.6717 dashboard passes a 0x188-byte buffer whose first 8 bytes are
/// the offline XUID; the account data after it is not decoded here.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfileEnumResult {
    pub xuid_offline: u64,
    pub account: [u8; 0x180],
}

impl ProfileEnumResult {
    pub const fn new() -> Self {
        Self { xuid_offline: 0, account: [0; 0x180] }
    }
}

impl Default for ProfileEnumResult {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = assert!(core::mem::size_of::<ProfileEnumResult>() == 0x188);

/// `XUSER_DATA`: a typed setting value. The 2.0.6717 dashboard reads the
/// data type byte at +0 and the value at +8; a string is its byte length at
/// +8 and its address at +0xC.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserData {
    pub data_type: u8,
    pub reserved: [u8; 7],
    pub value: [u32; 2],
}

impl UserData {
    /// Data type of a NUL-terminated UTF-16 string.
    pub const UNICODE: u8 = 4;

    /// A string of `bytes` bytes, NUL included, at `address`. Addresses
    /// are 32-bit.
    pub fn unicode(address: *const u16, bytes: u32) -> Self {
        Self { data_type: Self::UNICODE, reserved: [0; 7], value: [bytes, address as u32] }
    }
}

const _: () = assert!(core::mem::size_of::<UserData>() == 0x10);
