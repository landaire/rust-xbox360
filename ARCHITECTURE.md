# Architecture

Goal: a Rust/LLVM toolchain that compiles Rust to a XEX which runs on Xbox 360
(verified in Xenia), replacing Microsoft's proprietary toolchain. End target is
stdlib Rust with working multithreading and networking.

## Target CPU and ABI

The Xenon is a 64-bit big-endian PowerPC (PPE lineage) with VMX128. Titles run
with 32-bit effective addresses: `sizeof(void*) == 4` (ILP32) on 64-bit
registers. LLVM and Rust have no ILP32-on-PPC64 mode, so we target 32-bit
big-endian PowerPC, which the Xenon executes natively as a superset. 64-bit
integer ops lower to runtime helpers; VMX128 is deferred.

Microsoft ABI facts that constrain codegen:

- Args in r3-r10, FP in f1-f13, vector in v1-v13. Return in r3/f1/v1.
- Struct > 64 bits returned via hidden sret pointer in r3.
- r1 = stack pointer, r13 = thread/PCR pointer. r2 unused (no TOC).
- Nonvolatile: r14-r31, f14-f31, v64-v127, CR fields 2-5.
- 16-byte stack alignment, 64-byte parameter home area, 1344-byte red zone,
  back chain at [r1], LR save slot, no CR save area.
- No TOC and no function descriptors. Globals via absolute lis/addi. Calls go
  direct to the entry point; cross-module calls resolve through XEX import
  thunks, not a GOT.

For a self-contained homebrew title we only need the register-level convention
to match at import boundaries. `.pdata` unwind and millicode save/restore
helpers are required for full MS compatibility but not for a title to run.
With `rewrite`, `xbox360-asm-rewrite` gives a title shared save/restore
helpers of its own (`__xbox360_savegprlr_K_M`, `__xbox360_restgprlr_K_M`,
`__xbox360_restlr`) by rewriting the LTO assembly before it is assembled
and linked.

The parameter home area is required on hardware. Console callees store
r3-r10 into the caller's frame at sp+0x10..sp+0x50, including non-variadic
kernel functions and `DbgPrint`. The SysV frames LLVM emits do not reserve that
area, so a direct call from a small frame overwrites the caller's saved
registers and LR. Xenia does not reproduce this. A call through an import must
allocate a 0x50-byte frame first. The `xbox360-sys` import stubs do this, and
the std overlay calls the kernel through them.

## Pipeline

    Rust (.rs)
      -> rustc + LLVM 22, target powerpc-unknown-xbox360 (custom JSON)
      -> ELF32 PPC big-endian object(s)
      -> rust-lld with linker script, image base 0x82000000
      -> ELF32 PPC executable
      -> xex-link: ELF -> devkit-signed XEX in one step
      -> Xenia (--headless) runs it

`xex-link` is the single post-link step: it relayouts the PT_LOAD segments into
an Xbox PPC PE image (machine POWERPCBE 0x01F2, 32-bit optional header) in
memory, synthesizes the import thunks + IMPORT_LIBRARIES table, and wraps the
result as a devkit-signed XEX2 via acceleration's `Xex2Builder` -- all without
writing an intermediate ELF-to-EXE file. Title id, load address, media id,
versions, image/module flags, compression, encryption, and the export table are
read from a TOML manifest (`--manifest`; each example has an `xex.toml`); run
`xex-link --help`. Keeping this conversion an external post-link tool (rather
than folding it into LLVM) is the idiomatic pattern; see
[docs/llvm-integration.md](docs/llvm-integration.md).

Xenia loads the XEX by reconstructing the PE image, mapping sections, and
rewriting each kernel import thunk to a `sc 2` syscall trap dispatched to its
host implementation. So the XEX only needs correct import records naming
`(library, ordinal)`; Xenia supplies the kernel/Xam behavior.

## Imports

Kernel and Xam functions are imported by ordinal, not name. The XEX
IMPORT_LIBRARIES header (key 0x000103FF) lists library names
(`xboxkrnl.exe`, `xam.xex`) and, per library, an array of VAs pointing at
import records inside the image. Each record DWORD encodes
`type<<24 | libindex<<16 | ordinal`. Type 1 records are 16-byte code thunks in
`.text`; Xenia rewrites kernel thunks to `sc 2; blr`. Type 0 records are data
IAT slots.

Each library entry also carries the exporting module's identifier. The
hardware loader rejects a mismatch with `C0000225`. `xex-link` emits
`0x45dc17e0` for `xboxkrnl.exe`, `0xfca15c76` for `xam.xex`, and `0xeceb8109`
for `xbdm.xex`.

Key ordinals (verified in Xenia export tables):

- xboxkrnl.exe: DbgPrint 0x03, ExAllocatePoolWithTag 0x0A, ExCreateThread 0x0D.
- xam.xex networking (NetDll_*): WSAStartup 0x01, socket 0x03, closesocket
  0x04, bind 0x0B, connect 0x0C, listen 0x0D, accept 0x0E, select 0x0F,
  recv 0x12, recvfrom 0x14, send 0x16, sendto 0x18, inet_addr 0x1A,
  XNetStartup 0x33, XNetCleanup 0x34, XNetRandom 0x35.

## Threading model

`ExCreateThread(handle_out, stack_size, tid_out, xapi_startup, start, ctx,
flags)`. A raw thread (xapi_startup 0) enters `start` with r3 = ctx and returns
its exit code in r3. r13 points at the per-thread PCR; static TLS is at 0(r13).
Xenia runs each guest thread on a host thread; atomics use lwarx/stwcx.

## Networking model

All sockets are `NetDll_*` exports in xam.xex mapping to host BSD sockets
(AF_INET, TCP/UDP). `XNetStartup`/`WSAStartup` are effectively no-ops in Xenia.
Raw UDP/TCP works; XNet identity, QoS, DNS, and secure association do not.

## Reuse

- `acceleration` (path dependency, kept out of tree to avoid committing the
  retail AES key): `xex2::Xex2Builder` already emits a Xenia-loadable devkit
  XEX from a PE image. We extend it to populate a non-empty IMPORT_LIBRARIES
  table. Also provides LZX, page descriptors, RSA devkit signing, and shared
  newtypes (`xenon_types`).
- `binja-xex2` and Xenia are format references.

## Rust build

Custom target JSON consumed by stable rustc via `RUSTC_BOOTSTRAP=1` plus
`-Z build-std`. `~/src/rust` (a 1.100 dev tree) is reserved for the endgame:
building a compiler with a real std port (thread/net/sys) for this target so
that plain stdlib Rust compiles. Early milestones are `#![no_std]` and bind the
imports directly.
