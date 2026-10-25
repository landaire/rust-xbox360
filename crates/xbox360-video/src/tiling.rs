//! Conversion of RGBA pixels into the tiled 32 bpp layout that the display
//! controller scans out.
//!
//! The surface is a grid of 32x32-pixel tiles of 4 KB, row-major. Within a
//! tile, an even row `y` and row `y + 1` share 256 contiguous bytes at
//! `(y & 0x1e) * 128`. Those bytes are four 64-byte slots, each holding eight
//! pixels of both rows as runs of four: row `y` pixels 0-3, row `y + 1`
//! pixels 0-3, row `y` pixels 4-7, row `y + 1` pixels 4-7. Slot `k` holds
//! the 8-pixel group `k ^ s` of the tile row, where `s` is 2 when bit 3 of `y`
//! is set and 0 otherwise. Writing a row pair therefore fills each tile's
//! 256 bytes in address order, which suits write-combined memory.


/// Pixels per tile edge.
pub const TILE_EDGE: u32 = 32;

/// 32-bit words per tile.
const TILE_WORDS: usize = (TILE_EDGE * TILE_EDGE) as usize;

/// Scanout word of a pixel. The display reads the bytes B, G, R, X in
/// address order, so on this big-endian processor the word is
/// `B << 24 | G << 16 | R << 8`, which is the RGBA word shifted left by one
/// byte. Alpha is dropped.
#[inline(always)]
const fn scanout_word(pixel: u32) -> u32 {
    pixel << 8
}

/// Rows of the source image: `pixels` holds `height` rows of `stride` pixels,
/// of which the first `width` are visible.
#[derive(Clone, Copy)]
pub struct SourceImage<'a> {
    pub pixels: &'a [u32],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

impl SourceImage<'_> {
    fn row(&self, y: u32) -> Option<&[u32]> {
        if y >= self.height {
            return None;
        }
        let start = (y * self.stride) as usize;
        self.pixels.get(start..start + self.width as usize)
    }
}

/// A rectangle of whole tiles in a surface of `pitch_tiles` tiles per row.
#[derive(Clone, Copy)]
pub struct TiledTarget {
    pub pitch_tiles: u32,
    pub tiles_wide: u32,
    pub tiles_high: u32,
}

/// Fill the `target` rectangle that starts at `tiled[0]` from `source`.
/// Pixels of the rectangle outside the source are written black; the rest
/// of the surface is not written.
pub fn tile_image(tiled: &mut [u32], target: TiledTarget, source: SourceImage<'_>) {
    let tiles_wide = target.tiles_wide as usize;
    let surface_width = tiles_wide * TILE_EDGE as usize;
    for y in (0..target.tiles_high * TILE_EDGE).step_by(2) {
        let swap = ((y >> 2) & 2) as usize;
        let row_base = (y / TILE_EDGE) as usize * target.pitch_tiles as usize * TILE_WORDS + (((y & 0x1e) as usize) << 5);
        let Some(rows) = tiled.get_mut(row_base..row_base + (tiles_wide - 1) * TILE_WORDS + 64) else {
            return;
        };
        let (upper, lower) = (source.row(y), source.row(y + 1));
        if let (Some(upper), Some(lower)) = (upper, lower)
            && upper.len() >= surface_width
        {
            let pairs = upper.chunks_exact(TILE_EDGE as usize).zip(lower.chunks_exact(TILE_EDGE as usize));
            for (tile, (upper, lower)) in rows.chunks_mut(TILE_WORDS).zip(pairs) {
                for (slot, out) in tile[..64].chunks_exact_mut(16).enumerate() {
                    let x = (slot ^ swap) << 3;
                    write_slot(out, &upper[x..x + 8], &lower[x..x + 8]);
                }
            }
            continue;
        }
        for (tile_x, tile) in rows.chunks_mut(TILE_WORDS).enumerate() {
            for (slot, out) in tile[..64].chunks_exact_mut(16).enumerate() {
                let x = tile_x * TILE_EDGE as usize + ((slot ^ swap) << 3);
                write_slot(out, &group(upper, x), &group(lower, x));
            }
        }
    }
}

/// One 64-byte slot: four pixels of each row, then the next four of each.
#[inline(always)]
fn write_slot(out: &mut [u32], upper: &[u32], lower: &[u32]) {
    let (Ok(out), Ok(upper), Ok(lower)) =
        (<&mut [u32; 16]>::try_from(out), <&[u32; 8]>::try_from(upper), <&[u32; 8]>::try_from(lower))
    else {
        return;
    };
    out[0] = scanout_word(upper[0]);
    out[1] = scanout_word(upper[1]);
    out[2] = scanout_word(upper[2]);
    out[3] = scanout_word(upper[3]);
    out[4] = scanout_word(lower[0]);
    out[5] = scanout_word(lower[1]);
    out[6] = scanout_word(lower[2]);
    out[7] = scanout_word(lower[3]);
    out[8] = scanout_word(upper[4]);
    out[9] = scanout_word(upper[5]);
    out[10] = scanout_word(upper[6]);
    out[11] = scanout_word(upper[7]);
    out[12] = scanout_word(lower[4]);
    out[13] = scanout_word(lower[5]);
    out[14] = scanout_word(lower[6]);
    out[15] = scanout_word(lower[7]);
}

/// The eight pixels of `row` from `x`, black past its end.
fn group(row: Option<&[u32]>, x: usize) -> [u32; 8] {
    core::array::from_fn(|i| row.and_then(|row| row.get(x + i)).copied().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::vec;
    use std::vec::Vec;

    use super::*;

    /// Byte address of a pixel in a 32 bpp tiled surface, by the per-pixel
    /// formula that the host screenshot decoder uses.
    fn reference_address(x: u32, y: u32, aligned_width: u32) -> u32 {
        let macro_offset = ((x >> 5) + (y >> 5) * (aligned_width >> 5)) << 9;
        let micro = ((x & 7) + ((y & 6) << 2)) << 2;
        let offset = macro_offset + ((micro & !0xf) << 1) + (micro & 0xf) + ((y & 8) << 5) + ((y & 1) << 4);
        ((offset & !0x1ff) << 3)
            + ((y & 16) << 7)
            + ((offset & 0x1c0) << 2)
            + ((((y & 8) >> 2) + (x >> 3)) & 3) * 64
            + (offset & 0x3f)
    }

    #[test]
    fn matches_reference_layout_with_clipping() {
        check(70, 47, 75);
    }

    #[test]
    fn matches_reference_layout_full_rows() {
        check(96, 64, 100);
    }

    #[test]
    fn window_writes_only_its_tiles() {
        let (pitch_tiles, tiles_high) = (4u32, 2u32);
        let (width, height, stride) = (50u32, 40u32, 50u32);
        let pixels: Vec<u32> = (0..stride * height).map(|i| i.wrapping_mul(0x0101_0101).wrapping_add(7)).collect();
        let mut tiled = vec![0xdead_beef; (pitch_tiles * tiles_high) as usize * TILE_WORDS];
        let target = TiledTarget { pitch_tiles, tiles_wide: 2, tiles_high };
        let source = SourceImage { pixels: &pixels, width, height, stride };
        tile_image(&mut tiled[TILE_WORDS..], target, source);
        for y in 0..tiles_high * TILE_EDGE {
            for x in 0..pitch_tiles * TILE_EDGE {
                let word = tiled[reference_address(x, y, pitch_tiles * TILE_EDGE) as usize / 4];
                let expected = match x.checked_sub(TILE_EDGE) {
                    Some(wx) if wx < 2 * TILE_EDGE => {
                        if wx < width && y < height {
                            pixels[(y * stride + wx) as usize] << 8
                        } else {
                            0
                        }
                    }
                    _ => 0xdead_beef,
                };
                assert_eq!(word, expected, "pixel ({x}, {y})");
            }
        }
    }

    fn check(width: u32, height: u32, stride: u32) {
        let (tiles_wide, tiles_high) = (3, 2);
        let pixels: Vec<u32> = (0..stride * height).map(|i| i.wrapping_mul(0x0101_0101).wrapping_add(7)).collect();
        let mut tiled = vec![0xdead_beef; (tiles_wide * tiles_high) as usize * TILE_WORDS];
        let source = SourceImage { pixels: &pixels, width, height, stride };
        tile_image(&mut tiled, TiledTarget { pitch_tiles: tiles_wide, tiles_wide, tiles_high }, source);
        for y in 0..tiles_high * TILE_EDGE {
            for x in 0..tiles_wide * TILE_EDGE {
                let expected = if x < width && y < height { pixels[(y * stride + x) as usize] << 8 } else { 0 };
                let word = reference_address(x, y, tiles_wide * TILE_EDGE) as usize / 4;
                assert_eq!(tiled[word], expected, "pixel ({x}, {y})");
            }
        }
    }
}
