use core::fmt;

use crate::draw::TextureId;

/// Pixel buffer that does not describe a valid image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageLayoutError {
    TooLarge { width: u32, height: u32, max: u32 },
    PixelCount { width: u32, height: u32, expected: usize, actual: usize },
}

impl fmt::Display for ImageLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TooLarge { width, height, max } => write!(f, "image size {width}x{height} exceeds {max} in a dimension"),
            Self::PixelCount { width, height, expected, actual } => {
                write!(f, "image of {width}x{height} needs {expected} pixels but holds {actual}")
            }
        }
    }
}

impl core::error::Error for ImageLayoutError {}

/// Failure of a [`crate::texture::TextureStore`] operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureError {
    UnknownTexture { id: TextureId },
    SlotsExhausted,
    TooLarge { width: u32, height: u32, max: u32 },
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnknownTexture { id } => write!(f, "texture {id:?} does not exist"),
            Self::SlotsExhausted => f.write_str("no texture slot is left"),
            Self::TooLarge { width, height, max } => {
                write!(f, "texture size {width}x{height} exceeds the backend limit of {max}")
            }
        }
    }
}

impl core::error::Error for TextureError {}

/// Invalid [`crate::draw::DrawList::push`] input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawListError {
    IndexOutOfRange { index: u32, vertex_count: usize },
    TooManyVertices { count: usize },
    TooManyTriangles { count: usize },
}

impl fmt::Display for DrawListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IndexOutOfRange { index, vertex_count } => {
                write!(f, "vertex index {index} is not below the vertex count {vertex_count}")
            }
            Self::TooManyVertices { count } => write!(f, "{count} vertices exceed the addressable count"),
            Self::TooManyTriangles { count } => write!(f, "{count} triangles exceed the addressable count"),
        }
    }
}

impl core::error::Error for DrawListError {}
