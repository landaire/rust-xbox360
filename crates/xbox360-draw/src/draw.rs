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
/// unpremultiplies to 0. In a target pass ([`DrawList::begin_target`])
/// the alpha channel of `src` is `src.a * src.a` for [`Self::Normal`],
/// [`Self::Additive`] and [`Self::ReverseSubtract`]: the D3D source alpha
/// factor applied to the alpha channel of a straight target.
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

/// Color channels a command writes, in `D3DCOLORWRITEENABLE` bit order.
/// Channels outside the mask keep their target value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ColorWriteMask(u8);

impl ColorWriteMask {
    pub const RED: Self = Self(0x1);
    pub const GREEN: Self = Self(0x2);
    pub const BLUE: Self = Self(0x4);
    pub const ALPHA: Self = Self(0x8);
    pub const ALL: Self = Self(0xF);

    /// Keeps the four channel bits of `bits`.
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self(bits & Self::ALL.0)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for ColorWriteMask {
    /// Every channel, the device default.
    fn default() -> Self {
        Self::ALL
    }
}

/// A run of triangles sharing one texture, blend mode, clip and color write
/// mask.
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

    pub fn color_write(&self) -> ColorWriteMask {
        self.state.color_write
    }

    pub fn shadow(&self) -> Option<TextShadow> {
        self.state.shadow
    }

    /// Range into [`DrawList::triangles`]; never empty.
    pub fn triangles(&self) -> Range<u32> {
        self.triangles.clone()
    }
}

/// Commands that draw into a render target texture
/// ([`crate::texture::TextureStore::create_target`]) instead of the frame.
/// The pass first clears the whole texture to transparent black. Its
/// commands do not sample its own target.
#[derive(Clone, Debug, PartialEq)]
pub struct TargetPass {
    target: TextureId,
    /// Range into the list's commands.
    commands: Range<u32>,
}

impl TargetPass {
    pub fn target(&self) -> TextureId {
        self.target
    }
}

/// Triangles for one frame. The target passes are drawn first, in order,
/// then the frame commands; within a pass commands are drawn in order. A
/// pass may sample the targets of earlier passes. Every index is below the
/// vertex count, and the vertex count and three times the triangle count
/// fit in a `u32`.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawList {
    clear_color: Option<Rgba8>,
    vertices: Vec<Vertex>,
    triangles: Vec<[u32; 3]>,
    /// Target pass commands, then frame commands from `frame_start`.
    commands: Vec<DrawCommand>,
    passes: Vec<TargetPass>,
    /// Whether pushes go to the last target pass.
    in_target: bool,
    /// End of the last target pass in `commands`.
    frame_start: u32,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            clear_color: None,
            vertices: Vec::new(),
            triangles: Vec::new(),
            commands: Vec::new(),
            passes: Vec::new(),
            in_target: false,
            frame_start: 0,
        }
    }

    /// Color that fills the whole frame before its commands, ignoring clips.
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

    /// Commands drawn to the frame, after every target pass.
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands[self.frame_start as usize..]
    }

    pub fn target_passes(&self) -> &[TargetPass] {
        &self.passes
    }

    /// The commands of `pass`, which must belong to this list.
    pub fn pass_commands(&self, pass: &TargetPass) -> &[DrawCommand] {
        &self.commands[pass.commands.start as usize..pass.commands.end as usize]
    }

    /// The triangles of `command`, which must belong to this list.
    pub fn command_triangles(&self, command: &DrawCommand) -> &[[u32; 3]] {
        &self.triangles[command.triangles.start as usize..command.triangles.end as usize]
    }

    /// Removes everything, including the clear color and the target passes.
    pub fn clear(&mut self) {
        self.clear_color = None;
        self.vertices.clear();
        self.triangles.clear();
        self.commands.clear();
        self.passes.clear();
        self.in_target = false;
        self.frame_start = 0;
    }

    /// Sends the following pushes to a new pass that draws into `target`,
    /// until [`Self::end_target`]. Fails once a command was pushed to the
    /// frame or while another target pass is open.
    pub fn begin_target(&mut self, target: TextureId) -> Result<(), DrawListError> {
        if self.in_target {
            return Err(DrawListError::TargetPassOpen);
        }
        if self.commands.len() as u32 != self.frame_start {
            return Err(DrawListError::FrameStarted);
        }

        let start = self.commands.len() as u32;
        self.passes.push(TargetPass { target, commands: start..start });
        self.in_target = true;

        Ok(())
    }

    /// Sends the following pushes to the frame.
    pub fn end_target(&mut self) {
        self.in_target = false;
    }

    /// Appends triangles whose indices refer to `vertices`. Consecutive
    /// pushes with equal state extend the previous command. Invalid input
    /// leaves the list unchanged.
    pub fn push(&mut self, state: DrawState, vertices: &[Vertex], triangles: &[[u32; 3]]) -> Result<(), DrawListError> {
        if state.shadow.is_some() && state.texture.is_none() {
            return Err(DrawListError::ShadowWithoutTexture);
        }
        if state.shadow.is_some() && state.sampler.mapping != TextureMapping::Direct {
            return Err(DrawListError::ShadowMapping { mapping: state.sampler.mapping });
        }

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

        // A command never extends across a pass boundary.
        let pass_start = match self.passes.last() {
            Some(pass) if self.in_target => pass.commands.start,
            _ => self.frame_start,
        };
        let merged = if self.commands.len() as u32 > pass_start
            && let Some(last) = self.commands.last_mut()
            && last.triangles.end == start
            && last.state == state
        {
            last.triangles.end = end;
            true
        } else {
            false
        };
        if !merged {
            self.commands.push(DrawCommand { state, triangles: start..end });
        }

        if self.in_target
            && let Some(pass) = self.passes.last_mut()
        {
            pass.commands.end = self.commands.len() as u32;
            self.frame_start = pass.commands.end;
        }

        Ok(())
    }
}

impl Default for DrawList {
    fn default() -> Self {
        Self::new()
    }
}

/// A drop shadow drawn in the same pass as a textured command, as the XUI
/// text pixel program does. Only texel alpha is read. With `t0` the texel
/// alpha at the command's texture coordinates and `t1` the alpha one texel
/// up and left of them (`u - 1 / width`, `v - 1 / height`, same sampler),
/// `g = t0 * vertex.a` and `s = t1 * color.a`, the source has alpha
/// `a = g + s * (1 - g)` and color `(g * vertex.rgb + s * (1 - g) *
/// color.rgb) / a` (straight; 0 where `a` is 0), with the straight vertex
/// color. Premultiplied, that is `t0 * vertex` over `t1 * color`. The
/// texture mapping is direct.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextShadow {
    /// Straight (not premultiplied) shadow color.
    pub color: Rgba8,
}

/// The per-command state of [`DrawList::push`].
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DrawState {
    pub texture: Option<TextureId>,
    pub sampler: Sampler,
    pub blend: BlendMode,
    pub clip: Option<ClipRect>,
    pub color_write: ColorWriteMask,
    /// Requires a texture with direct mapping.
    pub shadow: Option<TextShadow>,
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

    #[test]
    fn text_shadow_requires_a_direct_texture() {
        let shadow = Some(TextShadow { color: Rgba8::TRANSPARENT });
        let mut list = DrawList::new();
        let without_texture = DrawState { shadow, ..DrawState::default() };
        assert_eq!(
            list.push(without_texture, &[], &[]),
            Err(DrawListError::ShadowWithoutTexture)
        );

        let sampler = Sampler { mapping: TextureMapping::Radial, ..Sampler::default() };
        let radial = DrawState {
            texture: Some(TextureId::new(0, 0)),
            sampler,
            shadow,
            ..DrawState::default()
        };
        assert_eq!(
            list.push(radial, &[], &[]),
            Err(DrawListError::ShadowMapping { mapping: TextureMapping::Radial })
        );
    }

    #[test]
    fn target_passes_precede_frame_commands_and_do_not_merge_across_passes() {
        let mut slots = crate::texture::TextureSlots::new();
        let first = slots.insert(()).unwrap();
        let second = slots.insert(()).unwrap();
        let mut list = DrawList::new();

        list.begin_target(first).unwrap();
        list.push(DrawState::default(), &vertices(3), &[[0, 1, 2]]).unwrap();
        list.end_target();
        list.begin_target(second).unwrap();
        list.end_target();
        list.push(DrawState::default(), &vertices(3), &[[0, 1, 2]]).unwrap();

        let passes = list.target_passes();
        assert_eq!(passes.len(), 2);
        assert_eq!(passes[0].target(), first);
        assert_eq!(list.pass_commands(&passes[0]).len(), 1);
        assert_eq!(passes[1].target(), second);
        assert!(list.pass_commands(&passes[1]).is_empty());
        assert_eq!(list.commands().len(), 1);
        assert_eq!(list.commands()[0].triangles(), 1..2);
    }

    #[test]
    fn begin_target_rejects_open_pass_and_started_frame() {
        let mut slots = crate::texture::TextureSlots::new();
        let target = slots.insert(()).unwrap();
        let mut list = DrawList::new();

        list.begin_target(target).unwrap();
        assert_eq!(list.begin_target(target), Err(DrawListError::TargetPassOpen));
        list.end_target();
        list.push(DrawState::default(), &vertices(3), &[[0, 1, 2]]).unwrap();

        assert_eq!(list.begin_target(target), Err(DrawListError::FrameStarted));
    }
}
