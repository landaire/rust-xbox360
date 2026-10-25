use alloc::vec::Vec;
use core::fmt;

use xbox360_draw::draw::AddressMode;
use xbox360_draw::draw::BlendMode;
use xbox360_draw::draw::ClipRect;
use xbox360_draw::draw::DrawList;
use xbox360_draw::draw::Filter;
use xbox360_draw::draw::Rgba8;
use xbox360_draw::draw::Sampler;
use xbox360_draw::draw::TextureId;
use xbox360_draw::draw::TextureMapping;
use xbox360_draw::draw::Vertex;
use xbox360_draw::error::TextureError;
use xbox360_draw::image::Image;
use xbox360_draw::texture::MAX_TEXTURE_DIMENSION;
use xbox360_draw::texture::TextureSlots;
use xbox360_draw::texture::TextureStore;
use xbox360_video::BackBuffer;
use xbox360_video::Display;
use xbox360_video::ShaderEpoch;
use xbox360_video::ShaderOwner;
use xbox360_video::memory::Caching;
use xbox360_video::memory::PhysicalBuffer;
use xbox360_video::ring::Ring;
use xbox360_video::ring::type0;
use xbox360_video::ring::type3;

use crate::shader;

/// Bytes of each of the two vertex arenas. Triangles are expanded to three
/// vertices of 20 bytes, so one arena holds about 52000 triangles.
const ARENA_BYTES: u32 = 0x0010_0000 * 3;

/// Texture rows are padded to the fetch constant's pitch unit.
const PITCH_TEXELS: u32 = 32;
const TEXEL_BYTES: u32 = 4;
/// Allocation granule of physical memory.
const PAGE_BYTES: u32 = 0x1000;

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

/// Blend control register: source factor bits 0-4, operation 5-7,
/// destination factor 8-12; the alpha channel repeats the layout from bit 16.
const fn blend_control(source: BlendFactor, operation: BlendOperation, destination: BlendFactor, alpha_destination: BlendFactor) -> u32 {
    let color = source as u32 | (operation as u32) << 5 | (destination as u32) << 8;
    let alpha = source as u32 | (operation as u32) << 5 | (alpha_destination as u32) << 8;
    color | alpha << 16
}

/// Replaces the target: the clear.
const BLEND_REPLACE: u32 = blend_control(BlendFactor::One, BlendOperation::Add, BlendFactor::Zero, BlendFactor::Zero);

/// Blend control of each mode (`BlendMode` documents the arithmetic).
fn blend(mode: BlendMode) -> u32 {
    use BlendFactor::*;
    use BlendOperation::*;
    match mode {
        BlendMode::Normal => blend_control(One, Add, OneMinusSourceAlpha, OneMinusSourceAlpha),
        BlendMode::Additive => blend_control(One, Add, One, One),
        BlendMode::Modulate => blend_control(Zero, Add, SourceColor, SourceAlpha),
        BlendMode::Min => blend_control(One, Min, One, One),
        BlendMode::Max => blend_control(One, Max, One, One),
        BlendMode::ReverseSubtract => blend_control(One, Subtract, One, One),
    }
}

/// Values of registers and packets as D3D writes them for 2D drawing, from
/// captured command streams (xenos-gpu.md). Where a field's meaning is
/// known it is built from named parts.
mod value {
    pub const MISC_05C8: u32 = 0x0002_0000;
    pub const MISC_0F01: u32 = 0x0000_200e;
    /// High byte of the surface register; the low bits hold the EDRAM pitch.
    pub const SURFACE_FLAGS: u32 = 0x1400_0000;
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
    /// registers r0-r2, pixel registers r0-r1, two vertex exports.
    pub const PROGRAM_CONTROL_DRAW: [u32; 3] = [0x1011_0102, 8, 0x0001_0003];
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
/// All four channels in the color write mask.
const COLOR_MASK_RGBA: u32 = 0xf;

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

/// A texture in GPU memory: linear 8:8:8:8 texels (CPU words 0xAARRGGBB),
/// rows padded to [`PITCH_TEXELS`].
struct Texture {
    image: Image,
    premultiplied: PhysicalBuffer,
    /// The unpremultiplied texels, made on first use by a straight-source
    /// blend mode.
    straight: Option<PhysicalBuffer>,
    pitch: u32,
}

impl Texture {
    fn new(image: Image) -> Result<Self, TextureError> {
        let pitch = image.width().get().next_multiple_of(PITCH_TEXELS);
        let premultiplied = upload(&image, pitch, |pixel| pixel)?;
        Ok(Self { image, premultiplied, straight: None, pitch })
    }

    fn buffer(&mut self, straight: bool) -> Result<&PhysicalBuffer, TextureError> {
        if !straight {
            return Ok(&self.premultiplied);
        }
        if self.straight.is_none() {
            self.straight = Some(upload(&self.image, self.pitch, unpremultiply)?);
        }
        Ok(self.straight.as_ref().unwrap_or(&self.premultiplied))
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

/// Texture fetch constant of a linear 2D texture.
fn texture_fetch_constant(address: u32, pitch: u32, width: u32, height: u32, sampler: Sampler) -> [u32; 6] {
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
    [
        (pitch / PITCH_TEXELS) << fetch::PITCH_SHIFT | clamp | fetch::TYPE_TEXTURE,
        address | fetch::ENDIAN_8_IN_32 << fetch::ENDIAN_SHIFT | fetch::FORMAT_8_8_8_8,
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
    Textured,
    Radial,
    Color,
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
    /// The programs the GPU holds, valid for the display's shader epoch.
    /// The vertex program is loaded once per epoch: loading vertex programs
    /// frame after frame stopped the GPU after some hundred frames on a
    /// development kit (xenos-gpu.md, Shader loads).
    loaded: Option<LoadedPrograms>,
    owner: ShaderOwner,
    vertex_program: Vec<u32>,
    textured_program: Vec<u32>,
    radial_program: Vec<u32>,
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
            textured_program: shader::textured_pixel_program(),
            radial_program: shader::radial_pixel_program(),
            color_program: shader::color_pixel_program(),
        })
    }

    /// Render `list` and flip `display` to it at the next vertical blank.
    /// Commands whose texture does not exist draw nothing, as in every
    /// backend.
    pub fn render(&mut self, display: &mut Display, list: &DrawList) -> Result<(), GpuError> {
        let triangles: usize = list.commands().iter().map(|command| list.command_triangles(command).len()).sum();
        // The clear rect and the resolve corners share the arena.
        let needed = (triangles * 3 + CLEAR_VERTICES + RESOLVE_CORNERS as usize) * shader::VERTEX_DWORDS as usize * 4;
        if needed > ARENA_BYTES as usize {
            return Err(GpuError::TooManyTriangles { triangles });
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
        frame_state(commands, back);
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
            textured: &self.textured_program,
            radial: &self.radial_program,
        };
        commands.registers(register::PROGRAM_CONTROL, &value::PROGRAM_CONTROL_DRAW);

        let size = back.size;
        let full = size.height << 16 | size.width;
        let (width, height) = (size.width as f32, size.height as f32);
        let clear = list.clear_color().unwrap_or(Rgba8::TRANSPARENT);
        let corner = |x: f32, y: f32| Vertex { position: [x, y], uv: [0.0, 0.0], color: clear };
        for vertex in [corner(0.0, 0.0), corner(width, 0.0), corner(width, height), corner(0.0, 0.0), corner(width, height), corner(0.0, height)] {
            vertices.push(&vertex);
        }
        draw(commands, &mut programs, PixelProgram::Color, arena_address, 0, CLEAR_VERTICES, BLEND_REPLACE, None, full);

        let source = list.vertices();
        for command in list.commands() {
            // Radial mapping measures from the center of the texture
            // coordinates, which the program expects at the origin.
            let center = match command.sampler().mapping {
                TextureMapping::Direct => [0.0, 0.0],
                TextureMapping::Radial => [RADIAL_CENTER, RADIAL_CENTER],
            };
            let first = vertices.vertex_count();
            for triangle in list.command_triangles(command) {
                for &index in triangle {
                    let vertex = source[index as usize];
                    vertices.push(&Vertex { uv: [vertex.uv[0] - center[0], vertex.uv[1] - center[1]], ..vertex });
                }
            }
            let count = vertices.vertex_count() - first;
            let mode = command.blend();
            let pixel = match command.texture() {
                None => PixelProgram::Color,
                Some(id) => {
                    let Some(texture) = self.textures.get_mut(id) else {
                        continue;
                    };
                    let (width, height, pitch) = (texture.image.width().get(), texture.image.height().get(), texture.pitch);
                    let Ok(buffer) = texture.buffer(mode.uses_straight_source()) else {
                        continue;
                    };
                    let constant = texture_fetch_constant(buffer.physical().0, pitch, width, height, command.sampler());
                    commands.registers(register::FETCH_CONSTANTS, &constant);
                    match command.sampler().mapping {
                        TextureMapping::Direct => PixelProgram::Textured,
                        TextureMapping::Radial => PixelProgram::Radial,
                    }
                }
            };
            draw(commands, &mut programs, pixel, arena_address, first, count, blend(mode), command.clip(), full);
        }

        let resolve_first = vertices.vertex_count();
        for (x, y) in [(0.0, 0.0), (width, 0.0), (0.0, height)] {
            vertices.push(&Vertex { position: [x, y], uv: [0.0, 0.0], color: Rgba8::TRANSPARENT });
        }
        resolve(commands, back, arena_address, resolve_first);
        self.loaded = Some(LoadedPrograms { epoch, pixel: programs.loaded });
        display.present_gpu_frame();
        Ok(())
    }
}

fn frame_state(commands: &mut Commands, back: BackBuffer) {
    let size = back.size;
    commands.registers(register::WINDOW_OFFSET, &[0, 0, size.height << 16 | size.width]);
    commands.registers(register::MISC_05C8, &[value::MISC_05C8]);
    commands.registers(register::MISC_0F01, &[value::MISC_0F01]);
    commands.registers(register::SURFACE, &[value::SURFACE_FLAGS | size.width, 0, 0]);
    commands.registers(register::MISC_2100, &[value::MISC_2100_DRAW, 0, 0]);
    commands.registers(register::COLOR_MASK, &[COLOR_MASK_RGBA]);
    commands.registers(register::STENCIL_210D, &[value::STENCIL_210D, value::ALPHA_REFERENCE]);
    let (width, height) = (size.width as f32, size.height as f32);
    commands.registers(register::VIEWPORT, &[float(width / 2.0), float(width / 2.0), float(-height / 2.0), float(height / 2.0), float(1.0), float(0.0)]);
    commands.registers(register::DEPTH_CONTROL, &[
        value::DEPTH_CONTROL_2D,
        blend(BlendMode::Normal),
        value::COLOR_CONTROL,
        0,
        value::CLIP_CONTROL,
        value::SETUP_CONTROL,
        value::VIEWPORT_TRANSFORM_PIXELS,
    ]);
    commands.registers(register::MODE_CONTROL, &[Mode::Draw as u32]);
    let normal = blend(BlendMode::Normal);
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
    textured: &'p [u32],
    radial: &'p [u32],
}

impl Programs<'_> {
    fn select(&mut self, commands: &mut Commands, pixel: PixelProgram) {
        if self.loaded == Some(pixel) {
            return;
        }
        let program = match pixel {
            PixelProgram::Color => self.color,
            PixelProgram::Textured => self.textured,
            PixelProgram::Radial => self.radial,
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
    clip: Option<ClipRect>,
    full: u32,
) {
    if count == 0 {
        return;
    }
    programs.select(commands, pixel);
    commands.registers(register::BLEND_CONTROL, &[blend_value]);
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

/// Copy the frame to `back`. The rect list of the copy is three arena
/// vertices from `first` in the draw layout, run through the resident
/// vertex program.
fn resolve(commands: &mut Commands, back: BackBuffer, arena: u32, first: usize) {
    let size = back.size;
    commands.registers(register::WINDOW_SCISSOR, &[0, size.height << 16 | size.width]);
    commands.registers(register::SURFACE, &[value::SURFACE_FLAGS | size.width, 0]);
    commands.registers(register::COPY_2301, &[0]);
    commands.registers(register::RESOLVE, &[value::RESOLVE_CONTROL, back.address.0, size.height << 16 | back.pitch, value::RESOLVE_DESTINATION_FORMAT]);
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
        let old = core::mem::replace(slot, Texture::new(image)?);
        self.retired.push((self.frame, old));
        self.textures_changed = true;
        Ok(())
    }

    fn destroy_texture(&mut self, id: TextureId) -> Result<(), TextureError> {
        let texture = self.textures.remove(id).ok_or(TextureError::UnknownTexture { id })?;
        self.retired.push((self.frame, texture));
        Ok(())
    }
}
