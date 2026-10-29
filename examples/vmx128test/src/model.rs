//! Scalar reference models. They use integer and scalar floating-point
//! operations only, never the vector unit.

use core::hint::black_box;

/// `f` applied to each word, with every word passed through `black_box` so
/// that the compiler keeps the computation scalar.
pub fn map<T: Copy, U>(x: [T; 4], f: impl Fn(T) -> U) -> [U; 4] {
    core::array::from_fn(|i| black_box(f(black_box(x[i]))))
}

pub fn map2<T: Copy, U>(x: [T; 4], y: [T; 4], f: impl Fn(T, T) -> U) -> [U; 4] {
    core::array::from_fn(|i| black_box(f(black_box(x[i]), black_box(y[i]))))
}

pub fn bits(x: [f32; 4]) -> [u32; 4] {
    x.map(f32::to_bits)
}

/// Rounds toward zero, keeping the sign of zero.
pub fn trunc(x: f32) -> f32 {
    let bits = x.to_bits();
    let exponent = ((bits >> 23) & 0xFF) as i32 - 127;
    if exponent < 0 {
        f32::from_bits(bits & 0x8000_0000)
    } else if exponent >= 23 {
        x
    } else {
        f32::from_bits(bits & !((1u32 << (23 - exponent)) - 1))
    }
}

pub fn floor(x: f32) -> f32 {
    let t = trunc(x);
    if x < 0.0 && t != x { (t as f64 - 1.0) as f32 } else { t }
}

pub fn ceil(x: f32) -> f32 {
    let t = trunc(x);
    if x > 0.0 && t != x { (t as f64 + 1.0) as f32 } else { t }
}

/// Nearest integer, ties to even (`ties_away = false`) or away from zero.
pub fn round(x: f32, ties_away: bool) -> f32 {
    let t = trunc(x);
    let fraction = (x as f64 - t as f64).abs();
    let step = if x < 0.0 { -1.0 } else { 1.0 };
    let odd = (t as f64 / 2.0).fract() != 0.0;
    if fraction > 0.5 || (fraction == 0.5 && (ties_away || odd)) {
        (t as f64 + step) as f32
    } else {
        t
    }
}

/// `x * 2^scale` truncated and saturated; NaN gives 0.
pub fn to_i32(x: f32, scale: u32) -> i32 {
    (x as f64 * (1u64 << scale) as f64) as i32
}

pub fn to_u32(x: f32, scale: u32) -> u32 {
    (x as f64 * (1u64 << scale) as f64) as u32
}

pub fn from_i32(x: i32, scale: u32) -> f32 {
    (x as f64 / (1u64 << scale) as f64) as f32
}

pub fn from_u32(x: u32, scale: u32) -> f32 {
    (x as f64 / (1u64 << scale) as f64) as f32
}

/// Bounds compare: bit 31 when not `a <= b`, bit 30 when not `a >= -b`.
pub fn bounds(a: f32, b: f32) -> u32 {
    (u32::from(!(a <= b)) << 31) | (u32::from(!(a >= -b)) << 30)
}

/// `lvlx` at `block + offset`, `block` 16-byte aligned.
pub fn load_left(block: &[u8; 16], offset: usize) -> [u8; 16] {
    core::array::from_fn(|i| if i < 16 - offset { block[offset + i] } else { 0 })
}

/// `lvrx` at `block + offset`: the bytes before the address, at the end.
pub fn load_right(block: &[u8; 16], offset: usize) -> [u8; 16] {
    core::array::from_fn(|i| if i >= 16 - offset { block[i - (16 - offset)] } else { 0 })
}

// D3D formats. Packing takes every integer field as 3.0 + x * 2^-22 and
// saturates; unpacking gives unsigned fields as 1.0 + x * 2^-23 and signed
// fields as 3.0 + x * 2^-22.

const ONE: u32 = 0x3F80_0000;
pub const THREE: u32 = 0x4040_0000;

fn unsigned_field(value: u32, width: u32) -> u32 {
    let max = (1u32 << width) - 1;
    let x = f32::from_bits(value);
    if x.is_nan() || !(x >= 3.0) {
        0
    } else if value >= THREE + max || x >= 4.0 {
        max
    } else {
        value - THREE
    }
}

fn signed_field(value: u32, width: u32) -> u32 {
    let max = (1i64 << (width - 1)) - 1;
    let min = -max;
    let x = f32::from_bits(value);
    let raw = if x.is_nan() {
        0
    } else if x >= 4.0 {
        max
    } else if !(x >= 2.0) {
        min
    } else {
        (value as i64 - THREE as i64).clamp(min, max)
    };
    raw as u32 & ((1u32 << width) - 1)
}

pub fn unsigned_value(field: u32) -> u32 {
    ONE | field
}

pub fn signed_value(field: u32, width: u32) -> u32 {
    let shift = 32 - width;
    let x = ((field << shift) as i32) >> shift;
    (THREE as i32 + x) as u32
}

/// f32 to the console's half float: no infinity or NaN, truncation toward
/// zero, saturation at 131008, values below 2^-14 to zero.
pub fn half(value: u32) -> u32 {
    let sign = (value >> 16) & 0x8000;
    let exponent = ((value >> 23) & 0xFF) as i32 - 127 + 15;
    if exponent >= 32 {
        sign | 0x7FFF
    } else if exponent <= 0 {
        sign
    } else {
        sign | ((exponent as u32) << 10) | ((value >> 13) & 0x3FF)
    }
}

pub fn unhalf(field: u32) -> u32 {
    let sign = (field & 0x8000) << 16;
    let exponent = (field >> 10) & 0x1F;
    if exponent == 0 {
        sign
    } else {
        sign | ((exponent + 127 - 15) << 23) | ((field & 0x3FF) << 13)
    }
}

/// The packed words of `value` (x, y, z, w as f32 bits): one word for the
/// 32-bit formats, two (high, low) for the 64-bit formats.
pub fn pack(kind: u32, value: [u32; 4]) -> (Option<u32>, u32) {
    let [x, y, z, w] = value;
    match kind {
        0 => (None, unsigned_field(w, 8) << 24 | unsigned_field(x, 8) << 16 | unsigned_field(y, 8) << 8 | unsigned_field(z, 8)),
        1 => (None, signed_field(x, 16) << 16 | signed_field(y, 16)),
        2 => (
            None,
            unsigned_field(w, 2) << 30 | signed_field(z, 10) << 20 | signed_field(y, 10) << 10 | signed_field(x, 10),
        ),
        3 => (None, half(x) << 16 | half(y)),
        4 => (Some(signed_field(x, 16) << 16 | signed_field(y, 16)), signed_field(z, 16) << 16 | signed_field(w, 16)),
        5 => (Some(half(x) << 16 | half(y)), half(z) << 16 | half(w)),
        6 => {
            let packed = u64::from(unsigned_field(w, 4)) << 60
                | u64::from(signed_field(z, 20)) << 40
                | u64::from(signed_field(y, 20)) << 20
                | u64::from(signed_field(x, 20));
            (Some((packed >> 32) as u32), packed as u32)
        }
        _ => unreachable!(),
    }
}

/// x, y, z, w (f32 bits) unpacked from words 2 and 3.
pub fn unpack(kind: u32, high: u32, low: u32) -> [u32; 4] {
    match kind {
        0 => [low >> 16 & 0xFF, low >> 8 & 0xFF, low & 0xFF, low >> 24].map(unsigned_value),
        1 => [signed_value(low >> 16, 16), signed_value(low & 0xFFFF, 16), 0, ONE],
        2 => [
            signed_value(low & 0x3FF, 10),
            signed_value(low >> 10 & 0x3FF, 10),
            signed_value(low >> 20 & 0x3FF, 10),
            unsigned_value(low >> 30),
        ],
        3 => [unhalf(low >> 16), unhalf(low & 0xFFFF), 0, ONE],
        4 => [high >> 16, high & 0xFFFF, low >> 16, low & 0xFFFF].map(|field| signed_value(field, 16)),
        5 => [high >> 16, high & 0xFFFF, low >> 16, low & 0xFFFF].map(unhalf),
        6 => {
            let packed = u64::from(high) << 32 | u64::from(low);
            [
                signed_value(packed as u32 & 0xF_FFFF, 20),
                signed_value((packed >> 20) as u32 & 0xF_FFFF, 20),
                signed_value((packed >> 40) as u32 & 0xF_FFFF, 20),
                unsigned_value((packed >> 60) as u32),
            ]
        }
        _ => unreachable!(),
    }
}

/// Words of `into` that `vpkd3d128` replaces for `mask` and `shift`.
pub fn pack_words(mask: u32, shift: u32) -> &'static [usize] {
    match (mask, shift) {
        (1, s) => [&[3usize][..], &[2], &[1], &[0]][s as usize],
        (2 | 3, 0) => &[2, 3],
        (2 | 3, 1) => &[1, 2],
        (2 | 3, 2) => &[0, 1],
        (2, 3) => &[0],
        (3, 3) => &[3],
        _ => unreachable!(),
    }
}

/// `vpkd3d128` of `value` into `into`.
pub fn pack_into(kind: u32, mask: u32, shift: u32, into: [u32; 4], value: [u32; 4]) -> [u32; 4] {
    let (high, low) = pack(kind, value);
    let packed = [0, 0, high.unwrap_or(0), low];
    let rotated: [u32; 4] = core::array::from_fn(|i| packed[(i + shift as usize) & 3]);
    let mut result = into;
    for &word in pack_words(mask, shift) {
        result[word] = rotated[word];
    }
    result
}
