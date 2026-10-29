//! Xenon VMX128 vector operations.
//!
//! The PPC64 toolchain (`powerpc64-unknown-xbox360`) compiles with
//! `+altivec,+vmx128`. LLVM then allocates all 128 vector registers and
//! emits the VMX128 forms for ordinary vector code, so `core::simd`
//! arithmetic, comparisons, selects, shuffles and conversions need nothing
//! from this crate. The crate covers what has no portable form: the dot
//! products, the word permute and rotate-insert, the D3D pack and unpack,
//! the left/right loads and stores, and the AltiVec operations whose exact
//! hardware behaviour matters (estimates, saturating conversions with a
//! scale, rounding modes, the bounds compare).
//!
//! Vectors are the 16-byte `core::simd` types. On this target `f32x4` and
//! 16-byte vectors of integers up to 32 bits pass in v1-v13 and return in
//! v1 in `extern "C"` functions, as `__vector4` does with the console
//! compiler.
//!
//! Word 0 is the most significant word, at the lowest address. The
//! semantics given here were checked on the console by `examples/vmx128test`.
//! Immediates are const generic parameters and are checked at compile time.

#![no_std]
#![cfg(target_feature = "vmx128")]
#![feature(abi_unadjusted, link_llvm_intrinsics, portable_simd, simd_ffi)]
#![allow(internal_features, improper_ctypes)]

use core::mem::transmute;
use core::simd::Simd;

pub use core::simd::f32x4;
pub use core::simd::i8x16;
pub use core::simd::i16x8;
pub use core::simd::i32x4;
pub use core::simd::mask32x4;
pub use core::simd::u8x16;
pub use core::simd::u16x8;
pub use core::simd::u32x4;

mod sealed {
    pub trait Sealed {}
}

/// A 16-byte vector type. Every implementor is plain data without padding,
/// so the conversions only reinterpret the 128 bits.
pub trait Vector: Copy + sealed::Sealed {
    fn to_words(self) -> i32x4;
    fn from_words(words: i32x4) -> Self;
}

macro_rules! vector {
    ($($ty:ty),*) => {$(
        impl sealed::Sealed for $ty {}
        impl Vector for $ty {
            #[inline(always)]
            fn to_words(self) -> i32x4 {
                // SAFETY: both types are 16 bytes of plain data.
                unsafe { transmute(self) }
            }
            #[inline(always)]
            fn from_words(words: i32x4) -> Self {
                // SAFETY: both types are 16 bytes of plain data.
                unsafe { transmute(words) }
            }
        }
    )*};
}

vector!(f32x4, i32x4, u32x4, i16x8, u16x8, i8x16, u8x16);

/// Bit-casts one vector type to another.
#[inline(always)]
pub fn cast<T: Vector, U: Vector>(value: T) -> U {
    U::from_words(value.to_words())
}

unsafe extern "unadjusted" {
    #[link_name = "llvm.ppc.vmx128.vmsum3fp"]
    fn vmsum3fp(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.vmx128.vmsum4fp"]
    fn vmsum4fp(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.vmx128.vpermwi"]
    fn vpermwi(b: i32x4, p: i32) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.vrlimi"]
    fn vrlimi(d: i32x4, b: i32x4, mask: i32, rot: i32) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.vpkd3d"]
    fn vpkd3d(d: i32x4, b: f32x4, kind: i32, mask: i32, shift: i32) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.vupkd3d"]
    fn vupkd3d(b: i32x4, kind: i32) -> f32x4;
    #[link_name = "llvm.ppc.vmx128.lvlx"]
    fn lvlx(p: *const u8) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.lvrx"]
    fn lvrx(p: *const u8) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.lvlxl"]
    fn lvlxl(p: *const u8) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.lvrxl"]
    fn lvrxl(p: *const u8) -> i32x4;
    #[link_name = "llvm.ppc.vmx128.stvlx"]
    fn stvlx(v: i32x4, p: *mut u8);
    #[link_name = "llvm.ppc.vmx128.stvrx"]
    fn stvrx(v: i32x4, p: *mut u8);
    #[link_name = "llvm.ppc.vmx128.stvlxl"]
    fn stvlxl(v: i32x4, p: *mut u8);
    #[link_name = "llvm.ppc.vmx128.stvrxl"]
    fn stvrxl(v: i32x4, p: *mut u8);

    #[link_name = "llvm.ppc.altivec.vmaddfp"]
    fn vmaddfp(a: f32x4, b: f32x4, c: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vnmsubfp"]
    fn vnmsubfp(a: f32x4, b: f32x4, c: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vmaxfp"]
    fn vmaxfp(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vminfp"]
    fn vminfp(a: f32x4, b: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrefp"]
    fn vrefp(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrsqrtefp"]
    fn vrsqrtefp(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vexptefp"]
    fn vexptefp(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vlogefp"]
    fn vlogefp(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrfim"]
    fn vrfim(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrfip"]
    fn vrfip(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrfiz"]
    fn vrfiz(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vrfin"]
    fn vrfin(a: f32x4) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vctsxs"]
    fn vctsxs(a: f32x4, scale: i32) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vctuxs"]
    fn vctuxs(a: f32x4, scale: i32) -> u32x4;
    #[link_name = "llvm.ppc.altivec.vcfsx"]
    fn vcfsx(a: i32x4, scale: i32) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vcfux"]
    fn vcfux(a: u32x4, scale: i32) -> f32x4;
    #[link_name = "llvm.ppc.altivec.vcmpeqfp"]
    fn vcmpeqfp(a: f32x4, b: f32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vcmpgefp"]
    fn vcmpgefp(a: f32x4, b: f32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vcmpgtfp"]
    fn vcmpgtfp(a: f32x4, b: f32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vcmpbfp"]
    fn vcmpbfp(a: f32x4, b: f32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vsel"]
    fn vsel(a: i32x4, b: i32x4, mask: i32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vslw"]
    fn vslw(a: i32x4, b: i32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vsrw"]
    fn vsrw(a: i32x4, b: i32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vsraw"]
    fn vsraw(a: i32x4, b: i32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vrlw"]
    fn vrlw(a: i32x4, b: i32x4) -> i32x4;
    #[link_name = "llvm.ppc.altivec.vperm"]
    fn vperm(a: i32x4, b: i32x4, control: u8x16) -> i32x4;
}

/// Three-component dot product (`vmsum3fp128`): every word is
/// `a0*b0 + a1*b1 + a2*b2`; word 3 of the inputs is ignored.
///
/// Not IEEE and not exactly rounded. Non-negative integer-valued inputs
/// give the exact sum, but [`dot4`] of `(1, 1, 1, 1)` and `(1, -1, 1, -1)`
/// gives -2^-28 rather than 0, and over 65536 random inputs the result was
/// up to 3.96 units in the last place of the largest product away from the
/// exact sum.
#[inline(always)]
pub fn dot3(a: f32x4, b: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vmsum3fp(a, b) }
}

/// Four-component dot product (`vmsum4fp128`): every word is the sum of
/// the four products. Rounding as for [`dot3`].
#[inline(always)]
pub fn dot4(a: f32x4, b: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vmsum4fp(a, b) }
}

/// Word permute (`vpermwi128`): result word `i` is word
/// `(P >> (6 - 2 * i)) & 3` of `value`. `P = 0x1B` is the identity,
/// `0xE4` reverses the words.
#[inline(always)]
pub fn permute_words<const P: u32, T: Vector>(value: T) -> T {
    const { assert!(P < 256, "the permute control has 8 bits") };
    // SAFETY: a pure register operation; the immediate is in range.
    T::from_words(unsafe { vpermwi(value.to_words(), P as i32) })
}

/// Rotate and insert words (`vrlimi128`): `t` is `value` rotated left by
/// `ROTATE` words (`t[i] = value[(i + ROTATE) & 3]`); result word `i` is
/// `t[i]` where bit `8 >> i` of `MASK` is set and `into[i]` elsewhere.
#[inline(always)]
pub fn rotate_insert<const MASK: u32, const ROTATE: u32, T: Vector>(into: T, value: T) -> T {
    const { assert!(MASK < 16 && ROTATE < 4, "mask has 4 bits, rotate 2") };
    // SAFETY: a pure register operation; the immediates are in range.
    T::from_words(unsafe {
        vrlimi(into.to_words(), value.to_words(), MASK as i32, ROTATE as i32)
    })
}

/// D3D vertex data formats of [`pack_d3d`] and [`unpack_d3d`].
///
/// Packing takes every integer field, signed or unsigned, as
/// `3.0 + x * 2^-22` (bit pattern `0x40400000 + x`) and saturates it to the
/// field: `0..=2^w - 1` for unsigned fields and `-(2^(w-1) - 1)..=2^(w-1) - 1`
/// for signed ones. Unpacking gives unsigned fields as `1.0 + x * 2^-23`
/// (`0x3F800000 | x`) and signed fields as `3.0 + x * 2^-22`; the most
/// negative signed value unpacks to a NaN. Half floats have no infinity or
/// NaN (largest value 131008); packing truncates toward zero and saturates.
pub mod d3d {
    /// `W[31:24] X[23:16] Y[15:8] Z[7:0]`, four unsigned bytes.
    pub const COLOR: u32 = 0;
    /// `X[31:16] Y[15:0]`, two signed halfwords. Unpacks z = 0.0, w = 1.0.
    pub const NORMSHORT2: u32 = 1;
    /// `W[31:30] Z[29:20] Y[19:10] X[9:0]`: unsigned 2 bits, three signed 10 bits.
    pub const NORMPACKED32: u32 = 2;
    /// `X[31:16] Y[15:0]`, two half floats. Unpacks z = 0.0, w = 1.0.
    pub const FLOAT16_2: u32 = 3;
    /// `X[63:48] Y[47:32] Z[31:16] W[15:0]`, four signed halfwords.
    pub const NORMSHORT4: u32 = 4;
    /// `X[63:48] Y[47:32] Z[31:16] W[15:0]`, four half floats.
    pub const FLOAT16_4: u32 = 5;
    /// `W[63:60] Z[59:40] Y[39:20] X[19:0]`: unsigned 4 bits, three signed 20 bits.
    pub const NORMPACKED64: u32 = 6;
}

/// D3D pack (`vpkd3d128`). `p` is `value` packed as `KIND` (one of the
/// [`d3d`] constants): a 32-bit result in word 3 or a 64-bit result in
/// words 2-3. `t` is `p` rotated left by `SHIFT` words. The result is
/// `into` with these words replaced by the words of `t`:
///
/// | `MASK` | `SHIFT` 0 | 1 | 2 | 3 |
/// | --- | --- | --- | --- | --- |
/// | 1 (32-bit) | w3 | w2 | w1 | w0 |
/// | 2 (64-bit) | w2, w3 | w1, w2 | w0, w1 | w0 |
/// | 3 (64-bit) | w2, w3 | w1, w2 | w0, w1 | w3 |
#[inline(always)]
pub fn pack_d3d<const KIND: u32, const MASK: u32, const SHIFT: u32>(
    into: u32x4,
    value: f32x4,
) -> u32x4 {
    const { assert!(KIND <= 6 && MASK >= 1 && MASK <= 3 && SHIFT < 4, "immediate out of range") };
    // SAFETY: a pure register operation; the immediates are in range.
    cast(unsafe { vpkd3d(cast(into), value, KIND as i32, MASK as i32, SHIFT as i32) })
}

/// D3D unpack (`vupkd3d128`) of word 3 (32-bit kinds) or words 2-3 (64-bit
/// kinds) of `value` into x, y, z, w; see [`d3d`] for the encodings.
#[inline(always)]
pub fn unpack_d3d<const KIND: u32>(value: u32x4) -> f32x4 {
    const { assert!(KIND <= 6, "kind out of range") };
    // SAFETY: a pure register operation; the immediate is in range.
    unsafe { vupkd3d(cast(value), KIND as i32) }
}

/// Load left (`lvlx128`): with `n = p & 15`, bytes `0..16 - n` of the result
/// are the memory at `p..(p | 15)`, the rest are zero.
///
/// # Safety
///
/// `p..(p | 15) + 1` must be readable.
#[inline(always)]
pub unsafe fn load_left<T: Vector>(p: *const u8) -> T {
    // SAFETY: the caller guarantees the bytes are readable.
    T::from_words(unsafe { lvlx(p) })
}

/// Load right (`lvrx128`): with `n = p & 15`, bytes `16 - n..16` of the
/// result are the memory at `(p & !15)..p`, the rest are zero; nothing is
/// loaded when `p` is 16-byte aligned.
///
/// # Safety
///
/// `(p & !15)..p` must be readable.
#[inline(always)]
pub unsafe fn load_right<T: Vector>(p: *const u8) -> T {
    // SAFETY: the caller guarantees the bytes are readable.
    T::from_words(unsafe { lvrx(p) })
}

/// [`load_left`] with the least-recently-used cache hint (`lvlxl128`).
///
/// # Safety
///
/// As for [`load_left`].
#[inline(always)]
pub unsafe fn load_left_lru<T: Vector>(p: *const u8) -> T {
    // SAFETY: the caller guarantees the bytes are readable.
    T::from_words(unsafe { lvlxl(p) })
}

/// [`load_right`] with the least-recently-used cache hint (`lvrxl128`).
///
/// # Safety
///
/// As for [`load_right`].
#[inline(always)]
pub unsafe fn load_right_lru<T: Vector>(p: *const u8) -> T {
    // SAFETY: the caller guarantees the bytes are readable.
    T::from_words(unsafe { lvrxl(p) })
}

/// Store left (`stvlx128`): stores bytes `0..16 - n` of `value` to
/// `p..(p | 15)`, with `n = p & 15`.
///
/// # Safety
///
/// `p..(p | 15) + 1` must be writable.
#[inline(always)]
pub unsafe fn store_left<T: Vector>(value: T, p: *mut u8) {
    // SAFETY: the caller guarantees the bytes are writable.
    unsafe { stvlx(value.to_words(), p) }
}

/// Store right (`stvrx128`): stores bytes `16 - n..16` of `value` to
/// `(p & !15)..p`, with `n = p & 15`; nothing when `p` is 16-byte aligned.
///
/// # Safety
///
/// `(p & !15)..p` must be writable.
#[inline(always)]
pub unsafe fn store_right<T: Vector>(value: T, p: *mut u8) {
    // SAFETY: the caller guarantees the bytes are writable.
    unsafe { stvrx(value.to_words(), p) }
}

/// [`store_left`] with the least-recently-used cache hint (`stvlxl128`).
///
/// # Safety
///
/// As for [`store_left`].
#[inline(always)]
pub unsafe fn store_left_lru<T: Vector>(value: T, p: *mut u8) {
    // SAFETY: the caller guarantees the bytes are writable.
    unsafe { stvlxl(value.to_words(), p) }
}

/// [`store_right`] with the least-recently-used cache hint (`stvrxl128`).
///
/// # Safety
///
/// As for [`store_right`].
#[inline(always)]
pub unsafe fn store_right_lru<T: Vector>(value: T, p: *mut u8) {
    // SAFETY: the caller guarantees the bytes are writable.
    unsafe { stvrxl(value.to_words(), p) }
}

/// Loads 16 bytes from any address with [`load_left`] and [`load_right`].
///
/// # Safety
///
/// `p..p + 16` must be readable.
#[inline(always)]
pub unsafe fn load_unaligned<T: Vector>(p: *const u8) -> T {
    // SAFETY: the two loads read exactly `p..p + 16`.
    let (left, right) = unsafe { (lvlx(p), lvrx(p.wrapping_add(16))) };
    T::from_words(left | right)
}

/// Stores 16 bytes to any address with [`store_left`] and [`store_right`].
///
/// # Safety
///
/// `p..p + 16` must be writable.
#[inline(always)]
pub unsafe fn store_unaligned<T: Vector>(value: T, p: *mut u8) {
    let words = value.to_words();
    // SAFETY: the two stores write exactly `p..p + 16`.
    unsafe {
        stvlx(words, p);
        stvrx(words, p.wrapping_add(16));
    }
}

/// Fused multiply-add `a * b + c` with one rounding (`vmaddfp128`).
#[inline(always)]
pub fn mul_add(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vmaddfp(a, b, c) }
}

/// Fused negative multiply-subtract `-(a * b - c)` (`vnmsubfp128`).
#[inline(always)]
pub fn neg_mul_sub(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vnmsubfp(a, b, c) }
}

/// Word-wise maximum (`vmaxfp128`).
#[inline(always)]
pub fn max(a: f32x4, b: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vmaxfp(a, b) }
}

/// Word-wise minimum (`vminfp128`).
#[inline(always)]
pub fn min(a: f32x4, b: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vminfp(a, b) }
}

/// Reciprocal estimate (`vrefp128`). `examples/vmx128test` measures the
/// relative error.
#[inline(always)]
pub fn recip_estimate(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrefp(a) }
}

/// Reciprocal square root estimate (`vrsqrtefp128`).
#[inline(always)]
pub fn rsqrt_estimate(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrsqrtefp(a) }
}

/// Base-2 exponential estimate (`vexptefp128`).
#[inline(always)]
pub fn exp2_estimate(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vexptefp(a) }
}

/// Base-2 logarithm estimate (`vlogefp128`).
#[inline(always)]
pub fn log2_estimate(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vlogefp(a) }
}

/// Rounds toward negative infinity (`vrfim128`).
#[inline(always)]
pub fn floor(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrfim(a) }
}

/// Rounds toward positive infinity (`vrfip128`).
#[inline(always)]
pub fn ceil(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrfip(a) }
}

/// Rounds toward zero (`vrfiz128`).
#[inline(always)]
pub fn trunc(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrfiz(a) }
}

/// Rounds to the nearest integer, ties to even (`vrfin128`).
#[inline(always)]
pub fn round_even(a: f32x4) -> f32x4 {
    // SAFETY: a pure register operation.
    unsafe { vrfin(a) }
}

/// `a * 2^SCALE` truncated toward zero and saturated to `i32`; NaN gives 0
/// (`vcfpsxws128`).
#[inline(always)]
pub fn to_i32_saturating<const SCALE: u32>(a: f32x4) -> i32x4 {
    const { assert!(SCALE < 32, "scale has 5 bits") };
    // SAFETY: a pure register operation; the immediate is in range.
    unsafe { vctsxs(a, SCALE as i32) }
}

/// `a * 2^SCALE` truncated toward zero and saturated to `u32`; NaN gives 0
/// (`vcfpuxws128`).
#[inline(always)]
pub fn to_u32_saturating<const SCALE: u32>(a: f32x4) -> u32x4 {
    const { assert!(SCALE < 32, "scale has 5 bits") };
    // SAFETY: a pure register operation; the immediate is in range.
    unsafe { vctuxs(a, SCALE as i32) }
}

/// `a / 2^SCALE` rounded to nearest (`vcsxwfp128`).
#[inline(always)]
pub fn from_i32<const SCALE: u32>(a: i32x4) -> f32x4 {
    const { assert!(SCALE < 32, "scale has 5 bits") };
    // SAFETY: a pure register operation; the immediate is in range.
    unsafe { vcfsx(a, SCALE as i32) }
}

/// `a / 2^SCALE` rounded to nearest (`vcuxwfp128`).
#[inline(always)]
pub fn from_u32<const SCALE: u32>(a: u32x4) -> f32x4 {
    const { assert!(SCALE < 32, "scale has 5 bits") };
    // SAFETY: a pure register operation; the immediate is in range.
    unsafe { vcfux(a, SCALE as i32) }
}

#[inline(always)]
fn mask(words: i32x4) -> mask32x4 {
    // SAFETY: vector compares give 0 or -1 in every word.
    unsafe { mask32x4::from_simd_unchecked(words) }
}

/// `a == b` per word (`vcmpeqfp128`); false for NaN.
#[inline(always)]
pub fn eq(a: f32x4, b: f32x4) -> mask32x4 {
    // SAFETY: a pure register operation.
    mask(unsafe { vcmpeqfp(a, b) })
}

/// `a >= b` per word (`vcmpgefp128`); false for NaN.
#[inline(always)]
pub fn ge(a: f32x4, b: f32x4) -> mask32x4 {
    // SAFETY: a pure register operation.
    mask(unsafe { vcmpgefp(a, b) })
}

/// `a > b` per word (`vcmpgtfp128`); false for NaN.
#[inline(always)]
pub fn gt(a: f32x4, b: f32x4) -> mask32x4 {
    // SAFETY: a pure register operation.
    mask(unsafe { vcmpgtfp(a, b) })
}

/// Bounds compare (`vcmpbfp128`): per word, bit 31 is set when
/// `a > bound` (not in bounds above) and bit 30 when `a < -bound`; other
/// bits are zero. A NaN in either input sets both bits.
#[inline(always)]
pub fn bounds(a: f32x4, bound: f32x4) -> u32x4 {
    // SAFETY: a pure register operation.
    cast(unsafe { vcmpbfp(a, bound) })
}

/// Bitwise select (`vsel128`): bits of `if_set` where `mask` is 1, of
/// `if_clear` where it is 0.
#[inline(always)]
pub fn select_bits<T: Vector>(mask: u32x4, if_set: T, if_clear: T) -> T {
    // SAFETY: a pure register operation.
    T::from_words(unsafe { vsel(if_clear.to_words(), if_set.to_words(), cast(mask)) })
}

/// Shifts every word left by the low 5 bits of the matching word of
/// `count` (`vslw128`).
#[inline(always)]
pub fn shl_words(a: u32x4, count: u32x4) -> u32x4 {
    // SAFETY: a pure register operation.
    cast(unsafe { vslw(cast(a), cast(count)) })
}

/// Logical right shift of every word by the low 5 bits of `count`
/// (`vsrw128`).
#[inline(always)]
pub fn shr_words(a: u32x4, count: u32x4) -> u32x4 {
    // SAFETY: a pure register operation.
    cast(unsafe { vsrw(cast(a), cast(count)) })
}

/// Arithmetic right shift of every word by the low 5 bits of `count`
/// (`vsraw128`).
#[inline(always)]
pub fn sra_words(a: i32x4, count: u32x4) -> i32x4 {
    // SAFETY: a pure register operation.
    unsafe { vsraw(a, cast(count)) }
}

/// Rotates every word left by the low 5 bits of `count` (`vrlw128`).
#[inline(always)]
pub fn rotate_words(a: u32x4, count: u32x4) -> u32x4 {
    // SAFETY: a pure register operation.
    cast(unsafe { vrlw(cast(a), cast(count)) })
}

/// Byte permute (`vperm128`): result byte `i` is byte `control[i] & 31` of
/// the 32-byte concatenation `a:b`.
#[inline(always)]
pub fn permute_bytes<T: Vector>(a: T, b: T, control: u8x16) -> T {
    // SAFETY: a pure register operation.
    T::from_words(unsafe { vperm(a.to_words(), b.to_words(), control) })
}

/// Splats word `I` of `value` (`vspltw128`).
#[inline(always)]
pub fn splat_word<const I: u32, T: Vector>(value: T) -> T {
    const { assert!(I < 4, "word index out of range") };
    let words = value.to_words();
    T::from_words(Simd::splat(words[I as usize]))
}

/// Interleaves words 0-1 of `a` and `b`: `(a0, b0, a1, b1)` (`vmrghw128`).
#[inline(always)]
pub fn merge_high<T: Vector>(a: T, b: T) -> T {
    T::from_words(core::simd::simd_swizzle!(a.to_words(), b.to_words(), [0, 4, 1, 5]))
}

/// Interleaves words 2-3 of `a` and `b`: `(a2, b2, a3, b3)` (`vmrglw128`).
#[inline(always)]
pub fn merge_low<T: Vector>(a: T, b: T) -> T {
    T::from_words(core::simd::simd_swizzle!(a.to_words(), b.to_words(), [2, 6, 3, 7]))
}

/// Bytes `N..N + 16` of the 32-byte concatenation `a:b` (`vsldoi128`).
#[inline(always)]
pub fn shift_left_octets<const N: u32, T: Vector>(a: T, b: T) -> T {
    let control = const {
        assert!(N < 16, "shift has 4 bits");
        let mut control = [0u8; 16];
        let mut i = 0;
        while i < 16 {
            control[i] = (N as usize + i) as u8;
            i += 1;
        }
        control
    };
    permute_bytes(a, b, u8x16::from_array(control))
}
