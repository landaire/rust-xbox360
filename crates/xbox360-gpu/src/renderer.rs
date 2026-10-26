use alloc::vec::Vec;
use core::fmt;
use core::num::NonZeroU32;

use xbox360_draw::draw::AddressMode;
use xbox360_draw::draw::BlendMode;
use xbox360_draw::draw::ClipRect;
use xbox360_draw::draw::ColorWriteMask;
use xbox360_draw::draw::DrawCommand;
use xbox360_draw::draw::DrawList;
use xbox360_draw::draw::Filter;
use xbox360_draw::draw::Rgba8;
use xbox360_draw::draw::Sampler;
use xbox360_draw::draw::TextureId;
use xbox360_draw::draw::TextureMapping;
use xbox360_draw::draw::Vertex;
use xbox360_draw::error::TextureError;
use xbox360_draw::image::AlphaMode;
use xbox360_draw::image::Image;
use xbox360_draw::texture::MAX_TEXTURE_DIMENSION;
use xbox360_draw::texture::TextureSlots;
use xbox360_draw::texture::TextureStore;
use xbox360_video::Display;
use xbox360_video::ShaderEpoch;
use xbox360_video::ShaderOwner;
use xbox360_video::memory::Caching;
use xbox360_video::memory::PhysicalBuffer;
use xbox360_video::ring::Ring;
use xbox360_video::ring::type0;
use xbox360_video::ring::type3;

use crate::shader;
use crate::shader::ShadowOutput;
use crate::shader::TexelConversion;

/// Bytes of each of the two vertex arenas. Triangles are expanded to three
/// vertices of 20 bytes, so one arena holds about 52000 triangles.
const ARENA_BYTES: u32 = 0x0010_0000 * 3;

/// Texture rows are padded to the fetch constant's pitch unit.
const PITCH_TEXELS: u32 = 32;
const TEXEL_BYTES: u32 = 4;
/// Allocation granule of physical memory.
const PAGE_BYTES: u32 = 0x1000;
/// Pixels per edge of a tile of the layout that resolves write (as in the
/// front buffers, console-display.md).
const TILE_EDGE: u32 = 32;
const TILE_BYTES: u32 = TILE_EDGE * TILE_EDGE * TEXEL_BYTES;

mod register {
    pub const WINDOW_OFFSET: u16 = 0x2080;
    pub const WINDOW_SCISSOR: u16 = 0x2081;
    pub const DEPTH_2203: u16 = 0x2203;
    pub const CLIP_CONTROL: u16 = 0x2204;
    pub const SURFACE: u16 = 0x2000;
    pub const MISC_2100: u16 = 0x2100;
    pub const COLOR_MASK: u16 = 0x2104;
    pub const STENCIL_210D: u16 = 0x210d;
    pub const VIEWPORT: u16 = 0x210f;
    pub const DEPTH_CONTROL: u16 = 0x2200;
    pub const BLEND_CONTROL: u16 = 0x2201;
    pub const MODE_CONTROL: u16 = 0x2208;
    pub const BLEND_CONTROL_1: u16 = 0x2209;
    pub const PROGRAM_CONTROL: u16 = 0x2180;
    pub const RESOLVE: u16 = 0x2318;
    pub const RESOLVE_INFO: u16 = 0x231b;
    pub const FETCH_CONSTANTS: u16 = 0x4800;
    /// Pixel float constants follow the 256 vertex constants.
    pub const PIXEL_CONSTANTS: u16 = 0x4400;
    pub const CONSTANTS_5000: u16 = 0x5000;
    /// Added to every vertex index of a draw.
    pub const INDEX_OFFSET: u16 = 0x2102;
    pub const COPY_2301: u16 = 0x2301;
    pub const MASK_2312: u16 = 0x2312;
    pub const MISC_05C8: u16 = 0x05c8;
    pub const MISC_0F01: u16 = 0x0f01;
    pub const FLUSH_0E00: u16 = 0x0e00;
    pub const FLUSH_0E40: u16 = 0x0e40;
    pub const COHERENCY_SIZE: u16 = 0x0a2f;
    pub const COHERENCY_STATUS: u16 = 0x0a31;
}

mod opcode {
    pub const DRAW: u8 = 0x22;
    pub const DRAW_SHORT: u8 = 0x36;
    pub const LOAD_SHADER: u8 = 0x2b;
    pub const STATE_3B: u8 = 0x3b;
    pub const WAIT: u8 = 0x3c;
    pub const RESOLVE_EVENT: u8 = 0x46;
}

/// Blend factors of the blend control register (D3D's D3DBLEND numbers).
#[derive(Clone, Copy)]
#[repr(u32)]
enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 4,
    SourceAlpha = 6,
    OneMinusSourceAlpha = 7,
}

/// `source * source factor (op) destination * destination factor`.
#[derive(Clone, Copy)]
#[repr(u32)]
enum BlendOperation {
    Add = 0,
    /// Source minus destination.
    Subtract = 1,
    Min = 2,
    Max = 3,
}

/// One channel group of the blend control register: source factor bits
/// 0-4, operation 5-7, destination factor 8-12.
const fn blend_channel(source: BlendFactor, operation: BlendOperation, destination: BlendFactor) -> u32 {
    source as u32 | (operation as u32) << 5 | (destination as u32) << 8
}

/// Blend control register: color channels in bits 0-12, alpha from bit 16.
const fn blend_control(color: u32, alpha: u32) -> u32 {
    color | alpha << 16
}

/// Replaces the target: the clear.
const BLEND_REPLACE: u32 = {
    let replace = blend_channel(BlendFactor::One, BlendOperation::Add, BlendFactor::Zero);
    blend_control(replace, replace)
};

/// Where a pass draws.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Destination {
    Frame,
    /// A render target: the alpha channel takes the source alpha factor
    /// (`BlendMode` documents the arithmetic).
    Target,
}

/// Blend control of each mode (`BlendMode` documents the arithmetic). The
/// pixel programs output premultiplied color for the modes whose source is
/// premultiplied, so the color source factor is one.
fn blend(mode: BlendMode, destination: Destination) -> u32 {
    use BlendFactor::*;
    use BlendOperation::*;
    let alpha_source = match destination {
        Destination::Frame => One,
        Destination::Target => SourceAlpha,
    };
    match mode {
        BlendMode::Normal => blend_control(blend_channel(One, Add, OneMinusSourceAlpha), blend_channel(alpha_source, Add, OneMinusSourceAlpha)),
        BlendMode::Additive => blend_control(blend_channel(One, Add, One), blend_channel(alpha_source, Add, One)),
        BlendMode::Modulate => blend_control(blend_channel(Zero, Add, SourceColor), blend_channel(Zero, Add, SourceAlpha)),
        BlendMode::Min => blend_control(blend_channel(One, Min, One), blend_channel(One, Min, One)),
        BlendMode::Max => blend_control(blend_channel(One, Max, One), blend_channel(One, Max, One)),
        BlendMode::ReverseSubtract => blend_control(blend_channel(One, Subtract, One), blend_channel(alpha_source, Subtract, One)),
    }
}

/// Surface register (0x2000) of a 32 bpp surface `width` pixels wide, as
/// D3D's surface setup (0x921237E0) computes it without multisampling:
/// the width rounded up to 32 in bits 18-31 and the EDRAM pitch, the width
/// rounded up to 80, in bits 0-13.
const fn surface_info(width: u32) -> u32 {
    width.next_multiple_of(32) << 18 | width.next_multiple_of(EDRAM_TILE_WIDTH)
}

/// Pixels per EDRAM tile row and rows per tile at 32 bpp, and the EDRAM
/// size in tiles (D3D's limit in 0x92123C48).
const EDRAM_TILE_WIDTH: u32 = 80;
const EDRAM_TILE_HEIGHT: u32 = 16;
const EDRAM_TILES: u32 = 0x800;

/// Values of registers and packets as D3D writes them for 2D drawing, from
/// captured command streams (xenos-gpu.md). Where a field's meaning is
/// known it is built from named parts.
mod value {
    pub const MISC_05C8: u32 = 0x0002_0000;
    pub const MISC_0F01: u32 = 0x0000_200e;
    pub const MISC_2100_DRAW: u32 = 0x00ff_ffff;
    pub const MISC_2100_RESOLVE: u32 = 0x0000_ffff;
    pub const STENCIL_210D: u32 = 0x00ff_ff00;
    /// Register 0x210E, the alpha reference as a float. D3D's default is 0;
    /// xshell writes 8/255.
    pub const ALPHA_REFERENCE: u32 = 0;
    /// Depth and stencil test off.
    pub const DEPTH_CONTROL_2D: u32 = 0x0070_0730;
    /// Color control with bits 0-2 and bit 3 at the defaults of the D3D
    /// render states that write them (7 and 0). With xshell's 0x8700000E
    /// and its reference of 8/255 the GPU left out the pixels whose alpha
    /// was below 8/255, which removed the faint edges of gradients.
    pub const COLOR_CONTROL: u32 = 0x8700_0007;
    pub const CLIP_CONTROL: u32 = 0x0001_0000;
    pub const SETUP_CONTROL: u32 = 0x0001_0000;
    /// Vertex positions are in pixels: no viewport transform.
    pub const VIEWPORT_TRANSFORM_PIXELS: u32 = 0x300;
    /// Program control for the vertex and pixel programs of `shader`: vertex
    /// registers r0-r2, pixel registers r0-r3, two vertex exports.
    pub const PROGRAM_CONTROL_DRAW: [u32; 3] = [0x1011_0302, 8, 0x0001_0003];
    pub const CONSTANTS_5000_DRAW: [u32; 3] = [0, 0x0002_5000, 0];
    pub const MASK_2312: u32 = 0xffff;
    /// Payloads of packet 0x3B at frame start, before the resolve and at
    /// the end of a frame.
    pub const STATE_3B_FRAME: u32 = 0x300;
    pub const STATE_3B_RESOLVE: u32 = 0x100;
    pub const STATE_3B_END: u32 = 0x7fff;
    pub const RESOLVE_CONTROL: u32 = 0x0010_0000;
    /// Destination format of the resolve: tiled 8:8:8:8 as the scanout reads.
    pub const RESOLVE_DESTINATION_FORMAT: u32 = 0x0100_0300;
    pub const RESOLVE_EVENT: u32 = 6;
    pub const COHERENCY_START: u32 = 0x0001_0000;
    pub const COHERENCY_RANGE: [u32; 2] = [0, 0x1000];
    pub const COHERENCY_BUSY: u32 = 0x8000_0000;
    pub const WAIT_INTERVAL: u32 = 8;
    /// Two of a slot's three vertex fetch constants are unused; D3D leaves
    /// these values in them.
    pub const UNUSED_VERTEX_FETCH: [u32; 2] = [1, 0];
}

/// Fields of fetch constants.
mod fetch {
    /// Type field of a vertex fetch constant (dword 0, bits 0-1).
    pub const TYPE_VERTEX: u32 = 3;
    /// Type field of a texture fetch constant.
    pub const TYPE_TEXTURE: u32 = 2;
    /// Endian field: swap the bytes of each 32-bit word.
    pub const ENDIAN_8_IN_32: u32 = 2;
    /// Endian field: bytes in memory order.
    pub const ENDIAN_NONE: u32 = 0;
    /// Texture dword 0: the texels are tiled.
    pub const TILED: u32 = 1 << 31;
    /// Vertex fetch constant dword 1 flag as D3D writes it.
    pub const VERTEX_FLAGS: u32 = 0x1000_0000;
    /// Vertex data size in dwords, bits 2-25 of dword 1.
    pub const VERTEX_SIZE_SHIFT: u32 = 2;
    /// Texture dword 1: format 8:8:8:8 (6) with the 8-in-32 swap in bits 6-7.
    pub const FORMAT_8_8_8_8: u32 = 6;
    pub const ENDIAN_SHIFT: u32 = 6;
    pub const PITCH_SHIFT: u32 = 22;
    pub const CLAMP_U_SHIFT: u32 = 10;
    pub const CLAMP_V_SHIFT: u32 = 13;
    pub const HEIGHT_SHIFT: u32 = 13;
    /// Texture dword 3 as D3D writes it, filters point.
    pub const SAMPLING_BASE: u32 = 0x0100_0c14;
    pub const MAG_FILTER_SHIFT: u32 = 19;
    pub const MIN_FILTER_SHIFT: u32 = 21;
    /// Texture dword 5: two-dimensional.
    pub const DIMENSION_2D: u32 = 0x200;
}

/// Texture coordinate clamp field of a texture fetch constant.
#[derive(Clone, Copy)]
#[repr(u32)]
enum Clamp {
    Wrap = 0,
    Mirror = 1,
    ClampToEdge = 2,
}

/// Filter fields of a texture fetch constant.
#[derive(Clone, Copy)]
#[repr(u32)]
enum TextureFilter {
    Point = 0,
    Linear = 1,
}

/// Mode control register.
#[derive(Clone, Copy)]
#[repr(u32)]
enum Mode {
    Draw = 4,
    Copy = 6,
}

/// Primitive type field of draw packets.
#[derive(Clone, Copy)]
#[repr(u32)]
enum Primitive {
    TriangleList = 4,
    RectList = 8,
}

/// Texture coordinate of the center of a radial gradient on both axes.
const RADIAL_CENTER: f32 = 0.5;

/// Corners of the resolve rect list.
const RESOLVE_CORNERS: u32 = 3;
/// Value written to the flush registers after a resolve.
const FLUSH: u32 = 1;
/// Vertices of the clear rect (two triangles).
const CLEAR_VERTICES: usize = 6;

/// Size dword of a vertex fetch constant for `dwords` of vertex data.
const fn vertex_fetch_size(dwords: u32) -> u32 {
    fetch::VERTEX_FLAGS | dwords << fetch::VERTEX_SIZE_SHIFT | fetch::ENDIAN_8_IN_32
}

/// Vertex index source of draw packets: generated from 0.
const INDEX_SOURCE_AUTO: u32 = 2;

/// Control dword of a draw packet.
const fn draw_control(count: u32, primitive: Primitive) -> u32 {
    count << 16 | INDEX_SOURCE_AUTO << 6 | primitive as u32
}

/// Shader kind of the shader load packet.
#[derive(Clone, Copy)]
#[repr(u32)]
enum ShaderKind {
    Vertex = 0,
    Pixel = 1,
}

/// Wait packet function: compare a register for equality.
const WAIT_REGISTER_EQUAL: u32 = 3;

/// Largest scissor coordinate.
const SCISSOR_MAX: i32 = 0x3fff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuError {
    /// No physical memory for the vertex arenas or a texture.
    OutOfMemory,
    /// A frame has more triangles than a vertex arena holds.
    TooManyTriangles { triangles: usize },
}

impl fmt::Display for GpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("out of physical memory for GPU data"),
            Self::TooManyTriangles { triangles } => write!(f, "{triangles} triangles exceed the vertex arena"),
        }
    }
}

impl core::error::Error for GpuError {}

/// A texture in GPU memory.
struct Texture {
    width: u32,
    height: u32,
    texels: Texels,
}

/// The texels of a [`Texture`].
enum Texels {
    /// From a premultiplied image.
    Premultiplied {
        image: Image,
        pitch: u32,
        premultiplied: PhysicalBuffer,
        /// The unpremultiplied texels, made on first use by a straight-source
        /// blend mode.
        straight: Option<PhysicalBuffer>,
    },
    /// From a straight image.
    Straight { pitch: u32, buffer: PhysicalBuffer },
    /// A render target: straight texels as a resolve writes them,
    /// transparent black until one does.
    Target { pitch_tiles: u32, buffer: PhysicalBuffer },
}

/// How texels are laid out in memory.
#[derive(Clone, Copy)]
enum TexelLayout {
    /// Rows of CPU words 0xAARRGGBB, `pitch` texels apart.
    Linear { pitch: u32 },
    /// Tiles of [`TILE_EDGE`] texels, `pitch_tiles` per row, as a resolve
    /// with destination format [`value::RESOLVE_DESTINATION_FORMAT`] writes
    /// them: bytes B, G, R, A in address order.
    Tiled { pitch_tiles: u32 },
}

/// Where a draw samples a texture and what its pixel program does with
/// each filtered texel.
#[derive(Clone, Copy)]
struct TextureSource {
    address: u32,
    layout: TexelLayout,
    conversion: TexelConversion,
}

impl Texture {
    fn new(image: Image) -> Result<Self, TextureError> {
        let (width, height) = (image.width().get(), image.height().get());
        let pitch = width.next_multiple_of(PITCH_TEXELS);
        let texels = match image.alpha_mode() {
            AlphaMode::Premultiplied => {
                let premultiplied = upload(&image, pitch, |pixel| pixel)?;
                Texels::Premultiplied { image, pitch, premultiplied, straight: None }
            }
            AlphaMode::Straight => Texels::Straight { pitch, buffer: upload(&image, pitch, |pixel| pixel)? },
        };
        Ok(Self { width, height, texels })
    }

    fn target(width: NonZeroU32, height: NonZeroU32) -> Result<Self, TextureError> {
        let (width, height) = (width.get(), height.get());
        let too_large = TextureError::TooLarge { width, height, max: MAX_TEXTURE_DIMENSION };
        if width > MAX_TEXTURE_DIMENSION || height > MAX_TEXTURE_DIMENSION {
            return Err(too_large);
        }

        let pitch_tiles = width.div_ceil(TILE_EDGE);
        let bytes = pitch_tiles * height.div_ceil(TILE_EDGE) * TILE_BYTES;
        let mut buffer = PhysicalBuffer::new(bytes, Caching::WriteCombined).ok_or(too_large)?;
        buffer.words().fill(0);

        Ok(Self { width, height, texels: Texels::Target { pitch_tiles, buffer } })
    }

    fn is_target(&self) -> bool {
        matches!(self.texels, Texels::Target { .. })
    }

    fn target_layout(&self) -> Option<TargetLayout> {
        let Texels::Target { pitch_tiles, buffer } = &self.texels else {
            return None;
        };

        let width = *pitch_tiles * TILE_EDGE;
        let height = self.height.next_multiple_of(TILE_EDGE);
        Some(TargetLayout { width, height, pitch_tiles: *pitch_tiles, address: buffer.physical().0 })
    }

    /// Straight texels are filtered first and premultiplied by the pixel
    /// program for the modes whose source is premultiplied.
    fn source(&mut self, mode: BlendMode) -> Result<TextureSource, TextureError> {
        let straight_source = mode.uses_straight_source();
        let straight_conversion = if straight_source { TexelConversion::Unchanged } else { TexelConversion::Premultiply };
        let (buffer, layout, conversion) = match &mut self.texels {
            Texels::Premultiplied { image, pitch, premultiplied, straight } => {
                let buffer = match (straight_source, straight) {
                    (false, _) => &*premultiplied,
                    (true, Some(buffer)) => &*buffer,
                    (true, straight) => &*straight.insert(upload(image, *pitch, unpremultiply)?),
                };
                (buffer, TexelLayout::Linear { pitch: *pitch }, TexelConversion::Unchanged)
            }
            Texels::Straight { pitch, buffer } => (&*buffer, TexelLayout::Linear { pitch: *pitch }, straight_conversion),
            Texels::Target { pitch_tiles, buffer } => (&*buffer, TexelLayout::Tiled { pitch_tiles: *pitch_tiles }, straight_conversion),
        };

        Ok(TextureSource { address: buffer.physical().0, layout, conversion })
    }

    /// Any existing representation; alpha is identical in each one.
    fn alpha_source(&self) -> TextureSource {
        let (buffer, layout) = match &self.texels {
            Texels::Premultiplied { pitch, premultiplied, .. } => {
                (&*premultiplied, TexelLayout::Linear { pitch: *pitch })
            }
            Texels::Straight { pitch, buffer } => {
                (&*buffer, TexelLayout::Linear { pitch: *pitch })
            }
            Texels::Target { pitch_tiles, buffer } => {
                (&*buffer, TexelLayout::Tiled { pitch_tiles: *pitch_tiles })
            }
        };
        TextureSource {
            address: buffer.physical().0,
            layout,
            conversion: TexelConversion::Unchanged,
        }
    }
}

fn upload(image: &Image, pitch: u32, convert: impl Fn(Rgba8) -> Rgba8) -> Result<PhysicalBuffer, TextureError> {
    let width = image.width().get();
    let height = image.height().get();
    let bytes = (pitch * height * TEXEL_BYTES).next_multiple_of(PAGE_BYTES);
    let too_large = TextureError::TooLarge { width, height, max: MAX_TEXTURE_DIMENSION };
    let mut buffer = PhysicalBuffer::new(bytes, Caching::WriteCombined).ok_or(too_large)?;
    let words = buffer.words();
    for (row, source) in image.pixels().chunks_exact(width as usize).enumerate() {
        let start = row * pitch as usize;
        for (texel, &pixel) in words[start..start + width as usize].iter_mut().zip(source) {
            *texel = argb(convert(pixel));
        }
    }
    Ok(buffer)
}

/// `Rgba8` (red in the low byte) as the CPU word 0xAARRGGBB the GPU reads
/// with its 8-in-32 swap.
const fn argb(pixel: Rgba8) -> u32 {
    let value = pixel.0;
    (value & 0xff00_ff00) | ((value >> 16) & 0xff) | ((value & 0xff) << 16)
}

/// Straight color of a premultiplied pixel; zero alpha gives zero.
fn unpremultiply(pixel: Rgba8) -> Rgba8 {
    let alpha = pixel.0 >> 24;
    if alpha == 0 {
        return Rgba8(0);
    }
    let channel = |shift: u32| (((pixel.0 >> shift) & 0xff) * 255 + alpha / 2) / alpha;
    Rgba8(channel(0).min(255) | channel(8).min(255) << 8 | channel(16).min(255) << 16 | alpha << 24)
}

/// Texture fetch constant of a 2D texture.
fn texture_fetch_constant(source: TextureSource, width: u32, height: u32, sampler: Sampler) -> [u32; 6] {
    let clamp_of = |mode: AddressMode| match mode {
        AddressMode::Wrap => Clamp::Wrap,
        AddressMode::Mirror => Clamp::Mirror,
        AddressMode::Clamp => Clamp::ClampToEdge,
    } as u32;
    let clamp = clamp_of(sampler.address_u) << fetch::CLAMP_U_SHIFT | clamp_of(sampler.address_v) << fetch::CLAMP_V_SHIFT;
    let filter = match sampler.filter {
        Filter::Nearest => TextureFilter::Point,
        Filter::Linear => TextureFilter::Linear,
    } as u32;
    let filters = filter << fetch::MAG_FILTER_SHIFT | filter << fetch::MIN_FILTER_SHIFT;
    // D3D's resolve (0x92134FF0) writes the destination texture's endian
    // field into the resolve's destination format, so tiled texels are
    // fetched with the endian of `RESOLVE_DESTINATION_FORMAT`: none.
    let (layout, endian) = match source.layout {
        TexelLayout::Linear { pitch } => ((pitch / PITCH_TEXELS) << fetch::PITCH_SHIFT, fetch::ENDIAN_8_IN_32),
        TexelLayout::Tiled { pitch_tiles } => (fetch::TILED | pitch_tiles << fetch::PITCH_SHIFT, fetch::ENDIAN_NONE),
    };
    [
        layout | clamp | fetch::TYPE_TEXTURE,
        source.address | endian << fetch::ENDIAN_SHIFT | fetch::FORMAT_8_8_8_8,
        (width - 1) | (height - 1) << fetch::HEIGHT_SHIFT,
        fetch::SAMPLING_BASE | filters,
        0,
        fetch::DIMENSION_2D,
    ]
}

const fn float(value: f32) -> u32 {
    value.to_bits()
}


/// Vertex data of a frame in an arena, in the vertex shader's layout.
struct Arena<'a> {
    words: &'a mut [u32],
    written: usize,
}

impl Arena<'_> {
    fn push(&mut self, vertex: &Vertex) {
        let words = &mut self.words[self.written..self.written + shader::VERTEX_DWORDS as usize];
        words[0] = vertex.position[0].to_bits();
        words[1] = vertex.position[1].to_bits();
        words[2] = vertex.uv[0].to_bits();
        words[3] = vertex.uv[1].to_bits();
        words[4] = vertex.color.0;
        self.written += shader::VERTEX_DWORDS as usize;
    }

    fn vertex_count(&self) -> usize {
        self.written / shader::VERTEX_DWORDS as usize
    }
}

/// Writes packets into the ring, one reservation each, so the ring wraps
/// only between packets.
struct Commands<'r> {
    ring: &'r mut Ring,
}

impl Commands<'_> {
    fn emit(&mut self, header: u32, prefix: &[u32], payload: &[u32]) {
        let length = 1 + prefix.len() + payload.len();
        let words = self.ring.reserve(length as u32);
        words[0] = header;
        words[1..=prefix.len()].copy_from_slice(prefix);
        words[1 + prefix.len()..].copy_from_slice(payload);
        self.ring.commit(length as u32);
    }

    fn registers(&mut self, first: u16, values: &[u32]) {
        self.emit(type0(first, values.len() as u32), &[], values);
    }

    fn packet(&mut self, code: u8, payload: &[u32]) {
        self.emit(type3(code, payload.len() as u32), &[], payload);
    }

    fn shader(&mut self, kind: ShaderKind, program: &[u32]) {
        self.emit(type3(opcode::LOAD_SHADER, program.len() as u32 + 2), &[kind as u32, program.len() as u32], program);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PixelProgram {
    Texture { mapping: TextureMapping, conversion: TexelConversion },
    Shadow(ShadowOutput),
    Color,
}

/// A textured pixel program for each texel conversion.
struct ConversionPrograms {
    unchanged: Vec<u32>,
    premultiply: Vec<u32>,
}

impl ConversionPrograms {
    fn new(program: fn(TexelConversion) -> Vec<u32>) -> Self {
        Self { unchanged: program(TexelConversion::Unchanged), premultiply: program(TexelConversion::Premultiply) }
    }

    fn program(&self, conversion: TexelConversion) -> &[u32] {
        match conversion {
            TexelConversion::Unchanged => &self.unchanged,
            TexelConversion::Premultiply => &self.premultiply,
        }
    }
}

/// The shadow program for each source color representation.
struct ShadowPrograms {
    premultiplied: Vec<u32>,
    straight: Vec<u32>,
}

/// GPU renderer and texture store.
pub struct Gpu {
    textures: TextureSlots<Texture>,
    arenas: [PhysicalBuffer; 2],
    frame: usize,
    /// Texture memory changed since the last frame: the GPU's caches must
    /// be made coherent before it samples.
    textures_changed: bool,
    /// Textures replaced or destroyed, with the number of frames submitted
    /// when they were retired; frames up to then may still sample them.
    retired: Vec<(usize, Texture)>,
    /// The programs the GPU holds, valid for the display's shader epoch, so
    /// that unchanged programs are not loaded again.
    loaded: Option<LoadedPrograms>,
    owner: ShaderOwner,
    vertex_program: Vec<u32>,
    textured_programs: ConversionPrograms,
    radial_programs: ConversionPrograms,
    shadow_programs: ShadowPrograms,
    color_program: Vec<u32>,
}

#[derive(Clone, Copy)]
struct LoadedPrograms {
    epoch: ShaderEpoch,
    pixel: Option<PixelProgram>,
}

impl Gpu {
    pub fn new() -> Result<Self, GpuError> {
        let arena = || PhysicalBuffer::new(ARENA_BYTES, Caching::WriteCombined).ok_or(GpuError::OutOfMemory);
        Ok(Self {
            textures: TextureSlots::new(),
            arenas: [arena()?, arena()?],
            frame: 0,
            textures_changed: false,
            retired: Vec::new(),
            loaded: None,
            owner: ShaderOwner::new(),
            vertex_program: shader::vertex_program(),
            textured_programs: ConversionPrograms::new(shader::textured_pixel_program),
            radial_programs: ConversionPrograms::new(shader::radial_pixel_program),
            shadow_programs: ShadowPrograms {
                premultiplied: shader::shadow_pixel_program(ShadowOutput::Premultiplied),
                straight: shader::shadow_pixel_program(ShadowOutput::Straight),
            },
            color_program: shader::color_pixel_program(),
        })
    }

    /// Render `list` and flip `display` to it at the next vertical blank.
    /// Target passes are drawn first, each into EDRAM and resolved into its
    /// target. Commands whose texture does not exist draw nothing, as in
    /// every backend.
    pub fn render(&mut self, display: &mut Display, list: &DrawList) -> Result<(), GpuError> {
        let vertices_of = |commands: &[DrawCommand]| -> usize { commands.iter().map(|command| list.command_triangles(command).len() * 3).sum() };
        // Each surface takes a clear rect and resolve corners from the arena.
        let surface_vertices = CLEAR_VERTICES + RESOLVE_CORNERS as usize;
        let mut needed = vertices_of(list.commands()) + surface_vertices;
        for pass in list.target_passes() {
            if let Some(target) = self.textures.get(pass.target()).and_then(Texture::target_layout) {
                needed += target.bands() as usize * (vertices_of(list.pass_commands(pass)) + surface_vertices);
            }
        }
        if needed * shader::VERTEX_DWORDS as usize * 4 > ARENA_BYTES as usize {
            return Err(GpuError::TooManyTriangles { triangles: needed / 3 });
        }

        let epoch = display.claim_shaders(self.owner);
        let (back, ring) = display.begin_gpu_frame();
        // The previous flip has taken effect, so every submitted frame has
        // finished: retired textures and the arena of two frames ago are free.
        let submitted = self.frame;
        self.retired.retain(|(retired_at, _)| *retired_at > submitted);
        let arena = &mut self.arenas[self.frame % 2];
        self.frame += 1;
        let arena_address = arena.physical().0;
        let mut vertices = Arena { words: arena.words(), written: 0 };

        let commands = &mut Commands { ring };
        let frame = Surface { width: back.size.width, height: back.size.height };
        surface_state(commands, frame);
        if self.textures_changed {
            coherency_flush(commands);
            self.textures_changed = false;
        }
        let pixel_loaded = match self.loaded {
            Some(loaded) if loaded.epoch == epoch => loaded.pixel,
            _ => {
                commands.shader(ShaderKind::Vertex, &self.vertex_program);
                None
            }
        };
        let mut programs = Programs {
            loaded: pixel_loaded,
            color: &self.color_program,
            textured: &self.textured_programs,
            radial: &self.radial_programs,
            shadow: &self.shadow_programs,
        };
        commands.registers(register::PROGRAM_CONTROL, &value::PROGRAM_CONTROL_DRAW);
        let mut pass = PassContext { commands, programs: &mut programs, textures: &mut self.textures, vertices: &mut vertices, arena: arena_address, list };

        let mut drew_target = false;
        for target_pass in list.target_passes() {
            let id = target_pass.target();
            let Some(target) = pass.textures.get(id).and_then(Texture::target_layout) else {
                continue;
            };

            let band_height = target.band_height();
            let mut top = 0;
            while top < target.height {
                let surface = Surface { width: target.width, height: band_height.min(target.height - top) };
                pass.begin_surface(surface);
                pass.clear(surface, Rgba8::TRANSPARENT);
                pass.draw_commands(list.pass_commands(target_pass), Some(id), Destination::Target, top, surface);
                let destination = ResolveDestination {
                    address: target.address + top / TILE_EDGE * target.pitch_tiles * TILE_BYTES,
                    pitch: target.pitch_tiles * TILE_EDGE,
                    height: surface.height,
                };
                pass.resolve(surface, destination);
                top += surface.height;
            }

            // Later passes and the frame sample what the resolves wrote.
            pass.commands.ring.wait_for_pipeline();
            coherency_flush(pass.commands);
            drew_target = true;
        }

        // A resolve leaves copy state behind.
        if drew_target {
            pass.begin_surface(frame);
        }
        pass.clear(frame, list.clear_color().unwrap_or(Rgba8::TRANSPARENT));
        pass.draw_commands(list.commands(), None, Destination::Frame, 0, frame);
        pass.resolve(frame, ResolveDestination { address: back.address.0, pitch: back.pitch, height: frame.height });

        self.loaded = Some(LoadedPrograms { epoch, pixel: programs.loaded });
        display.present_gpu_frame();
        Ok(())
    }
}

/// A rectangle of EDRAM from its base, drawn and resolved at once.
#[derive(Clone, Copy)]
struct Surface {
    width: u32,
    height: u32,
}

impl Surface {
    /// Bottom right corner (exclusive) as the scissor registers take it.
    fn corner(self) -> u32 {
        self.height << 16 | self.width
    }
}

/// Where a resolve writes: tiled texels, `pitch` per row.
#[derive(Clone, Copy)]
struct ResolveDestination {
    address: u32,
    pitch: u32,
    height: u32,
}

/// The memory of a render target texture in whole tiles. Its surfaces and
/// resolves cover whole tiles: on the development kit a 4x2 surface and
/// resolve stopped the GPU, a 128x64 one drew correctly. The texels past
/// the texture size are never sampled.
#[derive(Clone, Copy)]
struct TargetLayout {
    width: u32,
    height: u32,
    pitch_tiles: u32,
    address: u32,
}

impl TargetLayout {
    /// Rows of one EDRAM surface: the most whole tile rows whose EDRAM
    /// tiles fit the EDRAM. A taller target is drawn in bands.
    fn band_height(self) -> u32 {
        let tiles_per_row_group = self.width.div_ceil(EDRAM_TILE_WIDTH);
        let rows = EDRAM_TILES / tiles_per_row_group * EDRAM_TILE_HEIGHT;
        (rows / TILE_EDGE * TILE_EDGE).max(TILE_EDGE)
    }

    fn bands(self) -> u32 {
        self.height.div_ceil(self.band_height())
    }
}

/// What the draws of one frame share.
struct PassContext<'a, 'r, 'p> {
    commands: &'a mut Commands<'r>,
    programs: &'a mut Programs<'p>,
    textures: &'a mut TextureSlots<Texture>,
    vertices: &'a mut Arena<'a>,
    arena: u32,
    list: &'a DrawList,
}

impl PassContext<'_, '_, '_> {
    /// Window, surface, viewport, render state and program control for
    /// drawing into `surface`.
    fn begin_surface(&mut self, surface: Surface) {
        surface_state(self.commands, surface);
        self.commands.registers(register::PROGRAM_CONTROL, &value::PROGRAM_CONTROL_DRAW);
    }

    /// Fills `surface` with `color`, ignoring blending and the mask.
    fn clear(&mut self, surface: Surface, color: Rgba8) {
        let (width, height) = (surface.width as f32, surface.height as f32);
        let first = self.vertices.vertex_count();
        let corner = |x: f32, y: f32| Vertex { position: [x, y], uv: [0.0, 0.0], color };
        for vertex in [corner(0.0, 0.0), corner(width, 0.0), corner(width, height), corner(0.0, 0.0), corner(width, height), corner(0.0, height)] {
            self.vertices.push(&vertex);
        }
        draw(self.commands, self.programs, PixelProgram::Color, self.arena, first, CLEAR_VERTICES, BLEND_REPLACE, ColorWriteMask::ALL, None, surface.corner());
    }

    /// Draws `draw_commands` moved up by `top` rows. A command sampling
    /// `skip`, the pass's own target, draws nothing.
    fn draw_commands(&mut self, draw_commands: &[DrawCommand], skip: Option<TextureId>, destination: Destination, top: u32, surface: Surface) {
        let source = self.list.vertices();
        let shift = top as f32;
        for command in draw_commands {
            // Radial mapping measures from the center of the texture
            // coordinates, which the program expects at the origin.
            let center = match command.sampler().mapping {
                TextureMapping::Direct => [0.0, 0.0],
                TextureMapping::Radial => [RADIAL_CENTER, RADIAL_CENTER],
            };
            let mode = command.blend();
            let pixel = match command.texture() {
                None => PixelProgram::Color,
                Some(id) if Some(id) == skip => continue,
                Some(id) => {
                    let Some(texture) = self.textures.get_mut(id) else {
                        continue;
                    };
                    let (width, height) = (texture.width, texture.height);
                    let shadow = command.shadow();
                    let source = match shadow {
                        Some(_) => texture.alpha_source(),
                        None => {
                            let Ok(source) = texture.source(mode) else {
                                continue;
                            };
                            source
                        }
                    };
                    let constant = texture_fetch_constant(source, width, height, command.sampler());
                    self.commands.registers(register::FETCH_CONSTANTS, &constant);
                    match shadow {
                        // The program reads only alpha.
                        Some(shadow) => {
                            let color = shadow.color;
                            let channel = |value: u8| float(f32::from(value) / 255.0);
                            self.commands.registers(register::PIXEL_CONSTANTS, &[
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                channel(color.red()),
                                channel(color.green()),
                                channel(color.blue()),
                                channel(color.alpha()),
                                float(1.0 / width as f32),
                                float(1.0 / height as f32),
                                float(shader::SHADOW_ALPHA_EPSILON),
                                float(1.0),
                            ]);
                            let output = if mode.uses_straight_source() {
                                ShadowOutput::Straight
                            } else {
                                ShadowOutput::Premultiplied
                            };
                            PixelProgram::Shadow(output)
                        }
                        None => PixelProgram::Texture { mapping: command.sampler().mapping, conversion: source.conversion },
                    }
                }
            };

            let first = self.vertices.vertex_count();
            for triangle in self.list.command_triangles(command) {
                for &index in triangle {
                    let vertex = source[index as usize];
                    self.vertices.push(&Vertex {
                        position: [vertex.position[0], vertex.position[1] - shift],
                        uv: [vertex.uv[0] - center[0], vertex.uv[1] - center[1]],
                        ..vertex
                    });
                }
            }
            let count = self.vertices.vertex_count() - first;
            let clip = command.clip().map(|clip| ClipRect { top: clip.top - top as i32, bottom: clip.bottom - top as i32, ..clip });
            draw(self.commands, self.programs, pixel, self.arena, first, count, blend(mode, destination), command.color_write(), clip, surface.corner());
        }
    }

    /// Copies `surface` to `destination`.
    fn resolve(&mut self, surface: Surface, destination: ResolveDestination) {
        let first = self.vertices.vertex_count();
        let (width, height) = (surface.width as f32, surface.height as f32);
        for (x, y) in [(0.0, 0.0), (width, 0.0), (0.0, height)] {
            self.vertices.push(&Vertex { position: [x, y], uv: [0.0, 0.0], color: Rgba8::TRANSPARENT });
        }
        resolve(self.commands, surface, destination, self.arena, first);
    }
}

/// Window, surface, viewport and render state for drawing into `surface`.
fn surface_state(commands: &mut Commands, surface: Surface) {
    commands.registers(register::WINDOW_OFFSET, &[0, 0, surface.corner()]);
    commands.registers(register::MISC_05C8, &[value::MISC_05C8]);
    commands.registers(register::MISC_0F01, &[value::MISC_0F01]);
    commands.registers(register::SURFACE, &[surface_info(surface.width), 0, 0]);
    commands.registers(register::MISC_2100, &[value::MISC_2100_DRAW, 0, 0]);
    commands.registers(register::COLOR_MASK, &[u32::from(ColorWriteMask::ALL.bits())]);
    commands.registers(register::STENCIL_210D, &[value::STENCIL_210D, value::ALPHA_REFERENCE]);
    let (width, height) = (surface.width as f32, surface.height as f32);
    commands.registers(register::VIEWPORT, &[float(width / 2.0), float(width / 2.0), float(-height / 2.0), float(height / 2.0), float(1.0), float(0.0)]);
    let normal = blend(BlendMode::Normal, Destination::Frame);
    commands.registers(register::DEPTH_CONTROL, &[
        value::DEPTH_CONTROL_2D,
        normal,
        value::COLOR_CONTROL,
        0,
        value::CLIP_CONTROL,
        value::SETUP_CONTROL,
        value::VIEWPORT_TRANSFORM_PIXELS,
    ]);
    commands.registers(register::MODE_CONTROL, &[Mode::Draw as u32]);
    commands.registers(register::BLEND_CONTROL_1, &[normal, normal, normal]);
    commands.packet(opcode::STATE_3B, &[value::STATE_3B_FRAME]);
}

/// Make the GPU's caches coherent with memory the CPU wrote, then wait for
/// the flush, as D3D does before sampling new data.
fn coherency_flush(commands: &mut Commands) {
    commands.registers(register::COHERENCY_STATUS, &[value::COHERENCY_START]);
    commands.registers(register::COHERENCY_SIZE, &value::COHERENCY_RANGE);
    commands.packet(opcode::WAIT, &[WAIT_REGISTER_EQUAL, u32::from(register::COHERENCY_STATUS), 0, value::COHERENCY_BUSY, value::WAIT_INTERVAL]);
}

/// The pixel programs and the one the GPU holds.
struct Programs<'p> {
    loaded: Option<PixelProgram>,
    color: &'p [u32],
    textured: &'p ConversionPrograms,
    radial: &'p ConversionPrograms,
    shadow: &'p ShadowPrograms,
}

impl Programs<'_> {
    fn select(&mut self, commands: &mut Commands, pixel: PixelProgram) {
        if self.loaded == Some(pixel) {
            return;
        }
        let program = match pixel {
            PixelProgram::Color => self.color,
            PixelProgram::Texture { mapping: TextureMapping::Direct, conversion } => self.textured.program(conversion),
            PixelProgram::Texture { mapping: TextureMapping::Radial, conversion } => self.radial.program(conversion),
            PixelProgram::Shadow(ShadowOutput::Premultiplied) => &self.shadow.premultiplied,
            PixelProgram::Shadow(ShadowOutput::Straight) => &self.shadow.straight,
        };
        commands.shader(ShaderKind::Pixel, program);
        self.loaded = Some(pixel);
    }
}

/// Writes the vertex fetch constant and the index offset that make the
/// arena's vertices `first..` the draw's vertices 0... All draws of a frame
/// fetch from the arena base and are told apart by the index offset: the
/// GPU reuses transformed vertices by index, so draws whose fetch constant
/// moves the base while their indices restart at 0 drew the vertices of an
/// earlier draw.
fn vertex_source(commands: &mut Commands, arena: u32, first: usize, count: usize) {
    let dwords = (first + count) as u32 * shader::VERTEX_DWORDS;
    let constant = shader::VERTEX_FETCH_CONSTANT;
    // Three vertex fetch constants share a slot of six dwords; the first two
    // of the slot are unused and hold the value D3D leaves there.
    let slot = register::FETCH_CONSTANTS + (constant / 3 * 6) as u16;
    let [unused0, unused1] = value::UNUSED_VERTEX_FETCH;
    commands.registers(slot, &[unused0, unused1, unused0, unused1, arena | fetch::TYPE_VERTEX, vertex_fetch_size(dwords)]);
    commands.registers(register::CONSTANTS_5000, &value::CONSTANTS_5000_DRAW);
    commands.registers(register::INDEX_OFFSET, &[first as u32]);
}

#[allow(clippy::too_many_arguments)]
fn draw(
    commands: &mut Commands,
    programs: &mut Programs,
    pixel: PixelProgram,
    arena: u32,
    first: usize,
    count: usize,
    blend_value: u32,
    color_write: ColorWriteMask,
    clip: Option<ClipRect>,
    full: u32,
) {
    if count == 0 {
        return;
    }
    programs.select(commands, pixel);
    commands.registers(register::BLEND_CONTROL, &[blend_value]);
    // D3D's COLORWRITEENABLE setter (0x92125008) stores the state's bits
    // unchanged as bits 0-3 of this register.
    commands.registers(register::COLOR_MASK, &[u32::from(color_write.bits())]);
    // Window scissor: top left and bottom right (exclusive), y << 16 | x.
    let (top_left, bottom_right) = match clip {
        Some(clip) => {
            let clamp = |value: i32| value.clamp(0, SCISSOR_MAX) as u32;
            (clamp(clip.top) << 16 | clamp(clip.left), clamp(clip.bottom) << 16 | clamp(clip.right))
        }
        None => (0, full),
    };
    commands.registers(register::WINDOW_SCISSOR, &[top_left, bottom_right]);
    vertex_source(commands, arena, first, count);
    commands.packet(opcode::DRAW, &[0, draw_control(count as u32, Primitive::TriangleList)]);
}

/// Copy `surface` to `destination` (D3D's resolve, 0x92134FF0, writes the
/// destination's texel pitch and its height in rows into 0x231A). The rect
/// list of the copy is three arena vertices from `first` in the draw
/// layout, run through the resident vertex program.
fn resolve(commands: &mut Commands, surface: Surface, destination: ResolveDestination, arena: u32, first: usize) {
    commands.registers(register::WINDOW_SCISSOR, &[0, surface.corner()]);
    commands.registers(register::SURFACE, &[surface_info(surface.width), 0]);
    commands.registers(register::COPY_2301, &[0]);
    commands.registers(register::RESOLVE, &[value::RESOLVE_CONTROL, destination.address, destination.height << 16 | destination.pitch, value::RESOLVE_DESTINATION_FORMAT]);
    commands.packet(opcode::STATE_3B, &[value::STATE_3B_RESOLVE]);
    commands.registers(register::MODE_CONTROL, &[Mode::Copy as u32]);
    commands.registers(register::DEPTH_CONTROL, &[0]);
    commands.registers(register::DEPTH_2203, &[0]);
    commands.registers(register::MISC_2100, &[value::MISC_2100_RESOLVE, 0, 0]);
    commands.registers(register::CLIP_CONTROL, &[value::CLIP_CONTROL, value::SETUP_CONTROL, value::VIEWPORT_TRANSFORM_PIXELS]);
    commands.registers(register::MASK_2312, &[value::MASK_2312]);
    vertex_source(commands, arena, first, RESOLVE_CORNERS as usize);
    commands.packet(opcode::DRAW_SHORT, &[draw_control(RESOLVE_CORNERS, Primitive::RectList)]);
    commands.packet(opcode::RESOLVE_EVENT, &[value::RESOLVE_EVENT]);
    commands.registers(register::RESOLVE, &[0]);
    commands.registers(register::RESOLVE_INFO, &[0]);
    commands.registers(register::MISC_05C8, &[value::MISC_05C8]);
    commands.registers(register::FLUSH_0E00, &[FLUSH]);
    commands.registers(register::FLUSH_0E40, &[FLUSH]);
    commands.packet(opcode::STATE_3B, &[value::STATE_3B_END]);
}

impl TextureStore for Gpu {
    fn create_texture(&mut self, image: Image) -> Result<TextureId, TextureError> {
        let texture = Texture::new(image)?;
        self.textures_changed = true;
        self.textures.insert(texture)
    }

    fn update_texture(&mut self, id: TextureId, image: Image) -> Result<(), TextureError> {
        let slot = self.textures.get_mut(id).ok_or(TextureError::UnknownTexture { id })?;
        if slot.is_target() {
            return Err(TextureError::RenderTarget { id });
        }

        let old = core::mem::replace(slot, Texture::new(image)?);
        self.retired.push((self.frame, old));
        self.textures_changed = true;
        Ok(())
    }

    fn create_target(&mut self, width: NonZeroU32, height: NonZeroU32) -> Result<TextureId, TextureError> {
        let texture = Texture::target(width, height)?;
        self.textures_changed = true;
        self.textures.insert(texture)
    }

    fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError> {
        let texture = self.textures.remove(id).ok_or(TextureError::UnknownTexture { id })?;
        self.retired.push((self.frame, texture));
        Ok(())
    }
}
