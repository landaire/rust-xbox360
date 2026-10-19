use super::FrameSize;

/// Allocation the presenter could not make.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferKind {
    Ring,
    FrontBuffer,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayError {
    AlreadyOpen,

    UnsupportedMode { size: FrameSize },

    OutOfMemory { buffer: BufferKind },
}

impl core::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyOpen => f.write_str("another display owns the GPU engines"),
            Self::UnsupportedMode { size } => {
                write!(f, "display surface {}x{} is not a whole number of 32-pixel tiles", size.width, size.height)
            }
            Self::OutOfMemory { buffer: BufferKind::Ring } => f.write_str("out of physical memory for the command ring"),
            Self::OutOfMemory { buffer: BufferKind::FrontBuffer } => f.write_str("out of physical memory for a front buffer"),
        }
    }
}

impl core::error::Error for DisplayError {}
