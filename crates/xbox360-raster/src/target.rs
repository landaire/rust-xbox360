use core::num::NonZeroU32;

use crate::draw::ClipRect;
use crate::draw::Rgba8;
use crate::error::TargetError;

/// Caller-owned premultiplied RGBA pixel buffer with a row stride in pixels.
#[derive(Debug)]
pub struct RenderTarget<'a> {
    pixels: &'a mut [Rgba8],
    width: u32,
    height: u32,
    stride: u32,
}

impl<'a> RenderTarget<'a> {
    /// Largest width or height; geometry is clipped to this many pixels on
    /// each side of the origin.
    pub const MAX_DIMENSION: u32 = 16384;

    /// `pixels` holds `height` rows of `stride` pixels; the last row needs
    /// only `width` pixels.
    pub fn new(
        pixels: &'a mut [Rgba8],
        width: u32,
        height: u32,
        stride: u32,
    ) -> Result<Self, TargetError> {
        if width > Self::MAX_DIMENSION || height > Self::MAX_DIMENSION {
            return Err(TargetError::TooLarge { width, height });
        }

        if stride < width {
            return Err(TargetError::StrideTooSmall { stride, width });
        }

        let required = match height.checked_sub(1) {
            Some(last_row) => (last_row as usize)
                .checked_mul(stride as usize)
                .and_then(|start| start.checked_add(width as usize))
                .ok_or(TargetError::SizeOverflow {
                    width,
                    height,
                    stride,
                })?,
            None => 0,
        };
        if pixels.len() < required {
            return Err(TargetError::BufferTooSmall {
                required,
                actual: pixels.len(),
            });
        }

        Ok(Self {
            pixels,
            width,
            height,
            stride,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }

    /// The whole target as one band.
    pub fn band(&mut self) -> TargetBand<'_> {
        TargetBand {
            pixels: self.pixels,
            width: self.width,
            stride: self.stride,
            top: 0,
            bottom: self.height,
        }
    }

    /// Splits the target into disjoint bands of `rows_per_band` rows (the
    /// last band may be shorter) that can be rendered independently.
    pub fn bands(&mut self, rows_per_band: NonZeroU32) -> Bands<'_> {
        Bands {
            rest: self.pixels,
            width: self.width,
            stride: self.stride,
            next_top: 0,
            height: self.height,
            rows_per_band: rows_per_band.get(),
        }
    }
}

/// Rows `top..bottom` of a [`RenderTarget`].
#[derive(Debug)]
pub struct TargetBand<'a> {
    /// Starts at row `top`.
    pixels: &'a mut [Rgba8],
    width: u32,
    stride: u32,

    top: u32,
    bottom: u32,
}

impl<'a> TargetBand<'a> {
    /// All rows of an image of `height` rows of `width` pixels without
    /// padding, as the texels of a render target texture.
    pub(super) fn from_texels(pixels: &'a mut [Rgba8], width: u32, height: u32) -> Self {
        debug_assert_eq!(pixels.len(), width as usize * height as usize);

        Self {
            pixels,
            width,
            stride: width,
            top: 0,
            bottom: height,
        }
    }

    /// The band's pixels in target coordinates.
    pub(super) fn rect(&self) -> ClipRect {
        ClipRect {
            left: 0,
            top: self.top as i32,
            right: self.width as i32,
            bottom: self.bottom as i32,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    /// Exclusive.
    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    /// Row `y` in target coordinates; `top <= y < bottom`.
    pub fn row_mut(&mut self, y: u32) -> &mut [Rgba8] {
        let start = (y - self.top) as usize * self.stride as usize;

        &mut self.pixels[start..start + self.width as usize]
    }
}

/// Iterator of [`RenderTarget::bands`].
#[derive(Debug)]
pub struct Bands<'a> {
    rest: &'a mut [Rgba8],
    width: u32,
    stride: u32,
    next_top: u32,
    height: u32,
    rows_per_band: u32,
}

impl<'a> Iterator for Bands<'a> {
    type Item = TargetBand<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_top >= self.height {
            return None;
        }

        let top = self.next_top;
        let rows = self.rows_per_band.min(self.height - top);
        let bottom = top + rows;
        self.next_top = bottom;

        let rest = core::mem::take(&mut self.rest);
        let pixels = if bottom < self.height {
            let (band, rest) = rest.split_at_mut(rows as usize * self.stride as usize);
            self.rest = rest;
            band
        } else {
            rest
        };

        Some(TargetBand {
            pixels,
            width: self.width,
            stride: self.stride,
            top,
            bottom,
        })
    }
}
