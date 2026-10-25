//! PNG and JPEG decoding into premultiplied [`Rgba8`] images.

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod error;
pub mod stream;

use alloc::vec::Vec;
use core::num::NonZeroU32;

use zune_core::bytestream::ZCursor;
use zune_core::colorspace::ColorSpace;
use zune_core::options::DecoderOptions;
use zune_jpeg::JpegDecoder;
use zune_png::PngDecoder;

use xbox360_draw::draw::Rgba8;
pub(crate) use xbox360_draw::image::Image;
use xbox360_draw::texture::MAX_TEXTURE_DIMENSION;

use crate::error::ImageError;
use crate::error::JpegError;
use crate::error::PngError;

/// Encoded image file format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

impl ImageFormat {
    /// Identifies the format by the file signature.
    pub fn from_signature(bytes: &[u8]) -> Option<Self> {
        // The PNG file signature: a high-bit byte, "PNG", CR LF, EOF (^Z), LF.
        const PNG_SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
        // The JPEG start-of-image marker followed by the first marker prefix.
        const MARKER_PREFIX: u8 = 0xFF;
        const START_OF_IMAGE: u8 = 0xD8;
        const JPEG_SIGNATURE: [u8; 3] = [MARKER_PREFIX, START_OF_IMAGE, MARKER_PREFIX];

        if bytes.starts_with(&PNG_SIGNATURE) {
            Some(Self::Png)
        } else if bytes.starts_with(&JPEG_SIGNATURE) {
            Some(Self::Jpeg)
        } else {
            None
        }
    }
}

/// Largest decoded image in pixels (16 MiB of output). Bounds the memory a
/// hostile header can request; the largest genuine 6717 image has 307200
/// pixels.
pub const MAX_DECODED_PIXELS: usize = MAX_DECODED_EDGE * MAX_DECODED_EDGE;

/// The side of the largest square image within [`MAX_DECODED_PIXELS`].
const MAX_DECODED_EDGE: usize = 2048;

/// Decodes a PNG or JPEG file, selected by its signature.
pub fn decode(bytes: &[u8]) -> Result<Image, ImageError> {
    match ImageFormat::from_signature(bytes) {
        Some(ImageFormat::Png) => decode_png(bytes),
        Some(ImageFormat::Jpeg) => decode_jpeg(bytes),
        None => Err(ImageError::UnknownFormat),
    }
}

/// Decodes any PNG color type and bit depth. Sixteen-bit samples keep
/// their high byte; palette and tRNS transparency become alpha. Chunk
/// CRCs and the zlib Adler-32 checksum are verified.
pub fn decode_png(bytes: &[u8]) -> Result<Image, ImageError> {
    let options = decoder_options().png_set_strip_to_8bit(true);
    let mut decoder = PngDecoder::new_with_options(ZCursor::new(bytes), options);
    decoder
        .decode_headers()
        .map_err(|error| ImageError::Png(PngError(error)))?;

    let (Some((width, height)), Some(colorspace)) = (decoder.dimensions(), decoder.colorspace())
    else {
        return Err(ImageError::MissingHeaderInfo);
    };
    let components = match colorspace {
        ColorSpace::Luma => SourceComponents::Luma,
        ColorSpace::LumaA => SourceComponents::LumaAlpha,
        ColorSpace::RGB => SourceComponents::Rgb,
        ColorSpace::RGBA => SourceComponents::Rgba,
        colorspace => return Err(ImageError::UnsupportedColorspace { colorspace }),
    };

    let mut buffer = Buffer::allocate(width, height)?;
    decoder
        .decode_into(buffer.bytes_mut())
        .map_err(|error| ImageError::Png(PngError(error)))?;
    buffer.expand_in_place(components);

    Ok(buffer.into_image())
}

/// Decodes a baseline or progressive JPEG, rejecting non-conforming
/// streams. The result is opaque.
pub fn decode_jpeg(bytes: &[u8]) -> Result<Image, ImageError> {
    let options = decoder_options().jpeg_set_out_colorspace(ColorSpace::RGBA);
    let mut decoder = JpegDecoder::new_with_options(ZCursor::new(bytes), options);
    decoder
        .decode_headers()
        .map_err(|error| ImageError::Jpeg(JpegError(error)))?;

    let (Some((width, height)), Some(colorspace)) =
        (decoder.dimensions(), decoder.output_colorspace())
    else {
        return Err(ImageError::MissingHeaderInfo);
    };
    if colorspace != ColorSpace::RGBA {
        return Err(ImageError::UnsupportedColorspace { colorspace });
    }

    let mut buffer = Buffer::allocate(width, height)?;
    decoder
        .decode_into(buffer.bytes_mut())
        .map_err(|error| ImageError::Jpeg(JpegError(error)))?;
    buffer.expand_in_place(SourceComponents::Rgba);

    Ok(buffer.into_image())
}

/// Pixels being decoded, with dimensions checked against the limits.
struct Buffer {
    width: NonZeroU32,
    height: NonZeroU32,
    pixels: Vec<Rgba8>,
}

impl Buffer {
    /// Checks decoded header dimensions against the limits before the
    /// decoder allocates anything sized by them.
    fn allocate(width: usize, height: usize) -> Result<Self, ImageError> {
        let dimensions = 1..=MAX_TEXTURE_DIMENSION as usize;
        let (Some(width_nonzero), Some(height_nonzero)) = (
            NonZeroU32::new(width as u32).filter(|_| dimensions.contains(&width)),
            NonZeroU32::new(height as u32).filter(|_| dimensions.contains(&height)),
        ) else {
            return Err(ImageError::Dimensions {
                width,
                height,
                max: MAX_TEXTURE_DIMENSION,
            });
        };

        let count = width * height;
        if count > MAX_DECODED_PIXELS {
            return Err(ImageError::PixelBudget {
                pixels: count,
                budget: MAX_DECODED_PIXELS,
            });
        }

        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(count)
            .map_err(|_| ImageError::OutOfMemory { pixels: count })?;
        pixels.resize(count, Rgba8::TRANSPARENT);

        Ok(Self {
            width: width_nonzero,
            height: height_nonzero,
            pixels,
        })
    }

    fn into_image(self) -> Image {
        let Ok(image) = Image::new(self.width, self.height, self.pixels) else {
            unreachable!("allocate checks the dimensions against the texture limit");
        };

        image
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.pixels.len() * size_of::<Rgba8>();

        // SAFETY: `Rgba8` is a transparent `u32`; every byte pattern is valid
        // for both types and `u8` has no alignment requirement.
        unsafe { core::slice::from_raw_parts_mut(self.pixels.as_mut_ptr().cast::<u8>(), len) }
    }

    /// Converts tightly packed decoder output at the start of the pixel
    /// buffer into premultiplied pixels. Pixels are expanded from the last
    /// to the first so that no source byte is overwritten before it is read.
    fn expand_in_place(&mut self, components: SourceComponents) {
        let count = self.pixels.len();
        let bytes = self.bytes_mut();

        const OPAQUE: u8 = u8::MAX;
        const PIXEL_BYTES: usize = size_of::<Rgba8>();

        let source_bytes = components.bytes_per_pixel();
        for index in (0..count).rev() {
            let source = &bytes[index * source_bytes..(index + 1) * source_bytes];
            let [red, green, blue, alpha] = match components {
                SourceComponents::Luma => [source[0], source[0], source[0], OPAQUE],
                SourceComponents::LumaAlpha => [source[0], source[0], source[0], source[1]],
                SourceComponents::Rgb => [source[0], source[1], source[2], OPAQUE],
                SourceComponents::Rgba => [source[0], source[1], source[2], source[3]],
            };

            bytes[index * PIXEL_BYTES..(index + 1) * PIXEL_BYTES].copy_from_slice(&[
                premultiply(red, alpha),
                premultiply(green, alpha),
                premultiply(blue, alpha),
                alpha,
            ]);
        }

        // Memory order is red, green, blue, alpha; `Rgba8` keeps red in the
        // low byte of its value. This is a no-op on little-endian hosts.
        for pixel in &mut self.pixels {
            pixel.0 = u32::from_le(pixel.0);
        }
    }
}

/// Channel layout of decoder output, one byte per channel.
#[derive(Clone, Copy)]
enum SourceComponents {
    Luma,
    LumaAlpha,
    Rgb,
    Rgba,
}

impl SourceComponents {
    fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Luma => 1,
            Self::LumaAlpha => 2,
            Self::Rgb => 3,
            Self::Rgba => 4,
        }
    }
}

/// Strict decoding: checksums are verified and non-conforming streams are
/// rejected rather than repaired. Header sizes are checked by
/// [`Buffer::allocate`] before any buffer sized by them exists.
fn decoder_options() -> DecoderOptions {
    DecoderOptions::default().set_strict_mode(true)
}

/// Matches [`Rgba8::from_argb_straight`] rounding.
fn premultiply(channel: u8, alpha: u8) -> u8 {
    const MAX: u32 = u8::MAX as u32;

    ((u32::from(channel) * u32::from(alpha) + MAX / 2) / MAX) as u8
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use super::*;

    struct PngFixture<'a> {
        width: u32,
        height: u32,
        color: png::ColorType,
        depth: png::BitDepth,
        palette: Option<&'a [u8]>,
        transparency: Option<&'a [u8]>,
        data: &'a [u8],
    }

    fn encode_png(fixture: PngFixture<'_>) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut encoder = png::Encoder::new(&mut bytes, fixture.width, fixture.height);
        encoder.set_color(fixture.color);
        encoder.set_depth(fixture.depth);
        if let Some(palette) = fixture.palette {
            encoder.set_palette(palette);
        }
        if let Some(transparency) = fixture.transparency {
            encoder.set_trns(transparency);
        }
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(fixture.data).unwrap();
        writer.finish().unwrap();

        bytes
    }

    fn png_fixture(color: png::ColorType, data: &[u8]) -> PngFixture<'_> {
        PngFixture {
            width: 2,
            height: 1,
            color,
            depth: png::BitDepth::Eight,
            palette: None,
            transparency: None,
            data,
        }
    }

    fn decoded(bytes: &[u8]) -> Image {
        super::decode(bytes).unwrap()
    }

    #[test]
    fn png_rgba_is_premultiplied() {
        let bytes = encode_png(png_fixture(
            png::ColorType::Rgba,
            &[255, 128, 0, 255, 200, 100, 50, 128],
        ));

        let image = decoded(&bytes);
        assert_eq!((image.width().get(), image.height().get()), (2, 1));
        assert_eq!(
            image.into_pixels(),
            [Rgba8::new(255, 128, 0, 255), Rgba8::new(100, 50, 25, 128)]
        );
    }

    #[test]
    fn png_rgb_is_opaque() {
        let bytes = encode_png(png_fixture(png::ColorType::Rgb, &[1, 2, 3, 250, 251, 252]));

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [Rgba8::new(1, 2, 3, 255), Rgba8::new(250, 251, 252, 255)]
        );
    }

    #[test]
    fn png_rgb_color_key_becomes_transparent() {
        let mut fixture = png_fixture(png::ColorType::Rgb, &[10, 20, 30, 40, 50, 60]);
        let key = [0, 40, 0, 50, 0, 60];
        fixture.transparency = Some(&key);
        let bytes = encode_png(fixture);

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [Rgba8::new(10, 20, 30, 255), Rgba8::TRANSPARENT]
        );
    }

    #[test]
    fn png_gray_expands_to_rgb() {
        let bytes = encode_png(png_fixture(png::ColorType::Grayscale, &[0, 77]));

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [Rgba8::new(0, 0, 0, 255), Rgba8::new(77, 77, 77, 255)]
        );
    }

    #[test]
    fn png_gray_color_key_becomes_transparent() {
        let mut fixture = png_fixture(png::ColorType::Grayscale, &[77, 78]);
        let key = [0, 78];
        fixture.transparency = Some(&key);
        let bytes = encode_png(fixture);

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [Rgba8::new(77, 77, 77, 255), Rgba8::TRANSPARENT]
        );
    }

    #[test]
    fn png_gray_alpha_is_premultiplied() {
        let bytes = encode_png(png_fixture(
            png::ColorType::GrayscaleAlpha,
            &[200, 255, 200, 64],
        ));

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [Rgba8::new(200, 200, 200, 255), Rgba8::new(50, 50, 50, 64)]
        );
    }

    #[test]
    fn png_palette_with_partial_transparency() {
        let palette = [255, 0, 0, 0, 255, 0, 0, 0, 255];
        // Entries without a tRNS value are opaque.
        let transparency = [0, 128];
        let indices = [0, 1, 2, 1];
        let fixture = PngFixture {
            width: 4,
            height: 1,
            color: png::ColorType::Indexed,
            depth: png::BitDepth::Eight,
            palette: Some(&palette),
            transparency: Some(&transparency),
            data: &indices,
        };
        let bytes = encode_png(fixture);

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [
                Rgba8::TRANSPARENT,
                Rgba8::new(0, 128, 0, 128),
                Rgba8::new(0, 0, 255, 255),
                Rgba8::new(0, 128, 0, 128)
            ]
        );
    }

    #[test]
    fn png_packed_palette_indices() {
        let palette = [10, 10, 10, 20, 20, 20, 30, 30, 30, 40, 40, 40];
        // Two bits per index: 3, 2, 1, 0, 0, 1, 2, 3 packed MSB first.
        let fixture = PngFixture {
            width: 8,
            height: 1,
            color: png::ColorType::Indexed,
            depth: png::BitDepth::Two,
            palette: Some(&palette),
            transparency: None,
            data: &[0b1110_0100, 0b0001_1011],
        };
        let bytes = encode_png(fixture);

        let gray = |value: u8| Rgba8::new(value, value, value, 255);
        assert_eq!(
            decoded(&bytes).into_pixels(),
            [
                gray(40),
                gray(30),
                gray(20),
                gray(10),
                gray(10),
                gray(20),
                gray(30),
                gray(40)
            ]
        );
    }

    #[test]
    fn png_sixteen_bit_keeps_high_byte() {
        let fixture = PngFixture {
            width: 2,
            height: 1,
            color: png::ColorType::Rgba,
            depth: png::BitDepth::Sixteen,
            palette: None,
            transparency: None,
            data: &[
                0x12, 0xFF, 0x34, 0x00, 0x56, 0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x00, 0x00, 0x00,
                0x80, 0xFF,
            ],
        };
        let bytes = encode_png(fixture);

        assert_eq!(
            decoded(&bytes).into_pixels(),
            [
                Rgba8::new(0x12, 0x34, 0x56, 0xFF),
                Rgba8::new(0x80, 0x40, 0x00, 0x80)
            ]
        );
    }

    fn encode_jpeg(
        data: &[u8],
        width: u16,
        height: u16,
        color: jpeg_encoder::ColorType,
    ) -> Vec<u8> {
        let mut bytes = Vec::new();
        jpeg_encoder::Encoder::new(&mut bytes, 100)
            .encode(data, width, height, color)
            .unwrap();

        bytes
    }

    fn assert_close(actual: Rgba8, expected: [u8; 3]) {
        let channels = [actual.red(), actual.green(), actual.blue()];
        for (channel, wanted) in channels.into_iter().zip(expected) {
            assert!(
                channel.abs_diff(wanted) <= 2,
                "{actual:?} is not close to {expected:?}"
            );
        }
        assert_eq!(actual.alpha(), 255);
    }

    #[test]
    fn jpeg_color_is_opaque() {
        let data: Vec<u8> = [200u8, 40, 90].repeat(16 * 16);
        let bytes = encode_jpeg(&data, 16, 16, jpeg_encoder::ColorType::Rgb);

        let image = decoded(&bytes);
        assert_eq!((image.width().get(), image.height().get()), (16, 16));
        for pixel in image.into_pixels() {
            assert_close(pixel, [200, 40, 90]);
        }
    }

    #[test]
    fn jpeg_gray_expands_to_rgb() {
        let data = vec![90u8; 8 * 8];
        let bytes = encode_jpeg(&data, 8, 8, jpeg_encoder::ColorType::Luma);

        let image = decoded(&bytes);
        for pixel in image.into_pixels() {
            assert_eq!((pixel.red(), pixel.alpha()), (pixel.green(), 255));
            assert_eq!(pixel.red(), pixel.blue());
            assert_close(pixel, [90, 90, 90]);
        }
    }

    /// Pins the exact output of chroma upsampling and the inverse DCT, which
    /// must not change between hosts or decoder updates.
    #[test]
    fn jpeg_subsampled_gradient_decodes_exactly() {
        let data: Vec<u8> = (0..16u32 * 16)
            .flat_map(|index| {
                let (x, y) = (index % 16, index / 16);
                [(x * 16) as u8, (y * 16) as u8, (255 - x * 8 - y * 7) as u8]
            })
            .collect();
        let mut bytes = Vec::new();
        let mut encoder = jpeg_encoder::Encoder::new(&mut bytes, 90);
        encoder.set_sampling_factor(jpeg_encoder::SamplingFactor::R_4_2_0);
        encoder
            .encode(&data, 16, 16, jpeg_encoder::ColorType::Rgb)
            .unwrap();

        let image = decoded(&bytes);
        let rows: Vec<alloc::string::String> = image
            .pixels()
            .chunks(16)
            .map(|row| {
                let pixels: Vec<_> = row
                    .iter()
                    .map(|pixel| {
                        alloc::format!(
                            "{:02X}{:02X}{:02X}",
                            pixel.red(),
                            pixel.green(),
                            pixel.blue()
                        )
                    })
                    .collect();
                pixels.join(" ")
            })
            .collect();
        insta::assert_snapshot!(rows.join("\n"));
    }

    #[test]
    fn unknown_signature_is_rejected() {
        assert!(matches!(decode(b"GIF89a"), Err(ImageError::UnknownFormat)));
    }

    #[test]
    fn truncated_png_is_an_error() {
        let bytes = encode_png(png_fixture(png::ColorType::Rgb, &[1, 2, 3, 4, 5, 6]));

        assert!(matches!(
            decode(&bytes[..bytes.len() - 20]),
            Err(ImageError::Png(_))
        ));
    }

    #[test]
    fn truncated_jpeg_header_is_an_error() {
        let bytes = encode_jpeg(&[0u8; 8 * 8 * 3], 8, 8, jpeg_encoder::ColorType::Rgb);

        assert!(matches!(decode(&bytes[..12]), Err(ImageError::Jpeg(_))));
    }

    /// A tiny file must not make the decoder allocate for a huge header.
    #[test]
    fn oversized_png_headers_are_rejected_before_decoding() {
        let over_budget = encode_png(PngFixture {
            width: 4096,
            height: 1025,
            color: png::ColorType::Grayscale,
            depth: png::BitDepth::One,
            palette: None,
            transparency: None,
            data: &vec![0; 512 * 1025],
        });
        let too_wide = encode_png(PngFixture {
            width: MAX_TEXTURE_DIMENSION + 1,
            height: 1,
            color: png::ColorType::Grayscale,
            depth: png::BitDepth::One,
            palette: None,
            transparency: None,
            data: &vec![0; (MAX_TEXTURE_DIMENSION as usize + 1).div_ceil(8)],
        });

        assert!(matches!(
            decode(&over_budget),
            Err(ImageError::PixelBudget {
                pixels: 4_198_400,
                budget: MAX_DECODED_PIXELS
            })
        ));
        assert!(matches!(
            decode(&too_wide),
            Err(ImageError::Dimensions {
                width: 8193,
                height: 1,
                max: MAX_TEXTURE_DIMENSION
            })
        ));
    }

    #[test]
    fn png_with_bad_chunk_crc_is_rejected() {
        let mut bytes = encode_png(png_fixture(png::ColorType::Rgb, &[1, 2, 3, 4, 5, 6]));
        // The IHDR CRC follows the signature, chunk header and 13 data bytes.
        bytes[8 + 8 + 13] ^= 0xFF;

        assert!(matches!(decode(&bytes), Err(ImageError::Png(_))));
    }
}
