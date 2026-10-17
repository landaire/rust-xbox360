//! Video display information.

/// The 0x58-byte display information block that
/// [`VdGetCurrentDisplayInformation`](super::VdGetCurrentDisplayInformation) fills, with the fields in use named.
#[repr(C)]
pub struct DisplayInformation {
    pub surface_width: u16,
    _reserved_002: [u8; 2],
    pub colorspace: u8,
    _reserved_005: [u8; 3],
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub scaled_width: u32,
    pub scaled_height: u32,
    _reserved_020: [u32; 0xe],
}

impl DisplayInformation {
    pub const fn new() -> Self {
        Self {
            surface_width: 0,
            _reserved_002: [0; 2],
            colorspace: 0,
            _reserved_005: [0; 3],
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            scaled_width: 0,
            scaled_height: 0,
            _reserved_020: [0; 0xe],
        }
    }

    /// The block as [`VdGetCurrentDisplayInformation`](super::VdGetCurrentDisplayInformation) takes it.
    pub fn as_mut_ptr(&mut self) -> *mut Self {
        core::ptr::from_mut(self)
    }
}

const _: () = {
    assert!(core::mem::offset_of!(DisplayInformation, colorspace) == 4);
    assert!(core::mem::offset_of!(DisplayInformation, left) == 8);
    assert!(core::mem::offset_of!(DisplayInformation, scaled_height) == 0x1c);
    assert!(core::mem::size_of::<DisplayInformation>() == 0x58);
};
