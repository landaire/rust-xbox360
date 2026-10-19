//! Backend-independent draw list. The UI runtime produces one per frame and a
//! backend (software rasterizer or wgpu) consumes it.

use alloc::vec::Vec;
use core::ops::Range;

use crate::error::DrawListError;

/// Premultiplied RGBA color, one byte per channel, red in the low byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Rgba8(pub u32);

impl Rgba8 {
    pub const TRANSPARENT: Self = Self(0);
    pub const WHITE: Self = Self(0xFFFF_FFFF);

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(red as u32 | (green as u32) << 8 | (blue as u32) << 16 | (alpha as u32) << 24)
    }

    /// Converts an unpremultiplied XUI `0xAARRGGBB` color.
    pub fn from_argb_straight(argb: u32) -> Self {
        let alpha = (argb >> 24) & 0xFF;
        let premultiply = |channel: u32| ((channel * alpha + 127) / 255) as u8;

        Self::new(premultiply((argb >> 16) & 0xFF), premultiply((argb >> 8) & 0xFF), premultiply(argb & 0xFF), alpha as u8)
    }

    pub const fn red(self) -> u8 {
        self.0 as u8
    }

    pub const fn green(self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub const fn blue(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn alpha(self) -> u8 {
        (self.0 >> 24) as u8
    }
}

/// Identifies a texture owned by a backend's texture store. Only
/// [`crate::texture::TextureSlots`] creates ids; a destroyed texture's id
/// never resolves again.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureId {
    slot: u32,
    generation: u32,
}

impl TextureId {
    pub(crate) fn new(slot: u32, generation: u32) -> Self {
        Self { slot, generation }
    }

    pub(crate) fn slot(self) -> u32 {
        self.slot
    }

    pub(crate) fn generation(self) -> u32 {
        self.generation
    }
}

/// One vertex in target pixel coordinates (origin top left, y down).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    /// Normalized texture coordinates. Ignored for untextured commands.
    pub uv: [f32; 2],
    /// Color multiplied with the texel (or used alone when the command has
    /// no texture). Premultiplied, except straight for the blend modes that
    /// [`BlendMode::uses_straight_source`] names.
    pub color: Rgba8,
}

/// How a command's source color combines with the target, per channel
/// including alpha, saturated to 0..=255. `src` is the premultiplied texel
/// times the premultiplied vertex color. `straight` is the unpremultiplied
/// texel times the straight vertex color; a texel with alpha 0
/// unpremultiplies to 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum BlendMode {
    /// `dst = src + dst * (1 - src.a)`.
    #[default]
    Normal,
    /// `dst = src + dst`.
    Additive,
    /// `dst = dst * straight`.
    Modulate,
    /// `dst = min(dst, straight)`.
    Min,
    /// `dst = max(dst, straight)`.
    Max,
    /// `dst = src - dst`.
    ReverseSubtract,
}

impl BlendMode {
    /// Whether the vertex color is straight and texels are unpremultiplied
    /// before modulation, so that a zero opacity keeps the color.
    pub fn uses_straight_source(self) -> bool {
        match self {
            Self::Modulate | Self::Min | Self::Max => true,
            Self::Normal | Self::Additive | Self::ReverseSubtract => false,
        }
    }
}

/// Texture sampling filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Filter {
    Nearest,
    #[default]
    Linear,
}

/// Texture coordinate addressing outside 0..1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AddressMode {
    #[default]
    Clamp,
    Wrap,
    Mirror,
}

/// How interpolated texture coordinates select the sampled point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TextureMapping {
    /// Samples at `(u, v)`.
    #[default]
    Direct,
    /// Samples at `(2 * sqrt((u - 0.5)^2 + (v - 0.5)^2), 0.5)`: a radial
    /// gradient from a texture row, centered at uv `(0.5, 0.5)`.
    Radial,
}

/// Sampler state of a textured command.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Sampler {
    pub filter: Filter,
    pub address_u: AddressMode,
    pub address_v: AddressMode,
    pub mapping: TextureMapping,
}

/// Inclusive-exclusive pixel rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClipRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl ClipRect {
    pub fn intersect(self, other: Self) -> Self {
        Self {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub fn is_empty(self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }
}

/// A run of triangles sharing one texture, blend mode and clip.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawCommand {
    state: DrawState,
    /// Range into [`DrawList::triangles`].
    triangles: Range<u32>,
}

impl DrawCommand {
    pub fn texture(&self) -> Option<TextureId> {
        self.state.texture
    }

    pub fn sampler(&self) -> Sampler {
        self.state.sampler
    }

    pub fn blend(&self) -> BlendMode {
        self.state.blend
    }

    /// `None` draws to the whole target.
    pub fn clip(&self) -> Option<ClipRect> {
        self.state.clip
    }

    /// Range into [`DrawList::triangles`]; never empty.
    pub fn triangles(&self) -> Range<u32> {
        self.triangles.clone()
    }
}

/// Triangles for one frame, drawn in command order. Every index is below
/// the vertex count, and the vertex count and three times the triangle
/// count fit in a `u32`.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawList {
    clear_color: Option<Rgba8>,
    vertices: Vec<Vertex>,
    triangles: Vec<[u32; 3]>,
    commands: Vec<DrawCommand>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { clear_color: None, vertices: Vec::new(), triangles: Vec::new(), commands: Vec::new() }
    }

    /// Color that fills the whole target before the commands, ignoring clips.
    pub fn clear_color(&self) -> Option<Rgba8> {
        self.clear_color
    }

    pub fn set_clear_color(&mut self, color: Option<Rgba8>) {
        self.clear_color = color;
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Vertex indices into [`Self::vertices`].
    pub fn triangles(&self) -> &[[u32; 3]] {
        &self.triangles
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// The triangles of `command`, which must belong to this list.
    pub fn command_triangles(&self, command: &DrawCommand) -> &[[u32; 3]] {
        &self.triangles[command.triangles.start as usize..command.triangles.end as usize]
    }

    /// Removes everything, including the clear color.
    pub fn clear(&mut self) {
        self.clear_color = None;
        self.vertices.clear();
        self.triangles.clear();
        self.commands.clear();
    }

    /// Appends triangles whose indices refer to `vertices`. Consecutive
    /// pushes with equal state extend the previous command. Invalid input
    /// leaves the list unchanged.
    pub fn push(&mut self, state: DrawState, vertices: &[Vertex], triangles: &[[u32; 3]]) -> Result<(), DrawListError> {
        if let Some(&index) = triangles.as_flattened().iter().find(|index| **index as usize >= vertices.len()) {
            return Err(DrawListError::IndexOutOfRange { index, vertex_count: vertices.len() });
        }

        let vertex_count = self.vertices.len() + vertices.len();
        if u32::try_from(vertex_count).is_err() {
            return Err(DrawListError::TooManyVertices { count: vertex_count });
        }

        // Backends address the flattened index buffer with `u32`.
        let triangle_count = self.triangles.len() + triangles.len();
        if triangle_count.checked_mul(3).and_then(|count| u32::try_from(count).ok()).is_none() {
            return Err(DrawListError::TooManyTriangles { count: triangle_count });
        }

        if triangles.is_empty() {
            return Ok(());
        }

        let base = self.vertices.len() as u32;
        let start = self.triangles.len() as u32;
        self.vertices.extend_from_slice(vertices);
        self.triangles.extend(triangles.iter().map(|triangle| triangle.map(|index| base + index)));
        let end = triangle_count as u32;

        if let Some(last) = self.commands.last_mut()
            && last.triangles.end == start
            && last.state == state
        {
            last.triangles.end = end;
            return Ok(());
        }

        self.commands.push(DrawCommand { state, triangles: start..end });

        Ok(())
    }
}

impl Default for DrawList {
    fn default() -> Self {
        Self::new()
    }
}

/// The per-command state of [`DrawList::push`].
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DrawState {
    pub texture: Option<TextureId>,
    pub sampler: Sampler,
    pub blend: BlendMode,
    pub clip: Option<ClipRect>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vertices(count: usize) -> Vec<Vertex> {
        (0..count).map(|index| Vertex { position: [index as f32, 0.0], ..Vertex::default() }).collect()
    }

    #[test]
    fn push_rebases_indices_and_merges_equal_state() {
        let mut list = DrawList::new();
        list.push(DrawState::default(), &vertices(3), &[[0, 1, 2]]).unwrap();
        list.push(DrawState::default(), &vertices(4), &[[0, 1, 2], [0, 2, 3]]).unwrap();
        let additive = DrawState { blend: BlendMode::Additive, ..DrawState::default() };
        list.push(additive, &vertices(3), &[[2, 1, 0]]).unwrap();

        assert_eq!(list.vertices().len(), 10);
        assert_eq!(list.triangles(), [[0, 1, 2], [3, 4, 5], [3, 5, 6], [9, 8, 7]]);
        assert_eq!(list.commands().len(), 2);
        assert_eq!(list.commands()[0].triangles(), 0..3);
        assert_eq!(list.commands()[1].triangles(), 3..4);
        assert_eq!(list.command_triangles(&list.commands()[1]), [[9, 8, 7]]);
    }

    #[test]
    fn push_rejects_out_of_range_index_and_leaves_list_unchanged() {
        let mut list = DrawList::new();
        list.push(DrawState::default(), &vertices(3), &[[0, 1, 2]]).unwrap();
        let before = list.clone();

        let result = list.push(DrawState::default(), &vertices(3), &[[0, 1, 2], [1, 2, 3]]);

        assert_eq!(result, Err(DrawListError::IndexOutOfRange { index: 3, vertex_count: 3 }));
        assert_eq!(list, before);
    }

    #[test]
    fn push_without_triangles_adds_nothing() {
        let mut list = DrawList::new();
        list.push(DrawState::default(), &vertices(3), &[]).unwrap();

        assert!(list.vertices().is_empty());
        assert!(list.commands().is_empty());
    }
}
