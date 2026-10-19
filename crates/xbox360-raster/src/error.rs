pub use xbox360_draw::error::TextureError;

/// Invalid render target description.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TargetError {
    #[error("target size {width}x{height} exceeds the largest supported dimension")]
    TooLarge { width: u32, height: u32 },

    #[error("row stride {stride} is less than width {width}")]
    StrideTooSmall { stride: u32, width: u32 },

    #[error("target of {width}x{height} with stride {stride} exceeds the addressable size")]
    SizeOverflow {
        width: u32,
        height: u32,
        stride: u32,
    },

    #[error("target needs {required} pixels but the buffer holds {actual}")]
    BufferTooSmall { required: usize, actual: usize },
}
