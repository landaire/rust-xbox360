# Xenon (Xbox 360 PowerPC) instructions and toolchain notes

Instruction and ABI facts that inform the `powerpc-unknown-xbox360` target and
codegen. Kernel structures come from the binja-xex2 `ntos` headers.

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

- 128 vector registers (Vr0-Vr127), double stock AltiVec's 32. The calling
  convention must save the full nonvolatile VMX128 set (CONTEXT saves all 128).
- Unaligned SIMD idiom: `lvlx`/`lvrx` (+ `stvlx`/`stvrx`) with `lvsl`/`lvsr`
  shift-control vectors.
- VMX128-only ops (graphics): `vmsum3fp128`/`vmsum4fp128` (dot products),
  `vpermwi128`, `vrlimi128`, `vpkd3d128`/`vupkd3d128` (hardware pack/unpack to
  D3DCOLOR, FLOAT16, normalized-short). Upstream LLVM does not model VMX128
  natively; emit via inline `.long` if needed.

## Inline assembly

The assembler accepts the 64-bit and supervisor mnemonics for this target
(`ld`, `std`, `ldarx`, `stdcx.`, `cmpd`, `rldimi`, `rldicl`, `mtmsrd`), even
though the CPU model is 32-bit. `dcbz128` has no mnemonic; `dcbzl` assembles to
the same word. `asm!(".long 0x...")` emits a 32-bit PPC word inline for
instructions the assembler lacks: `lvlx`/`lvrx`/`stvlx`/`stvrx` and the VMX128
operations.

## Target specifications

The default `powerpc-unknown-xbox360` target uses 32-bit PowerPC code generation and keeps 64-bit integers in register pairs.

The experimental `powerpc64-unknown-xbox360` target enables LLVM's `+xbox360` feature. It retains 32-bit pointers and ELF32 output, but passes and returns each `i64` in one 64-bit GPR. The feature also reserves the 16-byte linkage area and eight 8-byte argument home slots observed in genuine binaries. It preserves nonvolatile GPRs at full width. It requires the matching experimental LLVM and Rust compiler builds. The unmodified Rust 1.98 release has neither the LLVM target nor the Xbox aggregate ABI classifier.

Buck exposes matching `xbox360-ppc64-ilp32-core`, `-std`, `-std-system`, and `-sysroot` platforms. The standard platforms and target remain the default.

Both targets have a maximum atomic width of 64 bits. The standard-library overlay implements 8-byte atomics with protected `ldarx`/`stdcx.` and `ld`/`std`.

Xenon predates VSX. Enable AltiVec if used, but do not enable `+vsx`. Xenon has `isel` and the `fsel`, `fres`, and `frsqrte` estimates. LLVM has no `-mcpu=xenon` model.

The reverse-engineered evidence and remaining LLVM gaps are in [`llvm-xbox360-abi-evidence.md`](llvm-xbox360-abi-evidence.md).

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
