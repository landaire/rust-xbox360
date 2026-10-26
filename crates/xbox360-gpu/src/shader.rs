//! Encoder for the subset of the Xenos shader microcode that the renderer
//! uses, and the renderer's programs. The encoding was derived from
//! observed programs and checked on hardware (docs: xenos-gpu.md in
//! openblades' docs/re-notes). Every program here is generated; no
//! microcode is copied.
//!
//! A program is control flow followed by fetch and ALU instructions of three
//! dwords each, addressed in three-dword units from the program start.

use alloc::vec::Vec;

/// Opcodes of control flow instructions (bits 44-47).
#[derive(Clone, Copy)]
#[repr(u64)]
enum ControlOpcode {
    Exec = 1,
    ExecEnd = 2,
    Allocate = 0xc,
}

/// Bit 41 of every observed exec instruction.
const EXEC_FLAG: u64 = 0x200;
/// Allocation kinds in bits 40-43 of an allocate instruction.
const ALLOCATE_POSITION: u64 = 0x200;
const ALLOCATE_PARAMETERS: u64 = 0x400;
const CONTROL_OPCODE_SHIFT: u64 = 12;
/// The high 16 bits of an instruction start at bit 32.
const CONTROL_HIGH_SHIFT: u64 = 32;
const LOW_HALF: u32 = 0xffff;

/// Control flow instruction (48 bits; two pack into three dwords).
pub enum ControlFlow {
    /// Runs `count` instructions from `address`. Bit i of `fetches` marks
    /// instruction i as a fetch, bit i of `vertex_fetches` as a vertex
    /// fetch. `end` ends the program after it.
    Exec { address: u32, count: u32, fetches: u32, vertex_fetches: u32, end: bool },
    /// Allocates the position export of a vertex shader.
    AllocatePosition,
    /// Allocates interpolator exports (vertex shader) or color exports
    /// (pixel shader).
    AllocateParameters,
    Nop,
}

impl ControlFlow {
    fn bits(&self) -> u64 {
        match *self {
            Self::Exec { address, count, fetches, vertex_fetches, end } => {
                let mut sequence = 0u32;
                for i in 0..count {
                    if fetches & (1 << i) != 0 {
                        sequence |= 1 << (2 * i);
                    }
                }
                let low = address | (count << 12) | (sequence << 16) | (vertex_fetches << 28);
                let opcode = if end { ControlOpcode::ExecEnd } else { ControlOpcode::Exec };
                u64::from(low) | (((opcode as u64) << CONTROL_OPCODE_SHIFT | EXEC_FLAG) << CONTROL_HIGH_SHIFT)
            }
            Self::AllocatePosition => ((ControlOpcode::Allocate as u64) << CONTROL_OPCODE_SHIFT | ALLOCATE_POSITION) << CONTROL_HIGH_SHIFT,
            Self::AllocateParameters => ((ControlOpcode::Allocate as u64) << CONTROL_OPCODE_SHIFT | ALLOCATE_PARAMETERS) << CONTROL_HIGH_SHIFT,
            Self::Nop => 0,
        }
    }
}

/// Packs control flow pairs: instruction A is dword 0 and the low half of
/// dword 1, instruction B the high half of dword 1 and dword 2.
fn control_flow(program: &mut Vec<u32>, instructions: &[ControlFlow]) {
    for pair in instructions.chunks(2) {
        let a = pair[0].bits();
        let b = pair.get(1).map_or(0, ControlFlow::bits);
        program.push(a as u32);
        program.push(((a >> 32) as u32 & LOW_HALF) | ((b as u32 & LOW_HALF) << 16));
        program.push((b >> 16) as u32);
    }
}

/// Destination swizzle of a fetch, 3 bits per component.
const fn swizzle(x: u32, y: u32, z: u32, w: u32) -> u32 {
    x | y << 3 | z << 6 | w << 9
}

const SELECT_ZERO: u32 = 4;
const SELECT_ONE: u32 = 5;
const SELECT_NONE: u32 = 7;

/// Data formats of vertex fetches.
const FORMAT_8_8_8_8: u32 = 0x06;
const FORMAT_32_32_FLOAT: u32 = 0x25;
/// Number format field: float data, or normalized unsigned bytes.
const NUMBER_FLOAT: u32 = 3;
const NUMBER_NORMALIZED: u32 = 0;

struct VertexFetch {
    destination: u32,
    constant: u32,
    /// Reuses the address of the previous full fetch.
    mini: bool,
    format: u32,
    number_format: u32,
    swizzle: u32,
    stride: u32,
    offset: u32,
}

impl VertexFetch {
    fn encode(&self) -> [u32; 3] {
        // Bit 29 is set in the first full fetch of a program.
        let first = if self.mini { 0 } else { 1 << 29 };
        let word0 = (self.destination << 12) | FETCH_FLAG_19 | ((self.constant / 3) << 20) | ((self.constant % 3) << 25) | first;
        let mini = if self.mini { 1 << 30 } else { 0 };
        let word1 = self.swizzle | (self.number_format << 12) | (self.format << 16) | mini;
        [word0, word1, self.stride | (self.offset << 8)]
    }
}

/// Texture fetch of fetch constant `constant` at the coordinates in
/// register `source` into `destination`, all components.
fn texture_fetch(destination: u32, source: u32, constant: u32) -> [u32; 3] {
    let word0 = FETCH_OPCODE_TEXTURE | (source << 5) | (destination << 12) | FETCH_FLAG_19 | (constant << 20) | TEXTURE_FETCH_FLAG_28;
    [word0, swizzle(0, 1, 2, 3) | TEXTURE_FETCH_FILTER_BITS, TEXTURE_FETCH_WORD2]
}

/// Fetch opcode of a texture fetch (a vertex fetch is 0).
const FETCH_OPCODE_TEXTURE: u32 = 1;
/// Set in every observed fetch.
const FETCH_FLAG_19: u32 = 1 << 19;
/// Set in every observed texture fetch.
const TEXTURE_FETCH_FLAG_28: u32 = 1 << 28;
/// Filter fields of the observed texture fetches (dword 1 above the swizzle).
const TEXTURE_FETCH_FILTER_BITS: u32 = 0x1f1f_f000;
/// Dword 2 of the observed texture fetches.
const TEXTURE_FETCH_WORD2: u32 = 0x4000;

/// Top byte of every observed ALU instruction (scalar half unused).
const ALU_WORD0_BASE: u32 = 0xc800_0000;
/// Bit 15 of dword 0: the destination is an export.
const ALU_EXPORT: u32 = 1 << 15;
/// Bits 29-31 of dword 2: the three operands are registers.
const ALU_SOURCES_REGISTERS: u32 = 0xe000_0000;

const VECTOR_MUL: u32 = 1;
const VECTOR_MAX: u32 = 2;
const EXPORT_POSITION: u32 = 62;
const MASK_ALL: u32 = 0xf;
const MASK_XY: u32 = 0x3;
/// Source swizzles that replicate y into z and w, for xy-only operands.
const SWIZZLE_XY: u32 = 0x00b0_b000;

/// Vector ALU instruction that exports `operation(a, b)` to `destination`.
/// The maximum of a register with itself is a move.
fn export(destination: u32, mask: u32, operation: u32, a: u32, b: u32, swizzles: u32) -> [u32; 3] {
    [ALU_WORD0_BASE | ALU_EXPORT | (mask << 16) | destination, swizzles, ALU_SOURCES_REGISTERS | (operation << 24) | (a << 16) | (b << 8)]
}

/// Scalar opcode field (dword 0 bits 26-31) of an ALU instruction without a
/// scalar operation, as in [`ALU_WORD0_BASE`].
const SCALAR_NONE: u32 = 50;
// Scalar opcodes. A scalar operation reads operand C through its swizzle
// and writes the components of its mask (dword 0 bits 20-23) of the
// destination, as the dashboard's radial gradient program uses them
// (xenos-gpu.md, Scalar operations).
/// `c.x + c.w` of the swizzled operand.
const SCALAR_ADD: u32 = 0;
/// Square root. The programs give it an operand with x in every component.
const SCALAR_SQRT: u32 = 40;
/// Source swizzle that replicates x into every component.
const SWIZZLE_XXXX: u32 = 0x6c;
/// Source swizzle x, y, y, y: puts y in w for [`SCALAR_ADD`].
const SWIZZLE_XYYY: u32 = 0xb0;
const MASK_X: u32 = 0x1;
const MASK_XYZ: u32 = 0x7;
/// Operand B swizzle (dword 1 bits 8-15) that replicates w into every
/// component.
const SWIZZLE_B_WWWW: u32 = 0x1b << 8;

/// Vector ALU instruction that writes `operation(a, b)` to register
/// `destination` under `mask`, all operands registers.
fn vector(destination: u32, mask: u32, operation: u32, a: u32, b: u32, swizzles: u32) -> [u32; 3] {
    [SCALAR_NONE << 26 | (mask << 16) | destination, swizzles, ALU_SOURCES_REGISTERS | (operation << 24) | (a << 16) | (b << 8)]
}

/// What a textured pixel program does with the fetched texel before it
/// multiplies it by the vertex color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TexelConversion {
    Unchanged,
    /// Multiplies the color by the texel's own alpha: straight texels drawn
    /// with a blend mode whose source is premultiplied.
    Premultiply,
}

/// The texel conversion instructions on register 0, ahead of the export.
fn convert_texel(program: &mut Vec<u32>, conversion: TexelConversion) {
    match conversion {
        TexelConversion::Unchanged => {}
        TexelConversion::Premultiply => program.extend(vector(0, MASK_XYZ, VECTOR_MUL, 0, 0, SWIZZLE_B_WWWW)),
    }
}

/// Instructions of [`convert_texel`].
fn conversion_instructions(conversion: TexelConversion) -> u32 {
    match conversion {
        TexelConversion::Unchanged => 0,
        TexelConversion::Premultiply => 1,
    }
}

/// Scalar ALU instruction that writes `operation(c)` of register `source`
/// to the x component of register `destination`. The vector half writes
/// nothing.
fn scalar(destination: u32, operation: u32, source: u32, swizzle: u32) -> [u32; 3] {
    [operation << 26 | MASK_X << 20 | destination, swizzle, ALU_SOURCES_REGISTERS | (VECTOR_MAX << 24) | (source << 16) | (source << 8) | source]
}

/// Vertex fetch constant the vertex shader reads.
pub const VERTEX_FETCH_CONSTANT: u32 = 95;
/// Dwords per vertex: position x, y, uv u, v, color (the `Vertex` layout).
pub const VERTEX_DWORDS: u32 = 5;

/// Vertex shader: position (pixels) to the position export with z = 0 and
/// w = 1, uv to interpolator 0, color (red in the low byte) to interpolator 1.
pub fn vertex_program() -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::Exec { address: 3, count: 3, fetches: 0b111, vertex_fetches: 0b111, end: false },
        ControlFlow::AllocatePosition,
        ControlFlow::Exec { address: 6, count: 1, fetches: 0, vertex_fetches: 0, end: false },
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 7, count: 2, fetches: 0, vertex_fetches: 0, end: true },
        ControlFlow::Nop,
    ]);
    let fetch = |destination, mini, format, number_format, swizzle, offset| {
        VertexFetch { destination, constant: VERTEX_FETCH_CONSTANT, mini, format, number_format, swizzle, stride: VERTEX_DWORDS, offset }.encode()
    };
    program.extend(fetch(2, false, FORMAT_32_32_FLOAT, NUMBER_FLOAT, swizzle(0, 1, SELECT_ZERO, SELECT_ONE), 0));
    program.extend(fetch(1, true, FORMAT_32_32_FLOAT, NUMBER_FLOAT, swizzle(0, 1, SELECT_NONE, SELECT_NONE), 2));
    program.extend(fetch(0, true, FORMAT_8_8_8_8, NUMBER_NORMALIZED, swizzle(0, 1, 2, 3), 4));
    program.extend(export(EXPORT_POSITION, MASK_ALL, VECTOR_MAX, 2, 2, 0));
    program.extend(export(0, MASK_XY, VECTOR_MAX, 1, 1, SWIZZLE_XY));
    program.extend(export(1, MASK_ALL, VECTOR_MAX, 0, 0, 0));
    program
}

/// Pixel shader: texture 0 at interpolator 0, converted, times
/// interpolator 1.
pub fn textured_pixel_program(conversion: TexelConversion) -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::Exec { address: 2, count: 1, fetches: 1, vertex_fetches: 0, end: false },
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 3, count: 1 + conversion_instructions(conversion), fetches: 0, vertex_fetches: 0, end: true },
        ControlFlow::Nop,
    ]);
    program.extend(texture_fetch(0, 0, 0));
    convert_texel(&mut program, conversion);
    program.extend(export(0, MASK_ALL, VECTOR_MUL, 0, 1, 0));
    program
}

/// Pixel shader: texture 0 at `(2 * length(r0.xy), r0.y^2)`, converted,
/// times interpolator 1, for texture coordinates that the renderer has
/// moved by -0.5 so that the center of the gradient is at the origin. The
/// textures it samples are one row high, so only x selects a texel.
pub fn radial_pixel_program(conversion: TexelConversion) -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::Exec { address: 2, count: 5, fetches: 0b1_0000, vertex_fetches: 0, end: false },
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 7, count: 1 + conversion_instructions(conversion), fetches: 0, vertex_fetches: 0, end: true },
        ControlFlow::Nop,
    ]);
    program.extend(vector(0, MASK_XY, VECTOR_MUL, 0, 0, 0));
    program.extend(scalar(0, SCALAR_ADD, 0, SWIZZLE_XYYY));
    program.extend(scalar(0, SCALAR_SQRT, 0, SWIZZLE_XXXX));
    program.extend(scalar(0, SCALAR_ADD, 0, SWIZZLE_XXXX));
    program.extend(texture_fetch(0, 0, 0));
    convert_texel(&mut program, conversion);
    program.extend(export(0, MASK_ALL, VECTOR_MUL, 0, 1, 0));
    program
}

/// Pixel shader: interpolator 1.
pub fn color_pixel_program() -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 1, count: 1, fetches: 0, vertex_fetches: 0, end: true },
    ]);
    program.extend(export(0, MASK_ALL, VECTOR_MAX, 1, 1, 0));
    program
}

/// An ALU operand: a temporary register or a float constant, read through
/// a source swizzle.
#[derive(Clone, Copy)]
enum Source {
    Register(u32, u32),
    Constant(u32, u32),
}

impl Source {
    fn index(self) -> u32 {
        match self {
            Self::Register(index, _) | Self::Constant(index, _) => index,
        }
    }

    fn swizzle(self) -> u32 {
        match self {
            Self::Register(_, swizzle) | Self::Constant(_, swizzle) => swizzle,
        }
    }

    fn is_register(self) -> u32 {
        u32::from(matches!(self, Self::Register(..)))
    }
}

/// Source swizzle selecting component `x`, `y`, `z`, `w` (0-3) into
/// components 0-3: 2 bits per component i at bits 2i, holding the selected
/// component minus i, modulo 4.
const fn source_swizzle(x: u32, y: u32, z: u32, w: u32) -> u32 {
    (x & 3) | ((y.wrapping_sub(1)) & 3) << 2 | ((z.wrapping_sub(2)) & 3) << 4 | ((w.wrapping_sub(3)) & 3) << 6
}

const X: u32 = 0;
const Y: u32 = 1;
const Z: u32 = 2;
const W: u32 = 3;
const MASK_Y: u32 = 0x2;
const MASK_Z: u32 = 0x4;
const MASK_W: u32 = 0x8;
const VECTOR_ADD: u32 = 0;
/// `a * b + c`.
const VECTOR_MULTIPLY_ADD: u32 = 11;
/// `register * constant` of one component each (see [`Alu::scalar`]).
const SCALAR_MULTIPLY_CONSTANT: u32 = 42;
/// `constant - register` of one component each.
const SCALAR_SUBTRACT_CONSTANT: u32 = 46;
/// `1 / c.w` of the swizzled operand C.
const SCALAR_RECIPROCAL: u32 = 19;
/// Dword 1 bits that negate operand A and operand B.
const NEGATE_A: u32 = 1 << 26;
const NEGATE_B: u32 = 1 << 25;

/// An ALU instruction with a vector and a scalar half writing the same
/// register (or export).
struct Alu {
    destination: u32,
    export: bool,
    vector: u32,
    vector_mask: u32,
    scalar: u32,
    scalar_mask: u32,
    a: Source,
    b: Source,
    c: Source,
    negate: u32,
}

impl Alu {
    /// A vector operation without a scalar half.
    fn vector(destination: u32, mask: u32, operation: u32, a: Source, b: Source, c: Source) -> Self {
        Self { destination, export: false, vector: operation, vector_mask: mask, scalar: SCALAR_NONE, scalar_mask: 0, a, b, c, negate: 0 }
    }

    /// Adds a scalar operation on operand C. The constant forms
    /// ([`SCALAR_MULTIPLY_CONSTANT`], [`SCALAR_SUBTRACT_CONSTANT`]) take
    /// constant C's component through swizzle component 3 and register
    /// r0's component through swizzle component 0, as the dashboard's text
    /// programs use them; swizzle components 1 and 2 stay at their
    /// identity, the only form observed.
    fn scalar(self, operation: u32, mask: u32) -> Self {
        Self { scalar: operation, scalar_mask: mask, ..self }
    }

    fn negate(self, negate: u32) -> Self {
        Self { negate, ..self }
    }

    fn export(self) -> Self {
        Self { export: true, ..self }
    }

    fn encode(&self) -> [u32; 3] {
        let export = if self.export { ALU_EXPORT } else { 0 };
        [
            self.scalar << 26 | self.scalar_mask << 20 | self.vector_mask << 16 | export | self.destination,
            self.negate | self.a.swizzle() << 16 | self.b.swizzle() << 8 | self.c.swizzle(),
            self.a.is_register() << 31 | self.b.is_register() << 30 | self.c.is_register() << 29 | self.vector << 24 | self.a.index() << 16 | self.b.index() << 8 | self.c.index(),
        ]
    }
}

/// Pixel constants the text shadow program reads.
pub const SHADOW_COLOR_CONSTANT: u32 = 2;
/// Smallest denominator used to convert a premultiplied shadow source.
pub const SHADOW_ALPHA_EPSILON: f32 = 1.0 / 65_536.0;
/// `(1 / texture width, 1 / texture height, alpha epsilon, 1)`.
pub const SHADOW_OFFSET_CONSTANT: u32 = 3;

/// Which source the shadow program outputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowOutput {
    /// Premultiplied color, from a premultiplied vertex color.
    Premultiplied,
    /// Straight color, from a straight vertex color.
    Straight,
}

/// Pixel shader of a textured command with a drop shadow (`TextShadow`):
/// texture 0 alpha at interpolator 0 (`t0`) and one texel up and left of
/// it (`t1`), the vertex color `v` in interpolator 1, the straight shadow
/// color `c` in [`SHADOW_COLOR_CONSTANT`]. With `g = t0 * v.a` and `s = t1
/// * c.a`: alpha `1 - (1 - g)(1 - s)`, color `t0 * v.rgb + s(1 - g) c.rgb`
/// (premultiplied output) or `(g * v.rgb + s(1 - g) c.rgb) / alpha`
/// (straight output). The arithmetic follows the dashboard's text shadow
/// program (xui-text.md).
pub fn shadow_pixel_program(output: ShadowOutput) -> Vec<u32> {
    use Source::Constant;
    use Source::Register;

    let identity = source_swizzle(X, Y, Z, W);
    let all = |component| source_swizzle(component, component, component, component);
    let xyz = source_swizzle(X, Y, Z, Z);
    let xy = source_swizzle(X, Y, Y, Y);
    let unused = Register(0, identity);


    let mut alu = Vec::new();
    if output == ShadowOutput::Premultiplied {
        // r3.rgb = t0 * v.rgb
        alu.push(Alu::vector(3, MASK_XYZ, VECTOR_MUL, Register(0, all(Y)), Register(1, xyz), unused));
    }
    // r0.y = g; r0.z = r0.x * c.w = s
    alu.push(
        Alu::vector(0, MASK_Y, VECTOR_MUL, Register(0, all(Y)), Register(1, all(W)), Constant(SHADOW_COLOR_CONSTANT, identity))
            .scalar(SCALAR_MULTIPLY_CONSTANT, MASK_Z),
    );
    // r0.w = 1 - s; r0.x = 1 - g
    alu.push(
        Alu::vector(
            0,
            MASK_W,
            VECTOR_ADD,
            Register(0, all(Z)),
            Constant(SHADOW_OFFSET_CONSTANT, all(W)),
            Constant(SHADOW_OFFSET_CONSTANT, source_swizzle(Y, Y, Z, W)),
        )
        .negate(NEGATE_A)
        .scalar(SCALAR_SUBTRACT_CONSTANT, MASK_X),
    );
    // r3.w = 1 - (1 - g)(1 - s)
    alu.push(
        Alu::vector(
            3,
            MASK_W,
            VECTOR_MULTIPLY_ADD,
            Register(0, all(X)),
            Register(0, all(W)),
            Constant(SHADOW_OFFSET_CONSTANT, all(W)),
        )
        .negate(NEGATE_A),
    );
    match output {
        ShadowOutput::Premultiplied => {
            // r0.x = s(1 - g)
            alu.push(Alu::vector(0, MASK_X, VECTOR_MUL, Register(0, all(Z)), Register(0, all(X)), unused));
            // r3.rgb = r0.x * c.rgb + r3.rgb
            alu.push(Alu::vector(3, MASK_XYZ, VECTOR_MULTIPLY_ADD, Register(0, all(X)), Constant(SHADOW_COLOR_CONSTANT, xyz), Register(3, xyz)));
        }
        ShadowOutput::Straight => {
            // r2.w = max(alpha, epsilon)
            alu.push(Alu::vector(
                2,
                MASK_W,
                VECTOR_MAX,
                Register(3, all(W)),
                Constant(SHADOW_OFFSET_CONSTANT, all(Z)),
                unused,
            ));
            // r0.x = s(1 - g); r0.z = 1 / r2.w
            alu.push(
                Alu::vector(0, MASK_X, VECTOR_MUL, Register(0, all(Z)), Register(0, all(X)), Register(2, all(W)))
                    .scalar(SCALAR_RECIPROCAL, MASK_Z),
            );
            // r2.xy = (g, s(1 - g)) / alpha
            alu.push(Alu::vector(
                2,
                MASK_XY,
                VECTOR_MUL,
                Register(0, source_swizzle(Y, X, X, X)),
                Register(0, all(Z)),
                Constant(0, identity),
            ));
            // r0.rgb = r2.y * c.rgb
            alu.push(Alu::vector(
                0,
                MASK_XYZ,
                VECTOR_MUL,
                Register(2, all(Y)),
                Constant(SHADOW_COLOR_CONSTANT, xyz),
                Constant(0, identity),
            ));
            // r3.rgb = r2.x * v.rgb + r0.rgb
            alu.push(Alu::vector(
                3,
                MASK_XYZ,
                VECTOR_MULTIPLY_ADD,
                Register(2, all(X)),
                Register(1, xyz),
                Register(0, xyz),
            ));
        }
    }
    let body = alu.len() as u32;
    let body_address = 5;

    let mut program = Vec::new();
    if body <= 6 {
        control_flow(&mut program, &[
            ControlFlow::Exec { address: 2, count: 3, fetches: 0b110, vertex_fetches: 0, end: false },
            ControlFlow::AllocateParameters,
            ControlFlow::Exec { address: body_address, count: body, fetches: 0, vertex_fetches: 0, end: false },
            ControlFlow::Exec { address: body_address + body, count: 1, fetches: 0, vertex_fetches: 0, end: true },
        ]);
    } else {
        control_flow(&mut program, &[
            ControlFlow::Exec { address: 2, count: 3, fetches: 0b110, vertex_fetches: 0, end: false },
            ControlFlow::AllocateParameters,
            ControlFlow::Exec { address: body_address, count: 6, fetches: 0, vertex_fetches: 0, end: false },
            ControlFlow::Exec { address: body_address + 6, count: body - 5, fetches: 0, vertex_fetches: 0, end: true },
        ]);
    }
    // r2.xy = r0.xy - offset
    program.extend(
        Alu::vector(2, MASK_XY, VECTOR_ADD, Register(0, xy), Constant(SHADOW_OFFSET_CONSTANT, xy), unused).negate(NEGATE_B).encode(),
    );
    // r0.y = t0 (alpha at r0), r0.x = t1 (alpha at r2)
    program.extend(alpha_fetch(0, 0, Y));
    program.extend(alpha_fetch(0, 2, X));
    for instruction in &alu {
        program.extend(instruction.encode());
    }
    program.extend(Alu::vector(0, MASK_ALL, VECTOR_MAX, Register(3, identity), Register(3, identity), unused).export().encode());
    program
}

/// Texture fetch of texture 0 at register `source` that writes the texel's
/// alpha to `component` of `destination` and leaves the others.
fn alpha_fetch(destination: u32, source: u32, component: u32) -> [u32; 3] {
    let mut select = [SELECT_NONE; 4];
    select[component as usize] = W;
    let [word0, _, word2] = texture_fetch(destination, source, 0);
    [word0, swizzle(select[0], select[1], select[2], select[3]) | TEXTURE_FETCH_FILTER_BITS, word2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_are_whole_instructions_after_their_control_flow() {
        for (program, control) in [(vertex_program(), 9), (textured_pixel_program(TexelConversion::Unchanged), 6), (color_pixel_program(), 3)] {
            assert_eq!(program.len() % 3, 0);
            assert!(program.len() > control);
        }
    }

    #[test]
    fn premultiply_adds_one_instruction_to_the_final_exec() {
        let programs: [(fn(TexelConversion) -> Vec<u32>, usize, usize); 2] = [(textured_pixel_program, 6, 2), (radial_pixel_program, 6, 6)];
        for (program, control, instructions) in programs {
            let unchanged = program(TexelConversion::Unchanged);
            let premultiplied = program(TexelConversion::Premultiply);
            assert_eq!(unchanged.len(), control + 3 * instructions);
            assert_eq!(premultiplied.len(), control + 3 * (instructions + 1));
            // The final exec is instruction A of the second control flow
            // pair: dword 3, count in bits 12-15.
            let exec_count = |program: &[u32]| program[3] >> 12 & 0xf;
            assert_eq!(exec_count(&premultiplied), exec_count(&unchanged) + 1);
        }
    }

    #[test]
    fn exec_addresses_point_past_the_control_flow() {
        // The vertex program's three control flow triplets occupy addresses 0-2.
        let program = vertex_program();
        assert_eq!(program[0] & 0xfff, 3);
        assert_eq!((program[0] >> 12) & 0xf, 3);
    }

    #[test]
    fn control_flow_pairs_split_instruction_b_across_dwords() {
        let mut program = Vec::new();
        control_flow(&mut program, &[ControlFlow::Nop, ControlFlow::Exec { address: 7, count: 1, fetches: 0, vertex_fetches: 0, end: true }]);
        assert_eq!(program, [0, 0x1007 << 16, 0x2200_0000]);
    }

    #[test]
    fn shadow_execs_keep_the_hardware_instruction_limit() {
        let counts = |program: &[u32]| (program[3] >> 12 & 0xf, program[4] >> 28);

        assert_eq!(counts(&shadow_pixel_program(ShadowOutput::Premultiplied)), (6, 1));
        assert_eq!(counts(&shadow_pixel_program(ShadowOutput::Straight)), (6, 3));
    }
}
