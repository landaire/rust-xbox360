use super::DisplayWindow;
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

    /// The displayed part of the surface does not start on a tile or does
    /// not lie inside the surface.
    UnsupportedWindow { surface: FrameSize, window: DisplayWindow },

    OutOfMemory { buffer: BufferKind },
}

impl core::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyOpen => f.write_str("another display owns the GPU engines"),
            Self::UnsupportedMode { size } => {
                write!(f, "display surface {}x{} is not a whole number of 32-pixel tiles", size.width, size.height)
            }
            Self::UnsupportedWindow { surface, window } => write!(
                f,
                "display window ({}, {})-({}, {}) of the {}x{} surface does not start on a 32-pixel tile or does not lie inside the surface",
                window.left, window.top, window.right, window.bottom, surface.width, surface.height
            ),
            Self::OutOfMemory { buffer: BufferKind::Ring } => f.write_str("out of physical memory for the command ring"),
            Self::OutOfMemory { buffer: BufferKind::FrontBuffer } => f.write_str("out of physical memory for a front buffer"),
        }
    }
}

impl core::error::Error for DisplayError {}
