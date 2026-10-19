/// `XVIDEO_MODE`, filled by [`XGetVideoMode`](super::XGetVideoMode).
/// Layout from Xenia; the dashboard 2.0.6717 reads `is_widescreen`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct VideoMode {
    pub display_width: u32,
    pub display_height: u32,
    /// Nonzero for an interlaced mode.
    pub is_interlaced: u32,
    /// Nonzero for a 16:9 display.
    pub is_widescreen: u32,
    /// Nonzero for a high definition mode.
    pub is_hi_def: u32,
    pub refresh_rate: f32,
    pub video_standard: u32,
    pub reserved: [u32; 5],
}

const _: () = assert!(core::mem::size_of::<VideoMode>() == 0x30);
