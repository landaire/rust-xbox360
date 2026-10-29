# Console VMX128 measurements

Measured on the development kit, kernel 2.0.21256, hardware thread 1.
The timebase is 50 MHz; cycle counts use 64 core cycles per tick (3.2 GHz).
Each timing column contains medians of seven timed repetitions from one
complete run. Before and after columns use separate title launches on the
same console, not measurements across several consoles.

## Baseline toolchain and validation

- LLVM 22.1.8, source `137023df40d95a06cd4393a7e4b26aae5b8dd7ee`.
- Rust 1.98, source `a199a11abab9720154e5e2eff1887960432e8c95`.
- Bootstrap `4c188e38c7d8fcc44fdc9200343bb4b7b2caaed9`.
- SDK C compiler 16.00.11886.00, `/O2`, compiling project-owned `cmp/kernels.c`.
- Target: big-endian PPC64 ISA, ILP32 console ABI, AltiVec and VMX128, no VSX.

The console run passed 451 instruction-model kernels, 33 memory kernels,
all transform cases, and 2808 external compiler-output cases. Each external
case checks buffers and returns against an independent scalar implementation.
Non-reference variants also check against their designated compiler reference.
The six fixed-global arrays are initialized, hashed and independently checked.
Clamp inputs exercise values below, at, inside and above both limits.
Initialized prefix guards are checked, including the lower boundary at +4.

Temporary assembly fault injections verified the checker itself:

| Injected fault | Cases | Rejected cases |
| --- | --- | --- |
| Integer add replaced by an immediate return | 6 | 6 |
| Float clamp replaced by a copy | 6 | 6 |
| One-word prefix overwrite at +4 | 3 | 3 |

The aligned controls passed during the prefix-overwrite test. All injected
instructions were removed before the final measurements.

The separate SIMD/ABI title passed 71 checks. The SF probe passed both its
normal SF=0 run and the optional SF=1 comparison run, enforcing each probe's
established mode-dependent or full-width behavior.

Raw results:

- [Instruction, memory and compiler timings](measurements/vmx128-console.csv)
- [Linked instruction statistics](measurements/vmx128-static.csv)
- [SIMD and vector ABI checks](measurements/vmx128-semantics.txt)
- [SF=0 probes](measurements/sfmode-sf0.csv)
- [SF=0/SF=1 probes](measurements/sfmode-both.csv)
- [Compiled SF-mode regression, before and after](measurements/sfmode-codegen.csv)

## Instruction costs

Classic AltiVec and corresponding VMX128 forms have the same measured cost.
High vector registers introduce no measurable instruction penalty here.
Reciprocal throughput uses independent chains; latency uses dependent chains.

| Operation | Classic latency, cycles | VMX128 v64+ latency, cycles | Classic reciprocal throughput, cycles | VMX128 v64+ reciprocal throughput, cycles |
| --- | --- | --- | --- | --- |
| Float add | 12.02 | 12.02 | 1.03 | 1.03 |
| Float multiply-add | 12.02 | 12.02 | 1.03 | 1.03 |
| Float maximum | 4.01 | 4.01 | 1.03 | 1.03 |
| Byte permute | 4.01 | 4.00 | 1.03 | 1.03 |
| Reciprocal estimate | 14.02 | 14.02 | 2.03 | 2.03 |
| Four-component dot product | 44.06, five-instruction sequence | 14.02, one instruction | 5.71 per dot | 1.40 per dot |

Dot products are not strict replacements for scalar reductions. Mixed-sign
products can leave a small residual after exact cancellation. Their timing
and matrix comparisons use positive dyadic inputs with exactly representable
products and sums. Mixed-sign accuracy is tested separately by vmx128test.

## Memory bandwidth

Copy bandwidth counts source reads plus destination writes, not only payload.
Thus a 16 KiB copy transfers 32 KiB for these tables. Payload throughput is
half the reported copy traffic bandwidth. Working-set size names one buffer.
Buffers are cached and use 64 KiB physical-page alignment.

| Copy instructions | 16 KiB traffic bytes/cycle | 16 KiB traffic GB/s | 16 MiB traffic bytes/cycle |
| --- | --- | --- | --- |
| `lwz` / `stw` | 3.602 | 11.528 | 0.320 |
| `ld` / `std` | 6.600 | 21.119 | 0.345 |
| `lfd` / `stfd` | 3.972 | 12.711 | 0.343 |
| Classic `lvx` / `stvx` | 8.191 | 26.210 | 0.341 |
| High-register `lvx128` / `stvx128` | 8.195 | 26.224 | 0.340 |

These are loop throughput measurements, not peak bus specifications. The
large-buffer result depends strongly on prefetch and store policy: the
VMX128 copy with `dcbt` 1024 bytes ahead plus `dcbz128` measured 1.56 traffic
bytes/cycle at 16 MiB. Merely changing to vector instructions did not produce
that improvement.

## Register pressure and matrix transforms

Handwritten 4x4 transforms process 480 vectors held in L1. Each row is the
best measured unroll for that sequence and register limit, not a claim that
four times as many registers provide four times the speed.

| Sequence | Vector registers | Vectors per iteration | Cycles per four-component vector |
| --- | --- | --- | --- |
| Six-instruction splat form | 32 | 4 | 26.163 |
| Six-instruction splat form | 128 | 12 | 20.607 |
| Three-instruction splat form | 32 | 8 | 20.729 |
| Three-instruction splat form | 128 | 32 | 20.509 |
| Dot-product form | 128 | 4 | 19.481 |

The larger register file permits more independent work in the first sequence.
A different classic sequence already approaches the larger-register result.

## Baseline compiler auto-vectorization

The 44 C kernels contain scalar loops, including restricted pointers,
reductions, conversions, fixed global arrays, and fixed-trip loops. This is
not a test of manually written SDK intrinsics. SIMD generated in these
kernels includes compiler loop/SLP transformations; it does not describe
instructions that an SDK library might execute.

| Compiler configuration | Kernels with generated SIMD, out of 44 |
| --- | --- |
| SDK C compiler `/O2` | 0 |
| Clang `-O3 -mno-altivec` | 0 |
| Clang `-O3 -mno-vmx128` | 25 |
| Clang `-O3`, VMX128 | 25 |
| Clang `-O3 -ffast-math`, VMX128 | 26 |
| Rust `-O3`, VMX128 | 24 |

More generated SIMD does not establish a performance advantage. Timings below
use 1024 elements, aligned pointers, and ordinary VMX128 Clang `-O3` without
fast math. The fixed 16-element kernel uses its actual count. Fixed-global
kernels run only at 1024, without duplicate alignment/size rows.

| Kernel | SDK cycles/element | Clang VMX128 cycles/element |
| --- | --- | --- |
| Restricted float add | 6.112 | 24.125 |
| Unsigned-byte average | 20.138 | 6.182 |
| Float clamp | 84.612 | 22.809 |
| Float-to-int conversion | 5.317 | 73.457 |

The SDK copy-u32 and set-f32 kernels call the project `_blkmov` substitute.
Their timings are marked as project-runtime measurements and excluded from
SDK speedup comparisons. No SDK library implementation is included.

| Comparison over the other 42 kernels | Kernels faster than SDK | Geometric mean SDK/LLVM time |
| --- | --- | --- |
| Ordinary Clang `-O3`, VMX128 | 18 | 0.762 |
| Per-kernel minimum across six Clang profiles | 27 | 1.066 |

A ratio above one favors LLVM. The second row selects separately for each
kernel from scalar, classic AltiVec and VMX128 builds, including fast-math
profiles; it is not the performance of one compiler configuration. Rust
results are not included in that minimum. Many ordinary LLVM loops remain
slower than SDK output. The instruction statistics record stack-memory
operations inside several such loops, including three in the float-add loop
and six in the float-to-int loop.

## Xbox-specific lowering

The updated compiler uses LLVM source
`2f714b11080e5a449de3969e3c4477cf5cbe92ab` and bootstrap
`f75e1005adc480d93d59f1ef7ad49554f12f6020`. The Rust source is unchanged.
The corresponding complete upstream LLVM branch is
`4ab22204eb45c45a92fc631f45c0270ea4697218`.

Changes specific to the Xbox target:

- Unaligned vector stores use rotated byte, halfword or word element stores
  instead of a vector stack spill followed by scalar reloads. Partial and
  volatile accesses retain exact memory widths.
- Non-strict scalar `f32`/`f64` to signed `i32` stores at alignment 4 or greater
  use `fctiwz` and `stfiwx` without an intermediate stack transfer.
- Signed halfword loads use `lhz` and `extsh` rather than the slower `lha`.
- Size optimization uses code-size vectorization costs, including alignment
  permutations and scalar memory transfers. Vectorizers remain enabled.
- Ordinary register-only C sibling calls can avoid a frame when the
  PPC64-ILP32 calling convention permits it. Interposable PIC calls retain
  their required call sequence.
- `__builtin_ppc_get_pcr()` permits direct r13-based addressing. Calls can
  change the PCR base; a captured value remains distinct across a call.
  This interface does not provide affinity or asynchronous sampling guarantees.
- Missing classic integer vector operations remain unavailable when VMX128
  registers and encodings are disabled. The classic-only comparison profiles
  do not imply a different processor.

The common vector register pressure model remains conservative at 32
registers because integer and floating-point allocations overlap. Machine
register allocation still has all 128 registers where their instruction
classes permit them.

The AltiVec saturation side-effect and conversion-immediate checks also apply
to non-Xbox targets: those are instruction-semantic corrections, not Xbox
performance preferences.

The permanent SIMD/ABI title passed 81 checks, including generic vector
accesses and scalar conversion stores on physical write-combined memory.
Separate temporary compiled probes checked exact-width partial and wide
vector accesses before their scaffolding was removed.

- [Updated SIMD and ABI checks](measurements/vmx128-lowering-semantics.txt)
- [Partial and wide memory checks](measurements/vmx128-partial-memory.txt)

Standalone kernel reductions compiled with the checksum-pinned Clang and
`-Os -ffreestanding`:

| Reduction | Baseline code bytes | Updated code bytes |
| --- | --- | --- |
| Conditional sibling call | 44 | 16 |
| Short matrix subtraction | 156 | 68 |

The sibling call has no stack frame. The matrix update uses a scalar inner
loop rather than a vector permutation and stack-transfer sequence. Ordinary
scalar dot-product and division expressions retain their original semantics;
the compatibility header uses native target intrinsics for hardware dot
products and reciprocal estimates.

The native compatibility test passed with the pinned compiler in C and C++.
Both kernel object sets, 17559 and 16547, built successfully (950 compile
actions). This is compilation coverage, not a kernel boot test.

### Updated compiler comparison

The complete updated run passed 451 instruction-model kernels, 33 memory
kernels, all transform cases and all 2808 external compiler-output cases.
The independent scalar oracles and compiler-reference checks reported no
mismatch. The run used hardware thread 1, seven repetitions and a 4 ms timing
target, matching the baseline.

| Compiler configuration | Baseline kernels with SIMD, out of 44 | Updated kernels with SIMD, out of 44 |
| --- | --- | --- |
| SDK C compiler `/O2` | 0 | 0 |
| Clang `-O3 -mno-altivec` | 0 | 0 |
| Clang `-O3 -mno-vmx128` | 25 | 35 |
| Clang `-O3`, VMX128 | 25 | 35 |
| Clang `-O3 -ffast-math`, VMX128 | 26 | 36 |
| Rust `-O3`, VMX128 | 24 | 32 |

Aligned 1024-element cases, ordinary Clang `-O3` with VMX128 and no fast math:

| Kernel | Baseline LLVM cycles/element | Updated LLVM cycles/element | Updated SDK cycles/element | LLVM before/after speedup |
| --- | --- | --- | --- | --- |
| Restricted float add | 24.125 | 8.370 | 6.113 | 2.88 |
| Unsigned-byte average | 6.182 | 3.654 | 20.139 | 1.69 |
| Float clamp | 22.809 | 7.510 | 85.312 | 3.04 |
| Float-to-int conversion | 73.457 | 4.266 | 5.309 | 17.22 |
| Signed-halfword dot product | 57.327 | 18.252 | 18.769 | 3.14 |
| Unsigned-byte-to-float conversion | 90.361 | 98.212 | 52.986 | 0.92 |
| Float dot product | 10.738 | 11.312 | 6.281 | 0.95 |

| Comparison over the 42 non-runtime kernels | Baseline wins against SDK | Updated wins against SDK | Baseline geometric mean SDK/LLVM | Updated geometric mean SDK/LLVM |
| --- | --- | --- | --- | --- |
| Ordinary Clang `-O3`, VMX128 | 18 | 28 | 0.762 | 1.669 |
| Per-kernel minimum across six Clang profiles | 27 | 31 | 1.066 | 2.097 |

Ordinary LLVM improved on 38 of the 42 kernels, with a geometric mean
before/after speedup of 2.190. This is not a uniform improvement: the two
conversion/reduction regressions above remain, and two other differences
are below 0.2 percent. One launch per compiler revision does not establish
statistical significance for small changes.

Throughput improvements also consume more code at `-O3`:

| Kernel | Baseline instructions | Updated instructions | Baseline stack-memory instructions in loops | Updated stack-memory instructions in loops |
| --- | --- | --- | --- | --- |
| Restricted float add | 51 | 230 | 3 | 0 |
| Float-to-int conversion | 42 | 207 | 6 | 0 |

The size-optimized kernel reductions above use `-Os`, not these throughput
settings. The memory measurements retain the earlier conclusion: changing
to vector instructions alone does not improve large-buffer bandwidth.

- [Updated instruction, memory and compiler timings](measurements/vmx128-lowering-console.csv)
- [Updated linked instruction statistics](measurements/vmx128-lowering-static.csv)

## Reproduction

From the repository root, set `XBOX` and `XEEDEE` if the default development
kit or client path is unsuitable. The runners hold the shared console lock,
remove stale result files, save debug output and return failure on test
failures, exceptions or timeout.

```sh
examples/vmx128test/run.sh /tmp/vmx128test
examples/sfmode/run.sh /tmp/sfmode
examples/sfmode/run.sh /tmp/sfmode-both sf1=1
TIMEOUT=1800 examples/vmx128bench/run.sh /tmp/vmx128bench
```

Compiler-comparison functions are generated from project C/Rust sources. Regeneration
requires the selected LLVM compiler, the pinned Rust toolchain, and an
installed SDK compiler. On Linux, `make_sdk.sh` runs that compiler under Wine.
It converts our COFF object and generates project-written ABI/runtime helpers;
it does not copy an SDK library.

```sh
SDK=/path/to/sdk WINE=/path/to/wine WINEPREFIX=/path/to/prefix \
  CLANG=/path/to/clang examples/vmx128bench/cmp/make.sh
```

`CLEXE` may select another installed compiler executable. Compiler flags and
versions are recorded in the generated assembly headers. The COFF converter
refuses unverified relocation addends rather than inferring their meaning.

Generate full tables from the recorded data:

```sh
python3 examples/vmx128bench/gen/report.py docs/measurements/vmx128-console.csv
python3 examples/vmx128bench/gen/cmp_report.py \
  docs/measurements/vmx128-static.csv docs/measurements/vmx128-console.csv
```

To regenerate instruction statistics, build the benchmark `[elf]` subtarget
and pass that file plus the matching `llvm-objdump` to `gen/insn_stats.py`.
The SDK `/O2` result concerns this compiler and these kernels, not every SDK
version or every possible vectorization option.
