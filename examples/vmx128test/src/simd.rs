//! Vector code that the compiler generates from `core::simd` against
//! scalar references. Every input passes through `black_box`, so the
//! operations run on the console rather than in the compiler.
//!
//! Float references are computed in f64 and rounded once to f32. For
//! addition, subtraction, multiplication, division and square root of f32
//! values that is the correctly rounded f32 result, since f64 has more than
//! 2 * 24 + 2 bits.

use core::hint::black_box;
use core::simd::prelude::*;
use std::simd::StdFloat;

use crate::model;
use crate::same;
use crate::Outcome;
use crate::Report;
use crate::Rng;

const ROUNDS: usize = 4096;

fn fbits(v: f32x4) -> [u32; 4] {
    v.to_bits().to_array()
}

fn floats(rng: &mut Rng, range: i32) -> [f32; 4] {
    core::array::from_fn(|_| rng.float(range))
}

fn ints(rng: &mut Rng) -> [i32; 4] {
    core::array::from_fn(|_| rng.next_u32() as i32)
}

/// Scalar code generation checks, run first.
pub fn run_codegen(report: &mut Report) {
    report.run("codegen.float_format", zero_tests_64bit);
}

pub fn run(report: &mut Report) {
    report.run("simd.f32.arithmetic", arithmetic);
    report.run("simd.f32.mul_add", mul_add);
    report.run("simd.f32.compare_select", compare_select);
    report.run("simd.f32.min_max_clamp", min_max_clamp);
    report.run("simd.f32.sign", sign);
    report.run("simd.f32.round", rounding);
    report.run("simd.f32.reduce", reduce);
    report.run("simd.convert", convert);
    report.run("simd.splat_shuffle", shuffles);
    report.run("simd.i32.arithmetic", int_arithmetic);
    report.run("simd.i32.shift_logical", int_shift_logical);
    report.run("simd.i32.compare_min_max", int_compare);
    report.run("simd.i16_u8.saturating", narrow);
    report.run("simd.load_store.aligned", aligned);
    report.run("simd.load_store.unaligned", unaligned);
}

/// 64-bit zero tests in `core`: titles run in 32-bit mode, where a
/// record-form instruction sets CR0 from the low 32 bits only, so a 64-bit
/// value with a zero low half must not be tested that way (f64::classify,
/// float and hexadecimal formatting).
fn zero_tests_64bit() -> Outcome {
    const FOUR: f64 = 4.0;
    let cases: [(f64, &str); 10] = [
        (FOUR, "4"),
        (1.0, "1"),
        (2.0, "2"),
        (0.5, "0.5"),
        (100.0, "100"),
        (3.51, "3.51"),
        (-7.25, "-7.25"),
        (1.0 / 3.0, "0.3333333333333333"),
        (1.0e-7, "0.0000001"),
        (123456789.0, "123456789"),
    ];
    for (value, want) in cases {
        same(format!("{value}").as_str(), want, || format!("constant {want}"))?;
        same(format!("{}", black_box(value)).as_str(), want, || format!("runtime {want}"))?;
    }
    same(format!("{FOUR}").as_str(), "4", || "inline constant".into())?;
    same(format!("{}", black_box(4.0f32)).as_str(), "4", || "f32".into())?;
    same(format!("{:e}", black_box(1234.5f64)).as_str(), "1.2345e3", || "exponent".into())?;
    same(black_box(4.0f64).classify(), std::num::FpCategory::Normal, || "classify 4.0".into())?;
    same(format!("{:#x}", black_box(0x4059_0000_0000_0000u64)).as_str(), "0x4059000000000000", || "u64 hex".into())?;
    same(black_box(0x1_0000_0000u64) != 0, true, || "u64 != 0".into())?;
    Ok(String::new())
}

fn arithmetic() -> Outcome {
    let mut rng = Rng::new(101);
    for round in 0..ROUNDS {
        let (a, b) = (floats(&mut rng, 30), floats(&mut rng, 30));
        let (va, vb) = (black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        let cases: [(&str, f32x4, [f32; 4]); 6] = [
            ("add", va + vb, model::map2(a, b, |x, y| (x as f64 + y as f64) as f32)),
            ("sub", va - vb, model::map2(a, b, |x, y| (x as f64 - y as f64) as f32)),
            ("mul", va * vb, model::map2(a, b, |x, y| (x as f64 * y as f64) as f32)),
            ("div", va / vb, model::map2(a, b, |x, y| (x as f64 / y as f64) as f32)),
            ("sqrt", va.abs().sqrt(), model::map(a, |x| (x.abs() as f64).sqrt() as f32)),
            ("neg", -va, model::map(a, |x| f32::from_bits(x.to_bits() ^ 0x8000_0000))),
        ];
        for (name, got, want) in cases {
            same(fbits(got), model::bits(want), || format!("{name} round {round} a {a:?} b {b:?}"))?;
        }
    }
    Ok(String::new())
}

fn mul_add() -> Outcome {
    let mut rng = Rng::new(103);
    for round in 0..ROUNDS {
        let (a, b, c) = (floats(&mut rng, 12), floats(&mut rng, 12), floats(&mut rng, 24));
        let got = black_box(f32x4::from_array(a)).mul_add(black_box(f32x4::from_array(b)), black_box(f32x4::from_array(c)));
        let want: [f32; 4] = core::array::from_fn(|i| black_box(black_box(a[i]).mul_add(b[i], c[i])));
        same(fbits(got), model::bits(want), || format!("round {round}"))?;
    }
    Ok(String::new())
}

fn specials(rng: &mut Rng) -> Vec<([f32; 4], [f32; 4])> {
    let mut pairs = vec![
        ([f32::NAN, 1.0, f32::NAN, -3.0], [2.0, f32::NAN, f32::NAN, -3.0]),
        ([f32::INFINITY, f32::NEG_INFINITY, 1.0e30, -1.0e-30], [1.0, -1.0, 1.0e30, 1.0e-30]),
    ];
    for _ in 0..ROUNDS {
        let a = floats(rng, 8);
        let mut b = floats(rng, 8);
        b[rng.next_u32() as usize % 4] = a[1];
        pairs.push((a, b));
    }
    pairs
}

fn compare_select() -> Outcome {
    let mut rng = Rng::new(109);
    for (a, b) in specials(&mut rng) {
        let (va, vb) = (black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        let cases: [(&str, mask32x4, [bool; 4]); 6] = [
            ("eq", va.simd_eq(vb), model::map2(a, b, |x, y| x == y)),
            ("ne", va.simd_ne(vb), model::map2(a, b, |x, y| x != y)),
            ("lt", va.simd_lt(vb), model::map2(a, b, |x, y| x < y)),
            ("le", va.simd_le(vb), model::map2(a, b, |x, y| x <= y)),
            ("gt", va.simd_gt(vb), model::map2(a, b, |x, y| x > y)),
            ("ge", va.simd_ge(vb), model::map2(a, b, |x, y| x >= y)),
        ];
        for (name, got, want) in cases {
            same(got.to_array(), want, || format!("{name} {a:?} {b:?}"))?;
            let selected = got.select(va, vb);
            let expected: [u32; 4] = core::array::from_fn(|i| if want[i] { a[i].to_bits() } else { b[i].to_bits() });
            same(fbits(selected), expected, || format!("select {name} {a:?} {b:?}"))?;
        }
        let nan = va.is_nan().to_array();
        same(nan, model::map(a, f32::is_nan), || format!("is_nan {a:?}"))?;
    }
    Ok(String::new())
}

fn min_max_clamp() -> Outcome {
    let mut rng = Rng::new(111);
    for (a, b) in specials(&mut rng) {
        let va = black_box(f32x4::from_array(a));
        let vb = black_box(f32x4::from_array(b));
        for (name, actual, expected) in [
            ("min", va.simd_min(vb).to_array(), model::map2(a, b, f32::min)),
            ("max", va.simd_max(vb).to_array(), model::map2(a, b, f32::max)),
        ] {
            for lane in 0..4 {
                if expected[lane].is_nan() {
                    same(actual[lane].is_nan(), true, || format!("{name} NaN lane {lane}"))?;
                } else {
                    same(actual[lane], expected[lane], || format!("{name} lane {lane} a {a:?} b {b:?}"))?;
                }
            }
        }
    }
    for round in 0..ROUNDS {
        let a = floats(&mut rng, 8);
        let actual = black_box(f32x4::from_array(a))
            .simd_clamp(f32x4::splat(-1.0), f32x4::splat(1.0));
        same(actual.to_array(), model::map(a, |x| x.clamp(-1.0, 1.0)), || format!("clamp round {round}"))?;
    }
    Ok(String::new())
}

fn sign() -> Outcome {
    let mut rng = Rng::new(113);
    for round in 0..ROUNDS {
        let (a, b) = (floats(&mut rng, 30), floats(&mut rng, 30));
        let (va, vb) = (black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        same(fbits(va.abs()), model::map(a, |x| x.to_bits() & 0x7FFF_FFFF), || format!("abs round {round}"))?;
        same(fbits(va.copysign(vb)), model::map2(a, b, |x, y| x.to_bits() & 0x7FFF_FFFF | y.to_bits() & 0x8000_0000), || {
            format!("copysign round {round}")
        })?;
        same(va.is_sign_negative().to_array(), model::map(a, |x| x.to_bits() >> 31 != 0), || format!("sign round {round}"))?;
    }
    Ok(String::new())
}

fn rounding() -> Outcome {
    let mut rng = Rng::new(127);
    let fixed = [0.5f32, 1.5, 2.5, -0.5, -1.5, -2.5, 0.49999997, -0.0, 3.5, -3.5, 8388607.5, 1.0e9];
    let mut inputs: Vec<[f32; 4]> = fixed.chunks(4).map(|c| c.try_into().unwrap()).collect();
    inputs.extend((0..ROUNDS).map(|_| floats(&mut rng, 25)));
    for x in inputs {
        let v = black_box(f32x4::from_array(x));
        let cases: [(&str, f32x4, [f32; 4]); 5] = [
            ("floor", v.floor(), model::map(x, model::floor)),
            ("ceil", v.ceil(), model::map(x, model::ceil)),
            ("trunc", v.trunc(), model::map(x, model::trunc)),
            ("round", v.round(), model::map(x, |x| model::round(x, true))),
            ("fract", v.fract(), model::map(x, |x| (x as f64 - model::trunc(x) as f64) as f32)),
        ];
        for (name, got, want) in cases {
            same(fbits(got), model::bits(want), || format!("{name} {x:?}"))?;
        }
    }
    Ok(String::new())
}

fn reduce() -> Outcome {
    let mut rng = Rng::new(131);
    for round in 0..ROUNDS {
        let a: [f32; 4] = core::array::from_fn(|_| rng.small(1 << 20));
        let v = black_box(f32x4::from_array(a));
        same(v.reduce_sum(), a.iter().map(|x| black_box(*x)).sum::<f32>(), || format!("sum round {round}"))?;
        same(v.reduce_max(), a.iter().copied().fold(f32::MIN, f32::max), || format!("max round {round}"))?;
        let i = ints(&mut rng);
        let vi = black_box(i32x4::from_array(i));
        same(vi.reduce_sum(), i.iter().fold(0i32, |s, x| s.wrapping_add(black_box(*x))), || format!("int sum round {round}"))?;
        same(vi.reduce_xor(), i.iter().fold(0i32, |s, x| s ^ black_box(*x)), || format!("int xor round {round}"))?;
    }
    Ok(String::new())
}

fn convert() -> Outcome {
    let mut rng = Rng::new(137);
    let fixed = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 2147483648.0, -2147483904.0, 4294967296.0, -1.0, -0.5];
    let mut inputs: Vec<[f32; 4]> = fixed.chunks(4).map(|c| c.try_into().unwrap()).collect();
    inputs.extend((0..ROUNDS).map(|_| floats(&mut rng, 34)));
    for x in inputs {
        let v = black_box(f32x4::from_array(x));
        same(v.cast::<i32>().to_array(), model::map(x, |x| x as i32), || format!("f32->i32 {x:?}"))?;
        same(v.cast::<u32>().to_array(), model::map(x, |x| x as u32), || format!("f32->u32 {x:?}"))?;
        same(v.cast::<i16>().to_array(), model::map(x, |x| x as i16), || format!("f32->i16 {x:?}"))?;
        let i = ints(&mut rng);
        let vi = black_box(i32x4::from_array(i));
        same(fbits(vi.cast::<f32>()), model::bits(model::map(i, |x| x as f64 as f32)), || format!("i32->f32 {i:?}"))?;
        let u = i.map(|x| x as u32);
        let vu = black_box(u32x4::from_array(u));
        same(fbits(vu.cast::<f32>()), model::bits(model::map(u, |x| x as f64 as f32)), || format!("u32->f32 {u:?}"))?;
        same(vi.cast::<i16>().to_array(), model::map(i, |x| x as i16), || format!("i32->i16 {i:?}"))?;
        same(vi.cast::<u8>().to_array(), model::map(i, |x| x as u8), || format!("i32->u8 {i:?}"))?;
        let bytes = black_box(u8x4::from_array(i.map(|x| x as u8)));
        same(bytes.cast::<u32>().to_array(), model::map(i, |x| x as u8 as u32), || format!("u8->u32 {i:?}"))?;
        let halves = black_box(i16x4::from_array(i.map(|x| x as i16)));
        same(halves.cast::<i32>().to_array(), model::map(i, |x| x as i16 as i32), || format!("i16->i32 {i:?}"))?;
    }
    Ok(String::new())
}

fn shuffles() -> Outcome {
    let mut rng = Rng::new(139);
    for round in 0..ROUNDS {
        let (a, b) = (ints(&mut rng), ints(&mut rng));
        let (va, vb) = (black_box(i32x4::from_array(a)), black_box(i32x4::from_array(b)));
        let cases: [(&str, i32x4, [i32; 4]); 10] = [
            ("splat", i32x4::splat(black_box(a[2])), [a[2]; 4]),
            ("splat lane", simd_swizzle!(va, [1, 1, 1, 1]), [a[1]; 4]),
            ("reverse", va.reverse(), [a[3], a[2], a[1], a[0]]),
            ("swap pairs", simd_swizzle!(va, [1, 0, 3, 2]), [a[1], a[0], a[3], a[2]]),
            ("rotate", va.rotate_elements_left::<1>(), [a[1], a[2], a[3], a[0]]),
            ("permute", simd_swizzle!(va, [3, 0, 2, 0]), [a[3], a[0], a[2], a[0]]),
            ("merge high", simd_swizzle!(va, vb, [0, 4, 1, 5]), [a[0], b[0], a[1], b[1]]),
            ("merge low", simd_swizzle!(va, vb, [2, 6, 3, 7]), [a[2], b[2], a[3], b[3]]),
            ("two inputs", simd_swizzle!(va, vb, [3, 5, 0, 6]), [a[3], b[1], a[0], b[2]]),
            ("concat shift", simd_swizzle!(va, vb, [1, 2, 3, 4]), [a[1], a[2], a[3], b[0]]),
        ];
        for (name, got, want) in cases {
            same(got.to_array(), want, || format!("{name} round {round}"))?;
        }
        let (even, odd) = va.deinterleave(vb);
        same((even.to_array(), odd.to_array()), ([a[0], a[2], b[0], b[2]], [a[1], a[3], b[1], b[3]]), || {
            format!("deinterleave round {round}")
        })?;
        let bytes: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let vbytes = black_box(u8x16::from_array(bytes));
        let swapped = simd_swizzle!(vbytes, [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]);
        let want: [u8; 16] = core::array::from_fn(|i| bytes[i ^ 3]);
        same(swapped.to_array(), want, || format!("byte swap round {round}"))?;
    }
    Ok(String::new())
}

fn int_arithmetic() -> Outcome {
    let mut rng = Rng::new(149);
    for round in 0..ROUNDS {
        let (a, b) = (ints(&mut rng), ints(&mut rng));
        let (va, vb) = (black_box(i32x4::from_array(a)), black_box(i32x4::from_array(b)));
        let cases: [(&str, i32x4, [i32; 4]); 5] = [
            ("add", va + vb, model::map2(a, b, i32::wrapping_add)),
            ("sub", va - vb, model::map2(a, b, i32::wrapping_sub)),
            ("mul", va * vb, model::map2(a, b, i32::wrapping_mul)),
            ("neg", -va, model::map(a, i32::wrapping_neg)),
            ("abs", va.abs(), model::map(a, i32::wrapping_abs)),
        ];
        for (name, got, want) in cases {
            same(got.to_array(), want, || format!("{name} round {round} a {a:?} b {b:?}"))?;
        }
    }
    Ok(String::new())
}

fn int_shift_logical() -> Outcome {
    let mut rng = Rng::new(151);
    for round in 0..ROUNDS {
        let (a, b) = (ints(&mut rng), ints(&mut rng));
        let n = b.map(|x| x & 31);
        let (va, vb, vn) = (black_box(i32x4::from_array(a)), black_box(i32x4::from_array(b)), black_box(i32x4::from_array(n)));
        let ua = va.cast::<u32>();
        let cases: [(&str, [i32; 4], [i32; 4]); 8] = [
            ("shl", (va << vn).to_array(), model::map2(a, n, |x, n| x << n)),
            ("sra", (va >> vn).to_array(), model::map2(a, n, |x, n| x >> n)),
            ("srl", (ua >> vn.cast::<u32>()).cast::<i32>().to_array(), model::map2(a, n, |x, n| ((x as u32) >> n) as i32)),
            ("shl splat", (va << 7).to_array(), model::map(a, |x| x << 7)),
            ("and", (va & vb).to_array(), model::map2(a, b, |x, y| x & y)),
            ("or", (va | vb).to_array(), model::map2(a, b, |x, y| x | y)),
            ("xor", (va ^ vb).to_array(), model::map2(a, b, |x, y| x ^ y)),
            ("andnot", (va & !vb).to_array(), model::map2(a, b, |x, y| x & !y)),
        ];
        for (name, got, want) in cases {
            same(got, want, || format!("{name} round {round}"))?;
        }
    }
    Ok(String::new())
}

fn int_compare() -> Outcome {
    let mut rng = Rng::new(157);
    for round in 0..ROUNDS {
        let a = ints(&mut rng);
        let mut b = ints(&mut rng);
        b[round % 4] = a[round % 4];
        let (va, vb) = (black_box(i32x4::from_array(a)), black_box(i32x4::from_array(b)));
        same(va.simd_eq(vb).to_array(), model::map2(a, b, |x, y| x == y), || format!("eq round {round}"))?;
        same(va.simd_lt(vb).to_array(), model::map2(a, b, |x, y| x < y), || format!("lt round {round}"))?;
        let (ua, ub) = (va.cast::<u32>(), vb.cast::<u32>());
        same(ua.simd_gt(ub).to_array(), model::map2(a, b, |x, y| x as u32 > y as u32), || format!("ugt round {round}"))?;
        same(va.simd_min(vb).to_array(), model::map2(a, b, i32::min), || format!("min round {round}"))?;
        same(ua.simd_max(ub).to_array(), model::map2(a, b, |x, y| (x as u32).max(y as u32)), || format!("umax round {round}"))?;
    }
    Ok(String::new())
}

fn narrow() -> Outcome {
    let mut rng = Rng::new(163);
    for round in 0..ROUNDS {
        let a: [i16; 8] = core::array::from_fn(|_| rng.next_u32() as i16);
        let b: [i16; 8] = core::array::from_fn(|_| rng.next_u32() as i16);
        let (va, vb) = (black_box(i16x8::from_array(a)), black_box(i16x8::from_array(b)));
        let want: [i16; 8] = core::array::from_fn(|i| black_box(a[i]).saturating_add(b[i]));
        same(va.saturating_add(vb).to_array(), want, || format!("i16 saturating_add round {round}"))?;
        let want: [i16; 8] = core::array::from_fn(|i| black_box(a[i]).wrapping_sub(b[i]));
        same((va - vb).to_array(), want, || format!("i16 sub round {round}"))?;
        let c: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let d: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let (vc, vd) = (black_box(u8x16::from_array(c)), black_box(u8x16::from_array(d)));
        let want: [u8; 16] = core::array::from_fn(|i| black_box(c[i]).saturating_sub(d[i]));
        same(vc.saturating_sub(vd).to_array(), want, || format!("u8 saturating_sub round {round}"))?;
        let want: [u8; 16] = core::array::from_fn(|i| black_box(c[i]).wrapping_add(d[i]));
        same((vc + vd).to_array(), want, || format!("u8 add round {round}"))?;
        let want: [u8; 16] = core::array::from_fn(|i| black_box(c[i]).max(d[i]));
        same(vc.simd_max(vd).to_array(), want, || format!("u8 max round {round}"))?;
    }
    Ok(String::new())
}

#[repr(C, align(16))]
struct Buffer([u8; 96]);

impl Buffer {
    fn pattern(seed: u8) -> Self {
        Self(core::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(13))))
    }
}

#[repr(C, align(16))]
struct Floats([f32; 16]);

fn aligned() -> Outcome {
    let source = Floats(core::array::from_fn(|i| i as f32 * 1.25 - 3.0));
    let mut target = Floats([0.0; 16]);
    for block in 0..4 {
        let v = f32x4::from_slice(black_box(&source.0[4 * block..4 * block + 4]));
        (v * f32x4::splat(2.0)).copy_to_slice(&mut target.0[4 * block..4 * block + 4]);
    }
    let want: [f32; 16] = core::array::from_fn(|i| (i as f32 * 1.25 - 3.0) * 2.0);
    same(target.0.map(f32::to_bits), want.map(f32::to_bits), || "f32 blocks".into())?;
    let bytes = Buffer::pattern(7);
    let v = black_box(u8x16::from_array(bytes.0[16..32].try_into().unwrap()));
    same(v.to_array(), <[u8; 16]>::try_from(&bytes.0[16..32]).unwrap(), || "bytes".into())?;
    Ok(String::new())
}

fn unaligned() -> Outcome {
    let source = Buffer::pattern(3);
    let value = u8x16::from_array(core::array::from_fn(|i| 0xA0 + i as u8));
    for offset in 0..64 {
        let got = u8x16::from_slice(black_box(&source.0[offset..offset + 16]));
        same(got.to_array(), <[u8; 16]>::try_from(&source.0[offset..offset + 16]).unwrap(), || {
            format!("u8x16 load offset {offset}")
        })?;
        let mut target = Buffer::pattern(9);
        let mut want = Buffer::pattern(9).0;
        black_box(value).copy_to_slice(black_box(&mut target.0[offset..offset + 16]));
        want[offset..offset + 16].copy_from_slice(&value.to_array());
        same(target.0, want, || format!("u8x16 store offset {offset}"))?;
    }
    let floats = Floats(core::array::from_fn(|i| i as f32 + 0.5));
    for offset in 0..12 {
        let got = f32x4::from_slice(black_box(&floats.0[offset..offset + 4]));
        let want: [u32; 4] = core::array::from_fn(|i| (offset as f32 + i as f32 + 0.5).to_bits());
        same(fbits(got), want, || format!("f32x4 load offset {offset}"))?;
        let mut target = Floats([0.0; 16]);
        black_box(got + f32x4::splat(1.0)).copy_to_slice(black_box(&mut target.0[offset..offset + 4]));
        let want: [u32; 16] = core::array::from_fn(|i| {
            if (offset..offset + 4).contains(&i) { (i as f32 + 1.5).to_bits() } else { 0 }
        });
        same(target.0.map(f32::to_bits), want, || format!("f32x4 store offset {offset}"))?;
    }
    Ok(String::new())
}
