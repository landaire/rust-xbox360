//! JPEG decoding from a byte stream that is read once, front to back, as a
//! media stream is. The decoder sees the stream through a window that
//! keeps recently read bytes, so it can step back over the segment it
//! inspected last.

use alloc::vec;
use alloc::vec::Vec;

use zune_core::bytestream::ZByteIoError;
use zune_core::bytestream::ZByteReaderTrait;
use zune_core::bytestream::ZSeekFrom;
use zune_core::colorspace::ColorSpace;
use zune_jpeg::JpegDecoder;

use xbox360_draw::draw::Rgba8;

use crate::decoder_options;
use crate::error::ImageError;
use crate::error::JpegError;

/// A source of bytes read in order.
pub trait ByteSource {
    /// Reads up to `buffer.len()` bytes and returns how many; 0 at the end.
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SourceFailed>;
}

impl<T> ByteSource for &mut T
where
    T: ByteSource + ?Sized,
{
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SourceFailed> {
        (**self).read(buffer)
    }
}

/// A [`ByteSource`] read failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("the byte source could not be read")]
pub struct SourceFailed;

/// Size and metadata of a JPEG stream, read before its pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JpegDimensions {
    pub width: usize,
    pub height: usize,
}

/// A JPEG stream whose headers are read before its pixels are decoded
/// into a caller buffer.
pub struct JpegStream<S> {
    decoder: JpegDecoder<StreamReader<S>>,
}

impl<S> JpegStream<S>
where
    S: ByteSource,
{
    pub fn new(source: S) -> Self {
        // Sizes are checked by the caller against its own limits; JPEG
        // dimensions are 16-bit.
        const MAX_EDGE: usize = u16::MAX as usize;

        let options = decoder_options()
            .jpeg_set_out_colorspace(ColorSpace::RGBA)
            .set_max_width(MAX_EDGE)
            .set_max_height(MAX_EDGE);

        Self {
            decoder: JpegDecoder::new_with_options(StreamReader::new(source), options),
        }
    }

    /// Reads the segments before the image data.
    pub fn read_headers(&mut self) -> Result<JpegDimensions, ImageError> {
        self.decoder
            .decode_headers()
            .map_err(|error| ImageError::Jpeg(JpegError(error)))?;

        let (Some((width, height)), Some(colorspace)) =
            (self.decoder.dimensions(), self.decoder.output_colorspace())
        else {
            return Err(ImageError::MissingHeaderInfo);
        };
        if colorspace != ColorSpace::RGBA {
            return Err(ImageError::UnsupportedColorspace { colorspace });
        }

        Ok(JpegDimensions { width, height })
    }

    /// Decodes the pixels into `pixels`, which holds width x height
    /// entries, opaque, in row order.
    pub fn decode_into(&mut self, pixels: &mut [Rgba8]) -> Result<(), ImageError> {
        let len = pixels.len() * size_of::<Rgba8>();
        // SAFETY: `Rgba8` is a transparent `u32`; every byte pattern is valid
        // for both types and `u8` has no alignment requirement.
        let bytes =
            unsafe { core::slice::from_raw_parts_mut(pixels.as_mut_ptr().cast::<u8>(), len) };
        self.decoder
            .decode_into(bytes)
            .map_err(|error| ImageError::Jpeg(JpegError(error)))?;

        // Memory order is red, green, blue, alpha; `Rgba8` keeps red in the
        // low byte. Alpha is 255, so the colors are already premultiplied.
        for pixel in pixels {
            pixel.0 = u32::from_le(pixel.0);
        }

        Ok(())
    }
}

/// A window over a [`ByteSource`] that keeps [`HISTORY`] bytes before the
/// cursor.
struct StreamReader<S> {
    source: S,
    window: Vec<u8>,
    /// Bytes of `window` that hold data.
    filled: usize,
    cursor: usize,
    /// Stream offset of `window[0]`.
    offset: u64,
    ended: bool,
}

/// Bytes kept before the cursor; one segment of a JPEG is at most 0xFFFF
/// bytes long.
const HISTORY: usize = 0x1_0000;

/// Bytes read from the source at a time at most.
const WINDOW: usize = 0x8_0000;

impl<S> StreamReader<S>
where
    S: ByteSource,
{
    fn new(source: S) -> Self {
        Self {
            source,
            window: vec![0; WINDOW],
            filled: 0,
            cursor: 0,
            offset: 0,
            ended: false,
        }
    }

    fn available(&self) -> usize {
        self.filled - self.cursor
    }

    /// Reads more of the source after dropping bytes older than the
    /// history. Returns false at the end of the source.
    fn fill(&mut self) -> Result<bool, ZByteIoError> {
        if self.ended {
            return Ok(false);
        }

        if self.filled == self.window.len() {
            let drop = self.cursor.saturating_sub(HISTORY);
            if drop == 0 {
                return Err(ZByteIoError::Generic("stream window is full"));
            }
            self.window.copy_within(drop..self.filled, 0);
            self.filled -= drop;
            self.cursor -= drop;
            self.offset += drop as u64;
        }

        let read = self
            .source
            .read(&mut self.window[self.filled..])
            .map_err(|_| ZByteIoError::Generic("byte source failed"))?;
        if read == 0 {
            self.ended = true;
            return Ok(false);
        }
        self.filled += read;

        Ok(true)
    }

    /// Makes `count` bytes available after the cursor when the source has
    /// them.
    fn ensure(&mut self, count: usize) -> Result<usize, ZByteIoError> {
        while self.available() < count && self.fill()? {}

        Ok(self.available())
    }

    /// Advances over `count` bytes, reading the source as needed.
    fn advance(&mut self, mut count: usize) -> Result<(), ZByteIoError> {
        loop {
            let step = count.min(self.available());
            self.cursor += step;
            count -= step;
            if count == 0 {
                return Ok(());
            }
            if !self.fill()? {
                return Err(ZByteIoError::NotEnoughBytes(count, 0));
            }
        }
    }
}

impl<S> ZByteReaderTrait for StreamReader<S>
where
    S: ByteSource,
{
    fn read_byte_no_error(&mut self) -> u8 {
        match self.ensure(1) {
            Ok(1..) => {
                let byte = self.window[self.cursor];
                self.cursor += 1;
                byte
            }
            _ => 0,
        }
    }

    fn read_exact_bytes(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        if buf.len() <= WINDOW - HISTORY {
            let available = self.ensure(buf.len())?;
            if available < buf.len() {
                return Err(ZByteIoError::NotEnoughBytes(buf.len(), available));
            }
            buf.copy_from_slice(&self.window[self.cursor..self.cursor + buf.len()]);
            self.cursor += buf.len();
            return Ok(());
        }

        let mut done = 0;
        while done < buf.len() {
            let read = self.read_bytes(&mut buf[done..])?;
            if read == 0 {
                return Err(ZByteIoError::NotEnoughBytes(buf.len(), done));
            }
            done += read;
        }

        Ok(())
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        let available = self.ensure(1)?;
        let count = available.min(buf.len());
        buf[..count].copy_from_slice(&self.window[self.cursor..self.cursor + count]);
        self.cursor += count;

        Ok(count)
    }

    fn peek_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ZByteIoError> {
        let wanted = buf.len().min(WINDOW - HISTORY);
        let count = self.ensure(wanted)?.min(wanted);
        buf[..count].copy_from_slice(&self.window[self.cursor..self.cursor + count]);

        Ok(count)
    }

    fn peek_exact_bytes(&mut self, buf: &mut [u8]) -> Result<(), ZByteIoError> {
        let count = self.peek_bytes(buf)?;
        if count < buf.len() {
            return Err(ZByteIoError::NotEnoughBytes(buf.len(), count));
        }

        Ok(())
    }

    fn z_seek(&mut self, from: ZSeekFrom) -> Result<u64, ZByteIoError> {
        let position = self.offset + self.cursor as u64;
        let target = match from {
            ZSeekFrom::Start(target) => target,
            ZSeekFrom::Current(step) => position
                .checked_add_signed(step)
                .ok_or(ZByteIoError::SeekError("seek before the stream start"))?,
            ZSeekFrom::End(_) => {
                return Err(ZByteIoError::SeekError("the stream end is not known"));
            }
        };

        if target < self.offset {
            return Err(ZByteIoError::SeekError("seek before the kept bytes"));
        }
        if target <= position {
            self.cursor = (target - self.offset) as usize;
        } else {
            self.advance((target - position) as usize)?;
        }

        Ok(target)
    }

    fn is_eof(&mut self) -> Result<bool, ZByteIoError> {
        Ok(self.ensure(1)? == 0)
    }

    fn z_position(&mut self) -> Result<u64, ZByteIoError> {
        Ok(self.offset + self.cursor as u64)
    }

    fn read_remaining(&mut self, sink: &mut Vec<u8>) -> Result<usize, ZByteIoError> {
        let mut total = 0;
        while self.ensure(1)? > 0 {
            sink.extend_from_slice(&self.window[self.cursor..self.filled]);
            total += self.available();
            self.cursor = self.filled;
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A source that hands out at most `chunk` bytes per read.
    struct Chunked<'a> {
        bytes: &'a [u8],
        chunk: usize,
    }

    impl ByteSource for Chunked<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SourceFailed> {
            let count = self.chunk.min(buffer.len()).min(self.bytes.len());
            buffer[..count].copy_from_slice(&self.bytes[..count]);
            self.bytes = &self.bytes[count..];
            Ok(count)
        }
    }

    #[test]
    fn window_steps_back_over_kept_bytes_and_skips_forward() {
        let bytes: Vec<u8> = (0..0x2_0000u32).map(|index| index as u8).collect();
        let mut reader = StreamReader::new(Chunked {
            bytes: &bytes,
            chunk: 7,
        });

        let mut head = [0u8; 4];
        reader.read_exact_bytes(&mut head).unwrap();
        assert_eq!(head, [0, 1, 2, 3]);

        reader.z_seek(ZSeekFrom::Current(0x1_8000)).unwrap();
        reader.z_seek(ZSeekFrom::Current(-2)).unwrap();
        let mut next = [0u8; 2];
        reader.read_exact_bytes(&mut next).unwrap();
        assert_eq!(next, [bytes[0x1_8002], bytes[0x1_8003]]);

        let mut rest = Vec::new();
        reader.read_remaining(&mut rest).unwrap();
        assert_eq!(rest.len(), bytes.len() - 0x1_8004);
        assert!(reader.is_eof().unwrap());
    }

    #[test]
    fn streamed_jpeg_matches_the_in_memory_decode() {
        let data: Vec<u8> = (0..40u32 * 24)
            .flat_map(|index| {
                let (x, y) = (index % 40, index / 40);
                [(x * 6) as u8, (y * 10) as u8, (255 - x * 3 - y * 4) as u8]
            })
            .collect();
        let mut bytes = Vec::new();
        jpeg_encoder::Encoder::new(&mut bytes, 85)
            .encode(&data, 40, 24, jpeg_encoder::ColorType::Rgb)
            .unwrap();

        let mut stream = JpegStream::new(Chunked {
            bytes: &bytes,
            chunk: 3,
        });
        let dimensions = stream.read_headers().unwrap();
        assert_eq!(
            dimensions,
            JpegDimensions {
                width: 40,
                height: 24
            }
        );
        let mut pixels = vec![Rgba8::TRANSPARENT; 40 * 24];
        stream.decode_into(&mut pixels).unwrap();

        assert_eq!(
            pixels.as_slice(),
            crate::decode_jpeg(&bytes).unwrap().pixels()
        );
    }
}
