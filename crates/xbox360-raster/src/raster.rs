//! Triangle setup and scan conversion. Positions snap to 1/256 pixel;
//! coverage is exact integer edge arithmetic with the top-left fill rule
//! and pixel centers at `(x + 0.5, y + 0.5)`.

use crate::draw::ClipRect;
use crate::draw::Rgba8;
use crate::draw::Vertex;

use super::blend::CHANNEL_BITS;
use super::target::RenderTarget;

const SUBPIXEL_BITS: u32 = 8;
/// Subpixels per pixel along each axis.
pub(super) const SUBPIXELS: i64 = 1 << SUBPIXEL_BITS;
const HALF_SUBPIXELS: i64 = SUBPIXELS / 2;

/// Vertices beyond this distance in pixels are clipped before snapping so
/// that edge arithmetic cannot overflow. Covers every target pixel.
const GUARD_BAND: f64 = RenderTarget::MAX_DIMENSION as f64;

/// A vertex with interpolated attributes, before snapping.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ClipVertex {
    pub x: f64,
    pub y: f64,
    pub u: f64,
    pub v: f64,
    pub color: VertexColor,
}

/// A vertex color: the input value, or interpolated by clipping. Input
/// colors stay integral because converting them to `f64` passes each
/// through memory on the console, and constant colors need no planes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum VertexColor {
    Exact(Rgba8),
    /// Red, green, blue and alpha in 0..=255.
    Interpolated([f64; 4]),
}

impl VertexColor {
    pub fn channels(self) -> [f64; 4] {
        match self {
            Self::Exact(color) => [
                f64::from(color.red()),
                f64::from(color.green()),
                f64::from(color.blue()),
                f64::from(color.alpha()),
            ],
            Self::Interpolated(channels) => channels,
        }
    }
}

impl ClipVertex {
    pub fn from_vertex(vertex: &Vertex) -> Self {
        Self {
            x: f64::from(vertex.position[0]),
            y: f64::from(vertex.position[1]),
            u: f64::from(vertex.uv[0]),
            v: f64::from(vertex.uv[1]),
            color: VertexColor::Exact(vertex.color),
        }
    }

    fn lerp(&self, other: &Self, t: f64) -> Self {
        let mix = |a: f64, b: f64| a + (b - a) * t;
        // Equal colors stay exact: interpolating equal values gives them.
        let color = match (self.color, other.color) {
            (VertexColor::Exact(first), VertexColor::Exact(second)) if first == second => {
                self.color
            }
            (first, second) => {
                let (first, second) = (first.channels(), second.channels());
                VertexColor::Interpolated([
                    mix(first[0], second[0]),
                    mix(first[1], second[1]),
                    mix(first[2], second[2]),
                    mix(first[3], second[3]),
                ])
            }
        };

        Self {
            x: mix(self.x, other.x),
            y: mix(self.y, other.y),
            u: mix(self.u, other.u),
            v: mix(self.v, other.v),
            color,
        }
    }
}

/// A snapped triangle with positive signed area, which with y down is
/// clockwise on screen.
#[derive(Clone, Copy, Debug)]
pub(super) struct Triangle {
    /// Subpixel positions.
    pub x: [i32; 3],
    pub y: [i32; 3],

    pub attributes: [ClipVertex; 3],

    /// Twice the area in square subpixels; positive.
    pub doubled_area: i64,
}

impl Triangle {
    fn new(vertices: [ClipVertex; 3]) -> Option<Self> {
        let x = [
            snap(vertices[0].x),
            snap(vertices[1].x),
            snap(vertices[2].x),
        ];
        let y = [
            snap(vertices[0].y),
            snap(vertices[1].y),
            snap(vertices[2].y),
        ];

        Self::from_snapped(x, y, vertices)
    }

    fn from_snapped(mut x: [i32; 3], mut y: [i32; 3], vertices: [ClipVertex; 3]) -> Option<Self> {
        let mut attributes = vertices;

        let mut doubled_area = (i64::from(x[1]) - i64::from(x[0]))
            * (i64::from(y[2]) - i64::from(y[0]))
            - (i64::from(x[2]) - i64::from(x[0])) * (i64::from(y[1]) - i64::from(y[0]));
        if doubled_area == 0 {
            return None;
        }

        if doubled_area < 0 {
            x.swap(1, 2);
            y.swap(1, 2);
            attributes.swap(1, 2);
            doubled_area = -doubled_area;
        }

        Some(Self {
            x,
            y,
            attributes,
            doubled_area,
        })
    }

    /// Edge setup for the pixels of `window`; `None` when the bounds of
    /// the triangle cover no pixel center inside it.
    pub fn scan(&self, window: ClipRect) -> Option<Scan> {
        let min_y = i64::from(self.y[0].min(self.y[1]).min(self.y[2]));
        let max_y = i64::from(self.y[0].max(self.y[1]).max(self.y[2]));
        let min_x = i64::from(self.x[0].min(self.x[1]).min(self.x[2]));
        let max_x = i64::from(self.x[0].max(self.x[1]).max(self.x[2]));

        // First and last pixel whose center lies within the bounds.
        let top = ((min_y + HALF_SUBPIXELS - 1) >> SUBPIXEL_BITS).max(i64::from(window.top));
        let bottom =
            (((max_y - HALF_SUBPIXELS) >> SUBPIXEL_BITS) + 1).min(i64::from(window.bottom));
        let left = ((min_x + HALF_SUBPIXELS - 1) >> SUBPIXEL_BITS).max(i64::from(window.left));
        let right = (((max_x - HALF_SUBPIXELS) >> SUBPIXEL_BITS) + 1).min(i64::from(window.right));
        if top >= bottom || left >= right {
            return None;
        }

        let wide = [
            self.edge(0, 1, top),
            self.edge(1, 2, top),
            self.edge(2, 0, top),
        ];
        let rows = bottom - top;
        let edges = match [
            wide[0].narrow(rows),
            wide[1].narrow(rows),
            wide[2].narrow(rows),
        ] {
            [Some(first), Some(second), Some(third)] => Edges::Narrow([first, second, third]),
            _ => Edges::Wide(wide),
        };

        // The window lies within the target, whose coordinates fit `i32`.
        Some(Scan {
            edges,
            row: top as i32,
            bottom: bottom as i32,
            left: left as i32,
            right: right as i32,
        })
    }

    /// The constraint of edge `from -> to` starting at pixel row `row`.
    /// With `E(p) = dx * (p.y - a.y) - dy * (p.x - a.x)` the interior is
    /// `E > 0`; pixels with `E == 0` belong to top and left edges only.
    fn edge(&self, from: usize, to: usize, row: i64) -> Edge<i64> {
        let (from_x, from_y) = (i64::from(self.x[from]), i64::from(self.y[from]));
        let delta_x = i64::from(self.x[to]) - from_x;
        let delta_y = i64::from(self.y[to]) - from_y;

        let is_top_left = delta_y < 0 || (delta_y == 0 && delta_x > 0);
        let threshold = if is_top_left { 0 } else { 1 };

        // E at the center of pixel (0, row) and its change per row; E falls
        // by `delta_y * SUBPIXELS` per pixel to the right.
        let center_y = row * SUBPIXELS + HALF_SUBPIXELS;
        let value = delta_x * (center_y - from_y) - delta_y * (HALF_SUBPIXELS - from_x);
        let row_step = delta_x * SUBPIXELS;

        match delta_y {
            // E increases with x: x >= ceil((threshold - value) / divisor).
            ..0 => {
                let divisor = -delta_y * SUBPIXELS;
                Edge::Left(FloorStepper::new(
                    threshold - value + divisor - 1,
                    -row_step,
                    divisor,
                ))
            }
            // E decreases with x: x <= floor((value - threshold) / divisor).
            1.. => {
                let divisor = delta_y * SUBPIXELS;
                Edge::Right(FloorStepper::new(value - threshold, row_step, divisor))
            }
            0 => Edge::Horizontal {
                value: value - threshold,
                step: row_step,
            },
        }
    }
}

/// Row-by-row coverage of one triangle within a window.
pub(super) struct Scan {
    edges: Edges,

    /// Next row and the exclusive end.
    row: i32,
    bottom: i32,
    /// Pixel bounds, exclusive at the end.
    left: i32,
    right: i32,
}

impl Scan {
    /// The next non-empty run of covered pixels, top to bottom.
    pub fn next_span(&mut self) -> Option<Span> {
        while self.row < self.bottom {
            let row = self.row;
            self.row += 1;

            let span = match &mut self.edges {
                Edges::Narrow(edges) => row_span(edges, self.left, self.right),
                Edges::Wide(edges) => row_span(edges, self.left, self.right),
            };
            if let Some(columns) = span {
                return Some(Span {
                    row,
                    start: columns.start,
                    end: columns.end,
                });
            }
        }

        None
    }
}

/// Covered pixels `start..end` of target row `row`.
#[derive(Clone, Copy, Debug)]
pub(super) struct Span {
    pub row: i32,
    pub start: i32,
    pub end: i32,
}

/// Covered columns `start..end` of a row.
struct Columns {
    start: i32,
    end: i32,
}

/// Edge state in 32-bit arithmetic when every value of the scanned rows
/// fits; 64-bit arithmetic is several instructions per operation on the
/// 32-bit console.
enum Edges {
    Narrow([Edge<i32>; 3]),
    Wide([Edge<i64>; 3]),
}

/// The covered columns of the current row of `edges`, which then advance
/// to the next row.
#[inline(always)]
fn row_span<T: Lane>(edges: &mut [Edge<T>; 3], left: i32, right: i32) -> Option<Columns> {
    let mut start = T::from(left);
    let mut end = T::from(right);
    let mut covered = true;
    for edge in edges.iter() {
        match edge {
            Edge::Left(bound) => {
                if bound.quotient > start {
                    start = bound.quotient;
                }
            }
            Edge::Right(bound) => {
                let last = bound.quotient + T::from(1);
                if last < end {
                    end = last;
                }
            }
            Edge::Horizontal { value, .. } => covered &= *value >= T::from(0),
        }
    }

    for edge in edges.iter_mut() {
        edge.advance();
    }

    // Both lie within `left..=right` after the comparisons.
    (covered && start < end).then(|| Columns {
        start: start.to_i32(),
        end: end.to_i32(),
    })
}

/// The integer types edge state is kept in.
trait Lane:
    Copy
    + Ord
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + From<i32>
{
    /// For values known to fit.
    fn to_i32(self) -> i32;
}

impl Lane for i32 {
    fn to_i32(self) -> i32 {
        self
    }
}

impl Lane for i64 {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

enum Edge<T> {
    /// First covered column.
    Left(FloorStepper<T>),
    /// Last covered column.
    Right(FloorStepper<T>),
    /// Row is covered when `value >= 0`.
    Horizontal { value: T, step: T },
}

impl<T: Lane> Edge<T> {
    #[inline(always)]
    fn advance(&mut self) {
        match self {
            Self::Left(bound) | Self::Right(bound) => bound.advance(),
            Self::Horizontal { value, step } => *value += *step,
        }
    }
}

impl Edge<i64> {
    /// This edge in 32-bit arithmetic, if every value it takes over `rows`
    /// rows, and every intermediate sum, fits.
    fn narrow(&self, rows: i64) -> Option<Edge<i32>> {
        // One below the maximum, so that the last column plus one fits.
        let fits = |value: i64| (i64::from(i32::MIN)..i64::from(i32::MAX)).contains(&value);
        let travels =
            |start: i64, step: i64| fits(start) && fits(step) && fits(start + step * rows);
        match self {
            Self::Left(bound) | Self::Right(bound) => {
                // The quotient advances by its step, or one more, per row.
                // Remainder plus step then stays below 2^31.
                const DIVISOR_LIMIT: i64 = 1 << 30;

                let bounded = bound.divisor <= DIVISOR_LIMIT
                    && travels(bound.quotient, bound.quotient_step)
                    && travels(bound.quotient, bound.quotient_step + 1);
                let narrow = bounded.then_some(FloorStepper {
                    quotient: bound.quotient as i32,
                    remainder: bound.remainder as i32,
                    quotient_step: bound.quotient_step as i32,
                    remainder_step: bound.remainder_step as i32,
                    divisor: bound.divisor as i32,
                })?;
                Some(if matches!(self, Self::Left(_)) {
                    Edge::Left(narrow)
                } else {
                    Edge::Right(narrow)
                })
            }
            Self::Horizontal { value, step } => {
                travels(*value, *step).then_some(Edge::Horizontal {
                    value: *value as i32,
                    step: *step as i32,
                })
            }
        }
    }
}

/// Tracks `floor((numerator + n * step) / divisor)` over `n` without
/// division.
struct FloorStepper<T> {
    quotient: T,
    /// In `0..divisor`.
    remainder: T,
    quotient_step: T,
    remainder_step: T,
    divisor: T,
}

impl FloorStepper<i64> {
    fn new(numerator: i64, step: i64, divisor: i64) -> Self {
        let start = floor_div(numerator, divisor);
        let step = floor_div(step, divisor);

        Self {
            quotient: start.quotient,
            remainder: start.remainder,
            quotient_step: step.quotient,
            remainder_step: step.remainder,
            divisor,
        }
    }
}

impl<T: Lane> FloorStepper<T> {
    #[inline(always)]
    fn advance(&mut self) {
        self.quotient += self.quotient_step;
        self.remainder += self.remainder_step;
        if self.remainder >= self.divisor {
            self.quotient += T::from(1);
            self.remainder -= self.divisor;
        }
    }
}

/// The most vertices [`ClipPolygon::clip`] produces, from the bound given
/// there.
const MAX_CLIP_VERTICES: usize = 10;

/// Guard-band clipping of one triangle. Each plane adds at most one vertex
/// per in/out transition, so the clip order bounds the vertex count at 4,
/// 5, 7 and 10 after each plane even when rounding breaks convexity.
struct ClipPolygon {
    vertices: [ClipVertex; MAX_CLIP_VERTICES],
    len: usize,
}

impl ClipPolygon {
    fn clip(triangle: [ClipVertex; 3]) -> Self {
        let mut polygon = Self {
            vertices: [triangle[0]; MAX_CLIP_VERTICES],
            len: triangle.len(),
        };
        polygon.vertices[..3].copy_from_slice(&triangle);

        for plane in [
            GuardPlane::Left,
            GuardPlane::Right,
            GuardPlane::Top,
            GuardPlane::Bottom,
        ] {
            let input = polygon.vertices;
            let input_len = polygon.len;
            polygon.len = 0;
            for index in 0..input_len {
                let previous = &input[(index + input_len - 1) % input_len];
                let current = &input[index];
                match (plane.is_inside(previous), plane.is_inside(current)) {
                    (true, true) => polygon.push(*current),
                    (false, true) => {
                        polygon.push(plane.intersect(previous, current));
                        polygon.push(*current);
                    }
                    (true, false) => polygon.push(plane.intersect(previous, current)),
                    (false, false) => {
                        // Both outside this plane; the edge contributes nothing.
                    }
                }
            }
        }

        polygon
    }

    fn push(&mut self, vertex: ClipVertex) {
        self.vertices[self.len] = vertex;
        self.len += 1;
    }
}

#[derive(Clone, Copy)]
enum GuardPlane {
    Left,
    Right,
    Top,
    Bottom,
}

impl GuardPlane {
    fn is_inside(self, vertex: &ClipVertex) -> bool {
        match self {
            Self::Left | Self::Top => self.coordinate(vertex) >= -GUARD_BAND,
            Self::Right | Self::Bottom => self.coordinate(vertex) <= GUARD_BAND,
        }
    }

    fn coordinate(self, vertex: &ClipVertex) -> f64 {
        match self {
            Self::Left | Self::Right => vertex.x,
            Self::Top | Self::Bottom => vertex.y,
        }
    }

    fn bound(self) -> f64 {
        match self {
            Self::Left | Self::Top => -GUARD_BAND,
            Self::Right | Self::Bottom => GUARD_BAND,
        }
    }

    /// Interpolates from the endpoint nearer the plane, which keeps the
    /// result precise for distant vertices. The order depends only on the
    /// endpoints, so triangles sharing an edge get the same point.
    fn intersect(self, first: &ClipVertex, second: &ClipVertex) -> ClipVertex {
        let bound = self.bound();
        let distance = |vertex: &ClipVertex| (self.coordinate(vertex) - bound).abs();
        let (start, end) =
            if (distance(first), first.x, first.y) < (distance(second), second.x, second.y) {
                (first, second)
            } else {
                (second, first)
            };

        let t = (bound - self.coordinate(start)) / (self.coordinate(end) - self.coordinate(start));
        let mut point = start.lerp(end, t);
        match self {
            Self::Left | Self::Right => point.x = bound,
            Self::Top | Self::Bottom => point.y = bound,
        }

        point
    }
}

struct FloorDivision {
    quotient: i64,
    /// In `0..divisor`.
    remainder: i64,
}

/// Calls `visit` for each snapped, non-degenerate triangle of `vertices`
/// after guard-band clipping. Triangles with non-finite positions are
/// dropped.
pub(super) fn for_each_triangle(vertices: [&Vertex; 3], mut visit: impl FnMut(&Triangle)) {
    let [first, second, third] = vertices;
    let finite = |vertex: &Vertex| vertex.position[0].is_finite() && vertex.position[1].is_finite();
    if !(finite(first) && finite(second) && finite(third)) {
        return;
    }

    let inside = |vertex: &Vertex| {
        vertex.position[0].abs() <= GUARD_BAND as f32
            && vertex.position[1].abs() <= GUARD_BAND as f32
    };
    let vertices = [
        ClipVertex::from_vertex(first),
        ClipVertex::from_vertex(second),
        ClipVertex::from_vertex(third),
    ];
    if inside(first) && inside(second) && inside(third) {
        // Snapped from the `f32` bits: no float to integer conversion.
        let x = [
            snap_f32(first.position[0]),
            snap_f32(second.position[0]),
            snap_f32(third.position[0]),
        ];
        let y = [
            snap_f32(first.position[1]),
            snap_f32(second.position[1]),
            snap_f32(third.position[1]),
        ];
        if let Some(triangle) = Triangle::from_snapped(x, y, vertices) {
            visit(&triangle);
        }
        return;
    }

    let polygon = ClipPolygon::clip(vertices);
    for index in 2..polygon.len {
        let fan = [
            polygon.vertices[0],
            polygon.vertices[index - 1],
            polygon.vertices[index],
        ];
        if let Some(triangle) = Triangle::new(fan) {
            visit(&triangle);
        }
    }
}

/// A window widened by one pixel on each side, in `f32`, for
/// [`misses_window`]. Built once per window: on the console each integer to
/// float conversion passes through memory.
#[derive(Clone, Copy, Debug)]
pub(super) struct WindowBounds {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl WindowBounds {
    pub fn new(window: ClipRect) -> Self {
        Self {
            left: (window.left - 1) as f32,
            top: (window.top - 1) as f32,
            right: (window.right + 1) as f32,
            bottom: (window.bottom + 1) as f32,
        }
    }
}

/// Whether a triangle certainly covers no pixel of the window, decided on
/// the unsnapped `f32` positions without any conversion. A pixel is
/// covered only if its center lies within the snapped bounds, which are
/// within half a subpixel of these, so a margin of one pixel is
/// conservative. Non-finite positions are never rejected here.
#[inline(always)]
pub(super) fn misses_window(vertices: [&Vertex; 3], bounds: WindowBounds) -> bool {
    let [first, second, third] = vertices;
    let (x, y) = (
        [first.position[0], second.position[0], third.position[0]],
        [first.position[1], second.position[1], third.position[1]],
    );
    let below =
        |values: [f32; 3], bound: f32| values[0] < bound && values[1] < bound && values[2] < bound;
    let above =
        |values: [f32; 3], bound: f32| values[0] > bound && values[1] > bound && values[2] > bound;

    below(y, bounds.top)
        || above(y, bounds.bottom)
        || below(x, bounds.left)
        || above(x, bounds.right)
}

/// Corner index bit of the right column of a rectangle.
const RIGHT_COLUMN: u32 = 1;
/// Corner index bit of the bottom row of a rectangle.
const BOTTOM_ROW: u32 = 2;
/// Opposite corner indices differ in both bits.
const OPPOSITE: u32 = RIGHT_COLUMN | BOTTOM_ROW;
/// One bit per corner index.
const ALL_CORNERS: u32 = (1 << (OPPOSITE + 1)) - 1;
/// Corners of a rectangle that each half covers.
const CORNERS_PER_HALF: u32 = 3;

/// A window in `f32` for [`rectangle_covers`], built once per window.
#[derive(Clone, Copy, Debug)]
pub(super) struct CoverBounds {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl CoverBounds {
    pub fn new(window: ClipRect) -> Self {
        Self {
            left: window.left as f32,
            top: window.top as f32,
            right: window.right as f32,
            bottom: window.bottom as f32,
        }
    }
}

/// Whether two triangles are the halves of an axis-aligned rectangle, split
/// along a diagonal, that covers every pixel center of the window. Their
/// shared diagonal is rasterized once, so together they cover the whole
/// rectangle. A rectangle reaching the window's edges covers the centers
/// half a pixel inside them after snapping to the nearest subpixel.
pub(super) fn rectangle_covers(
    first: [&Vertex; 3],
    second: [&Vertex; 3],
    bounds: CoverBounds,
) -> bool {
    // This runs for every pair of every opaque command in every band, so
    // most pairs must fail fast: each half of a covering rectangle spans
    // it, so `first` alone must reach every side. A NaN fails these.
    let [a, b, c] = [first[0].position, first[1].position, first[2].position];
    let reaches = a[0] <= bounds.left || b[0] <= bounds.left || c[0] <= bounds.left;
    let reaches = reaches && (a[0] >= bounds.right || b[0] >= bounds.right || c[0] >= bounds.right);
    let reaches = reaches && (a[1] <= bounds.top || b[1] <= bounds.top || c[1] <= bounds.top);
    if !(reaches && (a[1] >= bounds.bottom || b[1] >= bounds.bottom || c[1] >= bounds.bottom)) {
        return false;
    }

    // `min` and `max` skip a NaN; a NaN vertex then matches no corner.
    let vertices = [
        first[0], first[1], first[2], second[0], second[1], second[2],
    ];
    let [mut left, mut top] = vertices[0].position;
    let (mut right, mut bottom) = (left, top);
    for vertex in &vertices[1..] {
        let [x, y] = vertex.position;
        left = left.min(x);
        right = right.max(x);
        top = top.min(y);
        bottom = bottom.max(y);
    }
    let finite = left.is_finite() && right.is_finite() && top.is_finite() && bottom.is_finite();
    if !(finite
        && left <= bounds.left
        && right >= bounds.right
        && top <= bounds.top
        && bottom >= bounds.bottom)
    {
        return false;
    }

    let corner = |vertex: &Vertex| {
        let [x, y] = vertex.position;
        let column = if x == left {
            0
        } else if x == right {
            RIGHT_COLUMN
        } else {
            return None;
        };
        let row = if y == top {
            0
        } else if y == bottom {
            BOTTOM_ROW
        } else {
            return None;
        };
        Some(column | row)
    };
    // The corner a triangle of three distinct corners leaves out.
    let missing = |triangle: [&Vertex; 3]| {
        let mut seen = 0u32;
        for &vertex in &triangle {
            seen |= 1 << corner(vertex)?;
        }
        (seen.count_ones() == CORNERS_PER_HALF).then(|| (!seen & ALL_CORNERS).trailing_zeros())
    };
    let (Some(first_missing), Some(second_missing)) = (missing(first), missing(second)) else {
        return false;
    };

    // Opposite corners differ in both bits.
    first_missing ^ second_missing == OPPOSITE
}

/// Two triangles that are the halves of an axis-aligned rectangle with
/// affine attributes, drawn as the rectangle: same coverage, one span per
/// row and one attribute setup.
pub(super) struct AlignedRect {
    /// Top-left, top-right and bottom-left corners, for attribute planes.
    pub triangle: Triangle,
    /// Covered pixels, exclusive at the end.
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl AlignedRect {
    /// Triangles a rectangle is made of.
    pub const TRIANGLES: usize = 2;

    /// The rectangle when `first` and `second` split one along a diagonal,
    /// lie within the guard band, agree on the shared corners, and have
    /// attributes that are affine over it (each pair of opposite corners
    /// sums to the same values, so both triangles define one plane).
    pub fn new(first: [&Vertex; 3], second: [&Vertex; 3]) -> Option<Self> {
        // Written without adapters, which are not inlined when optimizing
        // for size. Within the guard band also excludes NaN and infinities.
        let vertices = [
            first[0], first[1], first[2], second[0], second[1], second[2],
        ];
        let [mut left, mut top] = vertices[0].position;
        let (mut right, mut bottom) = (left, top);
        for vertex in &vertices {
            let [x, y] = vertex.position;
            if !(x.abs() <= GUARD_BAND as f32 && y.abs() <= GUARD_BAND as f32) {
                return None;
            }
            left = if x < left { x } else { left };
            right = if x > right { x } else { right };
            top = if y < top { y } else { top };
            bottom = if y > bottom { y } else { bottom };
        }
        if !(left < right && top < bottom) {
            return None;
        }

        let corner = |vertex: &Vertex| {
            let [x, y] = vertex.position;
            let column = if x == left {
                0
            } else if x == right {
                RIGHT_COLUMN
            } else {
                return None;
            };
            let row = if y == top {
                0
            } else if y == bottom {
                BOTTOM_ROW
            } else {
                return None;
            };
            Some(column | row)
        };
        let mut corners: [Option<&Vertex>; OPPOSITE as usize + 1] = [None; OPPOSITE as usize + 1];
        let halves = [first, second];
        let mut missing = [0u32; AlignedRect::TRIANGLES];
        for (half, triangle) in halves.iter().enumerate() {
            let mut seen = 0u32;
            for &vertex in triangle {
                let index = corner(vertex)?;
                seen |= 1 << index;
                let slot = &mut corners[index as usize];
                match slot {
                    Some(other)
                        if other.uv[0] != vertex.uv[0]
                            || other.uv[1] != vertex.uv[1]
                            || other.color != vertex.color =>
                    {
                        return None;
                    }
                    _ => *slot = Some(vertex),
                }
            }
            if seen.count_ones() != CORNERS_PER_HALF {
                return None;
            }
            missing[half] = (!seen & ALL_CORNERS).trailing_zeros();
        }
        // The corners left out must be opposite, differing in both bits, so
        // that the triangles meet along a diagonal.
        if missing[0] ^ missing[1] != OPPOSITE {
            return None;
        }
        let [
            Some(top_left),
            Some(top_right),
            Some(bottom_left),
            Some(bottom_right),
        ] = corners
        else {
            return None;
        };

        // Sums of two `f32` values are exact in `f64`; of two bytes, in
        // `u16`.
        let sum = |a: f32, b: f32| f64::from(a) + f64::from(b);
        let affine_uv = sum(top_left.uv[0], bottom_right.uv[0])
            == sum(top_right.uv[0], bottom_left.uv[0])
            && sum(top_left.uv[1], bottom_right.uv[1]) == sum(top_right.uv[1], bottom_left.uv[1]);
        let channel_sum = |first: Rgba8, second: Rgba8, shift: u32| {
            ((first.0 >> shift) & u32::from(u8::MAX)) + ((second.0 >> shift) & u32::from(u8::MAX))
        };
        let affine_channel = |channel: u32| {
            let shift = channel * CHANNEL_BITS;
            channel_sum(top_left.color, bottom_right.color, shift)
                == channel_sum(top_right.color, bottom_left.color, shift)
        };
        let affine_color =
            affine_channel(0) && affine_channel(1) && affine_channel(2) && affine_channel(3);
        if !(affine_uv && affine_color) {
            return None;
        }

        let (snapped_left, snapped_top, snapped_right, snapped_bottom) = (
            snap_f32(left),
            snap_f32(top),
            snap_f32(right),
            snap_f32(bottom),
        );
        let triangle = Triangle::from_snapped(
            [snapped_left, snapped_right, snapped_left],
            [snapped_top, snapped_top, snapped_bottom],
            [
                ClipVertex::from_vertex(top_left),
                ClipVertex::from_vertex(top_right),
                ClipVertex::from_vertex(bottom_left),
            ],
        )?;

        // Centers on the left and top edges are covered, on the right and
        // bottom edges not, as with the two triangles.
        let first_center = |edge: i32| (edge + HALF_SUBPIXELS as i32 - 1) >> SUBPIXEL_BITS;
        Some(Self {
            triangle,
            left: first_center(snapped_left),
            top: first_center(snapped_top),
            right: first_center(snapped_right),
            bottom: first_center(snapped_bottom),
        })
    }
}

/// [`snap`] of a finite coordinate within the guard band, computed exactly
/// from its bits. On the console a float to integer conversion passes
/// through memory and stalls on the store.
fn snap_f32(coordinate: f32) -> i32 {
    // IEEE single precision: sign, 8 exponent bits biased by 127, and 23
    // stored mantissa bits below an implicit one.
    const MANTISSA_BITS: u32 = f32::MANTISSA_DIGITS - 1;
    const EXPONENT_MASK: u32 = 0xFF;
    const EXPONENT_BIAS: i32 = f32::MAX_EXP - 1;
    const IMPLICIT_ONE: u32 = 1 << MANTISSA_BITS;
    const SIGN_BIT: u32 = u32::BITS - 1;
    // `coordinate * SUBPIXELS = mantissa * 2^(exponent - BIAS - MANTISSA_BITS
    // + SUBPIXEL_BITS)`, so the mantissa has `SCALED_ZERO - exponent`
    // fraction bits.
    const SCALED_ZERO: i32 = EXPONENT_BIAS + MANTISSA_BITS as i32 - SUBPIXEL_BITS as i32;
    // With more fraction bits the magnitude is below a quarter subpixel.
    const NEGLIGIBLE_FRACTION_BITS: i32 = MANTISSA_BITS as i32 + 2;

    let bits = coordinate.to_bits();
    let exponent = ((bits >> MANTISSA_BITS) & EXPONENT_MASK) as i32;
    if exponent == 0 {
        // Zero or subnormal: within half a subpixel of zero.
        return 0;
    }

    let mantissa = ((bits & (IMPLICIT_ONE - 1)) | IMPLICIT_ONE) as i32;
    let negative = bits >> SIGN_BIT != 0;
    let fraction_bits = SCALED_ZERO - exponent;
    if fraction_bits <= 0 {
        // An integral number of subpixels; within the guard band it fits.
        let whole = mantissa << -fraction_bits;
        return if negative { -whole } else { whole };
    }
    if fraction_bits > NEGLIGIBLE_FRACTION_BITS {
        return 0;
    }

    // `floor(a + 0.5)` and `floor(0.5 - a)` for `a = mantissa / 2^fraction_bits`.
    let half = 1 << (fraction_bits - 1);
    if negative {
        -((mantissa + half - 1) >> fraction_bits)
    } else {
        (mantissa + half) >> fraction_bits
    }
}

/// Rounds a guard-band coordinate in pixels to the nearest subpixel.
fn snap(coordinate: f64) -> i32 {
    // Adding a half before the floor rounds to nearest.
    const HALF: f64 = 0.5;

    floor_to_i32(coordinate * SUBPIXELS as f64 + HALF)
}

/// Saturates outside the `i32` range.
fn floor_to_i32(value: f64) -> i32 {
    let truncated = value as i32;
    if f64::from(truncated) > value {
        truncated.saturating_sub(1)
    } else {
        truncated
    }
}

/// `divisor > 0`. Operands that fit 32 bits use the native divide; a 64-bit
/// division is a library call on the 32-bit console.
fn floor_div(numerator: i64, divisor: i64) -> FloorDivision {
    if let (Ok(numerator), Ok(divisor)) = (i32::try_from(numerator), i32::try_from(divisor)) {
        let quotient = numerator / divisor;
        let remainder = numerator - quotient * divisor;
        let (quotient, remainder) = if remainder < 0 {
            (quotient - 1, remainder + divisor)
        } else {
            (quotient, remainder)
        };
        return FloorDivision {
            quotient: i64::from(quotient),
            remainder: i64::from(remainder),
        };
    }

    let quotient = numerator / divisor;
    let remainder = numerator - quotient * divisor;
    if remainder < 0 {
        FloorDivision {
            quotient: quotient - 1,
            remainder: remainder + divisor,
        }
    } else {
        FloorDivision {
            quotient,
            remainder,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bit-level snap equals the `f64` one across the guard band,
    /// including every value near a half subpixel.
    #[test]
    fn snap_f32_matches_snap() {
        let mut state = 0x9E37_79B9u32;
        let check = |value: f32| {
            if value.is_finite() && value.abs() <= GUARD_BAND as f32 {
                assert_eq!(
                    snap_f32(value),
                    snap(f64::from(value)),
                    "{value:e} ({:#010x})",
                    value.to_bits()
                );
            }
        };
        for _ in 0..2_000_000 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            check(f32::from_bits(state));
            check((state % (1 << 24)) as f32 / 256.0 - 32768.0);
        }
        for subpixel in -(1i32 << 23)..(1 << 23) {
            let half = (subpixel as f32 + 0.5) / 256.0;
            check(half);
            check(f32::from_bits(half.to_bits() + 1));
            check(f32::from_bits(half.to_bits() - 1));
        }
        check(0.0);
        check(-0.0);
        check(f32::MIN_POSITIVE);
        check(-f32::from_bits(1));
    }
}
