//! Video display information.

/// The 0x58-byte display information block that
/// [`VdGetCurrentDisplayInformation`](super::VdGetCurrentDisplayInformation) fills, with the fields in use named.
#[repr(C)]
pub struct DisplayInformation {
    pub surface_width: u16,
    pub surface_height: u16,
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
            surface_height: 0,
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

/// The 0x94-byte system command buffer descriptor that
/// [`VdGetSystemCommandBuffer`](super::VdGetSystemCommandBuffer) fills and
/// [`VdSwap`](super::VdSwap) reads.
#[repr(C)]
pub struct SystemCommandBuffer {
    /// GPU address of the buffer, or 0 when the kernel has none.
    pub address: u32,
    pub dwords: u32,
    /// Nonzero when the following state is valid for the swap outputs.
    pub state: u32,
    _reserved_00c: [u32; 0x22],
}

impl SystemCommandBuffer {
    pub const fn new() -> Self {
        Self { address: 0, dwords: 0, state: 0, _reserved_00c: [0; 0x22] }
    }
}

const _: () = assert!(core::mem::size_of::<SystemCommandBuffer>() == 0x94);

/// One mode passed to the [`VdEnumerateVideoModes`](super::VdEnumerateVideoModes)
/// callback, with the fields the dashboard 2.0.6717 reads named. The size of
/// the record is not known; only these 0x10 bytes are read.
#[repr(C)]
pub struct VideoModeInfo {
    _reserved_000: u16,
    pub width: u16,
    pub height: u16,
    _reserved_006: u8,
    /// 3 for the display's native mode [INFERENCE].
    pub kind: u8,
    pub refresh_rate: f32,
    /// 0x2: also offered at 4:3 [INFERENCE]; 0x4: interlaced.
    pub flags: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(VideoModeInfo, width) == 2);
    assert!(core::mem::offset_of!(VideoModeInfo, kind) == 7);
    assert!(core::mem::offset_of!(VideoModeInfo, refresh_rate) == 8);
    assert!(core::mem::offset_of!(VideoModeInfo, flags) == 0xc);
};

/// The 0x10-byte parameter block of
/// [`VdSetDisplayModeOverride`](super::VdSetDisplayModeOverride).
#[repr(C)]
pub struct DisplayModeOverrideParameters {
    /// 0x10.
    pub size: u32,
    _reserved_004: [u32; 2],
    /// Retry index of the override.
    pub attempt: u8,
    _reserved_00d: [u8; 3],
}

impl DisplayModeOverrideParameters {
    pub const SIZE: u32 = 0x10;

    pub const fn new(attempt: u8) -> Self {
        Self { size: Self::SIZE, _reserved_004: [0; 2], attempt, _reserved_00d: [0; 3] }
    }
}

const _: () = assert!(core::mem::size_of::<DisplayModeOverrideParameters>() == 0x10);

/// The block [`VdGetDisplayDiscoveryData`](super::VdGetDisplayDiscoveryData)
/// fills. The dashboard 2.0.6717 stores 0x23 at +0 and reads the u32 at +4
/// and the unaligned u32 at +0x1D; the rest of the layout is not known.
#[repr(C, align(4))]
pub struct DisplayDiscoveryData {
    pub bytes: [u8; 0x50],
}

impl DisplayDiscoveryData {
    /// The value the dashboard stores at +0 before the call.
    pub const SIZE_FIELD: u32 = 0x23;

    pub const fn new() -> Self {
        let mut bytes = [0; 0x50];
        let size = Self::SIZE_FIELD.to_be_bytes();
        bytes[0] = size[0];
        bytes[1] = size[1];
        bytes[2] = size[2];
        bytes[3] = size[3];
        Self { bytes }
    }

    /// Big-endian u32 at `offset`.
    pub const fn word(&self, offset: usize) -> u32 {
        u32::from_be_bytes([self.bytes[offset], self.bytes[offset + 1], self.bytes[offset + 2], self.bytes[offset + 3]])
    }
}
