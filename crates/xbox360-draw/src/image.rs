//! Owned images, as textures are created from them.

use alloc::vec::Vec;
use core::num::NonZeroU32;

use crate::draw::Rgba8;
use crate::error::ImageLayoutError;
use crate::texture::ImageView;
use crate::texture::check_layout;

/// How the color channels of an image's pixels relate to their alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AlphaMode {
    /// Color multiplied by alpha.
    #[default]
    Premultiplied,
    /// Color independent of alpha, kept at alpha 0. A texture of straight
    /// texels is sampled as described at
    /// [`TextureStore::create_target`](crate::texture::TextureStore::create_target).
    Straight,
}

/// RGBA image, rows top to bottom, no padding, at most
/// [`MAX_TEXTURE_DIMENSION`](crate::texture::MAX_TEXTURE_DIMENSION) in either direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Image {
    width: NonZeroU32,
    height: NonZeroU32,
    pixels: Vec<Rgba8>,
    alpha_mode: AlphaMode,
}

impl Image {
    /// A premultiplied image.
    pub fn new(width: NonZeroU32, height: NonZeroU32, pixels: Vec<Rgba8>) -> Result<Self, ImageLayoutError> {
        check_layout(width, height, pixels.len())?;

        Ok(Self { width, height, pixels, alpha_mode: AlphaMode::Premultiplied })
    }

    /// An image of straight pixels.
    pub fn new_straight(width: NonZeroU32, height: NonZeroU32, pixels: Vec<Rgba8>) -> Result<Self, ImageLayoutError> {
        check_layout(width, height, pixels.len())?;

        Ok(Self { width, height, pixels, alpha_mode: AlphaMode::Straight })
    }

    pub fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
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
        let Ok(view) = ImageView::with_alpha_mode(self.width, self.height, &self.pixels, self.alpha_mode) else {
            unreachable!("the layout is checked on construction");
        };

        view
    }
}
