# Xbox 360 LLVM ABI evidence

This report records instruction-level evidence for the experimental LLVM
target. It uses reverse engineering of genuine Xbox 360 binaries plus
self-authored, black-box compiler probes. It does not rely on SDK
documentation, Xbox API headers or libraries, or source code.

## Inputs

| Input | SHA-256 |
| --- | --- |
| `xboxkrnl_21256.exe` | `c30d85d17f3be46b1f6afc1aaff07fce2272fd71b31e5f2b1b8f61c6258d773a` |
| `dash_2.0.6717.0.exe` | `4c9009cd5ed46b9319b0a20fc36018963bcfd374faaaa6ba0a13876bbd3a3672` |
| `hv.2.0.17559.0.bin` | `225d1c6907651126182e8a2ce74d7df59e7970c0086848114de20c35268609f5` |
| Xbox 360 `cl.exe` 16.00.11886.00 | `e35465b562f14a7f8eed5ca446a6f4f128477e86a676ea48dfeee38f1803bc9f` |
| Xbox 360 `cl.exe` 16.10.16045.00 | `21179754a5287cdcb8109053d8a2819cb742ba715f23dddeed6c2874072736b6` |

The executable images identify as PE32 with big-endian PowerPC machine type `0x01f2`. Binary Ninja 6.0.10601 was used for function boundaries, symbols, cross-references, and disassembly. Inferred decompiler prototypes were treated as search aids, not evidence. The conclusions below follow from instruction operands and caller-to-callee data flow.

## Predefined target macros

A header-free preprocessing probe expanded candidate macro names through
two-level stringization. This observes compiler-provided definitions without
including SDK headers or linking SDK libraries. Both compiler versions above
produced the same target-specific results:

| Macro | Expansion |
| --- | --- |
| `_XBOX_VER` | `200` |
| `_M_PPC` | `5401` |
| `_M_PPCBE` | `1` |
| `_WIN32` | `1` |

Common Xbox and Xenon spellings remained unexpanded, so the compiler does not
predefine `_XBOX`, `_XBOX360`, `__XBOX__`, `__XBOX360__`, `_XENON`,
`__XENON__`, or `_M_XENON`. The Clang target defines the four observed
target-specific macros and does not invent aliases. Compiler-identity macros
such as `_MSC_VER` are separate from target identification and remain governed
by Clang's Microsoft compatibility settings.

## Pointers are 32-bit values

`KeInitializeDpc` stores its routine and context pointer arguments into a DPC object with word stores:

```asm
0x80099684  stw     r4, 12(r3)
0x8009968c  stw     r5, 16(r3)
```

`KeCallAndWaitForDpcRoutine` constructs a callback record on the stack. It stores the callback and context pointers in adjacent 4-byte fields:

```asm
0x80096f44  stw     r10, 80(r1)
0x80096f4c  stw     r9, 84(r1)
```

The callback wrapper at `0x80096ae0` loads the same fields with word loads and invokes the loaded function pointer:

```asm
0x80096af4  lwz     r4, 4(r31)
0x80096af8  lwz     r11, 0(r31)
0x80096afc  mtctr   r11
0x80096b00  bctrl
```

Data pointers use the same representation. `KeSetTimerEx` loads and updates two linked-list pointers in 4-byte fields:

```asm
0x80097530  lwz     r10, 24(r31)
0x80097534  lwz     r9, 28(r31)
0x80097538  stw     r10, 0(r9)
0x8009753c  stw     r9, 4(r10)
```

These instruction sequences establish 4-byte data and function pointer objects. Full-width register saves do not imply 8-byte pointer objects. The callback wrapper preserves the physical nonvolatile `r31` with `std` and `ld`, but pointer fields held in that register are loaded with `lwz`. The ABI preserves all 64 bits of a nonvolatile GPR independently of the logical type currently held in it.

### Black-box MSVC confirmation

The Xbox 360 compiler was also tested with a self-authored translation unit
that includes no SDK headers and links no SDK libraries:

```c
__declspec(noinline) int pointer_size(void) {
    return sizeof(void *);
}

typedef char pointer_size_must_be_four[sizeof(void *) == 4 ? 1 : -1];
```

Microsoft Xbox 360 `cl.exe` version 16.00.11886.00 compiled the translation
unit successfully. Its assembly listing and the independent `dumpbin /disasm`
output both contain:

```asm
pointer_size:
38600004  li      r3, 4
4e800020  blr
```

This directly establishes that the compiler evaluates `sizeof(void *)` as 4
for its Xbox 360 target. The compile-time array bound independently requires
the same result.

## 64-bit integer arguments use one GPR

The dashboard calls the kernel `KeSetTimerEx` import at `0x922a4468`:

```asm
0x922a4440  li      r4, -0x2710
0x922a4458  lis     r5, 0x2
0x922a445c  ori     r5, r5, 0xbf20
0x922a4460  li      r6, 0x0
0x922a4464  addi    r3, r31, 0x1f0
0x922a4468  bl      KeSetTimerEx
```

The kernel entry at `0x800974d0` consumes the arguments without a register-pair reconstruction:

```asm
0x800974dc  mr      r31, r3
0x800974e0  mr      r30, r4
0x800974e4  mr      r28, r5
0x800974e8  mr      r29, r6
...
0x80097548  mr      r4, r30
0x80097550  mr      r3, r31
0x8009755c  bl      0x8009eb38
```

The downstream function performs a 64-bit comparison directly on `r4`:

```asm
0x8009eb60  cmpdi   cr6, r4, 0x0
```

This rules out the PPC32 SVR4 paired-register interpretation. The second argument occupies one 64-bit `r4`. The following arguments remain in `r5` and `r6`; no even-register alignment hole is inserted.

## 64-bit integer returns use one GPR

`KeQueryPerformanceFrequency` constructs its result directly in `r3`:

```asm
0x80098a34  lis     r3, 0x2fa
0x80098a38  ori     r3, r3, 0xf080
0x80098a3c  blr
```

The dashboard wrapper at `0x92114238` consumes and stores the returned value as one 64-bit register:

```asm
0x9211424c  bl      KeQueryPerformanceFrequency
0x92114250  mr      r11, r3
0x92114258  std     r11, 0(r31)
```

`KeQueryInterruptTime` provides a second independent example:

```asm
0x80096ab8  lis     r11, 0x8023
0x80096abc  ld      r3, 3808(r11)
0x80096ac0  blr
```

The ABI therefore returns a scalar `i64` in one 64-bit `r3`, not in `r3:r4`.

## Caller parameter home area

The variadic kernel `DbgPrint` entry at `0x800af5e8` stores the incoming variable argument registers before acquiring its frame:

```asm
0x800af5e8  mflr    r12
0x800af5ec  stw     r12, -8(r1)
0x800af5f0  std     r4, 24(r1)
0x800af5f4  std     r5, 32(r1)
0x800af5f8  std     r6, 40(r1)
0x800af5fc  std     r7, 48(r1)
0x800af600  std     r8, 56(r1)
0x800af604  std     r9, 64(r1)
0x800af608  std     r10, 72(r1)
0x800af60c  stwu    r1, -624(r1)
```

The offsets establish 8-byte argument slots for `r3` through `r10` at caller-SP offsets 16 through 79. A compatible caller must reserve a 16-byte linkage area followed by a 64-byte parameter home area. The ninth 8-byte integer argument begins at caller-SP offset 80.

## Variadic stack and `va_list` behavior

Self-authored caller and callee probes were compiled with Xbox 360 `cl.exe`
16.00.11886.00. The caller probe used only an external function declaration.
The callee probe used the standard C `stdarg` interface but no Xbox API
declarations or libraries. The generated objects were inspected with
`dumpbin /disasm`; no header implementation was used as evidence.

The relevant caller probe was:

```c
extern int sink(int tag, ...);

__declspec(noinline) int call_i32_overflow(void) {
    return sink(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}

__declspec(noinline) int call_mixed_overflow(void *pointer, __int64 value) {
    return sink(0, 1, 2, 3, 4, 5, 6, pointer, value, 7);
}
```

The relevant callee probes were:

```c
#include <stdarg.h>

__declspec(noinline) __int64 take_i64_then_i32(int tag, ...) {
    va_list args;
    __int64 wide;
    int narrow;
    va_start(args, tag);
    wide = va_arg(args, __int64);
    narrow = va_arg(args, int);
    va_end(args);
    return wide + narrow;
}

__declspec(noinline) int take_pointer(int tag, ...) {
    va_list args;
    void *pointer;
    va_start(args, tag);
    pointer = va_arg(args, void *);
    va_end(args);
    return pointer != 0;
}

__declspec(noinline) int take_first_overflow(
    int a, int b, int c, int d, int e, int f, int g, int h, ...) {
    va_list args;
    int value;
    va_start(args, h);
    value = va_arg(args, int);
    va_end(args);
    return value;
}
```

For `sink(0, 1, 2, ..., 10)`, the fixed argument and first seven variable
arguments occupy `r3` through `r10`. MSVC places the remaining `int` arguments
in distinct 8-byte slots and right-justifies each 32-bit value:

```asm
stw     r8,  84(r1)
stw     r9,  92(r1)
stw     r11, 100(r1)
```

A mixed probe places a stack-passed `__int64` at the start of its slot and the
following `int` in the low word of the next slot:

```asm
std     r4, 80(r1)
stw     r11, 92(r1)
```

A separate header-free probe called the same variadic declaration once with an
`int` and once with a `double`. Neither call writes CR bit 6. The integer call
is emitted as:

```asm
li      r4, 1
li      r3, 0
b       sink
```

The floating call copies the `double` into both `f1` and `r4` and then branches
without a `crxor` or `creqv`. The Xbox ABI therefore does not use the PPC32
SVR4 CR1 marker for variadic calls.

The compiler listing reports each local `va_list` object as 4 bytes. `va_start`
initializes it to the address immediately following the final fixed argument's
8-byte slot. Each supported `va_arg` operation aligns that cursor upward to 8
bytes, consumes one 8-byte slot, and then loads according to type:

- `int` and pointers: 4-byte load at slot offset 4.
- `__int64`: 8-byte load at slot offset 0.

For one fixed `int` in the `r3` slot, MSVC initializes the cursor to caller-SP
offset 24. For eight fixed integers, it initializes the cursor to offset 80;
the first `va_arg(int)` loads at offset 84. Before using the cursor, variadic
callees save each unused incoming argument register at full width into its
caller-provided home slot:

```asm
std     r4, 24(r1)
std     r5, 32(r1)
...
std     r10, 72(r1)
```

This independently matches the genuine `DbgPrint` prologue and establishes a
single contiguous sequence of 8-byte slots spanning register-home and overflow
arguments.

## Floating-point arguments and returns

Header-free fixed and variadic caller probes established the floating-point
register and stack rules. Representative declarations and calls were:

```c
extern double mixed_sink(
    int a, float b, int c, double d, int e, float f, int g, double h, int i);
extern int variadic_sink(int tag, ...);

__declspec(noinline) double call_mixed(
    float b, double d, float f, double h) {
    return mixed_sink(1, b, 2, d, 3, f, 4, h, 5);
}

__declspec(noinline) int call_variadic_fp(void) {
    return variadic_sink(
        10, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
}
```

Fixed `float` and `double` arguments use `f1` through `f13`. Every source
argument still consumes one positional 8-byte argument slot. A fixed
floating-point argument shadows the corresponding GPR position without
copying its value there. In the mixed probe, integer arguments use `r3`, `r5`,
`r7`, and `r9`; the final integer is right-justified in the next stack slot:

```asm
li      r3, 1
li      r5, 2
li      r7, 3
li      r9, 4
stw     r11, 84(r1)
```

After `f1` through `f13` are exhausted, fixed `double` arguments are stored at
the start of their 8-byte positional slots. Fixed `float` arguments are stored
in the low word. Probes with fifteen floating arguments produced:

```asm
stfd    fr13, 128(r1)
stfd    fr0,  136(r1)
```

and:

```asm
stfs    fr13, 132(r1)
stfs    fr0,  140(r1)
```

Variadic `double` arguments are duplicated into both locations while an FPR is
available. The first seven values after an integer tag use `f1` through `f7`
and `r4` through `r10`. The eighth and ninth values use `f8` and `f9` and are
also stored at caller-SP offsets 80 and 88. The compiler does not set CR bit 6
for this floating variadic call. Its integer-only variadic call clears the bit.

The callee-side `va_arg(args, double)` probe uses the same 4-byte cursor as the
integer probes, aligns it to 8 bytes, and loads the value from the start of the
slot with `lfd`. Fixed and variadic scalar floating-point results use `f1`.

## Aggregate arguments and returns

Self-authored aggregate probes covered sizes 1, 4, 5, 7, 8, 12, 16, 24, and
68 bytes. Representative definitions were:

```c
struct S5 { unsigned char b[5]; };
struct S12 { int a, b, c; };
struct S68 { int v[17]; };

extern int sink_s5(int tag, struct S5 value, int end);
extern int sink_s12(int tag, struct S12 value, int end);
extern int sink_s68(int tag, struct S68 value, int end);
```

Aggregates are divided into 8-byte chunks:

- Sizes 1 through 4 use one 32-bit GPR value.
- Sizes 5 through 8 use one full-width GPR.
- Larger values use consecutive full-width GPRs.
- A final partial chunk is left-justified in its 64-bit GPR or stack slot.

For example, an `S12` following an integer tag uses `X4` for its first eight
bytes and the high word of `X5` for its final four bytes. The following integer
uses `r6`.

The 68-byte probe crosses the register/stack boundary. After the tag in `r3`,
the first 56 aggregate bytes use `X4` through `X10`. The remaining full chunk,
four-byte residue, and following integer are placed at:

```asm
std     r10, 88(r1)
stw     r9,  96(r1)
stw     r8, 108(r1)
```

This establishes 16-byte alignment for the memory image of aggregates larger
than 8 bytes while retaining consecutive GPR assignment. The callee
reconstructs the same 68-byte object by storing `X4` through `X10` at offsets
32 through 80 and reading the incoming stack portion at offsets 88 and 96.

Aggregate returns use three classes:

- Sizes 1 through 4 return in `r3`.
- Sizes 5 through 8 return in `X3`.
- Larger aggregates use a hidden result pointer in `r3`; explicit arguments
  begin in `r4`.

LLVM frontends must express these source-level classes using scalar coercions,
`byval`, and `sret`: 5-8 byte returns are coerced to `i64`, larger returns use
`sret`, and fixed aggregate arguments use `byval`. Raw LLVM IR aggregate
returns do not retain enough source ABI classification for the backend and are
diagnosed.

## Stack and preserved GPR behavior

Kernel functions use a 32-bit stack pointer update and 16-byte frame sizes while saving nonvolatile GPR contents at full width. For example, `HalRegisterXamPowerDownCallback` at `0x8008af58` begins with:

```asm
0x8008af58  mflr    r12
0x8008af5c  stw     r12, -8(r1)
0x8008af60  std     r31, -16(r1)
0x8008af64  stwu    r1, -96(r1)
```

`KeSetTimerEx` uses a save helper beginning at `0x8014d060`:

```asm
0x8014d060  std     r26, -56(r1)
```

The helper continues through the higher nonvolatile GPRs before `KeSetTimerEx` executes `stwu r1, -144(r1)`. This is an ILP32 frame model with 64-bit GPR preservation, not an LP64 frame model.

The kernel image also contains a `.pdata` section at `0x8006be00..0x80072db0`. LLVM support intended to emit directly linkable Xbox PE objects will need compatible unwind metadata and constrained prologues. ELF-only experimentation does not satisfy that requirement.

## Hypervisor calls

The kernel contains a contiguous set of hypervisor syscall stubs. Each stub loads a service number into `r0`, executes `sc`, and returns without adapting the normal argument registers. For example, service `0x40` at `0x80148538` is:

```asm
0x80148538  li      r0, 0x40
0x8014853c  sc
0x80148540  blr
```

`XeKeysExecute` preserves six ordinary arguments in `r3` through `r8`, validates them, restores them to the same registers, and branches to this stub:

```asm
0x8014a024  mr      r31, r4
0x8014a028  mr      r30, r5
0x8014a02c  mr      r29, r6
0x8014a030  mr      r28, r7
0x8014a034  mr      r27, r8
...
0x8014a05c  mr      r8, r27
0x8014a060  mr      r7, r28
0x8014a064  mr      r6, r29
0x8014a068  mr      r5, r30
0x8014a06c  mr      r4, r31
0x8014a070  bl      0x80148538
```

The hypervisor boundary is therefore a small target-specific convention layered on the normal register ABI: service selector in `r0`, arguments in the ordinary GPR argument registers, result in `r3`. It should be represented by dedicated stubs or intrinsics. It should not change the C calling convention globally.

## Physical hardware smoke test

The `examples/chello` title was compiled with the patched Clang using
`--target=powerpc64-unknown-xbox360`, linked as ELF32, converted to XEX, and
launched on a physical Xbox 360 development kit through XBDM. The deployed XEX
had SHA-256
`80c2fee16d45b00976717c003838f06315848bdd60dee5a956e21e596289f879`.

The title repeatedly calls the kernel `DbgPrint` thunk and increments a
big-endian heartbeat word at the ELF symbol address `0x82030000`. Three
one-second-spaced reads through XBDM returned:

```text
43 48 00 36
43 48 00 45
43 48 00 54
```

This proves that the XEX loader accepted the image and that generated code
executed its 32-bit stack operations, 64-bit nonvolatile GPR saves, direct
kernel import thunk, global data access, loop, and call/return sequence on the
Xenon processor.

The first hardware build also exposed a PPC64-ABI assumption in
`PPCLinuxAsmPrinter::emitFunctionBodyEnd`: checking the PPC64 architecture
caused LLVM to append the 12-byte PPC64 ELF traceback-table prefix to every
Xbox function. That expanded the fixed-size `DbgPrint` import record from 16 to
28 bytes, which `xex-link` correctly rejected. Restricting the table to
`is64BitELFABI()` keeps Xbox ELF32 thunks at 16 bytes while preserving the
28-byte function body for ELF64 LV2.

The Rust `examples/stdhello` title was also compiled with the patched Rust
toolchain and LLVM backend, converted to XEX, and launched on the same physical
console. Its big-endian `STAGE` word at `0x82040000` reached:

```text
00 00 00 55
```

That terminal value is written only after the title allocates and iterates a
`Vec`, performs formatted output, creates, writes, reopens, and reads a file,
then starts and joins four threads sharing an `Arc<Mutex<u32>>`.

This run exposed two additional mixed-ABI defects. An indirect Rust trait call
used `bctrl`, but frame lowering only checked the ABI-selected link-register
view and omitted the LR save. Xbox ILP32 code can define either `LR` or `LR8`,
so `MustSaveLR` now checks both. The filesystem call then exposed an import
adapter that still read its ninth argument from PPC32 SysV `sp+8`. The Xbox
caller had placed it in the low word of the 8-byte home slot at `sp+0x50`.
Loading that incoming slot before constructing the import frame made the same
title reach `0x55` on hardware.

## LLVM consequences

The current experimental patch implements the evidence-backed subset:

- The `powerpc64-unknown-xbox360` triple selects the PPC64 ISA with an ILP32
  data layout and 32-bit address register classes.
- A legal `i64` argument consumes one of `X3` through `X10`.
- A scalar `i64` result uses `X3`.
- Calls reserve a 16-byte linkage area and eight 8-byte argument home slots.
- Pointer objects and ordinary pointer spills are 4 bytes.
- Fixed `f32` and `f64` arguments use `F1` through `F13` while consuming
  positional GPR or stack slots.
- Variadic `f64` arguments are duplicated into FPR and integer locations.
- Variadic calls do not use the PPC32 SVR4 CR1 marker.
- Fixed `byval` aggregates are packed into 8-byte GPR and stack chunks.
- The standard nonvolatile GPR set is saved and restored at full width,
  including when a register holds a pointer.
- The save area below the incoming stack pointer is available before a frame is
  claimed. CR2 uses the fixed `-4` slot; full-width GPR saves use their
  negative save-area offsets.
- The data layout, pointers, and emitted ELF objects remain 32-bit and
  big-endian.

LLVM's existing PS3 support supplies reusable PPC64 ILP32 target and data-layout
plumbing. The Xbox target deliberately does not inherit the PS3 ELFv1 function
descriptors or TOC calling convention; the binaries above use direct code
pointers and direct branch targets.

Only the C type model and pointer data layout are shared PPC64 ILP32 behavior.
The Xbox ELF32 object format, direct function pointers, 32-bit stack pointer,
linkage area, parameter home area, and calling convention are independent ABI
choices. Checks for those properties must remain Xbox-specific; applying them
to the ILP32 LV2 target would incorrectly replace its ELFv1, descriptor, TOC,
and stack conventions with the Xbox ABI.

The patch implements the observed overflow layout for scalar and aggregate
arguments. Scalar 32-bit values are right-justified in 8-byte slots, scalar
64-bit values start at the slot address, and aggregate residue is
left-justified. Aggregates larger than 8 bytes use a 16-byte-aligned memory
image.

Variadic definitions use a 4-byte cursor, save unused argument registers into
the caller-provided home area, and support `va_arg` for `i32`, pointers, `i64`,
and `f64`. The Clang and Rust classifiers pass fixed aggregates with `byval`,
coerce returns of at most 4 bytes to `i32` and returns of 5–8 bytes to `i64`,
and use `sret` for larger returns. Independently, Rust maps any PowerPC64
target with 32-bit pointers to the 32-bit PowerPC object architecture. For Xbox,
metadata wrapper objects match LLVM's ELF32 output.

LLVM also accepts fixed `f16` and `i128` arguments needed by Rust's
`compiler_builtins`. An `f16` value uses its integer bit pattern in one GPR
slot, and an `i128` value uses two consecutive 64-bit GPR slots. These are
target extensions, not MSVC compatibility claims: the observed Microsoft C
surface does not expose either source type.

The following items remain separate work and must not be claimed as implemented:

- Exact MSVC helper-thunk prologue selection and `.pdata` emission.
- Big-endian PowerPC COFF/PE object emission and relocations.
- VMX128 register and instruction support.
- A checked model for vectors, integers wider than 128 bits, variadic
  aggregates, and direct `f32` variadic IR.
- Dedicated hypervisor-call intrinsics or assembly stubs.

The target remains optional and experimental until those gaps are closed.
