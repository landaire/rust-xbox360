//! The texture fetch constant that describes the front buffer to `VdSwap`.

/// Fetch constant type field of a texture.
const TYPE_TEXTURE: u32 = 2;
/// Surface is tiled.
const TILED: u32 = 1 << 31;
/// Pitch field position, in units of 32 pixels.
const PITCH_SHIFT: u32 = 22;
/// k_8_8_8_8 texel format.
const FORMAT_8_8_8_8: u32 = 6;
/// Endian swap of each 32-bit word, bits 6-7.
const ENDIAN_8_IN_32: u32 = 2 << 6;
/// Height field position of the size dword; width is in bits 0-12.
const HEIGHT_SHIFT: u32 = 13;
/// Dimension field of dword 5: two-dimensional.
const DIMENSION_2D: u32 = 1 << 9;

/// Six dwords for a `width` x `height` 32 bpp tiled surface at virtual
/// address `base`, `pitch_tiles` tiles wide. `VdSwap` converts the address
/// in dword 1 to physical and copies the constant to fetch constant 15 when
/// the kernel supplies a system command buffer; otherwise it is unused.
pub fn front_buffer(base: u32, pitch_tiles: u32, width: u32, height: u32) -> [u32; 6] {
    [
        TILED | (pitch_tiles << PITCH_SHIFT) | TYPE_TEXTURE,
        base | ENDIAN_8_IN_32 | FORMAT_8_8_8_8,
        (width - 1) | ((height - 1) << HEIGHT_SHIFT),
        0,
        0,
        DIMENSION_2D,
    ]
}
