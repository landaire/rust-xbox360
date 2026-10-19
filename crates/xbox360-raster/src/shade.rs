//! Per-pixel color of a triangle. Attribute planes are set up once per
//! triangle in `f64` (IEEE operations without contraction, so identical on
//! every host) and converted to fixed point with 32 or 16 fraction bits. From there everything is
//! integer: each column that is a multiple of [`CHUNK`] evaluates the plane
//! exactly, which bounds the accumulated step error and makes each pixel's
//! value independent of where its span starts, and pixels step from it in
//! 16.16. No float conversion or 64-bit division happens per span, which on
//! the 32-bit console would be a library call each.
//!
//! Per-pixel helpers are always inlined: on the in-order console CPU a call
//! passes values through the stack, and reloading them stalls on the
//! preceding store.

use crate::draw::AddressMode;
use crate::draw::BlendMode;
use crate::draw::Filter;
use crate::draw::Rgba8;
use crate::draw::Sampler;
use crate::draw::TextureMapping;

use super::blend::blend_solid;
use super::blend::blend_span;
use super::blend::lerp;
use super::blend::modulate;
use super::blend::unpremultiply;
use super::raster::SUBPIXELS;
use super::raster::Triangle;
use super::raster::VertexColor;

const CHUNK: usize = 256;
/// The normalized texture coordinate radial mapping measures from, on both
/// axes.
const RADIAL_CENTER: f64 = 0.5;
/// Offset of a pixel center from its corner, and the half that rounds a
/// floor to nearest.
const PIXEL_CENTER: f64 = 0.5;
const FRACTION_BITS: u32 = 16;
const ONE: i64 = 1 << FRACTION_BITS;
const HALF: i32 = 1 << (FRACTION_BITS - 1);
/// Fraction bits of a filter weight, the high bits of a 16.16 fraction.
const WEIGHT_BITS: u32 = 8;
/// Pixel alignment of [`Scratch`] colors: a 16-byte vector of pixels.
const VECTOR_PIXELS: usize = VECTOR_BYTES / size_of::<Rgba8>();
/// Bytes per VMX vector; `#[repr(align)]` below repeats it as a literal.
const VECTOR_BYTES: usize = 16;

/// Decoded texture borrowed for one command.
#[derive(Clone, Copy, Debug)]
pub(super) struct TextureRef<'a> {
    pub pixels: &'a [Rgba8],
    pub width: u32,
    pub height: u32,
}

/// Interpolation of one triangle for one command.
pub(super) struct Shader<'a> {
    blend: BlendMode,
    color: ColorSource,
    texture: Option<TexturePlanes<'a>>,
}

impl<'a> Shader<'a> {
    pub fn new(
        triangle: &Triangle,
        texture: Option<TextureRef<'a>>,
        sampler: Sampler,
        blend: BlendMode,
    ) -> Self {
        let setup = PlaneSetup::new(triangle);
        let attributes = &triangle.attributes;

        let colors = attributes.map(|vertex| vertex.color);
        let color = match colors {
            [
                VertexColor::Exact(first),
                VertexColor::Exact(second),
                VertexColor::Exact(third),
            ] if first == second && first == third => ColorSource::Constant(first),
            _ => {
                let channels = colors.map(VertexColor::channels);
                let planes: [FixedPlane; 4] = core::array::from_fn(|channel| {
                    Plane::new(channels.map(|color| color[channel]), &setup).fixed(&setup)
                });
                ColorSource::Gradient {
                    steps: planes.each_ref().map(FixedPlane::floor_step),
                    planes,
                }
            }
        };

        let texture = texture.map(|texture| {
            // Radial mapping works on normalized coordinates around the
            // center; direct mapping on texel coordinates.
            let (u_scale, v_scale, center) = match sampler.mapping {
                TextureMapping::Direct => {
                    (f64::from(texture.width), f64::from(texture.height), 0.0)
                }
                TextureMapping::Radial => (1.0, 1.0, RADIAL_CENTER),
            };
            TexturePlanes::new(
                TextureSampler::new(texture, sampler),
                sampler.mapping,
                Plane::new(
                    attributes.map(|vertex| (vertex.u - center) * u_scale),
                    &setup,
                )
                .fixed(&setup),
                Plane::new(
                    attributes.map(|vertex| (vertex.v - center) * v_scale),
                    &setup,
                )
                .fixed(&setup),
            )
        });

        Self {
            blend,
            color,
            texture,
        }
    }

    /// Shades and blends pixels `start..end` of target row `y`. `scratch`
    /// holds intermediate colors; its contents do not matter.
    pub fn shade_span(
        &self,
        scratch: &mut Scratch,
        row: &mut [Rgba8],
        y: i32,
        start: i32,
        end: i32,
    ) {
        let target = &mut row[start as usize..end as usize];
        if let (None, ColorSource::Constant(color)) = (&self.texture, &self.color) {
            blend_solid(target, *color, self.blend);
            return;
        }

        let mut column = start as usize;
        let mut rest = target;
        while !rest.is_empty() {
            let anchor = column & !(CHUNK - 1);
            let skip = column - anchor;
            let (chunk, remainder) = rest.split_at_mut((CHUNK - skip).min(rest.len()));

            self.shade_chunk(
                scratch,
                chunk,
                ChunkOrigin {
                    column: anchor as i64,
                    row: i64::from(y),
                    skip,
                },
            );

            column += chunk.len();
            rest = remainder;
        }
    }

    fn shade_chunk(&self, scratch: &mut Scratch, target: &mut [Rgba8], origin: ChunkOrigin) {
        let source = scratch.for_target(target);

        match &self.texture {
            // Constant along the row, as in vertical gradients.
            None if self.color.is_constant_along_rows() => {
                let color = self.color.first(origin);
                blend_solid(target, color, self.blend);
                return;
            }
            None => {
                self.color.for_each_color(origin, source, StoreColor);
            }
            Some(texture) => {
                let u = texture.u.coordinate(origin, texture.u_step);
                let v = texture.v.coordinate(origin, texture.v_step);
                let straight = self.blend.uses_straight_source();
                match texture.mapping {
                    TextureMapping::Radial => texture.sampler.fill_radial(source, u, v),
                    TextureMapping::Direct => {
                        let (u, v) = (u.rebase(&texture.sampler.u), v.rebase(&texture.sampler.v));
                        let aligned = if texture.is_aligned {
                            texture.sampler.aligned_row(u.at(0), v.at(0), source.len())
                        } else {
                            None
                        };
                        match (aligned, &self.color) {
                            (Some(texels), ColorSource::Constant(Rgba8::WHITE)) if !straight => {
                                blend_span(target, texels, self.blend);
                                return;
                            }
                            (Some(texels), _) => source.copy_from_slice(texels),
                            (None, _) => texture.sampler.fill(source, u, v),
                        }
                    }
                }

                if straight {
                    for pixel in source.iter_mut() {
                        *pixel = unpremultiply(*pixel);
                    }
                }

                self.color.modulate(source, origin);
            }
        }

        blend_span(target, source, self.blend);
    }
}

/// Intermediate colors of one chunk, reused across spans to avoid clearing
/// it for each. Colors are placed at the target's alignment within 16
/// bytes, so that vector code can process both together.
#[repr(align(16))]
pub(super) struct Scratch([Rgba8; CHUNK + VECTOR_PIXELS - 1]);

impl Scratch {
    pub fn new() -> Self {
        Self([Rgba8::TRANSPARENT; CHUNK + VECTOR_PIXELS - 1])
    }

    fn for_target(&mut self, target: &[Rgba8]) -> &mut [Rgba8] {
        let phase = (target.as_ptr().addr() / size_of::<Rgba8>()) % VECTOR_PIXELS;

        &mut self.0[phase..phase + target.len()]
    }
}

/// Where stepping starts: the plane is evaluated at the chunk's anchor
/// column and advanced `skip` pixels to the first shaded pixel.
#[derive(Clone, Copy, Debug)]
struct ChunkOrigin {
    /// Target pixel of the anchor.
    column: i64,
    row: i64,
    skip: usize,
}

/// Vertex positions relative to the first vertex, in pixels.
struct PlaneSetup {
    origin: [f64; 2],
    first: [f64; 2],
    second: [f64; 2],
    inverse_area: f64,
}

impl PlaneSetup {
    fn new(triangle: &Triangle) -> Self {
        let subpixels = SUBPIXELS as f64;
        let position = |index: usize| {
            [
                f64::from(triangle.x[index]) / subpixels,
                f64::from(triangle.y[index]) / subpixels,
            ]
        };
        let origin = position(0);
        let relative = |index: usize| {
            let point = position(index);
            [point[0] - origin[0], point[1] - origin[1]]
        };

        Self {
            origin,
            first: relative(1),
            second: relative(2),
            inverse_area: (subpixels * subpixels) / triangle.doubled_area as f64,
        }
    }
}

/// `value = origin_value + dx * column_offset + dy * row_offset`.
struct Plane {
    origin_value: f64,
    dx: f64,
    dy: f64,
}

impl Plane {
    fn new(values: [f64; 3], setup: &PlaneSetup) -> Self {
        let first = values[1] - values[0];
        let second = values[2] - values[0];

        Self {
            origin_value: values[0],
            dx: (first * setup.second[1] - second * setup.first[1]) * setup.inverse_area,
            dy: (second * setup.first[0] - first * setup.second[0]) * setup.inverse_area,
        }
    }

    /// The plane in fixed point with its value at the center of target
    /// pixel (0, 0): 32 fraction bits when the coefficients fit, else 16.
    /// The coarse form covers gradients up to 2^31 units per pixel and
    /// values up to 2^45; beyond that coefficients are limited there.
    fn fixed(&self, setup: &PlaneSetup) -> FixedPlane {
        // Fraction bits beyond 16.16 of the fine form.
        const FINE_SHIFT: u32 = FRACTION_BITS;
        const COARSE_SHIFT: u32 = 0;
        const FINE: f64 = (ONE << FINE_SHIFT) as f64;
        const COARSE: f64 = (ONE << COARSE_SHIFT) as f64;
        const GRADIENT_LIMIT: i64 = 1 << 47;
        const VALUE_LIMIT: i64 = 1 << 61;

        let at_zero = self.origin_value
            + self.dx * (PIXEL_CENTER - setup.origin[0])
            + self.dy * (PIXEL_CENTER - setup.origin[1]);
        let fits = |value: f64, limit: i64| value.abs() <= limit as f64;
        let fine = fits(at_zero * FINE, VALUE_LIMIT)
            && fits(self.dx * FINE, GRADIENT_LIMIT)
            && fits(self.dy * FINE, GRADIENT_LIMIT);
        let (scale, shift) = if fine {
            (FINE, FINE_SHIFT)
        } else {
            (COARSE, COARSE_SHIFT)
        };

        FixedPlane {
            at_zero: round_clamped(at_zero * scale, VALUE_LIMIT),
            dx: round_clamped(self.dx * scale, GRADIENT_LIMIT),
            dy: round_clamped(self.dy * scale, GRADIENT_LIMIT),
            shift,
        }
    }
}

/// `value = at_zero + dx * column + dy * row` in fixed point with
/// `16 + shift` fraction bits for target pixel `(column, row)`. Pixel
/// coordinates below 2^14 keep every value below 3 * 2^61 in magnitude, so
/// evaluation cannot overflow.
#[derive(Clone, Copy, Debug)]
struct FixedPlane {
    at_zero: i64,
    dx: i64,
    dy: i64,
    /// Fraction bits beyond 16: 16 or 0.
    shift: u32,
}

impl FixedPlane {
    #[inline(always)]
    fn at(&self, column: i64, row: i64) -> i64 {
        self.at_zero + self.dx * column + self.dy * row
    }

    /// A value of this plane in 16.16, rounded to nearest.
    #[inline(always)]
    fn round(&self, value: i64) -> i64 {
        (value + ((1 << self.shift) >> 1)) >> self.shift
    }

    /// A value of this plane in 16.16, rounded down.
    #[inline(always)]
    fn floor(&self, value: i64) -> i64 {
        value >> self.shift
    }

    /// 16.16 step per pixel, rounded to nearest.
    fn round_step(&self) -> i64 {
        self.round(self.dx)
    }

    /// 16.16 step per pixel, rounded down.
    fn floor_step(&self) -> i64 {
        self.floor(self.dx)
    }

    /// 16.16 coordinates of a chunk, rounded to nearest.
    fn coordinate(&self, origin: ChunkOrigin, step: i64) -> Coordinate {
        let anchor = self.round(self.at(origin.column, origin.row));

        Coordinate {
            anchor,
            step,
            skip: origin.skip,
        }
    }
}

enum ColorSource {
    Constant(Rgba8),
    /// Red, green, blue and alpha planes with their 16.16 steps.
    Gradient {
        planes: [FixedPlane; 4],
        steps: [i64; 4],
    },
}

impl ColorSource {
    fn is_constant_along_rows(&self) -> bool {
        match self {
            Self::Constant(_) => true,
            Self::Gradient { steps, .. } => {
                steps[0] == 0 && steps[1] == 0 && steps[2] == 0 && steps[3] == 0
            }
        }
    }

    /// Multiplies `pixels` by the vertex color of each pixel.
    fn modulate(&self, pixels: &mut [Rgba8], origin: ChunkOrigin) {
        match self {
            Self::Constant(Rgba8::WHITE) => {
                // Modulating by white is the identity.
            }
            Self::Constant(color) => {
                for pixel in pixels.iter_mut() {
                    *pixel = modulate(*pixel, *color);
                }
            }
            Self::Gradient { .. } => self.for_each_color(origin, pixels, ModulateColor),
        }
    }

    /// The color of the first pixel of a chunk.
    fn first(&self, origin: ChunkOrigin) -> Rgba8 {
        let mut color = [Rgba8::TRANSPARENT];
        self.for_each_color(origin, &mut color, StoreColor);

        color[0]
    }

    /// Calls `visit` with each pixel of the chunk and its color. Values at
    /// covered pixels lie within the vertex colors; outside they clamp.
    #[inline(always)]
    fn for_each_color(&self, origin: ChunkOrigin, pixels: &mut [Rgba8], sink: impl ColorSink) {
        // Within these bounds the up to 256 steps of a chunk stay below
        // 2^30, so plain 32-bit adds cannot overflow and match exact
        // evaluation, and the values meet `color_byte`'s precondition.
        const VALUE_LIMIT: i64 = 1 << 29;
        const STEP_LIMIT: i64 = 1 << 21;

        let (anchors, steps) = match self {
            Self::Constant(color) => {
                for pixel in pixels.iter_mut() {
                    sink.apply(pixel, *color);
                }
                return;
            }
            Self::Gradient { planes, steps } => {
                let skip = origin.skip as i64;
                let value = |channel: usize| {
                    planes[channel].floor(planes[channel].at(origin.column, origin.row))
                        + steps[channel] * skip
                };
                ([value(0), value(1), value(2), value(3)], *steps)
            }
        };

        // Written out: array adapters are not inlined when optimizing for
        // size.
        let within = |value: i64, limit: i64| -limit <= value && value <= limit;
        let bounded = within(anchors[0], VALUE_LIMIT)
            && within(anchors[1], VALUE_LIMIT)
            && within(anchors[2], VALUE_LIMIT)
            && within(anchors[3], VALUE_LIMIT)
            && within(steps[0], STEP_LIMIT)
            && within(steps[1], STEP_LIMIT)
            && within(steps[2], STEP_LIMIT)
            && within(steps[3], STEP_LIMIT);
        if bounded {
            let (mut red, mut green, mut blue, mut alpha) = (
                anchors[0] as i32,
                anchors[1] as i32,
                anchors[2] as i32,
                anchors[3] as i32,
            );
            let (red_step, green_step, blue_step, alpha_step) = (
                steps[0] as i32,
                steps[1] as i32,
                steps[2] as i32,
                steps[3] as i32,
            );
            for pixel in pixels.iter_mut() {
                sink.apply(
                    pixel,
                    Rgba8::new(
                        color_byte(red),
                        color_byte(green),
                        color_byte(blue),
                        color_byte(alpha),
                    ),
                );
                red += red_step;
                green += green_step;
                blue += blue_step;
                alpha += alpha_step;
            }
            return;
        }

        // Steep gradients, as across very thin triangles, evaluate each
        // pixel exactly.
        for (index, pixel) in pixels.iter_mut().enumerate() {
            let channel = |channel: usize| {
                let value = anchors[channel] + steps[channel] * index as i64;
                color_byte(clamp_i64(value, -COLOR_BYTE_LIMIT, COLOR_BYTE_LIMIT) as i32)
            };
            sink.apply(
                pixel,
                Rgba8::new(channel(0), channel(1), channel(2), channel(3)),
            );
        }
    }
}

struct TexturePlanes<'a> {
    sampler: TextureSampler<'a>,
    mapping: TextureMapping,
    /// Texel units for direct mapping; normalized units relative to the
    /// center for radial mapping.
    u: FixedPlane,
    v: FixedPlane,
    /// 16.16 texels per pixel along x.
    u_step: i64,
    v_step: i64,
    /// One texel per pixel along x and none along y: texels can be read as
    /// a row when the coordinate falls on a texel center.
    is_aligned: bool,
}

impl<'a> TexturePlanes<'a> {
    fn new(
        sampler: TextureSampler<'a>,
        mapping: TextureMapping,
        u: FixedPlane,
        v: FixedPlane,
    ) -> Self {
        let u_step = u.round_step();
        let v_step = v.round_step();
        let is_aligned = mapping == TextureMapping::Direct && u_step == ONE && v_step == 0;

        Self {
            sampler,
            mapping,
            u,
            v,
            u_step,
            v_step,
            is_aligned,
        }
    }
}

/// 16.16 texel coordinates of a chunk: the value at the anchor column, the
/// step per pixel and the first shaded pixel's distance from the anchor.
/// Every pixel's coordinate is computed from the anchor alone, so it does
/// not depend on where the span starts.
#[derive(Clone, Copy, Debug)]
struct Coordinate {
    anchor: i64,
    step: i64,
    skip: usize,
}

impl Coordinate {
    /// Moves the anchor by whole address periods, which selects the same
    /// texels.
    fn rebase(self, axis: &Axis) -> Self {
        Self {
            anchor: axis.rebase(self.anchor),
            ..self
        }
    }

    /// Coordinate of shaded pixel `index`. The anchor is below 3 * 2^61
    /// and the step below 2^47 in magnitude, and the index below 512, so
    /// this cannot overflow.
    #[inline(always)]
    fn at(self, index: usize) -> i64 {
        self.anchor + self.step * (self.skip + index) as i64
    }

    /// Whether the coordinates of `len` shaded pixels step in `i32`
    /// without overflow, including the half-texel bilinear offset.
    fn is_narrow(self, len: usize) -> bool {
        const NARROW_LIMIT: i64 = 1 << 30;

        let narrow = -NARROW_LIMIT..=NARROW_LIMIT;

        narrow.contains(&self.step)
            && narrow.contains(&self.at(0))
            && narrow.contains(&self.at(len - 1))
    }
}

struct TextureSampler<'a> {
    texture: TextureRef<'a>,
    filter: Filter,
    u: Axis,
    v: Axis,
}

impl<'a> TextureSampler<'a> {
    fn new(texture: TextureRef<'a>, sampler: Sampler) -> Self {
        Self {
            texture,
            filter: sampler.filter,
            u: Axis::new(sampler.address_u, texture.width),
            v: Axis::new(sampler.address_v, texture.height),
        }
    }

    fn fill(&self, out: &mut [Rgba8], u: Coordinate, v: Coordinate) {
        if u.is_narrow(out.len()) && v.is_narrow(out.len()) {
            // Within the narrow range the values wrap identically in `i32`.
            let (mut u_value, u_step) = (u.at(0) as i32, u.step as i32);
            let (mut v_value, v_step) = (v.at(0) as i32, v.step as i32);
            if v_step == 0 {
                self.fill_row(out, u_value, u_step, v_value);
                return;
            }

            match self.filter {
                Filter::Nearest => {
                    for pixel in out.iter_mut() {
                        *pixel = self.nearest(u_value, v_value);
                        u_value = u_value.wrapping_add(u_step);
                        v_value = v_value.wrapping_add(v_step);
                    }
                }
                Filter::Linear => {
                    for pixel in out.iter_mut() {
                        *pixel = self.bilinear(u_value, v_value);
                        u_value = u_value.wrapping_add(u_step);
                        v_value = v_value.wrapping_add(v_step);
                    }
                }
            }
            return;
        }

        for (index, pixel) in out.iter_mut().enumerate() {
            let u_value = self.u.narrow(u.at(index));
            let v_value = self.v.narrow(v.at(index));
            *pixel = match self.filter {
                Filter::Nearest => self.nearest(u_value, v_value),
                Filter::Linear => self.bilinear(u_value, v_value),
            };
        }
    }

    /// [`Self::fill`] for a constant `v`, the common case of quads aligned to
    /// the target: rows and the vertical weight are resolved once, and each
    /// texel column is filtered vertically once while consecutive pixels
    /// share it. Same results as sampling each pixel.
    fn fill_row(&self, out: &mut [Rgba8], u_value: i32, u_step: i32, v_value: i32) {
        // One copy of the loop per horizontal addressing mode keeps the
        // mode dispatch out of the per-pixel code.
        let axis = &self.u;
        match (axis.mode, axis.mask) {
            (AddressMode::Clamp, _) => self.fill_row_with(
                out,
                u_value,
                u_step,
                v_value,
                ClampAddress {
                    last: axis.size as i32 - 1,
                },
            ),
            (AddressMode::Wrap, Some(mask)) => {
                self.fill_row_with(out, u_value, u_step, v_value, MaskAddress { mask })
            }
            (AddressMode::Wrap | AddressMode::Mirror, _) => {
                self.fill_row_with(out, u_value, u_step, v_value, AxisAddress { axis })
            }
        }
    }

    #[inline(always)]
    fn fill_row_with(
        &self,
        out: &mut [Rgba8],
        mut u_value: i32,
        u_step: i32,
        v_value: i32,
        address: impl Address,
    ) {
        match self.filter {
            Filter::Nearest => {
                let row = self.row(self.v.resolve(v_value >> FRACTION_BITS));
                for pixel in out.iter_mut() {
                    *pixel = row[address.resolve(u_value >> FRACTION_BITS) as usize];
                    u_value = u_value.wrapping_add(u_step);
                }
            }
            Filter::Linear => {
                let v = v_value - HALF;
                let rows = ColumnFilter {
                    upper: self.row(self.v.resolve(v >> FRACTION_BITS)),
                    lower: self.row(self.v.resolve((v >> FRACTION_BITS) + 1)),
                    weight: weight(v),
                };

                let first = (u_value - HALF) >> FRACTION_BITS;
                let mut cached = first;
                let (mut left, mut right) = (
                    rows.at(address.resolve(first)),
                    rows.at(address.resolve(first + 1)),
                );
                for pixel in out.iter_mut() {
                    let u = u_value - HALF;
                    let index = u >> FRACTION_BITS;
                    if index != cached {
                        if index == cached + 1 {
                            left = right;
                        } else {
                            left = rows.at(address.resolve(index));
                        }
                        right = rows.at(address.resolve(index + 1));
                        cached = index;
                    }

                    *pixel = lerp(left, right, weight(u));
                    u_value = u_value.wrapping_add(u_step);
                }
            }
        }
    }

    /// Samples a radial gradient from normalized 16.16 offsets to the center.
    /// The radius is the floor of the exact square root in 16.16.
    fn fill_radial(&self, out: &mut [Rgba8], u: Coordinate, v: Coordinate) {
        // Squares of offsets within this bound sum without overflow.
        const OFFSET_LIMIT: i64 = (1 << 31) - 1;

        let width = i64::from(self.texture.width);
        // The center of the texture's middle row: v = 0.5.
        let row = self
            .v
            .narrow(i64::from(self.texture.height) << (FRACTION_BITS - 1));
        for (index, pixel) in out.iter_mut().enumerate() {
            let u_offset = clamp_i64(u.at(index), -OFFSET_LIMIT, OFFSET_LIMIT);
            let v_offset = clamp_i64(v.at(index), -OFFSET_LIMIT, OFFSET_LIMIT);
            let squared = (u_offset * u_offset) as u64 + (v_offset * v_offset) as u64;
            let radius = squared.isqrt() as i64;
            // The radial mapping spans the texture row from the center to
            // half the unit square: `t = 2 * radius`.
            let column = self.u.narrow(2 * radius * width);

            *pixel = match self.filter {
                Filter::Nearest => self.nearest(column, row),
                Filter::Linear => self.bilinear(column, row),
            };
        }
    }

    /// The texels of a chunk sampled one texel per pixel exactly at texel
    /// centers, when they lie inside the texture; equal to [`Self::fill`].
    fn aligned_row(&self, u: i64, v: i64, len: usize) -> Option<&'a [Rgba8]> {
        let (x, y) = match self.filter {
            Filter::Nearest => (u >> FRACTION_BITS, v >> FRACTION_BITS),
            Filter::Linear => {
                const FRACTION_MASK: i64 = ONE - 1;

                let (u, v) = (
                    u.saturating_sub(i64::from(HALF)),
                    v.saturating_sub(i64::from(HALF)),
                );
                if (u | v) & FRACTION_MASK != 0 {
                    return None;
                }
                (u >> FRACTION_BITS, v >> FRACTION_BITS)
            }
        };

        let width = i64::from(self.texture.width);
        if x < 0 || y < 0 || x + len as i64 > width || y >= i64::from(self.texture.height) {
            return None;
        }

        let start = (y * width + x) as usize;
        Some(&self.texture.pixels[start..start + len])
    }

    fn row(&self, y: u32) -> &'a [Rgba8] {
        let width = self.texture.width as usize;
        let start = y as usize * width;

        &self.texture.pixels[start..start + width]
    }

    #[inline(always)]
    fn texel(&self, x: u32, y: u32) -> Rgba8 {
        self.texture.pixels[(y * self.texture.width + x) as usize]
    }

    #[inline(always)]
    fn nearest(&self, u: i32, v: i32) -> Rgba8 {
        self.texel(
            self.u.resolve(u >> FRACTION_BITS),
            self.v.resolve(v >> FRACTION_BITS),
        )
    }

    /// Filters vertically, then horizontally; weights have eight
    /// fractional bits.
    #[inline(always)]
    fn bilinear(&self, u: i32, v: i32) -> Rgba8 {
        let (u, v) = (u - HALF, v - HALF);
        let (column, row) = (u >> FRACTION_BITS, v >> FRACTION_BITS);
        let u_weight = weight(u);
        let v_weight = weight(v);

        let (left, right) = (self.u.resolve(column), self.u.resolve(column + 1));
        let (top, bottom) = (self.v.resolve(row), self.v.resolve(row + 1));
        let left_column = lerp(self.texel(left, top), self.texel(left, bottom), v_weight);
        let right_column = lerp(self.texel(right, top), self.texel(right, bottom), v_weight);

        lerp(left_column, right_column, u_weight)
    }
}

/// What [`ColorSource::for_each_color`] does with each pixel and its color,
/// a trait for the reason given at [`Address`].
trait ColorSink {
    fn apply(&self, pixel: &mut Rgba8, color: Rgba8);
}

struct StoreColor;

impl ColorSink for StoreColor {
    #[inline(always)]
    fn apply(&self, pixel: &mut Rgba8, color: Rgba8) {
        *pixel = color;
    }
}

struct ModulateColor;

impl ColorSink for ModulateColor {
    #[inline(always)]
    fn apply(&self, pixel: &mut Rgba8, color: Rgba8) {
        *pixel = modulate(*pixel, color);
    }
}

/// Two texture rows filtered vertically with one weight.
struct ColumnFilter<'a> {
    upper: &'a [Rgba8],
    lower: &'a [Rgba8],
    weight: u32,
}

impl ColumnFilter<'_> {
    /// `lerp` with weight 0 returns its first argument.
    #[inline(always)]
    fn at(&self, x: u32) -> Rgba8 {
        let x = x as usize;
        if self.weight == 0 {
            self.upper[x]
        } else {
            lerp(self.upper[x], self.lower[x], self.weight)
        }
    }
}

/// Texel addressing of one mode, a trait rather than closures because a
/// closure called from several places is not inlined when optimizing for
/// size.
trait Address {
    fn resolve(&self, index: i32) -> u32;
}

struct ClampAddress {
    last: i32,
}

impl Address for ClampAddress {
    #[inline(always)]
    fn resolve(&self, index: i32) -> u32 {
        clamp_i32(index, 0, self.last) as u32
    }
}

/// Wrapping with a power-of-two size.
struct MaskAddress {
    mask: u32,
}

impl Address for MaskAddress {
    #[inline(always)]
    fn resolve(&self, index: i32) -> u32 {
        index as u32 & self.mask
    }
}

struct AxisAddress<'a> {
    axis: &'a Axis,
}

impl Address for AxisAddress<'_> {
    #[inline(always)]
    fn resolve(&self, index: i32) -> u32 {
        self.axis.resolve(index)
    }
}

/// Bound of the texel indices [`Axis::resolve`] takes: the integer part of
/// a narrow 16.16 coordinate.
const INDEX_RANGE: u32 = 1 << (u32::BITS - 1 - FRACTION_BITS);

/// Texels beyond a clamped edge that [`Axis::narrow`] keeps.
const CLAMP_MARGIN: i64 = 2;

/// Texel addressing along one axis.
struct Axis {
    mode: AddressMode,
    size: u32,
    /// `size` for wrap and clamp, `2 * size` for mirror.
    period: u32,
    /// `u32::MAX / period`, for division-free reduction.
    reciprocal: u32,
    /// `period - 1` when the period is a power of two, so that reduction is
    /// a mask.
    mask: Option<u32>,
    /// Multiple of `period` that makes every texel index non-negative.
    bias: u32,
}

impl Axis {
    fn new(mode: AddressMode, size: u32) -> Self {
        let period = match mode {
            // A mirrored period is the texture and its reflection.
            AddressMode::Mirror => size * 2,
            AddressMode::Clamp | AddressMode::Wrap => size,
        };

        Self {
            mode,
            size,
            period,
            reciprocal: u32::MAX / period,
            mask: period.is_power_of_two().then(|| period - 1),
            bias: period * (INDEX_RANGE / period + 1),
        }
    }

    /// Moves a 16.16 coordinate by whole periods, which selects the same
    /// texels.
    fn rebase(&self, coordinate: i64) -> i64 {
        let period = i64::from(self.period) << FRACTION_BITS;
        match self.mode {
            AddressMode::Clamp => coordinate,
            // Masking and 32-bit division avoid the 64-bit division library
            // call of the 32-bit console where possible.
            AddressMode::Wrap | AddressMode::Mirror if self.mask.is_some() => {
                coordinate & (period - 1)
            }
            AddressMode::Wrap | AddressMode::Mirror => {
                match (i32::try_from(coordinate), i32::try_from(period)) {
                    (Ok(coordinate), Ok(period)) => i64::from(coordinate.rem_euclid(period)),
                    _ => coordinate.rem_euclid(period),
                }
            }
        }
    }

    /// Maps a 16.16 coordinate of any magnitude into the narrow range
    /// without changing the texels it selects.
    #[inline(always)]
    fn narrow(&self, coordinate: i64) -> i32 {
        match self.mode {
            // Two texels beyond either edge select the edge texels like any
            // farther coordinate, also for the bilinear neighbor.
            AddressMode::Clamp => clamp_i64(
                coordinate,
                -CLAMP_MARGIN * ONE,
                (i64::from(self.size) + CLAMP_MARGIN) * ONE,
            ) as i32,
            AddressMode::Wrap | AddressMode::Mirror => self.rebase(coordinate) as i32,
        }
    }

    /// Texel index for an integer coordinate within
    /// `-INDEX_RANGE..INDEX_RANGE`.
    #[inline(always)]
    fn resolve(&self, index: i32) -> u32 {
        match self.mode {
            AddressMode::Clamp => clamp_i32(index, 0, self.size as i32 - 1) as u32,
            AddressMode::Wrap => self.reduce(index),
            AddressMode::Mirror => {
                let reduced = self.reduce(index);
                if reduced >= self.size {
                    self.period - 1 - reduced
                } else {
                    reduced
                }
            }
        }
    }

    /// `index mod period`. The reciprocal underestimates the quotient by at
    /// most one for operands below `2^32 / period`, fixed by one subtraction.
    #[inline(always)]
    fn reduce(&self, index: i32) -> u32 {
        // A power-of-two period divides 2^32, so the mask of the two's
        // complement value is the non-negative remainder.
        if let Some(mask) = self.mask {
            return index as u32 & mask;
        }

        let operand = index.wrapping_add(self.bias as i32) as u32;
        let quotient = ((u64::from(operand) * u64::from(self.reciprocal)) >> u32::BITS) as u32;
        let remainder = operand - quotient * self.period;

        if remainder >= self.period {
            remainder - self.period
        } else {
            remainder
        }
    }
}

/// `value` rounded to the nearest integer, ties up, clamped to plus or
/// minus `limit`.
/// Rounding to nearest keeps values a rounding error below an integer.
/// Rounds and clamps in `f64` so that one exact conversion remains: each
/// float to integer conversion costs a library call on the console.
fn round_clamped(value: f64, limit: i64) -> i64 {
    let limit = limit as f64;
    let rounded = floor_f64(value + PIXEL_CENTER);
    let clamped = if rounded < -limit {
        -limit
    } else if rounded > limit {
        limit
    } else {
        rounded
    };

    // A NaN converts to 0.
    clamped as i64
}

/// `floor` without `std`: adding and removing 2^52 rounds to an integer,
/// which a comparison corrects. Values of 2^52 or more, infinities and NaN
/// are returned unchanged.
fn floor_f64(value: f64) -> f64 {
    const INTEGRAL: f64 = 4_503_599_627_370_496.0;

    if value.is_nan() || value.abs() >= INTEGRAL {
        return value;
    }

    let nearest = if value >= 0.0 {
        (value + INTEGRAL) - INTEGRAL
    } else {
        (value - INTEGRAL) + INTEGRAL
    };
    if nearest > value {
        nearest - 1.0
    } else {
        nearest
    }
}

/// The filter weight of a 16.16 coordinate's fraction.
#[inline(always)]
fn weight(coordinate: i32) -> u32 {
    const MASK: i32 = (1 << WEIGHT_BITS) - 1;

    ((coordinate >> (FRACTION_BITS - WEIGHT_BITS)) & MASK) as u32
}

/// The magnitude bound of [`color_byte`]'s input.
const COLOR_BYTE_LIMIT: i64 = 1 << 30;

/// Rounded 16.16 color channel, saturated to a byte; `value` is within
/// plus or minus 2^30. Branchless: the clamps depend on the data.
#[inline(always)]
fn color_byte(value: i32) -> u8 {
    let rounded = (value + HALF) >> FRACTION_BITS;
    const SIGN: u32 = i32::BITS - 1;
    const MAX: i32 = u8::MAX as i32;

    let non_negative = rounded & !(rounded >> SIGN);
    let over = (MAX - non_negative) >> SIGN;

    ((non_negative | over) & MAX) as u8
}

/// `Ord::clamp` is not inlined when optimizing for size.
#[inline(always)]
fn clamp_i32(value: i32, low: i32, high: i32) -> i32 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[inline(always)]
fn clamp_i64(value: i64, low: i64, high: i64) -> i64 {
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The row fast path, with its column cache and per-mode loops, equals
    /// sampling each pixel, for magnification, minification and negative
    /// steps in every addressing mode.
    #[test]
    fn fill_row_matches_per_pixel_sampling() {
        let mut state = 0x1234_5678u32;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state
        };
        let modes = [AddressMode::Clamp, AddressMode::Wrap, AddressMode::Mirror];
        for (width, height) in [(4u32, 4u32), (5, 3), (64, 64), (7, 1)] {
            let pixels: alloc::vec::Vec<Rgba8> =
                (0..width * height).map(|_| Rgba8(next())).collect();
            let texture = TextureRef {
                pixels: &pixels,
                width,
                height,
            };
            for filter in [Filter::Nearest, Filter::Linear] {
                for address_u in modes {
                    for address_v in modes {
                        let sampler = TextureSampler::new(
                            texture,
                            Sampler {
                                filter,
                                address_u,
                                address_v,
                                mapping: TextureMapping::Direct,
                            },
                        );
                        for _ in 0..50 {
                            let start = (next() % (1 << 22)) as i32 - (1 << 21);
                            let step = (next() % (1 << 19)) as i32 - (1 << 18);
                            let v = (next() % (1 << 22)) as i32 - (1 << 21);
                            let mut row = [Rgba8::TRANSPARENT; CHUNK];
                            sampler.fill_row(&mut row, start, step, v);
                            for (index, pixel) in row.iter().enumerate() {
                                let u = start + step * index as i32;
                                let expected = match filter {
                                    Filter::Nearest => sampler.nearest(u, v),
                                    Filter::Linear => sampler.bilinear(u, v),
                                };
                                assert_eq!(
                                    *pixel, expected,
                                    "{width}x{height} {filter:?} {address_u:?} {address_v:?} u {u} v {v}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn round_clamped_matches_integer_rounding() {
        let cases = [
            0.0,
            0.5,
            -0.5,
            1.49,
            -1.5,
            2.5,
            -2.5,
            1e15 + 0.5,
            -1e15 - 0.5,
            4.5e15,
            -4.5e15,
            1e300,
            -1e300,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            123456.75,
            -123456.25,
        ];
        for value in cases {
            let expected = if value.is_nan() {
                0
            } else {
                let floored = value + 0.5;
                let truncated = floored as i64;
                let floor = if (truncated as f64) > floored {
                    truncated.saturating_sub(1)
                } else {
                    truncated
                };
                floor.clamp(-(1 << 61), 1 << 61)
            };
            assert_eq!(round_clamped(value, 1 << 61), expected, "{value}");
        }
    }

    /// Lanes borrow in both directions when the channels of one pixel are
    /// above and below those of the other.
    #[test]
    fn lerp_matches_channel_formula() {
        let reference =
            |a: u32, b: u32, weight: u32| ((a * (256 - weight) + b * weight + 128) >> 8) as u8;
        for weight in 0..256u32 {
            for a in 0..256u32 {
                for b in 0..256u32 {
                    let first = Rgba8::new(a as u8, b as u8, b as u8, a as u8);
                    let second = Rgba8::new(b as u8, a as u8, a as u8, b as u8);
                    let expected = Rgba8::new(
                        reference(a, b, weight),
                        reference(b, a, weight),
                        reference(b, a, weight),
                        reference(a, b, weight),
                    );
                    assert_eq!(
                        lerp(first, second, weight),
                        expected,
                        "a {a} b {b} weight {weight}"
                    );
                }
            }
        }
    }
}
