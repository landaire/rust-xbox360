#!/usr/bin/env python3
"""Turns one or more vmx128bench results.csv files into markdown tables.

    python3 examples/vmx128bench/gen/report.py RUN1/results.csv [RUN2/results.csv ...]

Each value is the median over the runs of the per-run median; the spread
column gives the minimum and maximum per-run median (several runs) or the
minimum and maximum single measurement (one run).
"""

import collections
import statistics
import sys

GHZ = 3.2


def load(paths):
    runs = []
    for p in paths:
        rows = [line.rstrip("\n").split(",") for line in open(p) if line.strip()]
        runs.append(rows)
    return runs


def combine(values):
    """values: list of (med, lo, hi) from each run."""
    meds = [v[0] for v in values]
    m = statistics.median(meds)
    if len(values) > 1:
        return m, min(meds), max(meds)
    return values[0]


def fmt(x, nd=2):
    return ("%%.%df" % nd) % x


def spread(lo, hi, nd=2):
    return "%s-%s" % (fmt(lo, nd), fmt(hi, nd))


def section(runs, tag):
    out = collections.defaultdict(list)
    for rows in runs:
        for r in rows:
            if r and r[0] == tag:
                yield r


def main():
    runs = load(sys.argv[1:])
    nruns = len(runs)
    print("Runs combined: %d. Values: median; spread: %s." % (
        nruns, "min-max of per-run medians" if nruns > 1 else "min-max of the timed repetitions"))
    print()

    # info / clock
    print("### Timebase and clock")
    print()
    for r in section(runs[:1], "info"):
        print("- " + " ".join(r[1:]))
    print()
    clocks = collections.OrderedDict()
    for rows in runs:
        for r in rows:
            if r and r[0] == "clock":
                # name, description (may hold commas), iters, ..., m, lo, hi, label, cycles, check
                clocks.setdefault((r[1], " ".join(r[2:-8])), []).append(
                    (float(r[-6]), float(r[-5]), float(r[-4]), r[-1]))
    if clocks:
        print("| kernel | description | ticks/op | spread | cycles/op at 64 cycles/tick | check |")
        print("| --- | --- | --- | --- | --- | --- |")
        for (n, d), v in clocks.items():
            m, lo, hi = combine([x[:3] for x in v])
            print("| %s | %s | %s | %s | %s | %s |" % (n, d, fmt(m, 5), spread(lo, hi, 5), fmt(m * 64, 2),
                                                       ",".join(sorted(set(x[3] for x in v)))))
        print()

    # checks and probes
    print("### Checks")
    print()
    for rows in runs:
        for r in rows:
            if r and r[0] == "check" and (r[1] == "reg" and r[2] == "summary" or r[-1] == "FAIL"):
                print("- " + ",".join(r))
            if r and r[0] == "end":
                print("- " + ",".join(r))
    mem_ok = [r for r in section(runs[:1], "check") if r[1] == "mem"]
    if mem_ok:
        print("- mem kernels checked: %d, failed: %d" % (len(mem_ok), sum(1 for r in mem_ok if r[3] != "ok")))
    print()
    probes = list(section(runs[:1], "probe"))
    if probes:
        print("| probe | result vs model | input words | output words | model words | note |")
        print("| --- | --- | --- | --- | --- | --- |")
        for r in probes:
            print("| %s | %s | %s | %s | %s | %s |" % (r[1], r[2], r[3][3:], r[4][4:], r[5][6:], ",".join(r[6:])))
        print()

    # register kernels
    reg = collections.OrderedDict()
    for rows in runs:
        for r in rows:
            if r and r[0] == "reg":
                op, group, variant, kind, chains, seq, per, body = r[1:9]
                key = (op, variant, kind, int(chains))
                reg.setdefault(key, dict(group=group, seq=int(seq), per=per, v=[], ok=set()))
                reg[key]["v"].append((float(r[10]), float(r[11]), float(r[12])))
                reg[key]["ok"].add(r[15])
    if reg:
        print("### Latency and throughput (cycles)")
        print()
        print("Latency: one dependent chain. Throughput: independent chains (count in brackets), "
              "reciprocal throughput. Per instruction unless the unit column says step "
              "(a step is the whole sequence listed in the kernel description).")
        print()
        print("| op | variant | unit | instrs/step | latency | spread | rec. throughput | spread | check |")
        print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
        ops = []
        for (op, variant, kind, k) in reg:
            if (op, variant) not in ops:
                ops.append((op, variant))
        for op, variant in ops:
            lat = reg.get((op, variant, "lat", 1))
            thr = [(k, v) for (o, vv, kind, k), v in reg.items() if o == op and vv == variant and kind == "thr"]
            if not lat or variant == "128all":
                continue
            # the plain throughput kernel: the largest chain count up to 16
            thr = [t for t in thr if 1 < t[0] <= 16]
            if thr:
                kthr, tv = max(thr, key=lambda t: t[0])
                tm, tlo, thi = combine(tv["v"])
                oks = lat["ok"] | tv["ok"]
            else:
                kthr, tm, tlo, thi, oks = 0, float("nan"), float("nan"), float("nan"), lat["ok"]
            scale = lat["seq"] if lat["per"] == "seq" else 1
            lm, llo, lhi = combine(lat["v"])
            print("| %s | %s | %s | %d | %s | %s | %s [%d] | %s | %s |" % (
                op, variant, "step" if scale > 1 else "instr", lat["seq"], fmt(lm * scale),
                spread(llo * scale, lhi * scale), fmt(tm * scale), kthr, spread(tlo * scale, thi * scale),
                ",".join(sorted(oks))))
        print()
        # sweeps
        print("### Chains needed to saturate (cycles per instruction, or per step for dot4 cls)")
        print()
        sweeps = collections.OrderedDict()
        for (op, variant, kind, k), v in reg.items():
            if kind in ("thr", "lat") and variant in ("cls", "128all"):
                sweeps.setdefault((op, variant), {})[k] = v
        ks = sorted({k for d in sweeps.values() for k in d})
        sweeps = {key: d for key, d in sweeps.items() if len(d) > 3}
        print("| op | variant | " + " | ".join("K=%d" % k for k in ks) + " |")
        print("| --- | --- | " + " | ".join("---" for _ in ks) + " |")
        for (op, variant), d in sweeps.items():
            cells = []
            for k in ks:
                if k in d:
                    scale = d[k]["seq"] if d[k]["per"] == "seq" else 1
                    cells.append(fmt(combine(d[k]["v"])[0] * scale))
                else:
                    cells.append("")
            print("| %s | %s | %s |" % (op, variant, " | ".join(cells)))
        print()

    # transform
    xf = collections.OrderedDict()
    for rows in runs:
        for r in rows:
            if r and r[0] == "xform":
                xf.setdefault((r[1], r[2], r[3], r[4]), []).append((float(r[6]), float(r[7]), float(r[8]), r[9]))
    if xf:
        print("### 4x4 transform over 480 vectors in L1 (cycles per vector)")
        print()
        print("| kernel | form | vectors per iteration | registers | cycles/vector | spread | check |")
        print("| --- | --- | --- | --- | --- | --- | --- |")
        for (n, f, u, regs), v in xf.items():
            m, lo, hi = combine([x[:3] for x in v])
            print("| %s | %s | %s | %s | %s | %s | %s |" % (n, f, u, regs, fmt(m), spread(lo, hi),
                                                         ",".join(sorted(set(x[3] for x in v)))))
        print()

    # memory
    mem = collections.OrderedDict()
    sizes = []
    for rows in runs:
        for r in rows:
            if r and r[0] == "mem":
                key = (r[1], r[2], r[3])
                size = int(r[4])
                if size not in sizes:
                    sizes.append(size)
                mem.setdefault(key, {}).setdefault(size, []).append((float(r[6]), float(r[7]), float(r[8])))
    if mem:
        def sz(s):
            return "%dK" % (s >> 10) if s < (1 << 20) else ("%gM" % (s / (1 << 20)))
        for group in ("read", "write", "copy"):
            print("### Bandwidth, %s (bytes per cycle; GB/s = bytes/cycle x %.1f)" % (group, GHZ))
            print()
            print("| kernel | variant | " + " | ".join(sz(s) for s in sizes) + " |")
            print("| --- | --- | " + " | ".join("---" for _ in sizes) + " |")
            for (name, g, variant), d in mem.items():
                if g != group:
                    continue
                cells = [fmt(combine(d[s])[0]) if s in d else "" for s in sizes]
                print("| %s | %s | %s |" % (name, variant, " | ".join(cells)))
            print()
        print("Spread (min-max) at 16K, 512K and 16M:")
        print()
        print("| kernel | 16K | 512K | 16M |")
        print("| --- | --- | --- | --- |")
        for (name, g, variant), d in mem.items():
            cells = []
            for s in (16 << 10, 512 << 10, 16 << 20):
                if s in d:
                    m, lo, hi = combine(d[s])
                    cells.append(spread(lo, hi))
                else:
                    cells.append("")
            print("| %s | %s |" % (name, " | ".join(cells)))
        print()

    scan = list(section(runs[:1], "scan"))
    if scan:
        print("### Hardware thread scan")
        print()
        for r in scan:
            print("- " + ",".join(r[1:]))
        print()
    smt = list(section(runs, "smt"))
    if smt:
        print("### SMT and second core (rate per thread; unit per row)")
        print()
        print("| job | unit | alone | SMT pair: A | B | sum | other core: A | B | sum |")
        print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
        agg = collections.OrderedDict()
        for r in smt:
            agg.setdefault((r[1], r[2]), []).append([float(r[i]) for i in (4, 6, 7, 9, 11, 12, 14)])
        for (job, unit), vals in agg.items():
            cols = [statistics.median(c) for c in zip(*vals)]
            print("| %s | %s | %s |" % (job, unit, " | ".join(fmt(c) for c in cols)))
        print()


if __name__ == "__main__":
    main()
