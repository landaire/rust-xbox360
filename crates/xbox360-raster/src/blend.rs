//! Integer pixel arithmetic. Red/blue and green/alpha are processed as two
//! 16-bit lanes of one `u32` where the intermediate values fit. Per-pixel
//! helpers are always inlined for the reason given in `shade`.

use crate::draw::BlendMode;
use crate::draw::Rgba8;

/// Bits per channel of an `Rgba8`.
pub(super) const CHANNEL_BITS: u32 = 8;
/// The largest channel value, which is also full opacity.
const CHANNEL_MAX: u32 = u8::MAX as u32;
/// Red and blue (or green and alpha) of an `Rgba8` as two 16-bit lanes.
const LOW_LANES: u32 = 0x00FF_00FF;
/// The lowest bit of each 16-bit lane.
const LANE_ONES: u32 = 0x0001_0001;
/// Half of `2^CHANNEL_BITS` in each lane, for rounding a right shift by
/// [`CHANNEL_BITS`].
const LANE_HALVES: u32 = LANE_ONES << (CHANNEL_BITS - 1);
/// Half of `2^CHANNEL_BITS`, for rounding a right shift by
/// [`CHANNEL_BITS`].
const HALF_STEP: u32 = 1 << (CHANNEL_BITS - 1);

/// `src` holds at least as many pixels as `dst`.
// Closures rather than function items: a function item passed as
// `impl Fn` is called out of line when optimizing for size.
#[expect(clippy::redundant_closure)]
pub(super) fn blend_span(dst: &mut [Rgba8], src: &[Rgba8], mode: BlendMode) {
    match mode {
        BlendMode::Normal => normal_span(dst, src),
        BlendMode::Additive => combine(dst, src, |target, source| additive(target, source)),
        BlendMode::Modulate => combine(dst, src, |target, source| modulate(target, source)),
        BlendMode::Min => combine(dst, src, |target, source| min(target, source)),
        BlendMode::Max => combine(dst, src, |target, source| max(target, source)),
        BlendMode::ReverseSubtract => {
            combine(dst, src, |target, source| reverse_subtract(target, source))
        }
    }
}

pub(super) fn blend_solid(dst: &mut [Rgba8], color: Rgba8, mode: BlendMode) {
    match mode {
        BlendMode::Normal if color.alpha() == u8::MAX => dst.fill(color),
        // A zero source leaves the target unchanged.
        BlendMode::Normal | BlendMode::Additive if color == Rgba8::TRANSPARENT => {}
        BlendMode::Normal => {
            let inverse_alpha = CHANNEL_MAX - u32::from(color.alpha());
            for target in dst {
                *target = over(*target, color, inverse_alpha);
            }
        }
        BlendMode::Modulate if color == Rgba8::WHITE => {
            // Modulating by white is the identity.
        }
        BlendMode::Additive => {
            for target in dst {
                *target = additive(*target, color);
            }
        }
        BlendMode::Modulate => {
            for target in dst {
                *target = modulate(*target, color);
            }
        }
        BlendMode::Min => {
            for target in dst {
                *target = min(*target, color);
            }
        }
        BlendMode::Max => {
            for target in dst {
                *target = max(*target, color);
            }
        }
        BlendMode::ReverseSubtract => {
            for target in dst {
                *target = reverse_subtract(*target, color);
            }
        }
    }
}

/// Undoes premultiplication: `round(c * 255 / a)` per color channel, with
/// alpha 0 giving 0. The reciprocal gives the exact quotient because every
/// dividend is below `2^RECIPROCAL_BITS / 255`.
#[inline(always)]
pub(super) fn unpremultiply(texel: Rgba8) -> Rgba8 {
    const RECIPROCAL_BITS: u32 = 24;
    const ALPHAS: usize = 1 << CHANNEL_BITS;
    const RECIPROCALS: [u64; ALPHAS] = {
        let mut table = [0; ALPHAS];
        let mut alpha = 1;
        while alpha < ALPHAS {
            table[alpha] = (1u64 << RECIPROCAL_BITS).div_ceil(alpha as u64);
            alpha += 1;
        }
        table
    };

    let alpha = texel.alpha();
    if alpha == 0 {
        return Rgba8::TRANSPARENT;
    }

    let reciprocal = RECIPROCALS[usize::from(alpha)];
    let straight = |channel: u8| {
        let dividend = u64::from(channel) * u64::from(CHANNEL_MAX) + u64::from(alpha / 2);
        min_u64(
            (dividend * reciprocal) >> RECIPROCAL_BITS,
            u64::from(CHANNEL_MAX),
        ) as u8
    };

    Rgba8::new(
        straight(texel.red()),
        straight(texel.green()),
        straight(texel.blue()),
        alpha,
    )
}

/// Multiplies each channel of a texel by the matching vertex color channel.
#[inline(always)]
pub(super) fn modulate(texel: Rgba8, color: Rgba8) -> Rgba8 {
    Rgba8::new(
        mul_div_255(texel.red(), color.red()),
        mul_div_255(texel.green(), color.green()),
        mul_div_255(texel.blue(), color.blue()),
        mul_div_255(texel.alpha(), color.alpha()),
    )
}

/// `dst = src + dst * (255 - src.a) / 255`, saturated. Branchless: the
/// scaling is exact for factors 0 and 255, and branches on texel alpha
/// mispredict on the console.
#[inline(always)]
fn normal(dst: Rgba8, src: Rgba8) -> Rgba8 {
    over(dst, src, CHANNEL_MAX - u32::from(src.alpha()))
}

#[inline(always)]
fn over(dst: Rgba8, src: Rgba8, inverse_alpha: u32) -> Rgba8 {
    let red_blue = add_saturate_lanes(
        scale_lanes(dst.0 & LOW_LANES, inverse_alpha),
        src.0 & LOW_LANES,
    );
    let green_alpha = add_saturate_lanes(
        scale_lanes((dst.0 >> CHANNEL_BITS) & LOW_LANES, inverse_alpha),
        (src.0 >> CHANNEL_BITS) & LOW_LANES,
    );

    Rgba8(red_blue | green_alpha << CHANNEL_BITS)
}

#[inline(always)]
fn additive(dst: Rgba8, src: Rgba8) -> Rgba8 {
    let red_blue = add_saturate_lanes(dst.0 & LOW_LANES, src.0 & LOW_LANES);
    let green_alpha = add_saturate_lanes(
        (dst.0 >> CHANNEL_BITS) & LOW_LANES,
        (src.0 >> CHANNEL_BITS) & LOW_LANES,
    );

    Rgba8(red_blue | green_alpha << CHANNEL_BITS)
}

#[inline(always)]
fn reverse_subtract(dst: Rgba8, src: Rgba8) -> Rgba8 {
    let red_blue = sub_saturate_lanes(src.0 & LOW_LANES, dst.0 & LOW_LANES);
    let green_alpha = sub_saturate_lanes(
        (src.0 >> CHANNEL_BITS) & LOW_LANES,
        (dst.0 >> CHANNEL_BITS) & LOW_LANES,
    );

    Rgba8(red_blue | green_alpha << CHANNEL_BITS)
}

#[inline(always)]
fn min(dst: Rgba8, src: Rgba8) -> Rgba8 {
    let channel = |dst: u8, src: u8| if src < dst { src } else { dst };

    Rgba8::new(
        channel(dst.red(), src.red()),
        channel(dst.green(), src.green()),
        channel(dst.blue(), src.blue()),
        channel(dst.alpha(), src.alpha()),
    )
}

#[inline(always)]
fn max(dst: Rgba8, src: Rgba8) -> Rgba8 {
    let channel = |dst: u8, src: u8| if src > dst { src } else { dst };

    Rgba8::new(
        channel(dst.red(), src.red()),
        channel(dst.green(), src.green()),
        channel(dst.blue(), src.blue()),
        channel(dst.alpha(), src.alpha()),
    )
}

/// [`normal`] over a span, with VMX where the target supports it and the
/// two slices share their alignment within 16 bytes.
#[expect(clippy::redundant_closure, reason = "see `blend_span`")]
fn normal_span(dst: &mut [Rgba8], src: &[Rgba8]) {
    #[cfg(all(target_arch = "powerpc", target_os = "xbox360"))]
    {
        use super::vmx::LANES;

        let phase = |pointer: *const Rgba8| (pointer.addr() / size_of::<Rgba8>()) % LANES;
        let src = &src[..dst.len()];
        if dst.len() >= 2 * LANES && phase(dst.as_ptr()) == phase(src.as_ptr()) {
            let head = (LANES - phase(dst.as_ptr())) % LANES;
            let body = (dst.len() - head) / LANES * LANES;
            let (dst_head, dst_rest) = dst.split_at_mut(head);
            let (dst_body, dst_tail) = dst_rest.split_at_mut(body);
            combine(dst_head, &src[..head], |target, source| {
                normal(target, source)
            });
            // SAFETY: every Xbox 360 CPU implements VMX.
            unsafe { super::vmx::normal(dst_body, &src[head..head + body]) };
            combine(dst_tail, &src[head + body..], |target, source| {
                normal(target, source)
            });
            return;
        }
    }

    combine(dst, src, |target, source| normal(target, source));
}

/// `(a * (256 - weight) + b * weight + 128) / 256` per channel; `weight <
/// 256`. Written as `(b - a) * weight + a * 256` so that one multiply covers
/// two channels: the wrapping difference borrows across lanes, but every
/// lane's exact result lies in `0..2^16`, so the total is exact.
#[inline(always)]
pub(super) fn lerp(a: Rgba8, b: Rgba8, weight: u32) -> Rgba8 {
    let lane_lerp = |a: u32, b: u32| {
        b.wrapping_sub(a)
            .wrapping_mul(weight)
            .wrapping_add(a << CHANNEL_BITS)
            .wrapping_add(LANE_HALVES)
    };
    let red_blue = lane_lerp(a.0 & LOW_LANES, b.0 & LOW_LANES) >> CHANNEL_BITS;
    let green_alpha = lane_lerp(
        (a.0 >> CHANNEL_BITS) & LOW_LANES,
        (b.0 >> CHANNEL_BITS) & LOW_LANES,
    );

    Rgba8((red_blue & LOW_LANES) | (green_alpha & !LOW_LANES))
}

/// Applies `operation` to each target pixel and its source pixel. Indexed
/// loop: iterator adapters are not inlined when optimizing for size.
#[inline(always)]
fn combine(dst: &mut [Rgba8], src: &[Rgba8], operation: impl Fn(Rgba8, Rgba8) -> Rgba8) {
    let src = &src[..dst.len()];
    let mut index = 0;
    while index < dst.len() {
        dst[index] = operation(dst[index], src[index]);
        index += 1;
    }
}

/// `Ord::min` is not inlined when optimizing for size.
#[inline(always)]
fn min_u64(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

/// Rounded `a * b / 255`.
#[inline(always)]
fn mul_div_255(a: u8, b: u8) -> u8 {
    let product = u32::from(a) * u32::from(b) + HALF_STEP;

    ((product + (product >> CHANNEL_BITS)) >> CHANNEL_BITS) as u8
}

/// Rounded `lane * factor / 255` for both lanes; `factor <= 255`.
#[inline(always)]
fn scale_lanes(lanes: u32, factor: u32) -> u32 {
    let product = lanes * factor + LANE_HALVES;

    ((product + ((product >> CHANNEL_BITS) & LOW_LANES)) >> CHANNEL_BITS) & LOW_LANES
}

#[inline(always)]
fn add_saturate_lanes(a: u32, b: u32) -> u32 {
    let sum = a + b;
    let overflow = (sum >> CHANNEL_BITS) & LANE_ONES;

    (sum | (overflow * CHANNEL_MAX)) & LOW_LANES
}

#[inline(always)]
fn sub_saturate_lanes(a: u32, b: u32) -> u32 {
    // Bit 8 of each lane survives exactly when that lane did not borrow.
    let difference = a + (LANE_ONES << CHANNEL_BITS) - b;
    let keep = ((difference >> CHANNEL_BITS) & LANE_ONES) * CHANNEL_MAX;

    difference & keep
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODES: [BlendMode; 6] = [
        BlendMode::Normal,
        BlendMode::Additive,
        BlendMode::Modulate,
        BlendMode::Min,
        BlendMode::Max,
        BlendMode::ReverseSubtract,
    ];

    fn reference_channel(dst: u32, src: u32, src_alpha: u32, mode: BlendMode) -> u32 {
        match mode {
            BlendMode::Normal => (src + (dst * (255 - src_alpha) + 127) / 255).min(255),
            BlendMode::Additive => (src + dst).min(255),
            BlendMode::Modulate => (dst * src + 127) / 255,
            BlendMode::Min => dst.min(src),
            BlendMode::Max => dst.max(src),
            BlendMode::ReverseSubtract => src.saturating_sub(dst),
        }
    }

    /// Lane arithmetic and the constant-source paths match per-channel
    /// integer formulas for every premultiplied source channel and alpha
    /// against every target value.
    #[test]
    fn lane_arithmetic_matches_channel_formulas() {
        for mode in MODES {
            for src_alpha in 0..=255u32 {
                for src in (0..=src_alpha).step_by(3).chain([src_alpha]) {
                    for dst in 0..=255u32 {
                        let source = Rgba8::new(
                            src as u8,
                            (src_alpha - src) as u8,
                            src as u8,
                            src_alpha as u8,
                        );
                        let target = Rgba8::new(dst as u8, dst as u8, (255 - dst) as u8, dst as u8);

                        let mut blended = [target];
                        blend_span(&mut blended, &[source], mode);
                        let mut solid = [target];
                        blend_solid(&mut solid, source, mode);

                        let expected = Rgba8::new(
                            reference_channel(dst, src, src_alpha, mode) as u8,
                            reference_channel(dst, src_alpha - src, src_alpha, mode) as u8,
                            reference_channel(255 - dst, src, src_alpha, mode) as u8,
                            reference_channel(dst, src_alpha, src_alpha, mode) as u8,
                        );
                        assert_eq!(
                            blended[0], expected,
                            "{mode:?} src {source:?} dst {target:?}"
                        );
                        assert_eq!(solid[0], expected, "{mode:?} src {source:?} dst {target:?}");
                    }
                }
            }
        }
    }

    /// The arithmetic of the VMX `normal` kernel, in scalar `f32`: the
    /// fused multiply-add of every channel and factor is exact, so after
    /// the integer rounding steps it equals `scale_lanes`.
    #[test]
    fn vector_scale_formula_matches_lanes() {
        extern crate std;

        for channel in 0..=255u32 {
            for factor in 0..=255u32 {
                let product =
                    std::primitive::f32::mul_add(channel as f32, factor as f32, 128.0) as u32;
                let vector = (product + (product >> 8)) >> 8;
                assert_eq!(
                    vector,
                    scale_lanes(channel, factor),
                    "channel {channel} factor {factor}"
                );
            }
        }
    }

    #[test]
    fn modulate_rounds_to_nearest() {
        for texel in 0..=255u32 {
            for color in 0..=255u32 {
                let expected = ((texel * color + 127) / 255) as u8;
                assert_eq!(mul_div_255(texel as u8, color as u8), expected);
            }
        }
    }

    #[test]
    fn unpremultiply_rounds_to_nearest() {
        for alpha in 1..=255u32 {
            for channel in 0..=alpha {
                let expected = ((channel * 255 + alpha / 2) / alpha) as u8;
                let straight = unpremultiply(Rgba8::new(channel as u8, 0, 0, alpha as u8));
                assert_eq!(
                    straight,
                    Rgba8::new(expected, 0, 0, alpha as u8),
                    "channel {channel} alpha {alpha}"
                );
            }
        }

        assert_eq!(unpremultiply(Rgba8::new(9, 9, 9, 0)), Rgba8::TRANSPARENT);
    }
}
