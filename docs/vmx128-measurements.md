# Console VMX128 measurements

Measured on the development kit, kernel 2.0.21256, hardware thread 1.
The timebase is 50 MHz; cycle counts use 64 core cycles per tick (3.2 GHz).
Tables contain medians of seven timed repetitions from one complete run.
They are not measurements of several consoles or repeated title launches.

## Toolchain and validation

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

## Compiler auto-vectorization

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
