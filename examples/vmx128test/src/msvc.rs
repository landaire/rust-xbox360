//! The vector calling convention against `msvc/interop.c`, compiled by
//! the console compiler: arguments in v1-v13 and the parameter area,
//! vector returns, mixed scalar arguments, small structs, `char`, calls in
//! both directions, and nonvolatile vectors across calls.

use core::ffi::c_char;
use core::hint::black_box;
use core::simd::prelude::*;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

use crate::regs;
use crate::same;
use crate::Report;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S1([u8; 1]);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S2([u8; 2]);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S3([u8; 3]);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S4([u8; 4]);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S5([u8; 5]);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct S8([u8; 8]);

type Cb16 = extern "C" fn(a0: f32x4, a1: f32x4, a2: f32x4, a3: f32x4, a4: f32x4, a5: f32x4, a6: f32x4, a7: f32x4, a8: f32x4, a9: f32x4, a10: f32x4, a11: f32x4, a12: f32x4, a13: f32x4, a14: f32x4, a15: f32x4) -> f32x4;
type CbMix = extern "C" fn(i32, f32x4, f32, f32x4, i32, f64, f32x4, i32, f32) -> f32x4;

unsafe extern "C" {
    fn cl_sum16(a0: f32x4, a1: f32x4, a2: f32x4, a3: f32x4, a4: f32x4, a5: f32x4, a6: f32x4, a7: f32x4, a8: f32x4, a9: f32x4, a10: f32x4, a11: f32x4, a12: f32x4, a13: f32x4, a14: f32x4, a15: f32x4) -> f32x4;
    fn cl_after13(a0: f32x4, a1: f32x4, a2: f32x4, a3: f32x4, a4: f32x4, a5: f32x4, a6: f32x4, a7: f32x4, a8: f32x4, a9: f32x4, a10: f32x4, a11: f32x4, a12: f32x4, i: i32, a13: f32x4, j: i32) -> f32x4;
    fn cl_mix(i0: i32, a: f32x4, f0: f32, b: f32x4, i1: i32, d1: f64, c: f32x4, i2: i32, f1: f32) -> f32x4;
    fn cl_make(x: f32, y: f32, z: f32, w: f32) -> f32x4;
    fn cl_store(a: f32x4, out: *mut f32x4);
    fn cl_s1_in(k: i32, s: S1, m: i32) -> u32;
    fn cl_s2_in(k: i32, s: S2, m: i32) -> u32;
    fn cl_s3_in(k: i32, s: S3, m: i32) -> u32;
    fn cl_s4_in(k: i32, s: S4, m: i32) -> u32;
    fn cl_s5_in(k: i32, s: S5, m: i32) -> u32;
    fn cl_s8_in(k: i32, s: S8, m: i32) -> u32;
    fn cl_s1_out(seed: i32) -> S1;
    fn cl_s2_out(seed: i32) -> S2;
    fn cl_s3_out(seed: i32) -> S3;
    fn cl_s4_out(seed: i32) -> S4;
    fn cl_s5_out(seed: i32) -> S5;
    fn cl_s8_out(seed: i32) -> S8;
    fn cl_char_value(c: c_char) -> i32;
    fn cl_schar_value(c: i8) -> i32;
    fn cl_uchar_value(c: u8) -> i32;
    fn cl_short_value(c: i16) -> i32;
    fn cl_char_is_signed() -> i32;
    fn cl_char_from(v: i32) -> c_char;
    fn cl_uchar_from(v: i32) -> u8;
    fn cl_short_from(v: i32) -> i16;
    fn cl_call16(f: Cb16, p: *const f32x4) -> f32x4;
    fn cl_callmix(f: CbMix, p: *const f32x4) -> f32x4;
    fn cl_calls3(f: extern "C" fn(i32, S3, i32) -> u32, seed: i32) -> u32;
    fn cl_calls3out(f: extern "C" fn(i32) -> S3, seed: i32) -> u32;
    fn cl_calls5out(f: extern "C" fn(i32) -> S5, seed: i32) -> u32;
    fn cl_calls8(f: extern "C" fn(i32, S8, i32) -> u32, seed: i32) -> u32;
    fn cl_callchar(f: extern "C" fn(c_char, i8, u8, i16) -> i32, v: i32) -> i32;
    fn cl_keep_across(f: unsafe extern "C" fn(), p: *const f32x4) -> f32x4;
    fn cl_pressure(p: *const f32x4, out: *mut f32x4);
}

fn fbits(v: f32x4) -> [u32; 4] {
    v.to_bits().to_array()
}

/// Sum of `weight(k) * p[k]` in f64, which is exact for the inputs used.
fn weighted(p: &[f32x4], start: [f32; 4], weight: impl Fn(usize) -> f64) -> [u32; 4] {
    core::array::from_fn(|j| {
        let sum: f64 = p.iter().enumerate().map(|(k, v)| black_box(weight(k) * v[j] as f64)).sum();
        ((sum + start[j] as f64) as f32).to_bits()
    })
}

fn inputs() -> [f32x4; regs::LIVE] {
    regs::input()
}

extern "C" fn rs_sum16(a0: f32x4, a1: f32x4, a2: f32x4, a3: f32x4, a4: f32x4, a5: f32x4, a6: f32x4, a7: f32x4, a8: f32x4, a9: f32x4, a10: f32x4, a11: f32x4, a12: f32x4, a13: f32x4, a14: f32x4, a15: f32x4) -> f32x4 {
    let p = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15];
    f32x4::from_bits(u32x4::from_array(weighted(&p, [0.0; 4], |k| (k + 1) as f64)))
}

fn mix_reference(i0: i32, a: f32x4, f0: f32, b: f32x4, i1: i32, d1: f64, c: f32x4, i2: i32, f1: f32) -> [u32; 4] {
    let (a, b, c) = (a.to_array(), b.to_array(), c.to_array());
    let extra = [i0 as f64, f0 as f64 + f1 as f64, i1 as f64 + i2 as f64, d1];
    core::array::from_fn(|j| (black_box(a[j] as f64 + b[j] as f64 + c[j] as f64 + extra[j]) as f32).to_bits())
}

extern "C" fn rs_mix(i0: i32, a: f32x4, f0: f32, b: f32x4, i1: i32, d1: f64, c: f32x4, i2: i32, f1: f32) -> f32x4 {
    f32x4::from_bits(u32x4::from_array(mix_reference(i0, a, f0, b, i1, d1, c, i2, f1)))
}

fn bytes_value(bytes: &[u8]) -> u32 {
    let weights = [1u32, 1 << 8, 1 << 16, 1 << 24, 3, 5, 7, 11];
    bytes.iter().zip(weights).map(|(&b, w)| u32::from(b).wrapping_mul(w)).fold(0, u32::wrapping_add)
}

/// The value the `cl_sN_in` functions compute.
fn struct_value(bytes: &[u8], k: i32, m: i32) -> u32 {
    let k_shift = if bytes.len() <= 3 { 24 } else { 4 };
    bytes_value(bytes).wrapping_add((k << k_shift) as u32).wrapping_add(m as u32)
}

fn seeded<const N: usize>(seed: i32) -> [u8; N] {
    core::array::from_fn(|i| (seed + i as i32) as u8)
}

extern "C" fn rs_s3(k: i32, s: S3, m: i32) -> u32 {
    struct_value(&s.0, k, m)
}

extern "C" fn rs_s3_out(seed: i32) -> S3 {
    S3(seeded(seed))
}

extern "C" fn rs_s5_out(seed: i32) -> S5 {
    S5(seeded(seed))
}

extern "C" fn rs_s8(k: i32, s: S8, m: i32) -> u32 {
    struct_value(&s.0, k, m)
}

static CHARS: [AtomicI32; 4] = [const { AtomicI32::new(0) }; 4];

extern "C" fn rs_char(c: c_char, s: i8, u: u8, h: i16) -> i32 {
    CHARS[0].store(i32::from(c), Relaxed);
    CHARS[1].store(i32::from(s), Relaxed);
    CHARS[2].store(i32::from(u), Relaxed);
    CHARS[3].store(i32::from(h), Relaxed);
    i32::from(c) + i32::from(u)
}

/// Runs `cl_pressure` (v0-v127, nonvolatile saves through __savevmx_N);
/// a callee for the register tests.
pub extern "C" fn msvc_pressure() {
    let input = inputs();
    let mut out = f32x4::splat(0.0);
    // SAFETY: 96 input vectors, one output.
    unsafe { cl_pressure(black_box(input.as_ptr()), &mut out) };
    black_box(out);
}

/// Runs `cl_keep_across` around a call that clobbers v0-v63.
pub extern "C" fn msvc_keep_across() {
    let input = inputs();
    // SAFETY: 70 input vectors; the callee follows the C convention.
    let out = unsafe { cl_keep_across(regs::vmx_clobber_volatile, black_box(input.as_ptr())) };
    black_box(out);
}

pub fn run(report: &mut Report) {
    let p = inputs();
    report.run("msvc.call.sum16", || {
        // SAFETY: plain values.
        let got = unsafe { cl_sum16(p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15]) };
        same(fbits(got), weighted(&p[..16], [0.0; 4], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
    });
    report.run("msvc.call.after13", || {
        // SAFETY: plain values.
        let got = unsafe { cl_after13(p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], -77, p[13], 1234) };
        same(fbits(got), weighted(&p[..14], [-77.0, 1234.0, 0.0, 0.0], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
    });
    report.run("msvc.call.mix", || {
        // SAFETY: plain values.
        let got = unsafe { cl_mix(19, p[0], -2.5, p[1], 40, -6.75, p[2], -1, 0.125) };
        same(fbits(got), mix_reference(19, p[0], -2.5, p[1], 40, -6.75, p[2], -1, 0.125), || String::new()).map(|()| String::new())
    });
    report.run("msvc.call.return_and_store", || {
        // SAFETY: plain values and a valid output.
        let made = unsafe { cl_make(black_box(1.5), -2.0, 3.25, 1.0e20) };
        same(fbits(made), [1.5f32, -2.0, 3.25, 1.0e20].map(f32::to_bits), || "cl_make".into())?;
        let mut out = f32x4::splat(0.0);
        unsafe { cl_store(black_box(p[5]), &mut out) };
        same(fbits(out), fbits(p[5]), || "cl_store".into()).map(|()| String::new())
    });
    report.run("msvc.call.structs_in", || {
        let (k, m) = (0x35, 0x1234);
        // SAFETY: plain values.
        let got = unsafe {
            [
                cl_s1_in(k, S1(seeded(0x11)), m),
                cl_s2_in(k, S2(seeded(0x21)), m),
                cl_s3_in(k, S3(seeded(0x31)), m),
                cl_s4_in(k, S4(seeded(0x41)), m),
                cl_s5_in(k, S5(seeded(0x51)), m),
                cl_s8_in(k, S8(seeded(0x61)), m),
            ]
        };
        let want = [
            struct_value(&seeded::<1>(0x11), k, m),
            struct_value(&seeded::<2>(0x21), k, m),
            struct_value(&seeded::<3>(0x31), k, m),
            struct_value(&seeded::<4>(0x41), k, m),
            struct_value(&seeded::<5>(0x51), k, m),
            struct_value(&seeded::<8>(0x61), k, m),
        ];
        same(got, want, || "S1 S2 S3 S4 S5 S8".into()).map(|()| String::new())
    });
    report.run("msvc.call.structs_out", || {
        // SAFETY: plain values.
        unsafe {
            same(cl_s1_out(0x12), S1(seeded(0x12)), || "S1".into())?;
            same(cl_s2_out(0x22), S2(seeded(0x22)), || "S2".into())?;
            same(cl_s3_out(0x32), S3(seeded(0x32)), || "S3".into())?;
            same(cl_s4_out(0x42), S4(seeded(0x42)), || "S4".into())?;
            same(cl_s5_out(0x52), S5(seeded(0x52)), || "S5".into())?;
            same(cl_s8_out(0x62), S8(seeded(0x62)), || "S8".into())?;
        }
        Ok(String::new())
    });
    report.run("msvc.call.char", || {
        // SAFETY: plain values.
        unsafe {
            same(cl_char_is_signed(), 1, || "char is signed in cl.exe".into())?;
            same(i32::from(c_char::MIN), -128, || "c_char is signed in Rust".into())?;
            for v in -128..=127i32 {
                same(cl_char_value(black_box(v as c_char)), v, || format!("char {v}"))?;
                same(cl_schar_value(black_box(v as i8)), v, || format!("signed char {v}"))?;
                same(cl_uchar_value(black_box(v as u8)), v & 0xFF, || format!("unsigned char {v}"))?;
            }
            for v in (-32768..=32767i32).step_by(97) {
                same(cl_short_value(black_box(v as i16)), v, || format!("short {v}"))?;
            }
            for v in [0, 1, 127, 128, 255, 256, -1, -129, 0x1234_5680, i32::MIN] {
                same(cl_char_from(black_box(v)), v as c_char, || format!("char from {v}"))?;
                same(cl_uchar_from(black_box(v)), v as u8, || format!("unsigned char from {v}"))?;
                same(cl_short_from(black_box(v)), v as i16, || format!("short from {v}"))?;
            }
        }
        Ok(String::new())
    });
    report.run("msvc.callback.sum16", || {
        // SAFETY: 16 input vectors.
        let got = unsafe { cl_call16(rs_sum16, p.as_ptr()) };
        same(fbits(got), weighted(&p[..16], [0.0; 4], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
    });
    report.run("msvc.callback.mix", || {
        // SAFETY: 3 input vectors.
        let got = unsafe { cl_callmix(rs_mix, p.as_ptr()) };
        same(fbits(got), mix_reference(7, p[0], 1.5, p[1], -3, 2.25, p[2], 11, -0.5), || String::new()).map(|()| String::new())
    });
    report.run("msvc.callback.structs", || {
        // SAFETY: plain values.
        unsafe {
            same(cl_calls3(rs_s3, 0x31), struct_value(&seeded::<3>(0x31), 5, 9), || "S3 argument".into())?;
            same(cl_calls3out(rs_s3_out, 0x41), bytes_value(&seeded::<3>(0x41)), || "S3 return".into())?;
            same(cl_calls5out(rs_s5_out, 0x51), bytes_value(&seeded::<5>(0x51)), || "S5 return".into())?;
            same(cl_calls8(rs_s8, 0x21), struct_value(&seeded::<8>(0x21), 5, 9), || "S8 argument".into())?;
        }
        Ok(String::new())
    });
    report.run("msvc.callback.char", || {
        for v in [-128i32, -1, 0, 1, 100, 127, 200, 255] {
            // SAFETY: plain values.
            let ret = unsafe { cl_callchar(rs_char, v) };
            let got = CHARS.each_ref().map(|c| c.load(Relaxed));
            let want = [v as i8 as i32, v as i8 as i32, v as u8 as i32, (v * 300) as i16 as i32];
            same(got, want, || format!("arguments for {v}"))?;
            same(ret, want[0] + want[2], || format!("return for {v}"))?;
        }
        Ok(String::new())
    });
    report.run("msvc.callback.rust_to_rust16", || {
        let f: Cb16 = black_box(rs_sum16);
        let got = f(p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15]);
        same(fbits(got), weighted(&p[..16], [0.0; 4], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
    });
    report.run("msvc.pressure", || {
        let mut out = f32x4::splat(0.0);
        // SAFETY: 96 input vectors, one output.
        unsafe { cl_pressure(p.as_ptr(), &mut out) };
        same(fbits(out), weighted(&p[..96], [0.0; 4], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
    });
    let callbacks: [(&str, unsafe extern "C" fn()); 3] = [
        ("asm_clobber_v0_v63", regs::vmx_clobber_volatile),
        ("rust_pressure", regs::rust_pressure),
        ("rust_keep_across", regs::rust_keep_across),
    ];
    for (name, callback) in callbacks {
        report.run(&format!("msvc.keep_across.{name}"), || {
            // SAFETY: 70 input vectors; the callee follows the C convention.
            let got = unsafe { cl_keep_across(callback, p.as_ptr()) };
            same(fbits(got), weighted(&p[..70], [0.0; 4], |k| (k + 1) as f64), || String::new()).map(|()| String::new())
        });
    }
}
