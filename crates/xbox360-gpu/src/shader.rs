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

/// Vector ALU instruction that writes `operation(a, b)` to register
/// `destination` under `mask`, all operands registers.
fn vector(destination: u32, mask: u32, operation: u32, a: u32, b: u32) -> [u32; 3] {
    [SCALAR_NONE << 26 | (mask << 16) | destination, 0, ALU_SOURCES_REGISTERS | (operation << 24) | (a << 16) | (b << 8)]
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

/// Pixel shader: texture 0 at interpolator 0 times interpolator 1.
pub fn textured_pixel_program() -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::Exec { address: 2, count: 1, fetches: 1, vertex_fetches: 0, end: false },
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 3, count: 1, fetches: 0, vertex_fetches: 0, end: true },
        ControlFlow::Nop,
    ]);
    program.extend(texture_fetch(0, 0, 0));
    program.extend(export(0, MASK_ALL, VECTOR_MUL, 0, 1, 0));
    program
}

/// Pixel shader: texture 0 at `(2 * length(r0.xy), r0.y^2)` times
/// interpolator 1, for texture coordinates that the renderer has moved by
/// -0.5 so that the center of the gradient is at the origin. The textures
/// it samples are one row high, so only x selects a texel.
pub fn radial_pixel_program() -> Vec<u32> {
    let mut program = Vec::new();
    control_flow(&mut program, &[
        ControlFlow::Exec { address: 2, count: 5, fetches: 0b1_0000, vertex_fetches: 0, end: false },
        ControlFlow::AllocateParameters,
        ControlFlow::Exec { address: 7, count: 1, fetches: 0, vertex_fetches: 0, end: true },
        ControlFlow::Nop,
    ]);
    program.extend(vector(0, MASK_XY, VECTOR_MUL, 0, 0));
    program.extend(scalar(0, SCALAR_ADD, 0, SWIZZLE_XYYY));
    program.extend(scalar(0, SCALAR_SQRT, 0, SWIZZLE_XXXX));
    program.extend(scalar(0, SCALAR_ADD, 0, SWIZZLE_XXXX));
    program.extend(texture_fetch(0, 0, 0));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_are_whole_instructions_after_their_control_flow() {
        for (program, control) in [(vertex_program(), 9), (textured_pixel_program(), 6), (color_pixel_program(), 3)] {
            assert_eq!(program.len() % 3, 0);
            assert!(program.len() > control);
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
}
