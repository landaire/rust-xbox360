//! Every operation of `xbox360-vmx128` against the scalar models.

use core::hint::black_box;
use core::simd::prelude::*;

use xbox360_vmx128 as vmx;

use crate::model;
use crate::same;
use crate::Outcome;
use crate::Report;
use crate::Rng;

const ROUNDS: usize = 4096;

fn words<T: vmx::Vector>(v: T) -> [u32; 4] {
    vmx::cast::<T, u32x4>(v).to_array()
}

fn fbits(v: f32x4) -> [u32; 4] {
    v.to_bits().to_array()
}

fn floats(rng: &mut Rng, range: i32) -> [f32; 4] {
    core::array::from_fn(|_| rng.float(range))
}

fn random_words(rng: &mut Rng) -> [u32; 4] {
    core::array::from_fn(|_| rng.next_u32())
}

pub fn run(report: &mut Report) {
    report.run("vmx128.dot3.exact", || dot_exact(false));
    report.run("vmx128.dot4.exact", || dot_exact(true));
    report.run("vmx128.dot4.cancellation", dot_cancellation);
    report.run("vmx128.dot3.error", || dot_error(false));
    report.run("vmx128.dot4.error", || dot_error(true));
    report.run("vmx128.permute_words.all", permute_words);
    report.run("vmx128.rotate_insert.all", rotate_insert);
    report.run("vmx128.pack_d3d.all", pack_d3d);
    report.run("vmx128.pack_d3d.saturation", pack_saturation);
    report.run("vmx128.unpack_d3d.all", unpack_d3d);
    report.run("vmx128.load_left_right", loads);
    report.run("vmx128.store_left_right", stores);
    report.run("vmx128.load_store_unaligned", unaligned);
    report.run("vmx128.mul_add", mul_add);
    report.run("vmx128.max_min", max_min);
    report.run("vmx128.round", rounding);
    report.run("vmx128.convert", convert);
    report.run("vmx128.compare", compare);
    report.run("vmx128.bounds", bounds);
    report.run("vmx128.select_bits", select_bits);
    report.run("vmx128.word_shifts", word_shifts);
    report.run("vmx128.permute_bytes", permute_bytes);
    report.run("vmx128.splat_merge_octets", splat_merge_octets);
    report.run("vmx128.recip_estimate", recip_estimate);
    report.run("vmx128.rsqrt_estimate", rsqrt_estimate);
    report.run("vmx128.exp2_estimate", exp2_estimate);
    report.run("vmx128.log2_estimate", log2_estimate);
}

fn dot_reference(a: [f32; 4], b: [f32; 4], four: bool) -> f64 {
    let n = if four { 4 } else { 3 };
    (0..n).map(|i| black_box(a[i] as f64 * b[i] as f64)).sum()
}

fn dot(a: f32x4, b: f32x4, four: bool) -> f32x4 {
    if four { vmx::dot4(a, b) } else { vmx::dot3(a, b) }
}

/// Non-negative small integers: every product and partial sum is exact.
fn dot_exact(four: bool) -> Outcome {
    let mut rng = Rng::new(11);
    for round in 0..ROUNDS {
        let a: [f32; 4] = core::array::from_fn(|_| rng.small(1000).abs());
        let b: [f32; 4] = core::array::from_fn(|_| rng.small(1000).abs());
        let got = dot(black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)), four);
        let want = (dot_reference(a, b, four) as f32).to_bits();
        same(fbits(got), [want; 4], || format!("round {round} a {a:?} b {b:?}"))?;
    }
    Ok(String::new())
}

/// Mixed signs do not cancel exactly (values measured on the console).
fn dot_cancellation() -> Outcome {
    let cases = [
        ([1.0, 1.0, 1.0, 1.0], [1.0, -1.0, 1.0, -1.0], 0xB180_0000u32),
        ([8.0, -4.0, 2.0, 2.0], [0.25, 0.25, 0.25, 0.25], 0x3FFF_FFFF),
    ];
    for (a, b, want) in cases {
        let got = vmx::dot4(black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        same(fbits(got), [want; 4], || format!("{a:?}.{b:?}"))?;
    }
    Ok(String::new())
}

/// Error against the exact dot product, in units of the last place of
/// the largest product. Bound measured on the console over these inputs.
fn dot_error(four: bool) -> Outcome {
    const BOUND: f64 = 4.0;
    let mut rng = Rng::new(if four { 13 } else { 17 });
    let mut worst = 0.0f64;
    for round in 0..65536 {
        let a = floats(&mut rng, 10);
        let b = floats(&mut rng, 10);
        let got = dot(black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)), four).to_array();
        same(fbits(f32x4::from_array(got)), [got[0].to_bits(); 4], || format!("words differ, round {round}"))?;
        let n = if four { 4 } else { 3 };
        let largest = (0..n).map(|i| (a[i] as f64 * b[i] as f64).abs()).fold(0.0, f64::max);
        let ulp = 2f64.powi(largest.log2().floor() as i32 - 23);
        let error = (got[0] as f64 - dot_reference(a, b, four)).abs() / ulp;
        worst = worst.max(error);
        if error > BOUND {
            return Err(format!("round {round} a {a:?} b {b:?}: {} off by {error} ulp", got[0]));
        }
    }
    Ok(format!("max error {worst} ulp of the largest product (bound {BOUND})"))
}

macro_rules! permute_case {
    ($input:expr, $($p:literal)*) => {{
        let mut result = [[0u32; 4]; 256];
        $( result[$p] = words(vmx::permute_words::<$p, _>(black_box($input))); )*
        result
    }};
}

fn permute_words() -> Outcome {
    let input = u32x4::from_array([0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]);
    let got = permute_case!(input,
        0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
        32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
        64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95
        96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119
        120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 141 142 143
        144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160 161 162 163 164 165 166 167
        168 169 170 171 172 173 174 175 176 177 178 179 180 181 182 183 184 185 186 187 188 189 190 191
        192 193 194 195 196 197 198 199 200 201 202 203 204 205 206 207 208 209 210 211 212 213 214 215
        216 217 218 219 220 221 222 223 224 225 226 227 228 229 230 231 232 233 234 235 236 237 238 239
        240 241 242 243 244 245 246 247 248 249 250 251 252 253 254 255);
    let source = input.to_array();
    for p in 0..256usize {
        let want: [u32; 4] = core::array::from_fn(|i| source[(p >> (6 - 2 * i)) & 3]);
        same(got[p], want, || format!("permute {p:#04x}"))?;
    }
    Ok(String::new())
}

macro_rules! rotate_case {
    ($into:expr, $value:expr, $($mask:literal)*) => {{
        let mut result = [[[0u32; 4]; 4]; 16];
        $(
            result[$mask][0] = words(vmx::rotate_insert::<$mask, 0, _>(black_box($into), black_box($value)));
            result[$mask][1] = words(vmx::rotate_insert::<$mask, 1, _>(black_box($into), black_box($value)));
            result[$mask][2] = words(vmx::rotate_insert::<$mask, 2, _>(black_box($into), black_box($value)));
            result[$mask][3] = words(vmx::rotate_insert::<$mask, 3, _>(black_box($into), black_box($value)));
        )*
        result
    }};
}

fn rotate_insert() -> Outcome {
    let into = u32x4::from_array([0xA0A0_A0A0, 0xA1A1_A1A1, 0xA2A2_A2A2, 0xA3A3_A3A3]);
    let value = u32x4::from_array([0xB0B0_B0B0, 0xB1B1_B1B1, 0xB2B2_B2B2, 0xB3B3_B3B3]);
    let got = rotate_case!(into, value, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
    let (d, b) = (into.to_array(), value.to_array());
    for mask in 0..16 {
        for rotate in 0..4 {
            let want: [u32; 4] =
                core::array::from_fn(|i| if mask & (8 >> i) != 0 { b[(i + rotate) & 3] } else { d[i] });
            same(got[mask][rotate], want, || format!("mask {mask} rotate {rotate}"))?;
        }
    }
    Ok(String::new())
}

#[derive(Clone, Copy)]
enum Field {
    Unsigned(u32),
    Signed(u32),
    Half,
    Ignored,
}

fn fields(kind: u32) -> [Field; 4] {
    use Field::*;
    match kind {
        0 => [Unsigned(8); 4],
        1 => [Signed(16), Signed(16), Ignored, Ignored],
        2 => [Signed(10), Signed(10), Signed(10), Unsigned(2)],
        3 => [Half, Half, Ignored, Ignored],
        4 => [Signed(16); 4],
        5 => [Half; 4],
        6 => [Signed(20), Signed(20), Signed(20), Unsigned(4)],
        _ => unreachable!(),
    }
}

/// An in-range input component (f32 bits) for `field`. Half floats are
/// normal f16 values with random bits below the f16 mantissa, which the
/// pack truncates.
fn component(field: Field, rng: &mut Rng) -> u32 {
    let random = rng.next_u32();
    match field {
        Field::Unsigned(width) => model::THREE + (random & ((1 << width) - 1)),
        Field::Signed(width) => {
            let max = (1u32 << (width - 1)) - 1;
            let x = (random % (2 * max + 1)) as i32 - max as i32;
            model::signed_value(x as u32 & ((1 << width) - 1), width)
        }
        Field::Half => {
            let exponent = 1 + random % 31;
            let half = (random >> 16 & 0x8000) | exponent << 10 | (random >> 5 & 0x3FF);
            model::unhalf(half) | (rng.next_u32() & 0x1FFF)
        }
        Field::Ignored => rng.next_u32(),
    }
}

type PackFn = fn(u32x4, f32x4) -> u32x4;

macro_rules! pack_table {
    ($($kind:literal)*) => {
        [$(
            [
                [
                    vmx::pack_d3d::<$kind, 1, 0> as PackFn,
                    vmx::pack_d3d::<$kind, 1, 1>,
                    vmx::pack_d3d::<$kind, 1, 2>,
                    vmx::pack_d3d::<$kind, 1, 3>,
                ],
                [
                    vmx::pack_d3d::<$kind, 2, 0>,
                    vmx::pack_d3d::<$kind, 2, 1>,
                    vmx::pack_d3d::<$kind, 2, 2>,
                    vmx::pack_d3d::<$kind, 2, 3>,
                ],
                [
                    vmx::pack_d3d::<$kind, 3, 0>,
                    vmx::pack_d3d::<$kind, 3, 1>,
                    vmx::pack_d3d::<$kind, 3, 2>,
                    vmx::pack_d3d::<$kind, 3, 3>,
                ],
            ],
        )*]
    };
}

static PACK: [[[PackFn; 4]; 3]; 7] = pack_table!(0 1 2 3 4 5 6);

fn pack_d3d() -> Outcome {
    let mut rng = Rng::new(23);
    for kind in 0..7u32 {
        for mask in 1..4u32 {
            for shift in 0..4u32 {
                for round in 0..256 {
                    let into = random_words(&mut rng);
                    let value: [u32; 4] = fields(kind).map(|field| component(field, &mut rng));
                    let pack = black_box(PACK[kind as usize][mask as usize - 1][shift as usize]);
                    let got = pack(u32x4::from_array(into), f32x4::from_bits(u32x4::from_array(value)));
                    let want = model::pack_into(kind, mask, shift, into, value);
                    same(got.to_array(), want, || {
                        format!("kind {kind} mask {mask} shift {shift} round {round} value {value:08x?}")
                    })?;
                }
            }
        }
    }
    Ok(String::new())
}

/// Inputs outside each field's range and special values.
fn pack_saturation() -> Outcome {
    let specials: [f32; 14] =
        [0.0, -0.0, 0.5, 1.0, 1.999, 2.0, 2.5, 3.0, 3.999, 4.0, 100.0, -100.0, 131072.0, 1.0e-30];
    for kind in 0..7u32 {
        for (index, &x) in specials.iter().enumerate() {
            let value = [x.to_bits(), (-x).to_bits(), specials[(index + 3) % specials.len()].to_bits(), x.to_bits()];
            let pack = black_box(PACK[kind as usize][1][0]);
            let got = pack(u32x4::splat(0), f32x4::from_bits(u32x4::from_array(value)));
            let want = model::pack_into(kind, 2, 0, [0; 4], value);
            same(got.to_array(), want, || format!("kind {kind} value {:?}", value.map(f32::from_bits)))?;
        }
    }
    Ok(String::new())
}

type UnpackFn = fn(u32x4) -> f32x4;

static UNPACK: [UnpackFn; 7] = [
    vmx::unpack_d3d::<0>,
    vmx::unpack_d3d::<1>,
    vmx::unpack_d3d::<2>,
    vmx::unpack_d3d::<3>,
    vmx::unpack_d3d::<4>,
    vmx::unpack_d3d::<5>,
    vmx::unpack_d3d::<6>,
];

/// Replaces the most negative value of each signed field, which unpacks
/// to a NaN, and f16 denormals.
fn usable(kind: u32, high: u32, low: u32) -> (u32, u32) {
    let fix = |packed: u64, offset: u32, width: u32| {
        let field = packed >> offset & ((1 << width) - 1);
        if field == 1 << (width - 1) { packed + (1 << offset) } else { packed }
    };
    let fix_half = |packed: u64, offset: u32| {
        let field = packed >> offset & 0xFFFF;
        if field & 0x7C00 == 0 { packed | 0x0400 << offset } else { packed }
    };
    let mut packed = u64::from(high) << 32 | u64::from(low);
    match kind {
        1 | 4 => {
            for offset in [0, 16, 32, 48] {
                packed = fix(packed, offset, 16);
            }
        }
        2 => {
            for offset in [0, 10, 20] {
                packed = fix(packed, offset, 10);
            }
        }
        3 | 5 => {
            for offset in [0, 16, 32, 48] {
                packed = fix_half(packed, offset);
            }
        }
        6 => {
            for offset in [0, 20, 40] {
                packed = fix(packed, offset, 20);
            }
        }
        _ => {}
    }
    ((packed >> 32) as u32, packed as u32)
}

fn unpack_d3d() -> Outcome {
    let mut rng = Rng::new(29);
    for kind in 0..7u32 {
        for round in 0..ROUNDS {
            let (high, low) = usable(kind, rng.next_u32(), rng.next_u32());
            let input = [rng.next_u32(), rng.next_u32(), high, low];
            let got = black_box(UNPACK[kind as usize])(u32x4::from_array(input));
            let want = model::unpack(kind, high, low);
            same(fbits(got), want, || format!("kind {kind} round {round} words {high:08x} {low:08x}"))?;
        }
    }
    Ok(String::new())
}

#[repr(C, align(16))]
struct Blocks([u8; 64]);

impl Blocks {
    fn pattern(seed: u8) -> Self {
        Self(core::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(7))))
    }

    fn block(&self, index: usize) -> [u8; 16] {
        self.0[16 * index..16 * index + 16].try_into().unwrap()
    }
}

type LoadFn = unsafe fn(*const u8) -> u8x16;
type StoreFn = unsafe fn(u8x16, *mut u8);

fn loads() -> Outcome {
    let memory = Blocks::pattern(3);
    let variants: [(&str, LoadFn, bool); 4] = [
        ("lvlx", vmx::load_left::<u8x16>, true),
        ("lvrx", vmx::load_right::<u8x16>, false),
        ("lvlxl", vmx::load_left_lru::<u8x16>, true),
        ("lvrxl", vmx::load_right_lru::<u8x16>, false),
    ];
    for (name, load, left) in variants {
        for offset in 0..16 {
            // SAFETY: the address lies in block 1, which both forms read
            // from.
            let got = unsafe { black_box(load)(memory.0.as_ptr().add(16 + offset)) };
            let want = if left {
                model::load_left(&memory.block(1), offset)
            } else {
                model::load_right(&memory.block(1), offset)
            };
            same(got.to_array(), want, || format!("{name} offset {offset}"))?;
        }
    }
    Ok(String::new())
}

fn stores() -> Outcome {
    let value = u8x16::from_array(core::array::from_fn(|i| 0xC0 + i as u8));
    let variants: [(&str, StoreFn, bool); 4] = [
        ("stvlx", vmx::store_left::<u8x16>, true),
        ("stvrx", vmx::store_right::<u8x16>, false),
        ("stvlxl", vmx::store_left_lru::<u8x16>, true),
        ("stvrxl", vmx::store_right_lru::<u8x16>, false),
    ];
    for (name, store, left) in variants {
        for offset in 0..16 {
            let mut memory = Blocks::pattern(5);
            let mut want = Blocks::pattern(5).0;
            // SAFETY: both forms write only block 1.
            unsafe { black_box(store)(black_box(value), memory.0.as_mut_ptr().add(16 + offset)) };
            let bytes = value.to_array();
            if left {
                want[16 + offset..32].copy_from_slice(&bytes[..16 - offset]);
            } else {
                want[16..16 + offset].copy_from_slice(&bytes[16 - offset..]);
            }
            same(memory.0, want, || format!("{name} offset {offset}"))?;
        }
    }
    Ok(String::new())
}

fn unaligned() -> Outcome {
    let memory = Blocks::pattern(9);
    let value = u8x16::from_array(core::array::from_fn(|i| 0x50 + i as u8));
    for offset in 0..32 {
        // SAFETY: offset + 16 <= 64.
        let got: u8x16 = unsafe { vmx::load_unaligned(black_box(memory.0.as_ptr().add(offset))) };
        let want: [u8; 16] = memory.0[offset..offset + 16].try_into().unwrap();
        same(got.to_array(), want, || format!("load offset {offset}"))?;

        let mut target = Blocks::pattern(1);
        let mut expected = Blocks::pattern(1).0;
        // SAFETY: offset + 16 <= 64.
        unsafe { vmx::store_unaligned(black_box(value), black_box(target.0.as_mut_ptr().add(offset))) };
        expected[offset..offset + 16].copy_from_slice(&value.to_array());
        same(target.0, expected, || format!("store offset {offset}"))?;
    }
    Ok(String::new())
}

fn mul_add() -> Outcome {
    let mut rng = Rng::new(31);
    for round in 0..ROUNDS {
        let (a, b, c) = (floats(&mut rng, 12), floats(&mut rng, 12), floats(&mut rng, 24));
        let (va, vb, vc) = (f32x4::from_array(a), f32x4::from_array(b), f32x4::from_array(c));
        let fma: [f32; 4] = core::array::from_fn(|i| black_box(black_box(a[i]).mul_add(b[i], c[i])));
        let got = vmx::mul_add(black_box(va), black_box(vb), black_box(vc));
        same(fbits(got), model::bits(fma), || format!("mul_add round {round}"))?;
        let got = vmx::neg_mul_sub(black_box(va), black_box(vb), black_box(vc));
        let want: [f32; 4] = core::array::from_fn(|i| black_box(-black_box(a[i]).mul_add(b[i], -c[i])));
        same(fbits(got), model::bits(want), || format!("neg_mul_sub round {round}"))?;
    }
    Ok(String::new())
}

fn max_min() -> Outcome {
    let mut rng = Rng::new(37);
    for round in 0..ROUNDS {
        let (a, b) = (floats(&mut rng, 20), floats(&mut rng, 20));
        let (va, vb) = (f32x4::from_array(a), f32x4::from_array(b));
        let got = vmx::max(black_box(va), black_box(vb));
        same(fbits(got), model::bits(model::map2(a, b, |x, y| if x > y { x } else { y })), || {
            format!("max round {round}")
        })?;
        let got = vmx::min(black_box(va), black_box(vb));
        same(fbits(got), model::bits(model::map2(a, b, |x, y| if x < y { x } else { y })), || {
            format!("min round {round}")
        })?;
    }
    Ok(String::new())
}

fn rounding() -> Outcome {
    let mut rng = Rng::new(41);
    let specials = [0.5f32, 1.5, 2.5, -0.5, -1.5, -2.5, 0.49999997, -0.0, 0.0, 8388607.5, -8388607.5, 1.0e9];
    let mut inputs: Vec<[f32; 4]> = specials.chunks(4).map(|c| c.try_into().unwrap()).collect();
    inputs.extend((0..ROUNDS).map(|_| floats(&mut rng, 25)));
    for (round, x) in inputs.iter().enumerate() {
        let v = black_box(f32x4::from_array(*x));
        let cases: [(&str, f32x4, [f32; 4]); 4] = [
            ("floor", vmx::floor(v), model::map(*x, model::floor)),
            ("ceil", vmx::ceil(v), model::map(*x, model::ceil)),
            ("trunc", vmx::trunc(v), model::map(*x, model::trunc)),
            ("round_even", vmx::round_even(v), model::map(*x, |x| model::round(x, false))),
        ];
        for (name, got, want) in cases {
            same(fbits(got), model::bits(want), || format!("{name} round {round} x {x:?}"))?;
        }
    }
    Ok(String::new())
}

macro_rules! convert_case {
    ($x:expr, $i:expr, $u:expr, $($scale:literal)*) => {
        [$(
            (
                vmx::to_i32_saturating::<$scale>(black_box($x)).to_array(),
                vmx::to_u32_saturating::<$scale>(black_box($x)).to_array(),
                fbits(vmx::from_i32::<$scale>(black_box($i))),
                fbits(vmx::from_u32::<$scale>(black_box($u))),
            ),
        )*]
    };
}

fn convert() -> Outcome {
    let mut rng = Rng::new(43);
    let specials = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 2147483648.0, -2147483904.0, 4294967296.0, -1.0, -0.5];
    let mut inputs: Vec<[f32; 4]> = specials.chunks(4).map(|c| c.try_into().unwrap()).collect();
    inputs.extend((0..ROUNDS).map(|_| floats(&mut rng, 34)));
    for x in &inputs {
        let ints: [u32; 4] = random_words(&mut rng);
        let vx = f32x4::from_array(*x);
        let vi = u32x4::from_array(ints).cast::<i32>();
        let vu = u32x4::from_array(ints);
        let got = convert_case!(vx, vi, vu,
            0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
        for (scale, (to_i, to_u, from_i, from_u)) in got.into_iter().enumerate() {
            let scale = scale as u32;
            same(to_i, model::map(*x, |x| model::to_i32(x, scale)), || format!("to_i32 scale {scale} x {x:?}"))?;
            same(to_u, model::map(*x, |x| model::to_u32(x, scale)), || format!("to_u32 scale {scale} x {x:?}"))?;
            same(from_i, model::bits(model::map(ints, |v| model::from_i32(v as i32, scale))), || {
                format!("from_i32 scale {scale} ints {ints:08x?}")
            })?;
            same(from_u, model::bits(model::map(ints, |v| model::from_u32(v, scale))), || {
                format!("from_u32 scale {scale} ints {ints:08x?}")
            })?;
        }
    }
    Ok(String::new())
}

fn special_pairs(rng: &mut Rng) -> Vec<([f32; 4], [f32; 4])> {
    let mut pairs = vec![
        ([f32::NAN, 1.0, -0.0, 0.0], [1.0, f32::NAN, 0.0, -0.0]),
        ([f32::INFINITY, f32::NEG_INFINITY, 2.0, -2.0], [f32::INFINITY, 1.0, 2.0, -3.0]),
    ];
    for _ in 0..ROUNDS {
        let a = floats(rng, 4);
        let mut b = floats(rng, 4);
        b[rng.next_u32() as usize % 4] = a[0];
        pairs.push((a, b));
    }
    pairs
}

fn compare() -> Outcome {
    let mut rng = Rng::new(47);
    for (a, b) in special_pairs(&mut rng) {
        let (va, vb) = (black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        let cases: [(&str, mask32x4, [bool; 4]); 3] = [
            ("eq", vmx::eq(va, vb), model::map2(a, b, |x, y| x == y)),
            ("ge", vmx::ge(va, vb), model::map2(a, b, |x, y| x >= y)),
            ("gt", vmx::gt(va, vb), model::map2(a, b, |x, y| x > y)),
        ];
        for (name, got, want) in cases {
            same(got.to_array(), want, || format!("{name} {a:?} {b:?}"))?;
        }
    }
    Ok(String::new())
}

fn bounds() -> Outcome {
    let mut rng = Rng::new(53);
    for (a, b) in special_pairs(&mut rng) {
        let got = vmx::bounds(black_box(f32x4::from_array(a)), black_box(f32x4::from_array(b)));
        same(got.to_array(), model::map2(a, b, model::bounds), || format!("{a:?} {b:?}"))?;
    }
    Ok(String::new())
}

fn select_bits() -> Outcome {
    let mut rng = Rng::new(59);
    for round in 0..ROUNDS {
        let (m, a, b) = (random_words(&mut rng), random_words(&mut rng), random_words(&mut rng));
        let got = vmx::select_bits(
            black_box(u32x4::from_array(m)),
            black_box(u32x4::from_array(a)),
            black_box(u32x4::from_array(b)),
        );
        let want: [u32; 4] = core::array::from_fn(|i| black_box(a[i] & m[i] | b[i] & !m[i]));
        same(got.to_array(), want, || format!("round {round}"))?;
    }
    Ok(String::new())
}

fn word_shifts() -> Outcome {
    let mut rng = Rng::new(61);
    for round in 0..ROUNDS {
        let (a, n) = (random_words(&mut rng), random_words(&mut rng));
        let (va, vn) = (black_box(u32x4::from_array(a)), black_box(u32x4::from_array(n)));
        let cases: [(&str, [u32; 4], [u32; 4]); 4] = [
            ("shl", vmx::shl_words(va, vn).to_array(), model::map2(a, n, |x, n| x << (n & 31))),
            ("shr", vmx::shr_words(va, vn).to_array(), model::map2(a, n, |x, n| x >> (n & 31))),
            (
                "sra",
                vmx::sra_words(va.cast(), vn).cast::<u32>().to_array(),
                model::map2(a, n, |x, n| ((x as i32) >> (n & 31)) as u32),
            ),
            ("rotate", vmx::rotate_words(va, vn).to_array(), model::map2(a, n, |x, n| x.rotate_left(n & 31))),
        ];
        for (name, got, want) in cases {
            same(got, want, || format!("{name} round {round}"))?;
        }
    }
    Ok(String::new())
}

fn permute_bytes() -> Outcome {
    let mut rng = Rng::new(67);
    for round in 0..ROUNDS {
        let a: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let b: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let control: [u8; 16] = core::array::from_fn(|_| rng.next_u32() as u8);
        let got = vmx::permute_bytes(
            black_box(u8x16::from_array(a)),
            black_box(u8x16::from_array(b)),
            black_box(u8x16::from_array(control)),
        );
        let want: [u8; 16] = core::array::from_fn(|i| {
            let index = black_box(control[i] & 31) as usize;
            if index < 16 { a[index] } else { b[index - 16] }
        });
        same(got.to_array(), want, || format!("round {round}"))?;
    }
    Ok(String::new())
}

macro_rules! octets_case {
    ($a:expr, $b:expr, $($n:literal)*) => {
        [$( vmx::shift_left_octets::<$n, _>(black_box($a), black_box($b)).to_array(), )*]
    };
}

fn splat_merge_octets() -> Outcome {
    let a = u32x4::from_array([0x0001_0203, 0x0405_0607, 0x0809_0A0B, 0x0C0D_0E0F]);
    let b = u32x4::from_array([0x1011_1213, 0x1415_1617, 0x1819_1A1B, 0x1C1D_1E1F]);
    let (wa, wb) = (a.to_array(), b.to_array());
    let splats = [
        vmx::splat_word::<0, _>(black_box(a)),
        vmx::splat_word::<1, _>(black_box(a)),
        vmx::splat_word::<2, _>(black_box(a)),
        vmx::splat_word::<3, _>(black_box(a)),
    ];
    for (i, got) in splats.into_iter().enumerate() {
        same(got.to_array(), [wa[i]; 4], || format!("splat {i}"))?;
    }
    same(vmx::merge_high(black_box(a), black_box(b)).to_array(), [wa[0], wb[0], wa[1], wb[1]], || "merge_high".into())?;
    same(vmx::merge_low(black_box(a), black_box(b)).to_array(), [wa[2], wb[2], wa[3], wb[3]], || "merge_low".into())?;
    let (ba, bb): (u8x16, u8x16) = (vmx::cast(a), vmx::cast(b));
    let got = octets_case!(ba, bb, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
    for (n, got) in got.into_iter().enumerate() {
        let want: [u8; 16] = core::array::from_fn(|i| (n + i) as u8);
        same(got, want, || format!("shift_left_octets {n}"))?;
    }
    Ok(String::new())
}

/// Every mantissa of `binades` consecutive binades starting at `first`,
/// four at a time; returns the largest relative error against `exact`.
fn sweep(first: f32, binades: u32, estimate: fn(f32x4) -> f32x4, exact: fn(f64) -> f64) -> (f64, f32) {
    let start = first.to_bits();
    let mut worst = (0.0f64, first);
    for step in 0..(binades << 21) {
        let base = start + 4 * step;
        let x = f32x4::from_bits(u32x4::from_array([base, base + 1, base + 2, base + 3]));
        let got = estimate(black_box(x)).to_array();
        for (i, g) in got.into_iter().enumerate() {
            let xi = f32::from_bits(base + i as u32);
            let want = exact(xi as f64);
            let error = ((g as f64 - want) / want).abs();
            if error > worst.0 {
                worst = (error, xi);
            }
        }
    }
    worst
}

fn estimate_outcome(worst: (f64, f32), bound: f64, what: &str) -> Outcome {
    let summary = format!("{what}: max relative error {:.3e} = 2^{:.2} at {:e}", worst.0, worst.0.log2(), worst.1);
    if worst.0 <= bound { Ok(summary) } else { Err(format!("{summary}, bound {bound:.3e}")) }
}

/// The estimate depends on the mantissa only: every mantissa of [1, 2),
/// then exponent scaling on random inputs. Bound measured on the console.
fn recip_estimate() -> Outcome {
    const BOUND: f64 = 1.0 / 4096.0;
    let worst = sweep(1.0, 1, vmx::recip_estimate, |x| 1.0 / x);
    let mut rng = Rng::new(71);
    let mut scaled = (0.0f64, 0.0f32);
    for _ in 0..ROUNDS {
        let x = floats(&mut rng, 100);
        let got = vmx::recip_estimate(black_box(f32x4::from_array(x))).to_array();
        for i in 0..4 {
            let want = 1.0 / x[i] as f64;
            let error = ((got[i] as f64 - want) / want).abs();
            if error > scaled.0 {
                scaled = (error, x[i]);
            }
        }
    }
    let first = estimate_outcome(worst, BOUND, "[1,2) exhaustive")?;
    let other = estimate_outcome(scaled, worst.0, "2^-100..2^100 sampled")?;
    Ok(format!("{first}; {other}"))
}

fn rsqrt_estimate() -> Outcome {
    const BOUND: f64 = 1.0 / 4096.0;
    let worst = sweep(1.0, 2, vmx::rsqrt_estimate, |x| 1.0 / x.sqrt());
    estimate_outcome(worst, BOUND, "[1,4) exhaustive")
}

fn exp2_estimate() -> Outcome {
    const BOUND: f64 = 1.0 / 16.0;
    let mut rng = Rng::new(73);
    let mut worst = (0.0f64, 0.0f32);
    for _ in 0..65536 {
        let x: [f32; 4] = core::array::from_fn(|_| rng.small(1 << 20) / 32768.0);
        let got = vmx::exp2_estimate(black_box(f32x4::from_array(x))).to_array();
        for i in 0..4 {
            let want = (x[i] as f64).exp2();
            let error = ((got[i] as f64 - want) / want).abs();
            if error > worst.0 {
                worst = (error, x[i]);
            }
        }
    }
    estimate_outcome(worst, BOUND, "x in [-32, 32] sampled")
}

/// Absolute error, in the result's units.
fn log2_estimate() -> Outcome {
    const BOUND: f64 = 1.0 / 16.0;
    let mut rng = Rng::new(79);
    let mut worst = (0.0f64, 0.0f32);
    for _ in 0..65536 {
        let x = floats(&mut rng, 100).map(f32::abs);
        let got = vmx::log2_estimate(black_box(f32x4::from_array(x))).to_array();
        for i in 0..4 {
            let error = (got[i] as f64 - (x[i] as f64).log2()).abs();
            if error > worst.0 {
                worst = (error, x[i]);
            }
        }
    }
    let summary = format!("max absolute error {:.3e} = 2^{:.2} at {:e}", worst.0, worst.0.log2(), worst.1);
    if worst.0 <= BOUND { Ok(summary) } else { Err(format!("{summary}, bound {BOUND:.3e}")) }
}
