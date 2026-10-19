use core::fmt;

use zune_core::colorspace::ColorSpace;
use zune_jpeg::errors::DecodeErrors;
use zune_png::error::PngDecodeErrors;

/// Failure to decode an image file.
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("data does not start with a PNG or JPEG signature")]
    UnknownFormat,

    #[error("PNG decoding failed")]
    Png(#[source] PngError),

    #[error("JPEG decoding failed")]
    Jpeg(#[source] JpegError),

    #[error("decoder reported no size or colorspace after decoding the headers")]
    MissingHeaderInfo,

    #[error("decoder produced unsupported colorspace {colorspace:?}")]
    UnsupportedColorspace { colorspace: ColorSpace },

    #[error("image size {width}x{height} is outside 1..={max} in a dimension")]
    Dimensions {
        width: usize,
        height: usize,
        max: u32,
    },

    #[error("image of {pixels} pixels exceeds the budget of {budget}")]
    PixelBudget { pixels: usize, budget: usize },

    #[error("cannot allocate {pixels} pixels for a decoded image")]
    OutOfMemory { pixels: usize },
}

/// Error of the PNG decoder.
#[derive(Debug)]
pub struct PngError(pub PngDecodeErrors);

impl fmt::Display for PngError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl core::error::Error for PngError {}

/// Error of the JPEG decoder.
#[derive(Debug)]
pub struct JpegError(pub DecodeErrors);

impl fmt::Display for JpegError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl core::error::Error for JpegError {}
