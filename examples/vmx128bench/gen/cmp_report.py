#!/usr/bin/env python3
"""Tables of the compiler comparison (`ext` section) from one or more
results.csv files and the static statistics of gen/insn_stats.py.

    python3 gen/cmp_report.py STATS.csv RUN1/results.csv [RUN2/results.csv ...]

Values are core cycles per element (64 cycles per timebase tick), the
median over runs of the per-run medians. Checksums: every LLVM/Rust
variant of a kernels.c kernel is compared with msvc_, every form and
variant of a speed kernel with cs_sp_<op>_s (scalar source, scalar build).
"""

import collections
import csv
import statistics
import builtins
import sys


def print(*a, **k):
    if a and isinstance(a[0], str) and a[0].startswith("| kernel"):
        _last_header[0] = a[0].count("|") - 1
    builtins.print(*a, **k)

LLVM_VARIANTS = ["l3s", "l3a", "l3v", "lfa", "lfv", "l2v"]
AUTOVEC_VARIANTS = ["msvc", *LLVM_VARIANTS, "rs", "ra", "rv"]
SDK_RUNTIME_KERNELS = {"k_copy_u32_r", "k_set_f32"}
SPEED_VARIANTS = ["cs", "ca", "cv", "rs", "ra", "rv"]
# bytes moved per element, for bandwidth
SPEED_BYTES = {"saxpy": 12, "dot": 8, "mat4": 8, "copy": 8, "bright": 2, "u8f": 5, "f2u8": 5}
GHZ = 3.2


def load(paths):
    t = collections.defaultdict(list)  # (sym, n, off) -> [median]
    ck = {}
    for p in paths:
        for line in open(p):
            r = line.rstrip("\n").split(",")
            if r[0] != "ext" or len(r) < 10:
                continue
            key = (r[1], int(r[2]), int(r[3]))
            t[key].append(float(r[5]))
            ck.setdefault(key, set()).add(r[9])
    return {k: statistics.median(v) for k, v in t.items()}, {k: v for k, v in ck.items()}, \
        {k: (min(v), max(v)) for k, v in t.items()}


_last_header = [0]


def sep():
    print("|" + " --- |" * _last_header[0])


def f(x):
    return "" if x is None else ("%.2f" % x)


def main():
    stats = {r["name"]: r for r in csv.DictReader(open(sys.argv[1]))}
    t, ck, rng = load(sys.argv[2:])
    names = sorted({k[0].split("_", 1)[1] for k in t if k[0].split("_", 1)[1].startswith("k_")},
                   key=lambda n: [k[0] for k in t].index(next(k[0] for k in t if k[0].endswith("_" + n))))

    def same(sym, ref, n, off):
        a, b = ck.get((sym, n, off)), ck.get((ref, n, off))
        if a is None or b is None:
            return None
        return a == b and len(a) == 1

    bad = []
    for (sym, n, off), c in ck.items():
        base = sym.split("_", 1)[1]
        if base.startswith("k_"):
            ref = "msvc_" + base
        else:
            op = base.split("_")[1]
            ref = "cs_sp_%s_s" % op
        if sym != ref and same(sym, ref, n, off) is not True:
            bad.append((sym, n, off))
    print("Runs: %d. Checksum mismatches against the reference: %d%s" % (
        len(sys.argv) - 2, len(bad), "" if not bad else " (" + ", ".join("%s n=%d off=%d" % b for b in sorted(bad)) + ")"))
    print()

    def st(sym):
        return stats.get(sym)

    print("### Auto-vectorization: kernels.c, n = 1024, 16-byte aligned (cycles per element)")
    print()
    print("SDK = cl.exe /O2 (msvc_). LLVM C: l3s = -O3 -mno-altivec (scalar), l3a = -O3 -mno-vmx128 "
          "(classic AltiVec), l3v = -O3 (VMX128), lfv = -O3 -ffast-math (VMX128). Rust: rs scalar, "
          "ra classic AltiVec, rv VMX128. Speedup = SDK / l3v (above 1: LLVM faster). vec = l3v has "
          "vector instructions in its loop. l3v insns = classic/128-form vector instructions, "
          "highest vector register, stack loads/stores in loops. k_g_* work on 1024 fixed elements; "
          "k_add16_f32_r on 16 (its row is the n = 16 run).")
    print("SDK rows marked project runtime include the project _blkmov substitute, not the SDK "
          "library routine. Their SDK speedup ratios are omitted. Best LLVM includes C variants only.")
    print()
    print("| kernel | SDK | l3s | l3a | l3v | lfv | rs | ra | rv | speedup SDK/l3v | SDK/best LLVM | vec | l3v insns cls/128, vmax, stack | checks |")
    sep()
    for b in names:
        # k_add16_f32_r always does 16 elements: use its n = 16 entry
        nn = 16 if b == "k_add16_f32_r" else 1024
        v = {p: t.get(("%s_%s" % (p, b), nn, 0)) for p in AUTOVEC_VARIANTS}
        if v["msvc"] is None:
            continue
        llvm = [v[p] for p in LLVM_VARIANTS if v[p] is not None]
        s = st("l3v_" + b)
        insns = "%s/%s, v%s, %s" % (s["vec_classic"], s["vec_128"], s["max_vreg"], s["loop_stack_mem"]) if s else ""
        vec = "yes" if s and int(s["loop_vec"]) > 0 else "no"
        oks = [same("%s_%s" % (p, b), "msvc_" + b, nn, 0)
               for p in AUTOVEC_VARIANTS[1:] if v[p] is not None]
        chk = "equal" if oks and all(o is True for o in oks) else "MISSING" if None in oks else "DIFFER"
        runtime = b in SDK_RUNTIME_KERNELS
        cells = [f(v[p]) for p in ["msvc", "l3s", "l3a", "l3v", "lfv", "rs", "ra", "rv"]]
        if runtime:
            cells[0] += " (project runtime)"
        ratios = [f(v["msvc"] / v["l3v"]) if not runtime and v["l3v"] else "",
                  f(v["msvc"] / min(llvm)) if not runtime and llvm else ""]
        print("| %s | %s | %s |" % (b, " | ".join(cells), " | ".join([*ratios, vec, insns, chk])))
    print()

    print("### Auto-vectorization: alignment and size (cycles per element)")
    print()
    print("| kernel | SDK 1024 +4 | l3v 1024 +4 | lfv 1024 +4 | rv 1024 +4 | SDK 65536 | l3s 65536 | l3v 65536 | lfv 65536 | rv 65536 | SDK 16 | l3v 16 |")
    sep()
    for b in names:
        if b.startswith("k_g_") or b == "k_add16_f32_r" or b in SDK_RUNTIME_KERNELS:
            continue
        g = lambda p, n, o: t.get(("%s_%s" % (p, b), n, o))
        if g("msvc", 1024, 0) is None:
            continue
        cells = [g("msvc", 1024, 4), g("l3v", 1024, 4), g("lfv", 1024, 4), g("rv", 1024, 4),
                 g("msvc", 65536, 0), g("l3s", 65536, 0), g("l3v", 65536, 0), g("lfv", 65536, 0), g("rv", 65536, 0),
                 g("msvc", 16, 0), g("l3v", 16, 0)]
        print("| %s | %s |" % (b, " | ".join(f(c) for c in cells)))
    print()

    diffs = []
    for b in names:
        a, o = t.get(("l2v_" + b, 1024, 0)), t.get(("l3v_" + b, 1024, 0))
        if a and o and abs(a - o) / o > 0.03:
            diffs.append((b, a, o))
    print("O2 against O3 (VMX128, n = 1024 aligned): %s." % (
        "no kernel differs by more than 3%" if not diffs else
        "; ".join("%s %.2f vs %.2f" % d for d in diffs)))
    print()

    print("### Compiler-generated vector code: VMX128 vs classic AltiVec vs scalar (speed.c, kernels.rs)")
    print()
    print("`_s` = scalar source (auto-vectorized), `_v` = vector source (clang vector extensions / Rust "
          "portable SIMD), `_v8`/`_v16` = 8/16 vectors per iteration with all splats first, `_x` = "
          "VMX128 builtins (vmsum4fp128 + vrlimi128). C: -O3 -ffast-math; cs/ca/cv = scalar/classic/VMX128 "
          "build; Rust rs/ra/rv likewise (-O3, no fast-math). Cycles per element (one f32, u32 or byte "
          "channel); 16-byte aligned. GB/s uses the bytes each element moves (in + out) at 3.2 GHz.")
    print()
    for n in (1024, 65536):
        print("n = %d:" % n)
        print()
        print("| kernel | bytes/elem | cs | ca | cv | rs | ra | rv | cv GB/s | cv vs ca | cv: cls/128, vmax, stack | ca: vmax, stack | checks |")
        sep()
        seen = []
        for k in t:
            base = k[0].split("_", 1)[1]
            if base.startswith("sp_") and base not in seen:
                seen.append(base)
        for b in seen:
            op = b.split("_")[1]
            v = {p: t.get(("%s_%s" % (p, b), n, 0)) for p in SPEED_VARIANTS}
            if all(x is None for x in v.values()):
                continue
            sv, sa = st("cv_" + b), st("ca_" + b)
            oks = [same("%s_%s" % (p, b), "cs_sp_%s_s" % op, n, 0)
                   for p in SPEED_VARIANTS if v[p] is not None]
            chk = "equal" if oks and all(o is True for o in oks) else "MISSING" if None in oks else "DIFFER"
            gb = SPEED_BYTES[op] / v["cv"] * GHZ if v["cv"] else None
            print("| %s | %d | %s | %s | %s | %s | %s | %s |" % (
                b, SPEED_BYTES[op], " | ".join(f(v[p]) for p in SPEED_VARIANTS), f(gb),
                f(v["ca"] / v["cv"]) if v["ca"] and v["cv"] else "",
                "%s/%s, v%s, %s" % (sv["vec_classic"], sv["vec_128"], sv["max_vreg"], sv["loop_stack_mem"]) if sv else "",
                "v%s, %s" % (sa["max_vreg"], sa["loop_stack_mem"]) if sa else "", chk))
        print()
    if bad:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
