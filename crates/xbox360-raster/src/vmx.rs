//! VMX (AltiVec) versions of the per-pixel blend loops for the console.
//! Results equal the scalar code bit for bit. Xenon's VMX has no integer
//! multiply, so products are formed with the float multiply-add, which is
//! exact here: every operand and result is an integer below 2^24.
//!
//! VMX is enabled per function rather than for the whole target: the
//! console convention preserves only v64-v127 across calls, while LLVM's
//! AltiVec convention keeps values in v20-v31 across calls, and the kernel
//! imports save neither. The kernels therefore make no calls. They use the
//! compiler's SIMD intrinsics, which lower to instructions directly, rather
//! than the `core::arch` wrappers, which are functions that a function
//! with a target feature does not inline when optimizing for size.

use core::arch::powerpc::vector_float;
use core::arch::powerpc::vector_unsigned_char;
use core::arch::powerpc::vector_unsigned_int;
use core::intrinsics::simd::simd_add;
use core::intrinsics::simd::simd_cast;
use core::intrinsics::simd::simd_fma;
use core::intrinsics::simd::simd_saturating_add;
use core::intrinsics::simd::simd_shr;
use core::intrinsics::simd::simd_shuffle;
use core::intrinsics::simd::simd_sub;
use core::mem::transmute;

use crate::draw::Rgba8;

use super::blend::CHANNEL_BITS;

/// Pixels per vector.
pub(super) const LANES: usize = 4;

/// Rounded `channel * factor / 255` for the four bytes of `channels` and
/// `factors` selected by `part`, as `scale_lanes`: `p = channel * factor +
/// 128`, then `(p + (p >> 8)) >> 8`. A macro: `simd_shuffle` needs constant
/// indices.
macro_rules! scale {
    ($channels:expr, $factors:expr, $part:expr, $zero:expr, $rounding:expr, $eight:expr) => {{
        let channel: vector_unsigned_int =
            transmute::<vector_unsigned_char, _>(simd_shuffle($channels, $zero, $part));
        let factor: vector_unsigned_int =
            transmute::<vector_unsigned_char, _>(simd_shuffle($factors, $zero, $part));
        let product: vector_float = simd_fma(simd_cast(channel), simd_cast(factor), $rounding);
        let rounded: vector_unsigned_int = simd_cast(product);
        let scaled: vector_unsigned_int =
            simd_shr(simd_add(rounded, simd_shr(rounded, $eight)), $eight);
        transmute::<vector_unsigned_int, vector_unsigned_char>(scaled)
    }};
}

/// Bytes per vector.
const VECTOR_BYTES: usize = 16;
/// Bytes per pixel and per 32-bit word.
const WORD_BYTES: usize = VECTOR_BYTES / LANES;

/// Byte indices for `simd_shuffle` of two vectors: below [`VECTOR_BYTES`]
/// they select from the first, from there on from the second.
#[repr(simd)]
#[derive(Clone, Copy)]
// The length must be a literal for `repr(simd)` layout; it is
// `VECTOR_BYTES`, which the constructors below check through their types.
struct Indices([u32; 16]);

/// Bytes `4 * part..4 * part + 4` of the first vector, each zero-extended
/// to a big-endian word, taking the zero bytes from a zero second vector.
const fn widen(part: usize) -> [u32; VECTOR_BYTES] {
    let mut indices = [VECTOR_BYTES as u32; VECTOR_BYTES];
    let mut word = 0;
    while word < LANES {
        indices[word * WORD_BYTES + WORD_BYTES - 1] = (part * LANES + word) as u32;
        word += 1;
    }
    indices
}

/// `dst = src + dst * (255 - src.a) / 255` per channel with the rounding of
/// the scalar `over`. `dst` and `src` start 16-byte aligned and hold a
/// multiple of [`LANES`] pixels.
#[target_feature(enable = "altivec")]
pub(super) fn normal(dst: &mut [Rgba8], src: &[Rgba8]) {
    /// Each pixel's alpha replicated to its four bytes. `Rgba8` keeps alpha in
    /// the most significant byte, the first in memory on the big-endian
    /// console.
    const ALPHA: Indices = Indices(ALPHA_BYTES);
    const ALPHA_BYTES: [u32; VECTOR_BYTES] = {
        let mut indices = [0; VECTOR_BYTES];
        let mut byte = 0;
        while byte < VECTOR_BYTES {
            indices[byte] = (byte - byte % WORD_BYTES) as u32;
            byte += 1;
        }
        indices
    };

    const WIDEN_0: Indices = Indices(widen(0));
    const WIDEN_1: Indices = Indices(widen(1));
    const WIDEN_2: Indices = Indices(widen(2));
    const WIDEN_3: Indices = Indices(widen(3));

    /// The low byte of each word of two vectors, in order, then repeated.
    const LOW_BYTES: Indices = Indices(LOW_BYTE_INDICES);
    const LOW_BYTE_INDICES: [u32; VECTOR_BYTES] = {
        let mut indices = [0; VECTOR_BYTES];
        let mut byte = 0;
        while byte < VECTOR_BYTES {
            let word = byte % (2 * LANES);
            indices[byte] = (word * WORD_BYTES + WORD_BYTES - 1) as u32;
            byte += 1;
        }
        indices
    };

    /// The first half of each of two vectors.
    const HALVES: Indices = Indices(HALF_INDICES);
    const HALF_INDICES: [u32; VECTOR_BYTES] = {
        let mut indices = [0; VECTOR_BYTES];
        let mut byte = 0;
        while byte < VECTOR_BYTES {
            let half = VECTOR_BYTES / 2;
            indices[byte] = if byte < half {
                byte
            } else {
                byte - half + VECTOR_BYTES
            } as u32;
            byte += 1;
        }
        indices
    };

    debug_assert!(dst.len().is_multiple_of(LANES) && src.len() >= dst.len());
    debug_assert!(
        dst.as_ptr().addr().is_multiple_of(VECTOR_BYTES)
            && src.as_ptr().addr().is_multiple_of(VECTOR_BYTES)
    );

    // SAFETY: both slices are 16-byte aligned and every vector access lies
    // within them; the intrinsics get vectors of the types they require.
    unsafe {
        // Constants rather than locals so that they load from the constant
        // pool instead of being assembled on the stack.
        const ZERO: vector_unsigned_char = unsafe { transmute([0u8; VECTOR_BYTES]) };
        const ALL_ONES: vector_unsigned_char = unsafe { transmute([u8::MAX; VECTOR_BYTES]) };
        // Half of the 256 the rounded product is divided by.
        const ROUNDING: vector_float =
            unsafe { transmute([(1u32 << (CHANNEL_BITS - 1)) as f32; LANES]) };
        const EIGHT: vector_unsigned_int = unsafe { transmute([CHANNEL_BITS; LANES]) };
        let (zero, all_ones, rounding, eight) = (ZERO, ALL_ONES, ROUNDING, EIGHT);

        let mut index = 0;
        while index < dst.len() {
            // Dereferences rather than `read` and `write`, which are
            // functions that would be called.
            let target_pointer = dst.as_mut_ptr().add(index).cast::<vector_unsigned_char>();
            let source = *src.as_ptr().add(index).cast::<vector_unsigned_char>();
            let target = *target_pointer;

            let factor = simd_sub(
                all_ones,
                simd_shuffle::<_, _, vector_unsigned_char>(source, source, ALPHA),
            );
            let parts = [
                scale!(target, factor, WIDEN_0, zero, rounding, eight),
                scale!(target, factor, WIDEN_1, zero, rounding, eight),
                scale!(target, factor, WIDEN_2, zero, rounding, eight),
                scale!(target, factor, WIDEN_3, zero, rounding, eight),
            ];
            let low: vector_unsigned_char = simd_shuffle(parts[0], parts[1], LOW_BYTES);
            let high: vector_unsigned_char = simd_shuffle(parts[2], parts[3], LOW_BYTES);
            let narrowed: vector_unsigned_char = simd_shuffle(low, high, HALVES);

            *target_pointer = simd_saturating_add(narrowed, source);
            index += LANES;
        }
    }
}
