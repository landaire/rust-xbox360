# Xenon (Xbox 360 PowerPC) instructions and toolchain notes

Instruction and ABI facts that inform the `powerpc-unknown-xbox360` target and
codegen. Kernel structures come from the binja-xex2 `ntos` headers.

## Execution mode

Console title probes read MSR[SF]=0. Instruction availability, register
width, pointer width, ABI width and execution mode are separate properties.

- Integer record forms classify the low 32 bits, including `rldicl.`,
  `srd.`, `add.`, `and.` and `mulld.`. They cannot replace doubleword
  comparisons. Explicit `cmpd`, `cmpld`, `cmpdi` and `cmpldi` compare all
  64 bits.
- Add/subtract carry and overflow flags use the low word. Doubleword
  overflowing arithmetic and multiword carry chains require explicit
  doubleword comparisons. `srad`/`sradi` carry uses the doubleword sign
  and shifted-out bits; `sradi` followed by `addze` remains valid.
- CTR retains 64 bits, but `bdnz`/`bdz` test only the low word after
  decrement. Counts larger than 32 bits cannot use CTR loops.
- Loads use the low 32 bits of the effective address. Update forms
  retain the full computed base register value.

`examples/sfmode` probes each instruction in assembly, without relying on
compiler-generated doubleword comparisons. The SF=0 and SF=1 passes
produced 140 result lines with no unexpected result. A separate
compiler-output probe tested 11 boundary cases across eight outputs:
the previous compiler produced 13 incorrect outputs; the execution-mode
fix produced none.

## Atomics / reservations

- Reservations use `lwarx`/`stwcx.` and, for 64-bit, `ldarx`/`stdcx.`.
- Acquire/release ordering adds an `lwsync` (`0x7C2004AC`).
- Barriers: `sync` = `0x7c0004ac` (full), `lwsync` = `0x7C2004AC` (lightweight),
  `eieio` = `0x7c0006ac`.
- EMPIRICAL (this project): the DEVKIT kernel traps a title's `lwarx`/`stwcx.`
  unless it is the interrupt-protected form:
  `mtmsrd r13, 1` (disable EE, using the per-thread block pointer) immediately
  before `lwarx`, restore right after `stwcx.`, and every lwarx path reaches a
  stwcx. See the memory note `xbox360-lwarx-stwcx-trap`. This is what
  `core/sync/atomic_xbox360.rs` in the std overlay implements.

## Cache / memory

- 128-byte cache line. `dcbz128` zeroes a 128-byte line (the fast memset-0
  primitive); `dcbz` still only zeroes 32. `dcbt`/`dcbst`/`dcbf` are
  prefetch/store/flush. Any dcbz-based memset lowering must use `dcbz128` and
  128-byte alignment.
- LLVM target recommendation: model a 128-byte cache line; prefer 128-byte
  alignment for hot/streamed buffers.

## Timing / SMT

- `mftb`/`mftbu` read the 64-bit timebase. Frequency via
  `KeQueryPerformanceFrequency` (~50 MHz). The default target reads the two
  32-bit halves. The experimental target keeps the value in one 64-bit GPR.
- SMT yield is `or r31,r31,r31` (`0x7FFFFB78`, ~16 dispatch cycles). Better
  than a kernel yield for spin loops.

## SIMD (VMX / VMX128)

- 128 vector registers (v0-v127), four times stock AltiVec's 32. Xenon has no
  VSX, no vector integer multiply, and lacks some AltiVec instructions
  (`vmsum*`, `vmul[eo]*`, `vsum*`, `vmh[r]addshs`, `vmladduhm`, `dst*`);
  LLVM expands their intrinsics into scalar code. Disabling VMX128 registers
  and encodings does not make these missing classic instructions available.
- The PPC64 target (`+altivec,+vmx128`) allocates all 128 registers and emits
  the VMX128 forms for float arithmetic, fused multiply-add, min/max,
  estimates, rounding, conversions, compares, logical operations, selects,
  word shifts, permutes, splats and aligned loads and stores. Integer
  add/subtract/compare/min/max and byte or halfword operations have no
  VMX128 form and stay in v0-v31. The 128 forms run at the same speed as
  the classic forms, also in v64-v127. The PPC32 target does not use vector
  registers above v31.
- `lvsl128` and `lvsr128` raise an illegal instruction exception on the
  console; the classic `lvsl`/`lvsr` work. All other VMX128 loads and stores
  (`lvx128`, `lvxl128`, `lvlx128`, `lvrx128`, `lvlxl128`, `lvrxl128` and the
  stores) execute on cached memory.
- Cached unaligned access: `lvlx(p) | lvrx(p + 16)` loads the 16 bytes at `p`;
  `stvlx(v, p); stvrx(v, p + 16)` stores them. With `n = p & 15`, `lvlx`
  loads `p..(p | 15)` into bytes `0..16 - n` and `lvrx` loads
  `(p & !15)..p` into bytes `16 - n..16` (nothing when `p` is aligned); the
  other bytes are zero.
  Page-edge probes include an unmapped page on an aligned, zero-length right
  access. `lvlx128` faults with alignment exception 0x80000002 on physical
  write-combined memory; partial-vector pairs cannot implement generic
  unaligned pointers whose cache mode is unknown.
- Classic `stvebx`, `stvehx` and `stvewx` preserve adjacent bytes on physical
  write-combined memory at every naturally aligned lane.
- Saturating `vaddshs`, `vaddubs`, `vsubuhs`, `vctsxs` and `vctuxs` update
  sticky VSCR SAT, including when the vector result is unused. In-range
  conversions leave SAT clear. Native intrinsics preserve these side effects.
- VMX128-only operations: `vmsum3fp128`/`vmsum4fp128` (dot products),
  `vpermwi128` (word permute by immediate), `vrlimi128` (rotate and insert
  words), `vpkd3d128`/`vupkd3d128` (D3D vertex format pack and unpack).
  The dot products are not exactly rounded: they are exact for
  non-negative integer-valued inputs, (1,1,1,1).(1,-1,1,-1) gives -2^-28
  instead of 0, and over 65536 random inputs the error was at most 3.96
  units in the last place of the largest product. `vpkd3d128` takes every
  integer field as `3.0 + x * 2^-22` and saturates (signed fields to
  `±(2^(w-1) - 1)`); `vupkd3d128` gives unsigned fields as `1.0 + x * 2^-23`
  and signed ones as `3.0 + x * 2^-22`; half floats have no infinity or NaN
  and pack by truncation.

### Vector calling convention (PPC64 target)

The same as the console compiler's, for `extern "C"` and `extern "system"`:

- 16-byte vectors of `f32` or of integers up to 32 bits (`core::simd::f32x4`,
  `i32x4`, `u8x16`, ..., and `#[repr(transparent)]` wrappers, the
  equivalent of `__vector4`) are passed in v1-v13 and returned in v1. They
  use no GPR or FPR. Each takes a 16-byte aligned slot of the parameter
  area; the fourteenth and later vectors are stored there.
- Other SIMD types (`f64x2`, 8- or 32-byte vectors) and structs containing
  vectors are passed like structs of the same size. Vectors cannot be
  passed to variadic functions.
- v64-v127 are callee-saved, v0-v63 are volatile (v14-v31 included, unlike
  the AltiVec SVR4 ABI). A function saves the ones it uses below the GPR
  and FPR save areas, `vN` at `B - 16 * (128 - N)` from the incoming stack
  pointer with `B = -align16(8 + 8 * GPRs + 8 * FPRs)`, before the frame is
  allocated. VRSAVE and VSCR are not part of the convention.
- Structs of 1, 2 or 4 bytes pass right-justified in a GPR, 3 and 5 to 8
  bytes left-justified, larger ones in memory; only 1, 2, 4 and 8 byte
  structs return in r3, the others through a hidden pointer. `char` is
  signed, so `core::ffi::c_char` is `i8` on both targets.

`examples/vmx128test` checks this against functions compiled by the console
compiler, in both directions, including 70 vectors kept live across calls.

### Rust

The `xbox360-vmx128` crate (PPC64 only, nightly features) wraps the
operations without a portable form, with immediates as const generics:

```rust
use core::simd::f32x4;
use xbox360_vmx128 as vmx;

let d = vmx::dot3(a, b); // every word: a.x*b.x + a.y*b.y + a.z*b.z
let r = vmx::permute_words::<0xE4, _>(v); // reverse the words
let packed = vmx::pack_d3d::<{ vmx::d3d::FLOAT16_4 }, 2, 0>(into, v);
let x: f32x4 = unsafe { vmx::load_unaligned(ptr) };
```

Everything else is plain `core::simd`, including
`f32x4::simd_min`/`simd_max` without VSX.

### Vector registers in inline assembly

The PPC64 assembler accepts every VMX128 mnemonic, with registers written
as numbers (`vaddfp128 100, 2, 127`; `lvx128 64, 0, 3`). Rust `asm!`
operands can name v0-v127 explicitly when VMX128 is enabled. The `vreg`
class allocates v0-v31, so it remains usable with classic AltiVec
instructions. `clobber_abi("C")` and `clobber_abi("system")` on the PPC64
Xbox target include v0-v63; v64-v127 remain callee-saved. With VMX128
disabled, high-register clobbers reserve no allocatable vector values.
An assembly block must declare every register it changes. A
`global_asm!` function must follow the convention: v0-v63 are free,
v64-v127 must be saved (see `examples/vmx128test/src/probes.s`).
On the PPC32 target the VMX128 instructions need `.long` words.

### Console verification

`examples/vmx128test/run.sh /tmp/vmx128test-run` builds and runs the SIMD
and vector ABI tests, including SDK-compiled functions from our own C
sources. `examples/sfmode/run.sh /tmp/sfmode-run` probes execution-mode
semantics. `examples/vmx128bench/run.sh /tmp/vmx128bench-run` measures
instruction costs, bandwidth and compiler output. Set `XBOX` and
`XEEDEE` for another development kit or client. Each runner serializes
console access and saves the debug log and result file.
The [measurement report](vmx128-measurements.md) records the final timing,
bandwidth, compiler comparison and raw console data.

## Inline assembly

The assembler accepts the 64-bit and supervisor mnemonics for this target
(`ld`, `std`, `ldarx`, `stdcx.`, `cmpd`, `rldimi`, `rldicl`, `mtmsrd`), even
though the CPU model is 32-bit. `dcbz128` has no mnemonic; `dcbzl` assembles to
the same word. On the PPC32 target, `asm!(".long 0x...")` emits a 32-bit PPC
word inline for instructions the assembler lacks: `lvlx`/`lvrx`/`stvlx`/`stvrx`
and the VMX128 operations. The PPC64 assembler accepts them.

## Target specifications

The default `powerpc-unknown-xbox360` target uses 32-bit PowerPC code generation and keeps 64-bit integers in register pairs.

The experimental `powerpc64-unknown-xbox360` target enables LLVM's `+xbox360` feature. It retains 32-bit pointers and ELF32 output, but passes and returns each `i64` in one 64-bit GPR. The feature also reserves the 16-byte linkage area and eight 8-byte argument home slots observed in genuine binaries. It uses the console compiler's frame layout (LR at SP-8 before `stwu`, CR at -4, full-width GPR saves from -16 down, FPRs below them), returns the `sret` pointer in `r3`, does not extend `i8`/`i16` arguments or returns, never widens copies past their known alignment, and probes the stack every 4 KiB. It enables AltiVec and VMX128 and uses the console compiler's vector calling convention (see above). Code is static by default; position-independent code follows the 32-bit ELF PIC model. It requires the matching experimental LLVM and Rust compiler builds. The unmodified Rust 1.98 release has neither the LLVM target nor the Xbox aggregate ABI classifier.

Buck exposes matching `xbox360-ppc64-ilp32-core`, `-std`, `-std-system`, and `-sysroot` platforms. The standard platforms and target remain the default.

Both targets have a maximum atomic width of 64 bits. The standard-library overlay implements 8-byte atomics with protected `ldarx`/`stdcx.` and `ld`/`std`.

Xenon predates VSX. Enable AltiVec if used, but do not enable `+vsx`. Xenon has `isel` and the `fsel`, `fres`, and `frsqrte` estimates. LLVM has no `-mcpu=xenon` model.

The reverse-engineered evidence and remaining LLVM gaps are in [`llvm-xbox360-abi-evidence.md`](llvm-xbox360-abi-evidence.md).

### LLVM target predicates

Keep instruction width, pointer width, ABI and execution mode separate.
The PowerPC backend uses these shared predicates rather than repeating
Xbox/PPC64 combinations at individual call sites:

| Predicate | Decision |
| --- | --- |
| `isPPC64()` | 64-bit instructions and register classes |
| Data-layout pointer type | 32-bit or 64-bit addresses |
| `usesPPC64ABI()` | 64-bit ABI rules; false for the Xbox ILP32 ABI |
| `uses32BitSVR4ABI()` | SVR4 paths shared by PPC32 and Xbox ILP32 |
| `is32BitELFABI()` | Original PPC32-only rules, including CR spills |
| `uses64BitExecutionMode()` | MSR[SF]-dependent optimizations |

`PPC::getTargetFeatureString()` supplies the same Xbox defaults to the
assembler and target machine. VMX128 is enabled by default and accepts an
explicit `-vmx128`; the Xbox ABI feature cannot be disabled independently
of the target's data layout.

## Kernel ABI notes discovered building the std backend

- Xbox `OBJECT_ATTRIBUTES` is 12 bytes: `{ HANDLE RootDirectory; PANSI_STRING
  ObjectName; ULONG Attributes; }` (no Length/SecurityDescriptor). `ANSI_STRING`
  is `{ u16 Length; u16 MaximumLength; char* Buffer; }`. `IO_STATUS_BLOCK` is
  `{ u32 Status; u32 Information; }`.
- `NtRead/WriteFile` take 8 register args (handle, event, apc, apcctx, iosb, buf,
  len, byteoffset). `GENERIC_READ` alone does not grant read for `NtReadFile` --
  add `FILE_READ_DATA`. The devkit HDD rejects reads shorter than the bytes
  remaining to EOF and DMAs only into title (stack/image) memory, not pool heap.
- `DbgPrint` and other console callees store r3-r10 into the caller's
  parameter home area at sp+0x10..sp+0x50. A SysV caller frame without that
  area loses saved registers and LR; see `ARCHITECTURE.md`.
