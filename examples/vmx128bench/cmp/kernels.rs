//! Rust versions of the kernels in kernels.c (`k_*`, plain loops for the
//! auto-vectorizer) and speed.c (`sp_*_s` plain loops, `sp_*_v` portable
//! SIMD). `PFX_` is replaced by the variant prefix before compiling (see
//! make.sh). Kernels marked `_r` in C take Rust slices, which gives LLVM the
//! same no-alias facts as `restrict`; the others use raw pointers.
//!
//! The crate is compiled to assembly on its own and included in the title,
//! so nothing here may panic or refer to other Rust crates.

#![no_std]
#![feature(portable_simd, link_llvm_intrinsics, simd_ffi)]
#![allow(internal_features, improper_ctypes)]
#![allow(non_snake_case, non_upper_case_globals, clippy::missing_safety_doc)]

use core::simd::Simd;
use core::simd::f32x4;
use core::simd::num::SimdFloat;
use core::simd::num::SimdUint;
use core::simd::cmp::SimdPartialOrd;
use core::simd::Select;
use core::simd::simd_swizzle;
use core::simd::u8x16;
use core::simd::u32x4;
use core::slice::from_raw_parts as sl;
use core::slice::from_raw_parts_mut as slm;

type V4 = f32x4;

#[inline(always)]
fn len(n: i32) -> usize {
    n.max(0) as usize
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_saxpy(y: *mut f32, x: *const f32, a: f32, n: i32) {
    for i in 0..len(n) {
        unsafe { *y.add(i) = a * *x.add(i) + *y.add(i) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_saxpy_r(y: *mut f32, x: *const f32, a: f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n)), sl(x, len(n))) };
    for (y, x) in y.iter_mut().zip(x) {
        *y = a * *x + *y;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_scale_inplace(y: *mut f32, a: f32, n: i32) {
    for v in unsafe { slm(y, len(n)) } {
        *v *= a;
    }
}

macro_rules! binop_r {
    ($name:ident, $t:ty, |$a:ident, $b:ident| $e:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(c: *mut $t, a: *const $t, b: *const $t, n: i32) {
            let (c, a, b) = unsafe { (slm(c, len(n)), sl(a, len(n)), sl(b, len(n))) };
            for ((c, &$a), &$b) in c.iter_mut().zip(a).zip(b) {
                *c = $e;
            }
        }
    };
}

binop_r!(PFX_k_add_f32_r, f32, |a, b| a + b);
binop_r!(PFX_k_mul_f32_r, f32, |a, b| a * b);


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_sum_f32(x: *const f32, n: i32) -> f32 {
    let mut s = 0.0f32;
    for &v in unsafe { sl(x, len(n)) } {
        s += v;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_dot_f32(a: *const f32, b: *const f32, n: i32) -> f32 {
    let mut s = 0.0f32;
    for (&a, &b) in unsafe { sl(a, len(n)).iter().zip(sl(b, len(n))) } {
        s += a * b;
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_sum_i32(x: *const i32, n: i32) -> i32 {
    let mut s = 0i32;
    for &v in unsafe { sl(x, len(n)) } {
        s = s.wrapping_add(v);
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_dot_i16(a: *const i16, b: *const i16, n: i32) -> i32 {
    let mut s = 0i32;
    for (&a, &b) in unsafe { sl(a, len(n)).iter().zip(sl(b, len(n))) } {
        s = s.wrapping_add(i32::from(a) * i32::from(b));
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_max_f32(x: *const f32, n: i32) -> f32 {
    let x = unsafe { sl(x, len(n)) };
    let mut m = x[0];
    for &v in &x[1..] {
        m = if v > m { v } else { m };
    }
    m
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_add_i32(c: *mut i32, a: *const i32, b: *const i32, n: i32) {
    for i in 0..len(n) {
        unsafe { *c.add(i) = (*a.add(i)).wrapping_add(*b.add(i)) };
    }
}

binop_r!(PFX_k_add_i32_r, i32, |a, b| a.wrapping_add(b));
binop_r!(PFX_k_add_i16_r, i16, |a, b| a.wrapping_add(b));
binop_r!(PFX_k_add_u8_r, u8, |a, b| a.wrapping_add(b));
binop_r!(PFX_k_avg_u8_r, u8, |a, b| ((u32::from(a) + u32::from(b) + 1) >> 1) as u8);
binop_r!(PFX_k_addsat_u8_r, u8, |a, b| a.saturating_add(b));
binop_r!(PFX_k_and_u32_r, u32, |a, b| a & b);
binop_r!(PFX_k_mul_i32_r, i32, |a, b| a.wrapping_mul(b));
binop_r!(PFX_k_min_f32_r, f32, |a, b| if a < b { a } else { b });
binop_r!(PFX_k_max_i32_r, i32, |a, b| if a > b { a } else { b });

macro_rules! unop_r {
    ($name:ident, $t:ty, $u:ty, |$x:ident| $e:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name(y: *mut $u, x: *const $t, n: i32) {
            let (y, x) = unsafe { (slm(y, len(n)), sl(x, len(n))) };
            for (y, &$x) in y.iter_mut().zip(x) {
                *y = $e;
            }
        }
    };
}

unop_r!(PFX_k_shl_i32_r, i32, i32, |v| v << 3);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_clamp_f32_r(y: *mut f32, x: *const f32, lo: f32, hi: f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n)), sl(x, len(n))) };
    for (y, &v) in y.iter_mut().zip(x) {
        let v = if v < lo { lo } else { v };
        *y = if v > hi { hi } else { v };
    }
}

unop_r!(PFX_k_clamp_i16_r, i16, i16, |v| v.clamp(-1000, 1000));
unop_r!(PFX_k_abs_f32_r, f32, f32, |v| if v < 0.0 { -v } else { v });
unop_r!(PFX_k_abs_i32_r, i32, i32, |v| v.wrapping_abs());


unop_r!(PFX_k_f2i_r, f32, i32, |v| v as i32);
unop_r!(PFX_k_i2f_r, i32, f32, |v| v as f32);
unop_r!(PFX_k_u8_to_f32_r, u8, f32, |v| f32::from(v) * (1.0 / 255.0));


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_mat4_vec4_r(out: *mut [f32; 4], m: *const f32, inp: *const [f32; 4], n: i32) {
    let (out, inp, m) = unsafe { (slm(out, len(n)), sl(inp, len(n)), &*(m as *const [f32; 16])) };
    for (o, &[x, y, z, w]) in out.iter_mut().zip(inp) {
        *o = [
            m[0] * x + m[4] * y + m[8] * z + m[12] * w,
            m[1] * x + m[5] * y + m[9] * z + m[13] * w,
            m[2] * x + m[6] * y + m[10] * z + m[14] * w,
            m[3] * x + m[7] * y + m[11] * z + m[15] * w,
        ];
    }
}


unop_r!(PFX_k_copy_u32_r, u32, u32, |v| v);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_copy_u8(d: *mut u8, s: *const u8, n: i32) {
    for i in 0..len(n) {
        unsafe { *d.add(i) = *s.add(i) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_set_u32(d: *mut u32, v: u32, n: i32) {
    for i in 0..len(n) {
        unsafe { *d.add(i) = v };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_set_f32(d: *mut f32, v: f32, n: i32) {
    for i in 0..len(n) {
        unsafe { *d.add(i) = v };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_zero_u8(d: *mut u8, n: i32) {
    for i in 0..len(n) {
        unsafe { *d.add(i) = 0 };
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_stride2_f32_r(y: *mut f32, x: *const f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n)), sl(x, 2 * len(n))) };
    for (y, p) in y.iter_mut().zip(x.chunks_exact(2)) {
        *y = p[0] + p[1];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_gather_f32_r(y: *mut f32, x: *const f32, idx: *const i32, n: i32) {
    let (y, idx) = unsafe { (slm(y, len(n)), sl(idx, len(n))) };
    for (y, &i) in y.iter_mut().zip(idx) {
        *y = unsafe { *x.add(i as usize) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_offset1_f32_r(y: *mut f32, x: *const f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n)), sl(x, len(n) + 1)) };
    for (y, p) in y.iter_mut().zip(x.windows(2)) {
        *y = p[0] + p[1];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_fir3_f32_r(y: *mut f32, x: *const f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n)), sl(x, len(n) + 2)) };
    for (y, p) in y.iter_mut().zip(x.windows(3)) {
        *y = 0.25 * p[0] + 0.5 * p[1] + 0.25 * p[2];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_prefix_f32(y: *mut f32, n: i32) {
    for i in 1..len(n) {
        unsafe { *y.add(i) += *y.add(i - 1) };
    }
}


const GN: usize = 1024;

#[repr(C, align(16))]
pub struct G<T>([T; GN]);

#[unsafe(no_mangle)]
pub static mut PFX_g_a: G<f32> = G([0.0; GN]);
#[unsafe(no_mangle)]
pub static mut PFX_g_b: G<f32> = G([0.0; GN]);
#[unsafe(no_mangle)]
pub static mut PFX_g_c: G<f32> = G([0.0; GN]);
#[unsafe(no_mangle)]
pub static mut PFX_g_ia: G<i32> = G([0; GN]);
#[unsafe(no_mangle)]
pub static mut PFX_g_ib: G<i32> = G([0; GN]);
#[unsafe(no_mangle)]
pub static mut PFX_g_ic: G<i32> = G([0; GN]);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_g_add_f32() {
    let (a, b, c) = unsafe { (&(*(&raw const PFX_g_a)).0, &(*(&raw const PFX_g_b)).0, &mut (*(&raw mut PFX_g_c)).0) };
    for i in 0..GN {
        c[i] = a[i] + b[i];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_g_saxpy(s: f32) {
    let (a, b, c) = unsafe { (&(*(&raw const PFX_g_a)).0, &(*(&raw const PFX_g_b)).0, &mut (*(&raw mut PFX_g_c)).0) };
    for i in 0..GN {
        c[i] = s * a[i] + b[i];
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_g_add_i32() {
    let (a, b, c) = unsafe { (&(*(&raw const PFX_g_ia)).0, &(*(&raw const PFX_g_ib)).0, &mut (*(&raw mut PFX_g_ic)).0) };
    for i in 0..GN {
        c[i] = a[i].wrapping_add(b[i]);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_g_dot() -> f32 {
    let (a, b) = unsafe { (&(*(&raw const PFX_g_a)).0, &(*(&raw const PFX_g_b)).0) };
    let mut s = 0.0f32;
    for i in 0..GN {
        s += a[i] * b[i];
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_k_add16_f32_r(c: *mut f32, a: *const f32, b: *const f32) {
    let (c, a, b) = unsafe { (slm(c, 16), sl(a, 16), sl(b, 16)) };
    for i in 0..16 {
        c[i] = a[i] + b[i];
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_saxpy_s(y: *mut f32, x: *const f32, a: f32, n: i32) {
    unsafe { PFX_k_saxpy_r(y, x, a, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_saxpy_v(y: *mut V4, x: *const V4, a: f32, n: i32) {
    let (y, x) = unsafe { (slm(y, len(n) / 4), sl(x, len(n) / 4)) };
    let av = V4::splat(a);
    for (y, &x) in y.iter_mut().zip(x) {
        *y = av * x + *y;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_dot_s(a: *const f32, b: *const f32, n: i32) -> f32 {
    unsafe { PFX_k_dot_f32(a, b, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_dot_v(a: *const V4, b: *const V4, n: i32) -> f32 {
    let (a, b) = unsafe { (sl(a, len(n) / 4), sl(b, len(n) / 4)) };
    let mut s = [V4::splat(0.0); 4];
    for (a, b) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        for j in 0..4 {
            s[j] += a[j] * b[j];
        }
    }
    let s = (s[0] + s[1]) + (s[2] + s[3]);
    (s[0] + s[1]) + (s[2] + s[3])
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_mat4_s(out: *mut f32, m: *const f32, inp: *const f32, n: i32) {
    unsafe { PFX_k_mat4_vec4_r(out.cast(), m, inp.cast(), n) }
}

#[inline(always)]
fn splat4(v: V4) -> [V4; 4] {
    [
        simd_swizzle!(v, [0, 0, 0, 0]),
        simd_swizzle!(v, [1, 1, 1, 1]),
        simd_swizzle!(v, [2, 2, 2, 2]),
        simd_swizzle!(v, [3, 3, 3, 3]),
    ]
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_mat4_v(out: *mut V4, m: *const V4, inp: *const V4, n: i32) {
    let (out, inp, c) = unsafe { (slm(out, len(n)), sl(inp, len(n)), sl(m, 4)) };
    let (c0, c1, c2, c3) = (c[0], c[1], c[2], c[3]);
    for (o, &v) in out.iter_mut().zip(inp) {
        let [x, y, z, w] = splat4(v);
        *o = c0 * x + c1 * y + c2 * z + c3 * w;
    }
}

#[inline(always)]
unsafe fn mat4_block<const U: usize>(out: *mut V4, m: *const V4, inp: *const V4, n: i32) {
    let (out, inp, c) = unsafe { (slm(out, len(n)), sl(inp, len(n)), sl(m, 4)) };
    let (c0, c1, c2, c3) = (c[0], c[1], c[2], c[3]);
    for (o, v) in out.chunks_exact_mut(U).zip(inp.chunks_exact(U)) {
        let mut s = [[V4::splat(0.0); 4]; U];
        for j in 0..U {
            s[j] = splat4(v[j]);
        }
        for j in 0..U {
            o[j] = c0 * s[j][0] + c1 * s[j][1] + c2 * s[j][2] + c3 * s[j][3];
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_mat4_v8(out: *mut V4, m: *const V4, inp: *const V4, n: i32) {
    unsafe { mat4_block::<8>(out, m, inp, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_mat4_v16(out: *mut V4, m: *const V4, inp: *const V4, n: i32) {
    unsafe { mat4_block::<16>(out, m, inp, n) }
}

#[cfg(target_feature = "vmx128")]
unsafe extern "C" {
    #[link_name = "llvm.ppc.vmx128.vmsum4fp"]
    fn vmsum4fp(a: V4, b: V4) -> V4;
    #[link_name = "llvm.ppc.vmx128.vrlimi"]
    fn vrlimi(d: Simd<i32, 4>, b: Simd<i32, 4>, mask: i32, rot: i32) -> Simd<i32, 4>;
}

#[cfg(target_feature = "vmx128")]
#[inline(always)]
unsafe fn insert_word<const MASK: i32>(d: V4, b: V4) -> V4 {
    use core::simd::num::SimdInt;
    V4::from_bits(unsafe { vrlimi(d.to_bits().cast(), b.to_bits().cast(), MASK, 0) }.cast())
}

#[cfg(target_feature = "vmx128")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_mat4_x(out: *mut V4, m: *const V4, inp: *const V4, n: i32) {
    let (out, inp, c) = unsafe { (slm(out, len(n)), sl(inp, len(n)), sl(m, 4)) };
    let t0 = simd_swizzle!(c[0], c[2], [0, 4, 1, 5]);
    let t1 = simd_swizzle!(c[1], c[3], [0, 4, 1, 5]);
    let t2 = simd_swizzle!(c[0], c[2], [2, 6, 3, 7]);
    let t3 = simd_swizzle!(c[1], c[3], [2, 6, 3, 7]);
    let r0 = simd_swizzle!(t0, t1, [0, 4, 1, 5]);
    let r1 = simd_swizzle!(t0, t1, [2, 6, 3, 7]);
    let r2 = simd_swizzle!(t2, t3, [0, 4, 1, 5]);
    let r3 = simd_swizzle!(t2, t3, [2, 6, 3, 7]);
    for (o, &v) in out.iter_mut().zip(inp) {
        unsafe {
            let mut d = vmsum4fp(r0, v);
            d = insert_word::<4>(d, vmsum4fp(r1, v));
            d = insert_word::<2>(d, vmsum4fp(r2, v));
            d = insert_word::<1>(d, vmsum4fp(r3, v));
            *o = d;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_copy_s(d: *mut u32, s: *const u32, n: i32) {
    unsafe { PFX_k_copy_u32_r(d, s, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_copy_v(d: *mut u32x4, s: *const u32x4, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n) / 4), sl(s, len(n) / 4)) };
    for (d, &s) in d.iter_mut().zip(s) {
        *d = s;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_bright_s(d: *mut u8, s: *const u8, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n)), sl(s, len(n))) };
    for (d, &s) in d.iter_mut().zip(s) {
        *d = s.saturating_add(40);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_bright_v(d: *mut u8x16, s: *const u8x16, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n) / 16), sl(s, len(n) / 16)) };
    let k = u8x16::splat(40);
    for (d, &s) in d.iter_mut().zip(s) {
        *d = s.saturating_add(k);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_u8f_s(d: *mut f32, s: *const u8, n: i32) {
    unsafe { PFX_k_u8_to_f32_r(d, s, n) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_u8f_v(d: *mut V4, s: *const u8x16, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n) / 4), sl(s, len(n) / 16)) };
    let k = V4::splat(1.0 / 255.0);
    for (d, &b) in d.chunks_exact_mut(4).zip(s) {
        let w: [u32x4; 4] = [
            simd_swizzle!(b, [0, 1, 2, 3]).cast(),
            simd_swizzle!(b, [4, 5, 6, 7]).cast(),
            simd_swizzle!(b, [8, 9, 10, 11]).cast(),
            simd_swizzle!(b, [12, 13, 14, 15]).cast(),
        ];
        for j in 0..4 {
            d[j] = w[j].cast::<f32>() * k;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_f2u8_s(d: *mut u8, s: *const f32, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n)), sl(s, len(n))) };
    for (d, &v) in d.iter_mut().zip(s) {
        let v = v * 255.0 + 0.5;
        let v = if v < 0.0 { 0.0 } else { v };
        let v = if v > 255.0 { 255.0 } else { v };
        *d = v as u32 as u8;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PFX_sp_f2u8_v(d: *mut u8x16, s: *const V4, n: i32) {
    let (d, s) = unsafe { (slm(d, len(n) / 16), sl(s, len(n) / 4)) };
    let (k, h, lo) = (V4::splat(255.0), V4::splat(0.5), V4::splat(0.0));
    for (d, s) in d.iter_mut().zip(s.chunks_exact(4)) {
        let b: [u8x16; 4] = core::array::from_fn(|j| {
            // Compare and select: simd_max/simd_min (llvm.maximumnum) hit an
            // LLVM assertion on PowerPC AltiVec (FoldSetCC, "Illegal setcc for
            // integer").
            let v = s[j] * k + h;
            let v = v.simd_lt(lo).select(lo, v);
            let v = v.simd_gt(k).select(k, v);
            // SAFETY: same size, every bit pattern valid.
            unsafe { core::mem::transmute::<u32x4, u8x16>(v.cast::<u32>()) }
        });
        let p01: u8x16 = simd_swizzle!(b[0], b[1], [3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0]);
        let p23: u8x16 = simd_swizzle!(b[2], b[3], [3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0]);
        *d = simd_swizzle!(p01, p23, [0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23]);
    }
}
