//! Owned images, as textures are created from them.

use alloc::vec::Vec;
use core::num::NonZeroU32;

use crate::draw::Rgba8;
use crate::error::ImageLayoutError;
use crate::texture::ImageView;
use crate::texture::check_layout;

/// Premultiplied RGBA image, rows top to bottom, no padding, at
/// most [`MAX_TEXTURE_DIMENSION`](crate::texture::MAX_TEXTURE_DIMENSION) in either direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    width: NonZeroU32,
    height: NonZeroU32,
    pixels: Vec<Rgba8>,
}

impl Image {
    pub fn new(width: NonZeroU32, height: NonZeroU32, pixels: Vec<Rgba8>) -> Result<Self, ImageLayoutError> {
        check_layout(width, height, pixels.len())?;

        Ok(Self { width, height, pixels })
    }

    pub fn width(&self) -> NonZeroU32 {
        self.width
    }

    pub fn height(&self) -> NonZeroU32 {
        self.height
    }

    /// Exactly `width * height` pixels.
    pub fn pixels(&self) -> &[Rgba8] {
        &self.pixels
    }

    pub fn into_pixels(self) -> Vec<Rgba8> {
        self.pixels
    }

    pub fn view(&self) -> ImageView<'_> {
        let Ok(view) = ImageView::new(self.width, self.height, &self.pixels) else {
            unreachable!("the layout is checked on construction");
        };

        view
    }
}
