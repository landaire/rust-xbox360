#!/usr/bin/env python3
"""Generates the benchmark kernels of vmx128bench.

Writes kernels.s (included with global_asm!) and kernels.rs (the kernel
tables, inputs and expected results). Every vector instruction is
emitted as a .long word, because the pinned assembler knows no VMX128
mnemonic. The words of the 128 forms come from the encoding model of the
dumpbin sweep; classic AltiVec words use the standard VX/VA/VC/X layouts.
Register kernels are simulated here with an instruction model to produce
the expected output; the title compares its results with it.

The BUCK file runs this at build time (genrule `kernels`):
    python3 gen/gen_kernels.py OUTDIR
"""

import math
import os
import struct
import sys



def _vd(v):
    return ((v & 31) << 21) | (((v >> 5) & 3) << 2)


def _va(v):
    return ((v & 31) << 16) | (((v >> 5) & 1) << 5) | (((v >> 6) & 1) << 10)


def _vb(v):
    return ((v & 31) << 11) | ((v >> 5) & 3)


# mnemonic -> (primary, value over the low 26 bits, operand fields)
T128 = {}
for m, xo in [("lvsl128", 0x001), ("lvsr128", 0x041), ("lvewx128", 0x081), ("lvx128", 0x0C1),
              ("stvewx128", 0x181), ("stvx128", 0x1C1), ("lvxl128", 0x2C1), ("stvxl128", 0x3C1),
              ("lvlx128", 0x401), ("lvrx128", 0x441), ("stvlx128", 0x501), ("stvrx128", 0x541),
              ("lvlxl128", 0x601), ("lvrxl128", 0x641), ("stvlxl128", 0x701), ("stvrxl128", 0x741)]:
    # cl.exe and the kernel set IBM bit 30 (LSB 1) in every VX128_1 word.
    T128[m] = (4, xo | 0x2, ("D", "RA", "RB"))
T128["vsldoi128"] = (4, 0x010, ("D", "A", "B", "SHB"))
for m, xo in [("vaddfp128", 0x010), ("vsubfp128", 0x050), ("vmulfp128", 0x090),
              ("vmsum3fp128", 0x190), ("vmsum4fp128", 0x1D0), ("vand128", 0x210),
              ("vandc128", 0x250), ("vnor128", 0x290), ("vor128", 0x2D0), ("vxor128", 0x310)]:
    T128[m] = (5, xo, ("D", "A", "B"))
T128["vmaddfp128"] = (5, 0x0D0, ("D", "A", "B", "D"))
T128["vmaddcfp128"] = (5, 0x110, ("D", "A", "D", "B"))
T128["vnmsubfp128"] = (5, 0x150, ("D", "A", "B", "D"))
T128["vsel128"] = (5, 0x350, ("D", "A", "B", "D"))
T128["vperm128"] = (5, 0x000, ("D", "A", "B", "C3"))
for m, xo in [("vcmpeqfp128", 0x000), ("vcmpeqfp128.", 0x040), ("vcmpgefp128", 0x080),
              ("vcmpgefp128.", 0x0C0), ("vcmpgtfp128", 0x100), ("vcmpgtfp128.", 0x140),
              ("vmaxfp128", 0x280), ("vminfp128", 0x2C0), ("vmrghw128", 0x300),
              ("vmrglw128", 0x340)]:
    T128[m] = (6, xo, ("D", "A", "B"))
for m, xo in [("vcfpsxws128", 0x230), ("vcsxwfp128", 0x2B0), ("vspltw128", 0x730)]:
    T128[m] = (6, xo, ("D", "B", "U5"))
for m, xo in [("vrfin128", 0x370), ("vrefp128", 0x630), ("vrsqrtefp128", 0x670)]:
    T128[m] = (6, xo, ("D", "B"))
T128["vspltisw128"] = (6, 0x770, ("D", "U5"))
T128["vupkd3d128"] = (6, 0x7F0, ("D", "B", "D3DT"))
T128["vpermwi128"] = (6, 0x210, ("D", "B", "PERM8"))
T128["vpkd3d128"] = (6, 0x610, ("D", "B", "D3DT", "D3DM", "Z2"))
T128["vrlimi128"] = (6, 0x710, ("D", "B", "IMM4", "Z2"))

INS = {
    "D": lambda v: _vd(v), "A": lambda v: _va(v), "B": lambda v: _vb(v),
    "C3": lambda v: (v & 7) << 6, "RA": lambda v: (v & 31) << 16, "RB": lambda v: (v & 31) << 11,
    "SHB": lambda v: (v & 15) << 6, "U5": lambda v: (v & 31) << 16,
    "PERM8": lambda v: ((v & 31) << 16) | (((v >> 5) & 7) << 6),
    "IMM4": lambda v: (v & 15) << 16, "D3DT": lambda v: (v & 7) << 18,
    "D3DM": lambda v: (v & 3) << 16, "Z2": lambda v: (v & 3) << 6,
}
MAXV = {"D": 127, "A": 127, "B": 127, "C3": 7, "RA": 31, "RB": 31, "SHB": 15, "U5": 31,
        "PERM8": 255, "IMM4": 15, "D3DT": 7, "D3DM": 3, "Z2": 3}


def enc128(m, ops):
    p, val, fields = T128[m]
    w = (p << 26) | val
    seen = {}
    assert len(ops) == len(fields), (m, ops)
    for f, v in zip(fields, ops):
        assert 0 <= v <= MAXV[f], (m, f, v)
        if f in seen:
            assert seen[f] == v, ("tied operand differs", m, ops)
            continue
        seen[f] = v
        w |= INS[f](v)
    return w


# Classic AltiVec: VX form (XO 11 bits), VC form (Rc + XO 10 bits),
# VA form (vC + XO 6 bits), X form (primary 31).
VX = {"vaddfp": 10, "vsubfp": 74, "vmaxfp": 1034, "vminfp": 1098, "vrefp": 266,
      "vrsqrtefp": 330, "vrfin": 522, "vcfsx": 842, "vctsxs": 970, "vand": 1028,
      "vandc": 1092, "vor": 1156, "vxor": 1220, "vnor": 1284, "vmrghw": 140, "vmrglw": 396,
      "vspltw": 652, "vspltisw": 908, "vadduwm": 128, "vslw": 388}
VC = {"vcmpeqfp": 198, "vcmpgefp": 454, "vcmpgtfp": 710}
VA = {"vmaddfp": 46, "vnmsubfp": 47, "vperm": 43, "vsel": 42, "vsldoi": 44}
X31 = {"lvx": 103, "stvx": 231, "lvsl": 6, "lvsr": 38, "lvewx": 71, "stvewx": 199, "lvxl": 359,
       "stvxl": 487, "lvlx": 519, "lvrx": 551, "stvlx": 647, "stvrx": 679, "lvlxl": 775,
       "lvrxl": 807, "stvlxl": 903, "stvrxl": 935}


def enc_classic(m, ops):
    for o in ops:
        assert 0 <= o <= 31, (m, ops)
    if m in VX:
        xo = VX[m]
        if m in ("vrefp", "vrsqrtefp", "vrfin"):
            d, b = ops
            a = 0
        elif m in ("vspltw", "vcfsx", "vctsxs"):
            d, b, a = ops  # asm order vD, vB, UIMM; UIMM sits in the vA field
        elif m == "vspltisw":
            d, a = ops
            b = 0
        else:
            d, a, b = ops
        return (4 << 26) | (d << 21) | (a << 16) | (b << 11) | xo
    base = m.rstrip(".")
    if base in VC:
        d, a, b = ops
        rc = 1 if m.endswith(".") else 0
        return (4 << 26) | (d << 21) | (a << 16) | (b << 11) | (rc << 10) | VC[base]
    if m in VA:
        if m in ("vmaddfp", "vnmsubfp"):
            d, a, c, b = ops  # asm order vD, vA, vC, vB
        elif m == "vsldoi":
            d, a, b, c = ops
            assert c <= 15
        else:
            d, a, b, c = ops
        return (4 << 26) | (d << 21) | (a << 16) | (b << 11) | (c << 6) | VA[m]
    if m in X31:
        d, ra, rb = ops
        return (31 << 26) | (d << 21) | (ra << 16) | (rb << 11) | (X31[m] << 1)
    raise KeyError(m)


def enc(m, ops):
    return enc128(m, ops) if m in T128 else enc_classic(m, ops)


def text(m, ops):
    regs = []
    fields = T128[m][2] if m in T128 else None
    for i, o in enumerate(ops):
        if m in X31 and i > 0:
            regs.append("r%d" % o)
        elif fields and fields[i] in ("RA", "RB"):
            regs.append("r%d" % o)
        elif fields and fields[i] in ("D", "A", "B", "C3"):
            regs.append("v%d" % o)
        elif fields is None and not (m in ("vspltw", "vcfsx", "vctsxs", "vspltisw", "vsldoi") and i == len(ops) - 1):
            regs.append("v%d" % o)
        else:
            regs.append(str(o))
    return "%s %s" % (m, ",".join(regs))


def vinsn(m, *ops):
    return "\t.long 0x%08X\t# %s" % (enc(m, ops), text(m, ops))


# A vector is a tuple of four u32 words, word 0 at the lowest address.


def f2u(f):
    try:
        return struct.unpack(">I", struct.pack(">f", f))[0]
    except OverflowError:
        return 0x7F800000 if f > 0 else 0xFF800000


def u2f(u):
    return struct.unpack(">f", struct.pack(">I", u & 0xFFFFFFFF))[0]


def rnd(x):
    return u2f(f2u(x))


def F(*fs):
    assert len(fs) == 4
    return tuple(f2u(f) for f in fs)


def W(*ws):
    assert len(ws) == 4
    return tuple(w & 0xFFFFFFFF for w in ws)


def vbytes(v):
    return b"".join(struct.pack(">I", w) for w in v)


def bvec(b):
    return tuple(struct.unpack(">4I", b))


def lanes(v):
    return [u2f(w) for w in v]


def fmap(fn, *vs):
    return F(*[rnd(fn(*xs)) for xs in zip(*[lanes(v) for v in vs])])


def s32(u):
    return u - (1 << 32) if u & 0x80000000 else u


def trunc_sat_s32(x):
    if x != x:
        return 0
    t = math.trunc(x) if abs(x) < 1e30 else (1 << 40) * (1 if x > 0 else -1)
    return max(-(1 << 31), min((1 << 31) - 1, t)) & 0xFFFFFFFF


def f16_bits(x):
    """f32 -> f16 per our ISA model: truncation, no inf/NaN."""
    u = f2u(x)
    sign = (u >> 31) << 15
    a = abs(x)
    if a == 0:
        return sign
    e = math.floor(math.log2(a))
    if e < -14:
        return sign
    if a >= 131008.0:
        return sign | 0x7FFF
    m = int((a / 2.0 ** e - 1.0) * 1024)
    return sign | ((e + 15) << 10) | m


def f16_value(h):
    s = -1.0 if h & 0x8000 else 1.0
    e = (h >> 10) & 31
    m = h & 1023
    if e == 0:
        return 0.0 * s
    return s * (1.0 + m / 1024.0) * 2.0 ** (e - 15)


def sim(m, regs, ops):
    """Applies one instruction to the register dict."""
    g = lambda r: regs[r]
    if m in ("vaddfp", "vaddfp128"):
        d, a, b = ops
        regs[d] = fmap(lambda x, y: x + y, g(a), g(b))
    elif m in ("vsubfp", "vsubfp128"):
        d, a, b = ops
        regs[d] = fmap(lambda x, y: x - y, g(a), g(b))
    elif m == "vmulfp128":
        d, a, b = ops
        regs[d] = fmap(lambda x, y: x * y, g(a), g(b))
    elif m == "vmaddfp":
        d, a, c, b = ops
        regs[d] = fmap(lambda x, y, z: x * y + z, g(a), g(c), g(b))
    elif m == "vnmsubfp":
        d, a, c, b = ops
        regs[d] = fmap(lambda x, y, z: -(x * y - z), g(a), g(c), g(b))
    elif m == "vmaddfp128":
        d, a, b, _ = ops
        regs[d] = fmap(lambda x, y, z: x * y + z, g(a), g(b), g(d))
    elif m == "vmaddcfp128":
        d, a, _, b = ops
        regs[d] = fmap(lambda x, y, z: x * y + z, g(a), g(d), g(b))
    elif m == "vnmsubfp128":
        d, a, b, _ = ops
        regs[d] = fmap(lambda x, y, z: -(x * y - z), g(a), g(b), g(d))
    elif m in ("vmsum4fp128", "vmsum3fp128"):
        d, a, b = ops
        n = 4 if m == "vmsum4fp128" else 3
        t = sum(x * y for x, y in list(zip(lanes(g(a)), lanes(g(b))))[:n])
        regs[d] = F(*([rnd(t)] * 4))
    elif m in ("vmaxfp", "vmaxfp128"):
        d, a, b = ops
        regs[d] = fmap(max, g(a), g(b))
    elif m in ("vand", "vand128", "vor", "vor128", "vxor", "vxor128", "vandc", "vandc128",
               "vnor", "vnor128"):
        d, a, b = ops
        k = m.replace("128", "")
        fn = {"vand": lambda x, y: x & y, "vor": lambda x, y: x | y, "vxor": lambda x, y: x ^ y,
              "vandc": lambda x, y: x & ~y & 0xFFFFFFFF,
              "vnor": lambda x, y: ~(x | y) & 0xFFFFFFFF}[k]
        regs[d] = tuple(fn(x, y) for x, y in zip(g(a), g(b)))
    elif m == "vslw":
        d, a, b = ops
        regs[d] = tuple((x << (y & 31)) & 0xFFFFFFFF for x, y in zip(g(a), g(b)))
    elif m == "vadduwm":
        d, a, b = ops
        regs[d] = tuple((x + y) & 0xFFFFFFFF for x, y in zip(g(a), g(b)))
    elif m == "vsel":
        d, a, b, c = ops
        regs[d] = tuple((cc & bb) | (~cc & aa & 0xFFFFFFFF) for aa, bb, cc in zip(g(a), g(b), g(c)))
    elif m == "vsel128":
        d, a, b, _ = ops
        regs[d] = tuple((cc & bb) | (~cc & aa & 0xFFFFFFFF) for aa, bb, cc in zip(g(a), g(b), g(d)))
    elif m in ("vperm", "vperm128"):
        d, a, b, c = ops
        ab = vbytes(g(a)) + vbytes(g(b))
        regs[d] = bvec(bytes(ab[x & 31] for x in vbytes(g(c))))
    elif m in ("vsldoi", "vsldoi128"):
        d, a, b, sh = ops
        regs[d] = bvec((vbytes(g(a)) + vbytes(g(b)))[sh:sh + 16])
    elif m in ("vspltw", "vspltw128"):
        d, b, u = ops
        regs[d] = (g(b)[u],) * 4
    elif m in ("vspltisw", "vspltisw128"):
        d, s = ops
        v = s - 32 if s >= 16 else s
        regs[d] = (v & 0xFFFFFFFF,) * 4
    elif m in ("vmrghw", "vmrghw128"):
        d, a, b = ops
        x, y = g(a), g(b)
        regs[d] = (x[0], y[0], x[1], y[1])
    elif m in ("vmrglw", "vmrglw128"):
        d, a, b = ops
        x, y = g(a), g(b)
        regs[d] = (x[2], y[2], x[3], y[3])
    elif m.rstrip(".") in ("vcmpgtfp", "vcmpgtfp128", "vcmpeqfp", "vcmpeqfp128",
                           "vcmpgefp", "vcmpgefp128"):
        d, a, b = ops
        k = m.rstrip(".").replace("128", "")
        fn = {"vcmpgtfp": lambda x, y: x > y, "vcmpeqfp": lambda x, y: x == y,
              "vcmpgefp": lambda x, y: x >= y}[k]
        regs[d] = tuple(0xFFFFFFFF if fn(x, y) else 0 for x, y in zip(lanes(g(a)), lanes(g(b))))
    elif m in ("vrefp", "vrefp128"):
        d, b = ops
        regs[d] = fmap(lambda x: 1.0 / x, g(b))
    elif m in ("vrsqrtefp", "vrsqrtefp128"):
        d, b = ops
        regs[d] = fmap(lambda x: 1.0 / math.sqrt(x), g(b))
    elif m in ("vrfin", "vrfin128"):
        d, b = ops
        regs[d] = fmap(lambda x: float(round(x)), g(b))
    elif m in ("vcfsx", "vcsxwfp128"):
        d, b, u = ops
        regs[d] = F(*[s32(w) / 2.0 ** u for w in g(b)])
    elif m in ("vctsxs", "vcfpsxws128"):
        d, b, u = ops
        regs[d] = tuple(trunc_sat_s32(x * 2.0 ** u) for x in lanes(g(b)))
    elif m == "vpermwi128":
        d, b, imm = ops
        x = g(b)
        regs[d] = tuple(x[(imm >> (6 - 2 * i)) & 3] for i in range(4))
    elif m == "vrlimi128":
        d, b, mask, rot = ops
        x, old = g(b), g(d)
        regs[d] = tuple(x[(i + rot) & 3] if mask & (8 >> i) else old[i] for i in range(4))
    elif m == "vpkd3d128":
        d, b, t, mask, z = ops
        assert t == 5 and mask in (2, 3) and z == 0
        x = lanes(g(b))
        h = [f16_bits(v) for v in x]
        p = (0, 0, (h[0] << 16) | h[1], (h[2] << 16) | h[3])
        old = g(d)
        regs[d] = (old[0], old[1], p[2], p[3])
    elif m == "vupkd3d128":
        d, b, t = ops
        x = g(b)
        if t == 5:
            hs = [x[2] >> 16, x[2] & 0xFFFF, x[3] >> 16, x[3] & 0xFFFF]
            regs[d] = F(*[f16_value(h) for h in hs])
        elif t == 0:
            w = x[3]
            # model layout W X Y Z (msb..lsb); result lanes x y z w
            bx, by, bz, bw = (w >> 16) & 255, (w >> 8) & 255, w & 255, w >> 24
            regs[d] = tuple(0x3F800000 | v for v in (bx, by, bz, bw))
        elif t == 1:
            w = x[3]
            hx, hy = w >> 16, w & 0xFFFF
            sx = lambda h: h - 65536 if h & 0x8000 else h
            regs[d] = ((0x40400000 + sx(hx)) & 0xFFFFFFFF, (0x40400000 + sx(hy)) & 0xFFFFFFFF,
                       0, f2u(1.0))
        else:
            raise KeyError(t)
    else:
        raise KeyError(m)



ASM = []
REG_KERNELS = []  # dicts for the Rust table
MEM_KERNELS = []
INPUT = []  # u32 words, every block 16-byte aligned
EXPECT = []


def add_input(vecs):
    off = len(INPUT)
    for v in vecs:
        INPUT.extend(v)
    return off


def add_expect(vecs):
    off = len(EXPECT)
    for v in vecs:
        EXPECT.extend(v)
    return off


def func_begin(name):
    ASM.append("")
    ASM.append("\t.section .text.vb.%s,\"ax\",@progbits" % name)
    ASM.append("\t.p2align 6")
    ASM.append("\t.globl %s" % name)
    ASM.append("\t.type %s,@function" % name)
    ASM.append("%s:" % name)


def func_end(name):
    ASM.append("\tblr")
    ASM.append("\t.size %s, .-%s" % (name, name))


def load_vec(reg, gpr_base, offset):
    """Loads vector register `reg` from offset(gpr_base); uses r11."""
    ASM.append("\tli 11, %d" % offset)
    if reg < 32:
        ASM.append(vinsn("lvx", reg, gpr_base, 11))
    else:
        ASM.append(vinsn("lvx128", reg, gpr_base, 11))


def store_vec(reg, gpr_base, offset):
    ASM.append("\tli 11, %d" % offset)
    if reg < 32:
        ASM.append(vinsn("stvx", reg, gpr_base, 11))
    else:
        ASM.append(vinsn("stvx128", reg, gpr_base, 11))


class Op:
    """A dependent sequence that one chain repeats.

    seqs: variant -> list of (mnemonic, operand tokens). Tokens: 'S' is the
    chain state, 'T0'.. are per-chain temporaries, other strings are shared
    constants, ints are immediates (written as ('#', n)).
    """

    def __init__(self, name, group, seqs, consts, state, ntemps=0, low=(), tol=None,
                 per="instr", chk_tol=None, note="", states=None):
        self.name = name
        self.group = group
        self.seqs = seqs
        self.consts = consts
        self.state = state
        self.states = states
        self.ntemps = ntemps
        self.low = set(low)
        self.tol = tol
        self.per = per
        self.chk_tol = chk_tol
        self.note = note


POOLS = {"cls": (0, 32), "128lo": (0, 32), "128hi": (64, 128), "128all": (0, 128), "mix": (0, 32)}


def alloc(op, variant, k):
    lo, hi = POOLS[variant]
    names = sorted(op.consts)
    regmap = {}
    nxt_low = 0
    nxt = lo
    for c in names:
        if c in op.low and lo >= 8:
            regmap[c] = nxt_low
            nxt_low += 1
    if lo == 0:
        for c in names:
            if c in op.low:
                regmap[c] = nxt
                nxt += 1
    for c in names:
        if c not in regmap:
            regmap[c] = nxt
            nxt += 1
    chains = []
    for i in range(k):
        cr = {"S": nxt}
        nxt += 1
        for t in range(op.ntemps):
            cr["T%d" % t] = nxt
            nxt += 1
        chains.append(cr)
    if nxt > hi:
        return None
    return regmap, chains


def max_chains(op, variant):
    lo, hi = POOLS[variant]
    nconst = len([c for c in op.consts if not (c in op.low and lo >= 8)])
    return (hi - lo - nconst) // (1 + op.ntemps)


def resolve(tok, regmap, cr):
    if isinstance(tok, tuple):
        return tok[1]
    if tok in cr:
        return cr[tok]
    return regmap[tok]


TARGET_BODY = 128


def gen_reg_kernel(op, variant, k, kind, reps=None, chk=False):
    seq = op.seqs[variant]
    a = alloc(op, variant, k)
    if a is None:
        return None
    regmap, chains = a
    if reps is None:
        reps = max(1, -(-TARGET_BODY // (len(seq) * k)))
    name = "k_%s_%s_%s%d" % (op.name, variant, kind, k)
    if chk:
        name = "k_%s_%s_chk%d" % (op.name, variant, k)
    # inputs: constants then chain states
    init = []
    for c in sorted(op.consts):
        init.append((regmap[c], op.consts[c]))
    for i, cr in enumerate(chains):
        st = op.states[i % len(op.states)] if op.states else op.state
        init.append((cr["S"], st))
        for t in range(op.ntemps):
            init.append((cr["T%d" % t], W(0, 0, 0, 0)))
    in_off = add_input([v for _, v in init])
    body = []
    for _ in range(reps):
        for ins in seq:
            for cr in chains:
                if ins[0] == "raw":
                    # a scalar instruction given as text; not simulated
                    body.append(("raw", ins[1]))
                    continue
                body.append((ins[0], tuple(resolve(t, regmap, cr) for t in ins[1:])))
    iters = 1 if chk else 2
    regs = {r: v for r, v in init}
    for _ in range(iters):
        for m, ops in body:
            if m != "raw":
                sim(m, regs, ops)
    outs = [cr["S"] for cr in chains]
    exp_off = add_expect([regs[r] for r in outs])
    func_begin(name)
    for i, (r, _) in enumerate(init):
        load_vec(r, 4, 16 * i)
    ASM.append("\tmtctr 3")
    ASM.append("\t.p2align 5")
    ASM.append("1:")
    for m, ops in body:
        ASM.append("\t" + ops if m == "raw" else vinsn(m, *ops))
    ASM.append("\tbdnz 1b")
    for i, r in enumerate(outs):
        store_vec(r, 5, 16 * i)
    func_end(name)
    tol = op.chk_tol if chk else op.tol
    REG_KERNELS.append(dict(
        name=name, op=op.name, group=op.group, variant=variant, kind="chk" if chk else kind,
        chains=k, seq=len(seq), body=len(body), ops=len(body), input=in_off,
        out_vecs=len(outs), expect=exp_off, check_iters=iters, tol=tol, per=op.per))
    return name


H = 0x80000000  # sign bit


def ops_list():
    L = []
    A = lambda *s: s
    # FP add: alternate +c and -c so the value stays bounded and exact.
    L.append(Op("vaddfp", "fp", {
        "cls": [A("vaddfp", "S", "S", "P"), A("vaddfp", "S", "S", "N")],
        "128lo": [A("vaddfp128", "S", "S", "P"), A("vaddfp128", "S", "S", "N")],
        "128hi": [A("vaddfp128", "S", "S", "P"), A("vaddfp128", "S", "S", "N")],
        "128all": [A("vaddfp128", "S", "S", "P"), A("vaddfp128", "S", "S", "N")],
        "mix": [A("vaddfp", "S", "S", "P"), A("vaddfp128", "S", "S", "N")],
    }, {"P": F(0.5, 1.0, 2.0, 0.25), "N": F(-0.5, -1.0, -2.0, -0.25)}, F(1.0, 2.0, 3.0, 4.0),
        states=[F(1.0, 2.0, 3.0, 4.0), F(-8.0, 16.0, 0.5, 6.0)]))
    # Multiply: vmulfp128 against vmaddfp with a -0.0 addend.
    L.append(Op("vmulfp", "fp", {
        "cls": [A("vmaddfp", "S", "S", "M2", "Z"), A("vmaddfp", "S", "S", "MH", "Z")],
        "128lo": [A("vmulfp128", "S", "S", "M2"), A("vmulfp128", "S", "S", "MH")],
        "128hi": [A("vmulfp128", "S", "S", "M2"), A("vmulfp128", "S", "S", "MH")],
        "128all": [A("vmulfp128", "S", "S", "M2"), A("vmulfp128", "S", "S", "MH")],
    }, {"M2": F(2.0, 4.0, 0.5, 8.0), "MH": F(0.5, 0.25, 2.0, 0.125), "Z": W(H, H, H, H)},
        F(3.0, -5.0, 7.0, 1.5)))
    # Fused multiply-add, chain through the addend.
    madd_c = {"MA": F(1.0, 1.0, 1.0, 1.0), "P": F(0.5, 1.0, 2.0, 0.25), "N": F(-0.5, -1.0, -2.0, -0.25)}
    L.append(Op("vmaddfp_acc", "fp", {
        # classic vmaddfp vD,vA,vC,vB = vA*vC+vB: state in vB
        "cls": [A("vmaddfp", "S", "P", "MA", "S"), A("vmaddfp", "S", "N", "MA", "S")],
        "128lo": [A("vmaddfp128", "S", "P", "MA", "S"), A("vmaddfp128", "S", "N", "MA", "S")],
        "128hi": [A("vmaddfp128", "S", "P", "MA", "S"), A("vmaddfp128", "S", "N", "MA", "S")],
        "128all": [A("vmaddfp128", "S", "P", "MA", "S"), A("vmaddfp128", "S", "N", "MA", "S")],
    }, madd_c, F(1.0, 2.0, 3.0, 4.0)))
    # Fused multiply-add, chain through a multiplicand: x = x*m + c.
    madd_m = {"M2": F(2.0, 2.0, 2.0, 2.0), "MH": F(0.5, 0.5, 0.5, 0.5), "P": F(1.0, 2.0, 4.0, 0.5),
              "N": F(-0.5, -1.0, -2.0, -0.25)}
    L.append(Op("vmaddfp_mul", "fp", {
        "cls": [A("vmaddfp", "S", "S", "M2", "P"), A("vmaddfp", "S", "S", "MH", "N")],
        # vmaddcfp128 vD,vA,vD,vB = vA*vD+vB
        "128lo": [A("vmaddcfp128", "S", "M2", "S", "P"), A("vmaddcfp128", "S", "MH", "S", "N")],
        "128hi": [A("vmaddcfp128", "S", "M2", "S", "P"), A("vmaddcfp128", "S", "MH", "S", "N")],
        "128all": [A("vmaddcfp128", "S", "M2", "S", "P"), A("vmaddcfp128", "S", "MH", "S", "N")],
    }, madd_m, F(1.0, 3.0, -2.0, 5.0)))
    # x = x*m + c with vmaddfp128 only: the addend must be copied first.
    L.append(Op("vmaddfp_copy", "fp", {
        "128lo": [A("vor128", "T0", "P", "P"), A("vmaddfp128", "T0", "S", "M2", "T0"),
                  A("vor128", "S", "N", "N"), A("vmaddfp128", "S", "T0", "MH", "S")],
        "128hi": [A("vor128", "T0", "P", "P"), A("vmaddfp128", "T0", "S", "M2", "T0"),
                  A("vor128", "S", "N", "N"), A("vmaddfp128", "S", "T0", "MH", "S")],
    }, madd_m, F(1.0, 3.0, -2.0, 5.0), ntemps=1, per="seq",
        note="vor128 copy + vmaddfp128 per multiply-add; latency is per 2-instruction pair"))
    L.append(Op("vnmsubfp", "fp", {
        # -(a*c - b): with a=1, chain through b: x -> -(c - x) = x - c
        "cls": [A("vnmsubfp", "S", "MA", "P", "S"), A("vnmsubfp", "S", "MA", "N", "S")],
        "128lo": [A("vnmsubfp128", "S", "MA", "P", "S"), A("vnmsubfp128", "S", "MA", "N", "S")],
        "128hi": [A("vnmsubfp128", "S", "MA", "P", "S"), A("vnmsubfp128", "S", "MA", "N", "S")],
    }, madd_c, F(1.0, 2.0, 3.0, 4.0)))
    L.append(Op("vmaxfp", "fp", {
        "cls": [A("vmaxfp", "S", "S", "P"), A("vmaxfp", "S", "S", "N")],
        "128lo": [A("vmaxfp128", "S", "S", "P"), A("vmaxfp128", "S", "S", "N")],
        "128hi": [A("vmaxfp128", "S", "S", "P"), A("vmaxfp128", "S", "S", "N")],
    }, {"P": F(0.5, 1.0, 2.0, 0.25), "N": F(-0.5, -1.0, -2.0, -0.25)}, F(1.0, 2.0, 3.0, 4.0)))
    # Permute: rotate words left by one through a control vector.
    rot = W(0x04050607, 0x08090A0B, 0x0C0D0E0F, 0x00010203)
    L.append(Op("vperm", "perm", {
        "cls": [A("vperm", "S", "S", "S", "C")],
        "128lo": [A("vperm128", "S", "S", "S", "C")],
        "128hi": [A("vperm128", "S", "S", "S", "C")],
        "128all": [A("vperm128", "S", "S", "S", "C")],
    }, {"C": rot}, W(1, 2, 3, 4), low=("C",),
        states=[W(1, 2, 3, 4), W(5, 6, 7, 8)]))
    # Select with the state as the mask: sel(0, ~0, x) = x.
    L.append(Op("vsel", "logic", {
        "cls": [A("vsel", "S", "Z0", "ONE", "S")],
        "128lo": [A("vsel128", "S", "Z0", "ONE", "S")],
        "128hi": [A("vsel128", "S", "Z0", "ONE", "S")],
    }, {"Z0": W(0, 0, 0, 0), "ONE": W(-1, -1, -1, -1)}, W(0x12345678, 0x9ABCDEF0, 0x0F0F0F0F, 0xF0F0F0F0)))
    # Select with the state as the first data input (classic only form):
    L.append(Op("vsel_data", "logic", {
        "cls": [A("vsel", "S", "S", "B", "MK")],
    }, {"B": W(0xAAAAAAAA, 0x55555555, 0x0, 0xFFFFFFFF), "MK": W(0, 0, 0, 0)},
        W(0x12345678, 0x9ABCDEF0, 0x0F0F0F0F, 0xF0F0F0F0)))
    L.append(Op("vxor", "logic", {
        "cls": [A("vxor", "S", "S", "K")],
        "128lo": [A("vxor128", "S", "S", "K")],
        "128hi": [A("vxor128", "S", "S", "K")],
        "128all": [A("vxor128", "S", "S", "K")],
    }, {"K": W(0x0F0F0F0F, 0x33333333, 0x55555555, 0xFFFFFFFF)}, W(1, 2, 3, 4),
        states=[W(1, 2, 3, 4), W(9, 8, 7, 6)]))
    L.append(Op("vand", "logic", {
        "cls": [A("vand", "S", "S", "K")],
        "128lo": [A("vand128", "S", "S", "K")],
        "128hi": [A("vand128", "S", "S", "K")],
    }, {"K": W(-1, -2, -1, 0x7FFFFFFF)}, W(0x11111111, 0x22222222, 0x33333333, 0x44444444)))
    L.append(Op("vor", "logic", {
        "cls": [A("vor", "S", "S", "S")],
        "128lo": [A("vor128", "S", "S", "S")],
        "128hi": [A("vor128", "S", "S", "S")],
    }, {}, W(0x11111111, 0x22222222, 0x33333333, 0x44444444)))
    L.append(Op("vadduwm", "int", {
        "cls": [A("vadduwm", "S", "S", "K")],
    }, {"K": W(1, 2, 3, 4)}, W(10, 20, 30, 40)))
    L.append(Op("vspltw", "perm", {
        "cls": [A("vspltw", "S", "S", ("#", 1)), A("vspltw", "S", "S", ("#", 2))],
        "128lo": [A("vspltw128", "S", "S", ("#", 1)), A("vspltw128", "S", "S", ("#", 2))],
        "128hi": [A("vspltw128", "S", "S", ("#", 1)), A("vspltw128", "S", "S", ("#", 2))],
    }, {}, W(1, 2, 3, 4)))
    L.append(Op("vmrghw", "perm", {
        "cls": [A("vmrghw", "S", "S", "K")],
        "128lo": [A("vmrghw128", "S", "S", "K")],
        "128hi": [A("vmrghw128", "S", "S", "K")],
    }, {"K": W(7, 8, 9, 10)}, W(1, 2, 3, 4)))
    L.append(Op("vsldoi", "perm", {
        "cls": [A("vsldoi", "S", "S", "S", ("#", 4))],
        "128lo": [A("vsldoi128", "S", "S", "S", ("#", 4))],
        "128hi": [A("vsldoi128", "S", "S", "S", ("#", 4))],
    }, {}, W(1, 2, 3, 4)))
    # Compare: x alternates between all-ones (NaN) and 0.0 against -1.0.
    for rec in ("", "."):
        nm = "vcmpgtfp" + ("_rec" if rec else "")
        L.append(Op(nm, "cmp", {
            "cls": [A("vcmpgtfp" + rec, "S", "S", "K")],
            "128lo": [A("vcmpgtfp128" + rec, "S", "S", "K")],
            "128hi": [A("vcmpgtfp128" + rec, "S", "S", "K")],
        }, {"K": F(-1.0, -1.0, 1.0, -1.0)}, F(1.0, -2.0, 3.0, 0.0)))
    # Estimates: chain applies the estimate repeatedly.
    L.append(Op("vrefp", "est", {
        "cls": [A("vrefp", "S", "S")],
        "128lo": [A("vrefp128", "S", "S")],
        "128hi": [A("vrefp128", "S", "S")],
        "128all": [A("vrefp128", "S", "S")],
    }, {}, F(4.0, 0.5, -8.0, 3.0), tol=0.25, chk_tol=1.0 / 2048))
    L.append(Op("vrsqrtefp", "est", {
        "cls": [A("vrsqrtefp", "S", "S")],
        "128lo": [A("vrsqrtefp128", "S", "S")],
        "128hi": [A("vrsqrtefp128", "S", "S")],
    }, {}, F(16.0, 0.25, 4.0, 2.0), tol=0.25, chk_tol=1.0 / 2048))
    # Conversions float -> s32 -> float (values are integral).
    L.append(Op("vcvt", "cvt", {
        "cls": [A("vctsxs", "S", "S", ("#", 0)), A("vcfsx", "S", "S", ("#", 0))],
        "128lo": [A("vcfpsxws128", "S", "S", ("#", 0)), A("vcsxwfp128", "S", "S", ("#", 0))],
        "128hi": [A("vcfpsxws128", "S", "S", ("#", 0)), A("vcsxwfp128", "S", "S", ("#", 0))],
    }, {}, F(3.0, -7.0, 1000.0, 0.0)))
    L.append(Op("vcvt_scaled", "cvt", {
        "cls": [A("vctsxs", "S", "S", ("#", 4)), A("vcfsx", "S", "S", ("#", 4))],
        "128lo": [A("vcfpsxws128", "S", "S", ("#", 4)), A("vcsxwfp128", "S", "S", ("#", 4))],
    }, {}, F(3.0625, -7.5, 100.25, 0.0)))
    L.append(Op("vrfin", "cvt", {
        "cls": [A("vrfin", "S", "S")],
        "128lo": [A("vrfin128", "S", "S")],
        "128hi": [A("vrfin128", "S", "S")],
    }, {}, F(2.5, -3.5, 7.25, 0.75)))
    # Dot products: 4-term weights sum to 1 so x stays fixed after step 1.
    q = F(0.25, 0.25, 0.25, 0.25)
    L.append(Op("dot4", "dot", {
        "128lo": [A("vmsum4fp128", "S", "S", "Q")],
        "128hi": [A("vmsum4fp128", "S", "S", "Q")],
        "128all": [A("vmsum4fp128", "S", "S", "Q")],
        # products, then two rotate+add steps (result splatted like vmsum4fp128)
        "cls": [A("vmaddfp", "T0", "S", "Q", "Z"), A("vsldoi", "T1", "T0", "T0", ("#", 8)),
                A("vaddfp", "T0", "T0", "T1"), A("vsldoi", "T1", "T0", "T0", ("#", 4)),
                A("vaddfp", "S", "T0", "T1")],
    }, {"Q": q, "Z": W(H, H, H, H)}, F(1.0, 2.0, 3.0, 4.0), ntemps=2, per="seq",
        # Positive inputs only: the dot-product unit is not exact for mixed
        # signs (probe dot4_neg).
        states=[F(1.0, 2.0, 3.0, 4.0), F(8.0, 4.0, 2.0, 2.0)]))
    t3 = F(0.5, 0.25, 0.25, 9.0)
    L.append(Op("dot3", "dot", {
        "128lo": [A("vmsum3fp128", "S", "S", "Q")],
        "128hi": [A("vmsum3fp128", "S", "S", "Q")],
        "cls": [A("vmaddfp", "T0", "S", "Q", "Z"), A("vand", "T0", "T0", "WM"),
                A("vsldoi", "T1", "T0", "T0", ("#", 8)), A("vaddfp", "T0", "T0", "T1"),
                A("vsldoi", "T1", "T0", "T0", ("#", 4)), A("vaddfp", "S", "T0", "T1")],
    }, {"Q": t3, "Z": W(H, H, H, H), "WM": W(-1, -1, -1, 0)}, F(1.0, 2.0, 3.0, 4.0), ntemps=2,
        per="seq"))
    # Word permute by immediate: rotate words left by one.
    L.append(Op("permwi", "perm", {
        "128lo": [A("vpermwi128", "S", "S", ("#", (1 << 6) | (2 << 4) | (3 << 2) | 0))],
        "128hi": [A("vpermwi128", "S", "S", ("#", (1 << 6) | (2 << 4) | (3 << 2) | 0))],
        "cls": [A("vperm", "S", "S", "S", "C")],
    }, {"C": rot}, W(1, 2, 3, 4), low=("C",)))
    # Rotate-and-insert: words 1,2 of x from B rotated by one word.
    L.append(Op("rlimi", "perm", {
        "128lo": [A("vrlimi128", "S", "B", ("#", 6), ("#", 1))],
        "128hi": [A("vrlimi128", "S", "B", ("#", 6), ("#", 1))],
        "cls": [A("vsldoi", "T0", "B", "B", ("#", 4)), A("vsel", "S", "S", "T0", "MK")],
    }, {"B": W(11, 12, 13, 14), "MK": W(0, -1, -1, 0)}, W(1, 2, 3, 4), ntemps=1, per="seq"))
    # D3D pack/unpack round trip through FLOAT16_4.
    L.append(Op("pkupk_f16", "d3d", {
        "128lo": [A("vpkd3d128", "T0", "S", ("#", 5), ("#", 2), ("#", 0)),
                  A("vupkd3d128", "S", "T0", ("#", 5))],
        "128hi": [A("vpkd3d128", "T0", "S", ("#", 5), ("#", 2), ("#", 0)),
                  A("vupkd3d128", "S", "T0", ("#", 5))],
    }, {}, F(1.5, -2.25, 0.5, 100.0), ntemps=1, per="seq"))
    # Record-form compares: one record form among seven plain ones, and a
    # record form followed by a dependent branch or CR move.
    allt = F(1.0, 2.0, 3.0, 4.0)
    kneg = {"K": F(-1.0, -1.0, 1.0, -1.0)}
    mk = lambda sfx, rec: A("vcmpgtfp" + sfx + rec, "T0", "S", "K")
    L.append(Op("cmp_rec_1of8", "cmp", {
        v: [mk(sfx, ".")] + [mk(sfx, "")] * 7
        for v, sfx in (("cls", ""), ("128lo", "128"), ("128hi", "128"))
    }, kneg, allt, ntemps=1, per="seq"))
    L.append(Op("cmp_rec_bc", "cmp", {
        v: [mk(sfx, "."), ("raw", "bc 12, 24, 3f\n3:")]
        for v, sfx in (("cls", ""), ("128lo", "128"), ("128hi", "128"))
    }, kneg, allt, ntemps=1, per="seq"))
    L.append(Op("cmp_rec_mfocrf", "cmp", {
        v: [mk(sfx, "."), ("raw", "mfocrf 0, 2")]
        for v, sfx in (("cls", ""), ("128lo", "128"), ("128hi", "128"))
    }, kneg, allt, ntemps=1, per="seq"))
    L.append(Op("cmp_plain_mfocrf", "cmp", {
        v: [mk(sfx, ""), ("raw", "mfocrf 0, 2")]
        for v, sfx in (("cls", ""), ("128lo", "128"))
    }, kneg, allt, ntemps=1, per="seq"))
    # Mixed latencies: a 12-cycle FP op and a 4-cycle op from independent
    # chains, alternating.
    pn = {"P": F(0.5, 1.0, 2.0, 0.25), "N": F(-0.5, -1.0, -2.0, -0.25),
          "X": W(0x0F0F0F0F, 0x33333333, 0x55555555, 0xFFFFFFFF), "C": rot}
    for nm, cls_simple, v128_simple in (
            ("mix_fp_xor", A("vxor", "T0", "T0", "X"), A("vxor128", "T0", "T0", "X")),
            ("mix_fp_perm", A("vperm", "T0", "T0", "T0", "C"), A("vperm128", "T0", "T0", "T0", "C")),
            ("mix_fp_splat", A("vspltw", "T0", "T0", ("#", 1)), A("vspltw128", "T0", "T0", ("#", 1)))):
        L.append(Op(nm, "mixlat", {
            "cls": [A("vaddfp", "S", "S", "P"), cls_simple, A("vaddfp", "S", "S", "N"), cls_simple],
            "128lo": [A("vaddfp128", "S", "S", "P"), v128_simple, A("vaddfp128", "S", "S", "N"), v128_simple],
            "128hi": [A("vaddfp128", "S", "S", "P"), v128_simple, A("vaddfp128", "S", "S", "N"), v128_simple],
        }, pn, F(1.0, 2.0, 3.0, 4.0), ntemps=1, low=("C",), per="seq"))
    # Two FP ops then one simple op (the 2:1 mix of a transform's inner loop).
    L.append(Op("mix_fp2_xor", "mixlat", {
        "128lo": [A("vaddfp128", "S", "S", "P"), A("vaddfp128", "S", "S", "N"), A("vxor128", "T0", "T0", "X")],
        "cls": [A("vaddfp", "S", "S", "P"), A("vaddfp", "S", "S", "N"), A("vxor", "T0", "T0", "X")],
    }, pn, F(1.0, 2.0, 3.0, 4.0), ntemps=1, low=("C",), per="seq"))
    return L


SWEEP_K = [1, 2, 3, 4, 5, 6, 8, 10, 12, 14, 16, 20, 24, 28, 32, 48, 64, 96, 120]
SWEEP_OPS = {"vaddfp", "vmaddfp_acc", "vmaddfp_mul", "vmulfp", "vperm", "vxor", "vrefp", "dot4"}


def gen_reg():
    for op in ops_list():
        # Same chain count for every variant, so that it fits in v0-v31.
        low = [v for v in ("cls", "128lo") if v in op.seqs]
        kthr = min([16] + [max_chains(op, v) for v in low])
        for variant in ("cls", "128lo", "128hi", "mix"):
            if variant not in op.seqs:
                continue
            gen_reg_kernel(op, variant, 1, "lat")
            gen_reg_kernel(op, variant, kthr, "thr")
            if op.chk_tol is not None:
                gen_reg_kernel(op, variant, 4, "chk", reps=1, chk=True)
        if op.name in SWEEP_OPS:
            for variant in ("cls", "128all"):
                if variant not in op.seqs:
                    continue
                for k in SWEEP_K:
                    if k > max_chains(op, variant):
                        continue
                    if variant == "cls" and k in (1, kthr):
                        continue  # the plain lat/thr kernels already cover these
                    gen_reg_kernel(op, variant, k, "thr")


PROBES = []


def gen_probe(name, m, ops_tmpl, inputs, desc):
    """One instruction applied once: v1 = op(v2 [, v3]); stores v1."""
    regs = {1: W(0, 0, 0, 0)}
    for i, v in enumerate(inputs):
        regs[2 + i] = v
    sim(m, regs, ops_tmpl)
    in_off = add_input([regs.get(1)] + list(inputs))
    exp_off = add_expect([regs[1]])
    fname = "p_" + name
    func_begin(fname)
    load_vec(1, 4, 0)
    for i in range(len(inputs)):
        load_vec(2 + i, 4, 16 * (i + 1))
    ASM.append(vinsn(m, *ops_tmpl))
    store_vec(1, 5, 0)
    func_end(fname)
    PROBES.append(dict(name=name, f=fname, input=in_off, expect=exp_off, desc=desc,
                       ninputs=1 + len(inputs)))


def gen_probes():
    gen_probe("upk_d3dcolor", "vupkd3d128", (1, 2, 0), [W(0, 0, 0, 0x80402010)],
              "vupkd3d128 type 0 of word3 0x80402010; model: lanes x,y,z,w = 0x3F8000xx of bytes X,Y,Z,W")
    gen_probe("upk_normshort2", "vupkd3d128", (1, 2, 1), [W(0, 0, 0, 0x1234FFFE)],
              "vupkd3d128 type 1 of word3 0x1234FFFE; model: 3.0+x*2^-22 bit patterns, z=0, w=1.0")
    gen_probe("upk_f16x4", "vupkd3d128", (1, 2, 5), [W(0, 0, 0x3C00C000, 0x38005640)],
              "vupkd3d128 type 5 of words2,3; f16 1.0,-2.0,0.5,100.0")
    gen_probe("pk_f16x4", "vpkd3d128", (1, 2, 5, 2, 0), [F(1.0, -2.0, 0.5, 100.0)],
              "vpkd3d128 type 5 mask 2 shift 0 of 1,-2,0.5,100; model: words 2,3 = 3C00C000 38005640")
    gen_probe("dot4_neg", "vmsum4fp128", (1, 2, 3), [F(1, 1, 1, 1), F(1, -1, 1, -1)],
              "vmsum4fp128 (1,1,1,1).(1,-1,1,-1); exact result 0")
    gen_probe("rlimi_m9r3", "vrlimi128", (1, 2, 9, 3), [W(11, 12, 13, 14)],
              "vrlimi128 mask 9 (x,w) rot 3 into zero vector")
    gen_probe("permwi_1b", "vpermwi128", (1, 2, 0x1B), [W(11, 12, 13, 14)],
              "vpermwi128 imm 0x1B = fields [0|1|2|3]: identity")
    gen_probe("permwi_e4", "vpermwi128", (1, 2, 0xE4), [W(11, 12, 13, 14)],
              "vpermwi128 imm 0xE4 = fields [3|2|1|0]: word reverse")


QPROBES = []


def gen_qprobes():
    """One memory-form instruction each, executed once (section `insn`):
    r4 = 64 bytes of input (16-byte aligned), r5 = 64 bytes of output.
    Loads use EA = r4 + 4 and store the result with stvx128 to r5; stores
    take v1 (or v65) loaded from r4 and store to r5 + 4."""
    loads = ["lvsl128", "lvsr128", "lvewx128", "lvx128", "lvxl128", "lvlx128", "lvrx128",
             "lvlxl128", "lvrxl128", "lvsl", "lvsr", "lvewx", "lvxl", "lvlx", "lvrx", "lvlxl", "lvrxl"]
    stores = ["stvewx128", "stvx128", "stvxl128", "stvlx128", "stvrx128", "stvlxl128", "stvrxl128",
              "stvewx", "stvxl", "stvlx", "stvrx", "stvlxl", "stvrxl"]
    for m in loads + stores:
        for reg in ((1, 65) if m.endswith("128") else (1,)):
            name = "q_%s_v%d" % (m, reg)
            func_begin(name)
            ASM.append("\tli 11, 4")
            ASM.append("\tli 12, 0")
            if m in loads:
                ASM.append(vinsn("vxor128", reg, reg, reg))
                ASM.append(vinsn(m, reg, 4, 11))
                ASM.append(vinsn("stvx128", reg, 5, 12))
            else:
                ASM.append(vinsn("lvx128", reg, 4, 12))
                ASM.append(vinsn(m, reg, 5, 11))
            func_end(name)
            QPROBES.append(dict(name=name, insn=m, reg=reg))




def r(n):
    return "%d" % n


VEC_OFF_REGS = [0, 14, 15, 16, 17, 18, 19, 20]  # rA values holding 0,16,..,112 (0 = literal)


def mem_prologue():
    # r3 passes, r4 src, r5 dst, r6 bytes per pass (multiple of 128)
    for i, g in enumerate(VEC_OFF_REGS[1:], start=1):
        ASM.append("\tli %d, %d" % (g, 16 * i))
    ASM.append("\tli 21, 128")


def mem_kernel(name, group, kind, variant, body_fn, setup_fn=None, end_fn=None, src_off=0,
               dst_off=0, pattern=0, insns=0, note=""):
    func_begin(name)
    mem_prologue()
    if setup_fn:
        setup_fn()
    ASM.append("2:")
    ASM.append("\tmr 7, 4")
    ASM.append("\tmr 8, 5")
    ASM.append("\tsrwi 9, 6, 7")
    ASM.append("\tmtctr 9")
    ASM.append("\t.p2align 5")
    ASM.append("1:")
    body_fn()
    ASM.append("\taddi 7, 7, 128")
    ASM.append("\taddi 8, 8, 128")
    ASM.append("\tbdnz 1b")
    ASM.append("\taddic. 3, 3, -1")
    ASM.append("\tbne 2b")
    if end_fn:
        end_fn()
    func_end(name)
    MEM_KERNELS.append(dict(name=name, group=group, kind=kind, variant=variant, src_off=src_off,
                            dst_off=dst_off, pattern=pattern, insns=insns, note=note))


def vload(reg, i, base=7):
    m = "lvx" if reg < 32 else "lvx128"
    return vinsn(m, reg, VEC_OFF_REGS[i], base)


def vstore(reg, i, base=8):
    m = "stvx" if reg < 32 else "stvx128"
    return vinsn(m, reg, VEC_OFF_REGS[i], base)


def xor(d, a, b):
    return vinsn("vxor" if max(d, a, b) < 32 else "vxor128", d, a, b)


def zero_accs(accs):
    for a in accs:
        ASM.append(xor(a, a, a))


def store_accs(accs):
    for i, a in enumerate(accs):
        ASM.append("\tli 11, %d" % (16 * i))
        ASM.append(vinsn("stvx" if a < 32 else "stvx128", a, 5, 11))


def gen_mem():
    def rd_vec(base, acc, dcbt=None):
        def body():
            if dcbt:
                ASM.append("\tdcbt 22, 7")
            for i in range(8):
                ASM.append(vload(base + i, i))
            for i in range(8):
                ASM.append(xor(acc + (i & 3), acc + (i & 3), base + i))
        return body

    for variant, base, acc in (("lvx", 0, 8), ("lvx128lo", 0, 8), ("lvx128hi", 64, 72)):
        accs = list(range(acc, acc + 4))
        if variant == "lvx128lo":
            def body(base=base, acc=acc):
                for i in range(8):
                    ASM.append(vinsn("lvx128", base + i, VEC_OFF_REGS[i], 7))
                for i in range(8):
                    ASM.append(vinsn("vxor128", acc + (i & 3), acc + (i & 3), base + i))
        else:
            body = rd_vec(base, acc)
        mem_kernel("m_rd_" + variant, "read", "read", variant, body,
                   setup_fn=lambda accs=accs: zero_accs(accs),
                   end_fn=lambda accs=accs: store_accs(accs), insns=17)
    # prefetch distances
    for dist in (256, 512, 1024, 2048):
        accs = list(range(72, 76))
        def setup(dist=dist, accs=accs):
            ASM.append("\tli 22, %d" % dist)
            zero_accs(accs)
        mem_kernel("m_rd_dcbt%d" % dist, "read", "read", "lvx128hi+dcbt%d" % dist,
                   rd_vec(64, 72, dcbt=True), setup_fn=setup,
                   end_fn=lambda accs=accs: store_accs(accs), insns=18)
    # loads only; the last line is stored for the check
    def rd_nouse():
        for i in range(8):
            ASM.append(vinsn("lvx128", 64 + i, VEC_OFF_REGS[i], 7))

    def st_last():
        for i in range(8):
            ASM.append("\tli 11, %d" % (16 * i))
            ASM.append(vinsn("stvx128", 64 + i, 5, 11))
    mem_kernel("m_rd_nouse", "read", "readlast", "lvx128 load only", rd_nouse, end_fn=st_last,
               insns=8)
    # scalar reads
    def rd_lwz():
        accs = [10, 12, 0, 31]
        for grp in range(4):
            for i in range(8):
                ASM.append("\tlwz %d, %d(7)" % (23 + i, 32 * grp + 4 * i))
            for i in range(8):
                a = accs[i & 3]
                ASM.append("\txor %d, %d, %d" % (a, a, 23 + i))

    def gpr_zero():
        for a in (10, 12, 0, 31):
            ASM.append("\tli %d, 0" % a)

    def gpr_store64():
        for i, a in enumerate((10, 12, 0, 31)):
            ASM.append("\tstd %d, %d(5)" % (a, 8 * i))
    mem_kernel("m_rd_lwz", "read", "readgpr", "lwz", rd_lwz, setup_fn=gpr_zero,
               end_fn=gpr_store64, insns=64)

    def rd_ld():
        accs = [10, 12, 0, 31]
        for grp in range(2):
            for i in range(8):
                ASM.append("\tld %d, %d(7)" % (23 + i, 64 * grp + 8 * i))
            for i in range(8):
                a = accs[i & 3]
                ASM.append("\txor %d, %d, %d" % (a, a, 23 + i))
    mem_kernel("m_rd_ld", "read", "readgpr", "ld", rd_ld, setup_fn=gpr_zero, end_fn=gpr_store64,
               insns=32)

    def rd_lfd():
        for i in range(16):
            ASM.append("\tlfd %d, %d(7)" % (i % 8, 8 * i))

    def st_lfd():
        for i in range(8):
            ASM.append("\tstfd %d, %d(5)" % (i, 64 + 8 * i))
    mem_kernel("m_rd_lfd", "read", "readlastf", "lfd load only", rd_lfd, end_fn=st_lfd, insns=16)

    # unaligned reads with lvlx/lvrx (src + 4); vor joins the halves
    def rd_lvlx():
        for i in range(8):
            ASM.append(vinsn("lvlx", i, VEC_OFF_REGS[i], 7))
            ASM.append(vinsn("lvrx", 16 + i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 7))
        for i in range(8):
            ASM.append(vinsn("vor", i, i, 16 + i))
        for i in range(8):
            ASM.append(xor(8 + (i & 3), 8 + (i & 3), i))
    accs = list(range(8, 12))
    mem_kernel("m_rd_lvlx", "read", "read", "lvlx+lvrx unaligned", rd_lvlx,
               setup_fn=lambda: zero_accs(range(8, 12)),
               end_fn=lambda: store_accs(range(8, 12)), src_off=4, insns=33)

    def rd_lvlx128():
        for i in range(8):
            ASM.append(vinsn("lvlx128", 64 + i, VEC_OFF_REGS[i], 7))
            ASM.append(vinsn("lvrx128", 80 + i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 7))
        for i in range(8):
            ASM.append(vinsn("vor128", 64 + i, 64 + i, 80 + i))
        for i in range(8):
            ASM.append(xor(72 + (i & 3), 72 + (i & 3), 64 + i))
    mem_kernel("m_rd_lvlx128", "read", "read", "lvlx128+lvrx128 unaligned", rd_lvlx128,
               setup_fn=lambda: zero_accs(range(72, 76)),
               end_fn=lambda: store_accs(range(72, 76)), src_off=4, insns=33)

    # unaligned reads with lvsl + lvx + vperm (classic realignment)
    # v0..v8 hold nine consecutive aligned blocks, v12..v19 the realigned
    # data, v20 the lvsl control and v24..v27 the accumulators. v8 (the first
    # block of the next line) moves to v0 at the end of the iteration.
    def rd_lvsl_setup():
        zero_accs(range(24, 28))
        ASM.append(vinsn("lvsl", 20, 0, 4))
        ASM.append(vinsn("lvx", 0, 0, 4))

    def rd_lvsl():
        for i in range(1, 9):
            ASM.append(vinsn("lvx", i, VEC_OFF_REGS[i] if i < 8 else 21, 7))
        for i in range(8):
            ASM.append(vinsn("vperm", 12 + i, i, i + 1, 20))
        for i in range(8):
            ASM.append(xor(24 + (i & 3), 24 + (i & 3), 12 + i))
        ASM.append(vinsn("vor", 0, 8, 8))
    mem_kernel("m_rd_lvsl", "read", "read", "lvsl+lvx+vperm unaligned", rd_lvsl,
               setup_fn=rd_lvsl_setup, end_fn=lambda: store_accs(range(24, 28)), src_off=4,
               insns=26)

    def load_pattern_vec(base):
        def f():
            for i in range(8):
                ASM.append("\tli 11, %d" % (16 * i))
                ASM.append(vinsn("lvx" if base < 32 else "lvx128", base + i, 4, 11))
        return f

    def wr_vec(base, dcbz=False, m128=False):
        def body():
            if dcbz:
                ASM.append("\t.long 0x%08X\t# dcbz128 0,r8" % (0x7C2007EC | (8 << 11)))
            for i in range(8):
                if m128:
                    ASM.append(vinsn("stvx128", base + i, VEC_OFF_REGS[i], 8))
                else:
                    ASM.append(vstore(base + i, i))
        return body
    mem_kernel("m_wr_stvx", "write", "write", "stvx", wr_vec(0), setup_fn=load_pattern_vec(0),
               pattern=128, insns=8)
    mem_kernel("m_wr_stvx128hi", "write", "write", "stvx128hi", wr_vec(64),
               setup_fn=load_pattern_vec(64), pattern=128, insns=8)
    mem_kernel("m_wr_dcbz_stvx128", "write", "write", "dcbz128+stvx128", wr_vec(64, dcbz=True),
               setup_fn=load_pattern_vec(64), pattern=128, insns=9)

    def wr_dcbz():
        ASM.append("\t.long 0x%08X\t# dcbz128 0,r8" % (0x7C2007EC | (8 << 11)))
    mem_kernel("m_wr_dcbz", "write", "write", "dcbz128 only", wr_dcbz, pattern=0, insns=1)

    def wr_stw_setup():
        for i in range(8):
            ASM.append("\tlwz %d, %d(4)" % (23 + i, 4 * i))

    def wr_stw():
        for j in range(32):
            ASM.append("\tstw %d, %d(8)" % (23 + (j & 7), 4 * j))
    mem_kernel("m_wr_stw", "write", "write", "stw", wr_stw, setup_fn=wr_stw_setup, pattern=32,
               insns=32)

    def wr_std_setup():
        for i in range(8):
            ASM.append("\tld %d, %d(4)" % (23 + i, 8 * i))

    def wr_std():
        for j in range(16):
            ASM.append("\tstd %d, %d(8)" % (23 + (j & 7), 8 * j))
    mem_kernel("m_wr_std", "write", "write", "std", wr_std, setup_fn=wr_std_setup, pattern=64,
               insns=16)

    def wr_stfd_setup():
        for i in range(8):
            ASM.append("\tlfd %d, %d(4)" % (i, 8 * i))

    def wr_stfd():
        for j in range(16):
            ASM.append("\tstfd %d, %d(8)" % (j & 7, 8 * j))
    mem_kernel("m_wr_stfd", "write", "write", "stfd", wr_stfd, setup_fn=wr_stfd_setup, pattern=64,
               insns=16)

    def wr_stvlx():
        for i in range(8):
            ASM.append(vinsn("stvlx", i, VEC_OFF_REGS[i], 8))
            ASM.append(vinsn("stvrx", i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 8))
    mem_kernel("m_wr_stvlx", "write", "write", "stvlx+stvrx unaligned", wr_stvlx,
               setup_fn=load_pattern_vec(0), dst_off=4, pattern=128, insns=16)

    def wr_stvlx128():
        for i in range(8):
            ASM.append(vinsn("stvlx128", 64 + i, VEC_OFF_REGS[i], 8))
            ASM.append(vinsn("stvrx128", 64 + i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 8))
    mem_kernel("m_wr_stvlx128", "write", "write", "stvlx128+stvrx128 unaligned", wr_stvlx128,
               setup_fn=load_pattern_vec(64), dst_off=4, pattern=128, insns=16)

    def cp_vec(base, m128=False, dcbz=False, dcbt=False):
        def body():
            if dcbt:
                ASM.append("\tdcbt 22, 7")
            if dcbz:
                ASM.append("\t.long 0x%08X\t# dcbz128 0,r8" % (0x7C2007EC | (8 << 11)))
            for i in range(8):
                ASM.append(vinsn("lvx128" if m128 else "lvx", base + i, VEC_OFF_REGS[i], 7))
            for i in range(8):
                ASM.append(vinsn("stvx128" if m128 else "stvx", base + i, VEC_OFF_REGS[i], 8))
        return body
    mem_kernel("m_cp_lvx", "copy", "copy", "lvx/stvx", cp_vec(0), insns=16)
    mem_kernel("m_cp_lvx128hi", "copy", "copy", "lvx128/stvx128 hi", cp_vec(64, m128=True),
               insns=16)
    mem_kernel("m_cp_dcbz", "copy", "copy", "lvx128/stvx128+dcbz128", cp_vec(64, True, dcbz=True),
               insns=17)
    for dist in (512, 1024):
        mem_kernel("m_cp_dcbt%d_dcbz" % dist, "copy", "copy",
                   "lvx128/stvx128+dcbt%d+dcbz128" % dist,
                   cp_vec(64, True, dcbz=True, dcbt=True),
                   setup_fn=lambda dist=dist: ASM.append("\tli 22, %d" % dist), insns=18)

    def cp_lwz():
        for grp in range(4):
            for i in range(8):
                ASM.append("\tlwz %d, %d(7)" % (23 + i, 32 * grp + 4 * i))
            for i in range(8):
                ASM.append("\tstw %d, %d(8)" % (23 + i, 32 * grp + 4 * i))
    mem_kernel("m_cp_lwz", "copy", "copy", "lwz/stw", cp_lwz, insns=64)

    def cp_ld():
        for grp in range(2):
            for i in range(8):
                ASM.append("\tld %d, %d(7)" % (23 + i, 64 * grp + 8 * i))
            for i in range(8):
                ASM.append("\tstd %d, %d(8)" % (23 + i, 64 * grp + 8 * i))
    mem_kernel("m_cp_ld", "copy", "copy", "ld/std", cp_ld, insns=32)

    def cp_lfd():
        for grp in range(2):
            for i in range(8):
                ASM.append("\tlfd %d, %d(7)" % (i, 64 * grp + 8 * i))
            for i in range(8):
                ASM.append("\tstfd %d, %d(8)" % (i, 64 * grp + 8 * i))
    mem_kernel("m_cp_lfd", "copy", "copy", "lfd/stfd", cp_lfd, insns=32)

    def cp_lvlx():
        for i in range(8):
            ASM.append(vinsn("lvlx", i, VEC_OFF_REGS[i], 7))
            ASM.append(vinsn("lvrx", 16 + i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 7))
        for i in range(8):
            ASM.append(vinsn("vor", i, i, 16 + i))
        for i in range(8):
            ASM.append(vstore(i, i))
    mem_kernel("m_cp_lvlx", "copy", "copy", "lvlx+lvrx src+4 -> stvx", cp_lvlx, src_off=4,
               insns=32)

    def cp_stvlx():
        for i in range(8):
            ASM.append(vinsn("lvx", i, VEC_OFF_REGS[i], 7))
        for i in range(8):
            ASM.append(vinsn("stvlx", i, VEC_OFF_REGS[i], 8))
            ASM.append(vinsn("stvrx", i, VEC_OFF_REGS[i + 1] if i < 7 else 21, 8))
    mem_kernel("m_cp_stvlx", "copy", "copy", "lvx -> stvlx+stvrx dst+4", cp_stvlx, dst_off=4,
               insns=24)


XFORMS = []


def gen_xform(form, u, pool):
    """out[i] = M * in[i] for 4x4 M (columns at r4, 4 vectors) over r6 vectors
    at r4+64, written to r5. r3 passes."""
    lo, hi = pool
    name = "x_%s_u%d_%s" % (form, u, "v32" if hi == 32 else "v128")
    m128 = hi > 32
    regs = iter(range(lo, hi))
    cols = [next(regs) for _ in range(4)]
    zero = next(regs) if not m128 else None
    if form == "dot":
        cols = cols  # rows of M for dot products
    per = []
    try:
        for _ in range(u):
            if form == "splat6":
                per.append([next(regs) for _ in range(6)])  # X SX SY SZ SW ACC
            elif form == "splat3":
                per.append([next(regs) for _ in range(3)])  # X S ACC
            elif form == "dot":
                per.append([next(regs) for _ in range(5)])  # X O T1 T2 T3
    except StopIteration:
        return None
    L = lambda m, *o: ASM.append(vinsn(m, *o))
    ld = "lvx128" if m128 else "lvx"
    st = "stvx128" if m128 else "stvx"
    func_begin(name)
    for i, c in enumerate(cols):
        ASM.append("\tli 11, %d" % (16 * i))
        L(ld, c, 4, 11)
    if zero is not None:
        # -0.0 addend for vmaddfp: all-ones shifted left by 31 per word
        L("vspltisw", zero, 31)
        L("vslw", zero, zero, zero)
    ASM.append("2:")
    ASM.append("\taddi 7, 4, 64")
    ASM.append("\tmr 8, 5")
    ASM.append("\tli 9, %d" % u)
    ASM.append("\tdivwu 9, 6, 9")
    ASM.append("\tmtctr 9")
    ASM.append("\t.p2align 5")
    ASM.append("1:")
    for j, p in enumerate(per):
        ASM.append("\tli 11, %d" % (16 * j))
        L(ld, p[0], 7, 11)
    mul = (lambda acc, s, c: L("vmulfp128", acc, s, c)) if m128 else \
        (lambda acc, s, c: L("vmaddfp", acc, s, c, zero))
    madd = (lambda acc, s, c: L("vmaddfp128", acc, s, c, acc)) if m128 else \
        (lambda acc, s, c: L("vmaddfp", acc, s, c, acc))
    splat = (lambda d, b, k: L("vspltw128", d, b, k)) if m128 else (lambda d, b, k: L("vspltw", d, b, k))
    if form == "splat6":
        for k in range(4):
            for p in per:
                splat(p[1 + k], p[0], k)
        for k in range(4):
            for p in per:
                (mul if k == 0 else madd)(p[5], p[1 + k], cols[k])
        outs = [p[5] for p in per]
    elif form == "splat3":
        for k in range(4):
            for p in per:
                splat(p[1], p[0], k)
            for p in per:
                (mul if k == 0 else madd)(p[2], p[1], cols[k])
        outs = [p[2] for p in per]
    else:  # dot: rows in cols; o = dot(r0,x); insert dot(r1..r3,x)
        for p in per:
            L("vmsum4fp128", p[1], p[0], cols[0])
        for k in range(1, 4):
            for p in per:
                L("vmsum4fp128", p[1 + k], p[0], cols[k])
        for k in range(1, 4):
            for p in per:
                L("vrlimi128", p[1], p[1 + k], 8 >> k, 0)
        outs = [p[1] for p in per]
    for j, o in enumerate(outs):
        ASM.append("\tli 11, %d" % (16 * j))
        L(st, o, 8, 11)
    ASM.append("\taddi 7, 7, %d" % (16 * u))
    ASM.append("\taddi 8, 8, %d" % (16 * u))
    ASM.append("\tbdnz 1b")
    ASM.append("\taddic. 3, 3, -1")
    ASM.append("\tbne 2b")
    func_end(name)
    XFORMS.append(dict(name=name, form=form, u=u, regs="v0-v31" if hi == 32 else "v0-v127",
                       rowmajor=form == "dot"))
    return name


def gen_xforms():
    for form, us32, us128 in (("splat6", (1, 2, 4), (4, 8, 12, 16, 20)),
                              ("splat3", (1, 2, 4, 8), (4, 8, 16, 24, 32)),
                              ("dot", (), (1, 2, 4, 8, 16, 24))):
        for u in us32:
            gen_xform(form, u, (0, 32))
        for u in us128:
            gen_xform(form, u, (0, 128))




CLOCKS = []


def gen_clock():
    """Scalar dependent chains used to relate timebase ticks to core cycles.
    r3 iterations, r5 out. Each kernel stores r7 (or f1) to out."""
    def k(name, desc, setup, body, n, store, steps=False):
        func_begin(name)
        ASM.extend("\t" + s for s in setup)
        ASM.append("\tmtctr 3")
        ASM.append("\t.p2align 5")
        ASM.append("1:")
        for _ in range(n):
            ASM.extend("\t" + s for s in body)
        ASM.append("\tbdnz 1b")
        ASM.extend("\t" + s for s in store)
        func_end(name)
        # steps: report per repetition of `body` rather than per instruction
        CLOCKS.append(dict(name=name, desc=desc, ops=n if steps else n * len(body)))
    k("c_add", "add r7 r7 r8 dependent", ["li 7, 0", "li 8, 1"], ["add 7, 7, 8"], 128,
      ["std 7, 0(5)"])
    k("c_add_ind", "add in 4 independent chains", ["li 7, 0", "li 8, 1", "li 9, 0", "li 10, 0", "li 12, 0"],
      ["add 7, 7, 8", "add 9, 9, 8", "add 10, 10, 8", "add 12, 12, 8"], 32,
      ["add 7, 7, 9", "add 7, 7, 10", "add 7, 7, 12", "std 7, 0(5)"])
    k("c_mulld", "mulld dependent", ["li 7, 3", "li 8, 1"], ["mulld 7, 7, 8"], 64, ["std 7, 0(5)"])
    k("c_fadd", "fadd dependent", ["lfd 1, 0(4)", "lfd 2, 8(4)"], ["fadd 1, 1, 2"], 128,
      ["stfd 1, 0(5)"])
    k("c_fmadd", "fmadd dependent", ["lfd 1, 0(4)", "lfd 2, 8(4)", "lfd 3, 16(4)"],
      ["fmadd 1, 1, 3, 2"], 128, ["stfd 1, 0(5)"])
    k("c_loop", "empty bdnz loop", [], [], 1, ["li 7, 0", "std 7, 0(5)"])
    k("c_nop", "nop (ori 0 0 0) x128", [], ["nop"], 128, ["li 7, 0", "std 7, 0(5)"])
    # Store-to-load round trips through the 16 bytes at out+16, per step.
    # vinsn() lines start with a tab; strip it for this list.
    v = lambda m, *o: vinsn(m, *o).lstrip("\t")
    zero = ["li 7, 0", "std 7, 0(5)"]
    k("c_lhs_gpr", "stw then dependent lwz then addi (per step)", ["li 7, 0"],
      ["stw 7, 16(5)", "lwz 7, 16(5)", "addi 7, 7, 1"], 32, ["std 7, 0(5)"], steps=True)
    k("c_lhs_fpr", "stfd then dependent lfd (per step)", ["lfd 1, 0(4)"],
      ["stfd 1, 16(5)", "lfd 1, 16(5)"], 32, zero, steps=True)
    k("c_lhs_vec", "stvx128 then dependent lvx128 (per step)", ["li 11, 16", v("lvx128", 1, 4, 11)],
      [v("stvx128", 1, 5, 11), v("lvx128", 1, 5, 11)], 32, zero, steps=True)
    k("c_lhs_vec_cls", "stvx then dependent lvx (per step)", ["li 11, 16", v("lvx", 1, 4, 11)],
      [v("stvx", 1, 5, 11), v("lvx", 1, 5, 11)], 32, zero, steps=True)
    k("c_v2g2v", "stvx128 lwz stw lvx128 round trip (per step)", ["li 11, 16", v("lvx128", 1, 4, 11)],
      [v("stvx128", 1, 5, 11), "lwz 7, 16(5)", "stw 7, 16(5)", v("lvx128", 1, 5, 11)], 32, zero,
      steps=True)
    k("c_g2f2g", "stw lfs stfs lwz round trip (per step)", ["li 7, 0"],
      ["stw 7, 16(5)", "lfs 1, 16(5)", "stfs 1, 16(5)", "lwz 7, 16(5)"], 32, zero, steps=True)
    k("c_vsplat_g", "stw lvx128 vspltw128 stvx128 lwz round trip (per step)",
      ["li 7, 0", "li 11, 16"],
      ["stw 7, 16(5)", v("lvx128", 1, 5, 11), v("vspltw128", 2, 1, 0), v("stvx128", 2, 5, 11),
       "lwz 7, 16(5)"], 16, zero, steps=True)
    # Halfword loads: a pointer-chasing chain through the halfword 16 at
    # out+16 (latency), and eight independent loads (throughput).
    hw_setup = ["li 7, 16", "sth 7, 16(5)"]
    k("c_lhz_chain", "lhzx dependent through the loaded offset (per load)", hw_setup,
      ["lhzx 7, 5, 7"], 64, ["std 7, 0(5)"], steps=True)
    k("c_lha_chain", "lhax dependent through the loaded offset (per load)", hw_setup,
      ["lhax 7, 5, 7"], 64, ["std 7, 0(5)"], steps=True)
    k("c_lhz_extsh_chain", "lhzx then extsh dependent (per load)", hw_setup,
      ["lhzx 7, 5, 7", "extsh 7, 7"], 64, ["std 7, 0(5)"], steps=True)
    k("c_lwz_chain", "lwzx dependent through the loaded offset (per load)",
      ["li 7, 16", "stw 7, 16(5)"], ["lwzx 7, 5, 7"], 64, ["std 7, 0(5)"], steps=True)
    regs8 = [7, 8, 9, 10, 11, 12, 0, 6]
    k("c_lha_thr", "8 independent lha (per load)", hw_setup,
      ["lha %d, 16(5)" % r for r in regs8], 16, ["li 7, 0", "std 7, 0(5)"])
    k("c_lhz_extsh_thr", "8 independent lhz+extsh (per instruction)", hw_setup,
      ["lhz %d, 16(5)" % r for r in regs8] + ["extsh %d, %d" % (r, r) for r in regs8], 16,
      ["li 7, 0", "std 7, 0(5)"])
    k("c_lhz_thr", "8 independent lhz (per load)", hw_setup,
      ["lhz %d, 16(5)" % r for r in regs8], 16, ["li 7, 0", "std 7, 0(5)"])



SAVE_V = list(range(20, 32)) + list(range(64, 128))
FRAME = 1664


def gen_trampoline():
    """vb_call(ctx): see harness.rs CallCtx. Saves r14-r31, f14-f31, v20-v31
    and v64-v127, calls ctx.func ctx.reps times with r3-r8 and f1/f2 from
    ctx, and records the timebase before and after."""
    name = "vb_call"
    func_begin(name)
    a = ASM.append
    a("\tmflr 0")
    a("\tstw 0, -8(1)")
    a("\tstwu 1, -%d(1)" % FRAME)
    # 0..0x60: back chain, scratch and the callee's parameter home area
    for i, g in enumerate(range(14, 32)):
        a("\tstd %d, %d(1)" % (g, 0x60 + 8 * i))
    for i, f in enumerate(range(14, 32)):
        a("\tstfd %d, %d(1)" % (f, 0x60 + 144 + 8 * i))
    vbase = 0x60 + 288
    a("\taddi 11, 1, %d" % (vbase + 15))
    a("\tclrrwi 11, 11, 4")
    for i, v in enumerate(SAVE_V):
        a("\tli 12, %d" % (16 * i))
        a(vinsn("stvx128", v, 11, 12))
    assert vbase + 16 + 16 * len(SAVE_V) <= FRAME
    # The kernels do not follow the calling convention (they may use any
    # register but r1, r2 and r13), so the context pointer and the call
    # count live in the frame.
    a("\tstw 3, 0x58(1)")
    a("\tlwz 0, 4(3)")
    a("\tstw 0, 0x5C(1)")
    a("\tisync")
    a("\tmftb 0")
    a("\tstd 0, 64(3)")
    a("3:")
    a("\tlwz 12, 0x58(1)")
    a("\tlwz 0, 0(12)")
    a("\tmtctr 0")
    a("\tlwz 3, 8(12)")
    a("\tlwz 4, 12(12)")
    a("\tlwz 5, 16(12)")
    a("\tlwz 6, 20(12)")
    a("\tlwz 7, 24(12)")
    a("\tlwz 8, 28(12)")
    a("\tlfd 1, 40(12)")
    a("\tlfd 2, 48(12)")
    a("\tbctrl")
    a("\tlwz 12, 0x5C(1)")
    a("\taddic. 12, 12, -1")
    a("\tstw 12, 0x5C(1)")
    a("\tbne 3b")
    a("\tmftb 0")
    a("\tlwz 12, 0x58(1)")
    a("\tstd 0, 72(12)")
    a("\tstw 3, 32(12)")
    a("\tstfd 1, 56(12)")
    a("\taddi 11, 1, %d" % (vbase + 15))
    a("\tclrrwi 11, 11, 4")
    for i, v in enumerate(SAVE_V):
        a("\tli 12, %d" % (16 * i))
        a(vinsn("lvx128", v, 11, 12))
    for i, g in enumerate(range(14, 32)):
        a("\tld %d, %d(1)" % (g, 0x60 + 8 * i))
    for i, f in enumerate(range(14, 32)):
        a("\tlfd %d, %d(1)" % (f, 0x60 + 144 + 8 * i))
    a("\taddi 1, 1, %d" % FRAME)
    a("\tlwz 0, -8(1)")
    a("\tmtlr 0")
    func_end(name)

    func_begin("vb_mftb")
    a("\tmftb 4")
    a("\tstd 4, 0(3)")
    func_end("vb_mftb")




def rust_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def write_outputs(out):
    with open(os.path.join(out, "kernels.s"), "w") as f:
        f.write("# Generated by gen/gen_kernels.py. Do not edit.\n")
        f.write("\n".join(ASM))
        f.write("\n\t.text\n")
    o = []
    w = o.append
    w("// Generated by gen/gen_kernels.py. Do not edit.")
    w("")
    w("use crate::harness::KernelFn;")
    w("")
    syms = [k["name"] for k in REG_KERNELS] + [k["name"] for k in MEM_KERNELS] + \
        [x["name"] for x in XFORMS] + [c["name"] for c in CLOCKS] + [p["f"] for p in PROBES] + [q["name"] for q in QPROBES]
    w("unsafe extern \"C\" {")
    for s in syms:
        w("    fn %s();" % s)
    w("}")
    w("")
    w("#[repr(C, align(16))]")
    w("pub struct Words<const N: usize>(pub [u32; N]);")
    w("")
    w("pub static INPUT: Words<%d> = Words([" % len(INPUT))
    for i in range(0, len(INPUT), 8):
        w("    " + " ".join("0x%08X," % x for x in INPUT[i:i + 8]))
    w("]);")
    w("pub static EXPECT: Words<%d> = Words([" % len(EXPECT))
    for i in range(0, len(EXPECT), 8):
        w("    " + " ".join("0x%08X," % x for x in EXPECT[i:i + 8]))
    w("]);")
    w("")
    w("pub struct RegKernel {")
    w("    pub name: &'static str,")
    w("    pub op: &'static str,")
    w("    pub group: &'static str,")
    w("    pub variant: &'static str,")
    w("    /// \"lat\", \"thr\" or \"chk\".")
    w("    pub kind: &'static str,")
    w("    pub f: KernelFn,")
    w("    pub chains: u32,")
    w("    /// Instructions in one chain step.")
    w("    pub seq: u32,")
    w("    /// Vector instructions in one loop iteration.")
    w("    pub body: u32,")
    w("    pub input: usize,")
    w("    pub inputs: usize,")
    w("    pub outputs: usize,")
    w("    pub expect: usize,")
    w("    pub check_iters: u32,")
    w("    /// Relative tolerance per float lane; 0 means bit-exact.")
    w("    pub tol: f32,")
    w("    /// \"instr\": report per instruction; \"seq\": per chain step.")
    w("    pub per: &'static str,")
    w("}")
    w("")
    w("pub static REG_KERNELS: &[RegKernel] = &[")
    for k in REG_KERNELS:
        nin = k["out_vecs"]  # not used; inputs computed below
        w("    RegKernel { name: %s, op: %s, group: %s, variant: %s, kind: %s, f: %s, chains: %d, seq: %d, body: %d, input: %d, inputs: 0, outputs: %d, expect: %d, check_iters: %d, tol: %r, per: %s }," % (
            rust_str(k["name"]), rust_str(k["op"]), rust_str(k["group"]), rust_str(k["variant"]),
            rust_str(k["kind"]), k["name"], k["chains"], k["seq"], k["body"], k["input"],
            k["out_vecs"], k["expect"], k["check_iters"],
            float(k["tol"] or 0.0), rust_str(k["per"])))
    w("];")
    w("")
    w("pub struct MemKernel {")
    w("    pub name: &'static str,")
    w("    pub group: &'static str,")
    w("    /// read, readgpr, readlast, readlastf, write, copy")
    w("    pub kind: &'static str,")
    w("    pub variant: &'static str,")
    w("    pub f: KernelFn,")
    w("    pub src_off: usize,")
    w("    pub dst_off: usize,")
    w("    /// Write pattern period in bytes taken from the start of src; 0 = zeros.")
    w("    pub pattern: usize,")
    w("    /// Instructions per 128-byte line.")
    w("    pub insns: u32,")
    w("}")
    w("")
    w("pub static MEM_KERNELS: &[MemKernel] = &[")
    for k in MEM_KERNELS:
        w("    MemKernel { name: %s, group: %s, kind: %s, variant: %s, f: %s, src_off: %d, dst_off: %d, pattern: %d, insns: %d }," % (
            rust_str(k["name"]), rust_str(k["group"]), rust_str(k["kind"]), rust_str(k["variant"]),
            k["name"], k["src_off"], k["dst_off"], k["pattern"], k["insns"]))
    w("];")
    w("")
    w("pub struct Xform {")
    w("    pub name: &'static str,")
    w("    pub form: &'static str,")
    w("    pub unroll: u32,")
    w("    pub regs: &'static str,")
    w("    /// The matrix vectors are rows (dot form) rather than columns.")
    w("    pub rowmajor: bool,")
    w("    pub f: KernelFn,")
    w("}")
    w("")
    w("pub static XFORMS: &[Xform] = &[")
    for x in XFORMS:
        w("    Xform { name: %s, form: %s, unroll: %d, regs: %s, rowmajor: %s, f: %s }," % (
            rust_str(x["name"]), rust_str(x["form"]), x["u"], rust_str(x["regs"]),
            "true" if x["rowmajor"] else "false", x["name"]))
    w("];")
    w("")
    w("pub struct Clock {")
    w("    pub name: &'static str,")
    w("    pub desc: &'static str,")
    w("    pub ops: u32,")
    w("    pub f: KernelFn,")
    w("}")
    w("")
    w("pub static CLOCKS: &[Clock] = &[")
    for c in CLOCKS:
        w("    Clock { name: %s, desc: %s, ops: %d, f: %s }," % (
            rust_str(c["name"]), rust_str(c["desc"]), c["ops"], c["name"]))
    w("];")
    w("")
    w("pub struct Probe {")
    w("    pub name: &'static str,")
    w("    pub desc: &'static str,")
    w("    pub input: usize,")
    w("    pub inputs: usize,")
    w("    pub expect: usize,")
    w("    pub f: KernelFn,")
    w("}")
    w("")
    w("pub struct QProbe {")
    w("    pub name: &'static str,")
    w("    pub f: KernelFn,")
    w("}")
    w("")
    w("pub static QPROBES: &[QProbe] = &[")
    for q in QPROBES:
        w("    QProbe { name: %s, f: %s }," % (rust_str(q["name"]), q["name"]))
    w("];")
    w("")
    w("pub static PROBES: &[Probe] = &[")
    for p in PROBES:
        w("    Probe { name: %s, desc: %s, input: %d, inputs: %d, expect: %d, f: %s }," % (
            rust_str(p["name"]), rust_str(p["desc"]), p["input"], p["ninputs"], p["expect"], p["f"]))
    w("];")
    with open(os.path.join(out, "kernels.rs"), "w") as f:
        f.write("\n".join(o) + "\n")


def main():
    out = sys.argv[1]
    os.makedirs(out, exist_ok=True)
    gen_trampoline()
    gen_clock()
    gen_reg()
    gen_probes()
    gen_qprobes()
    gen_mem()
    gen_xforms()
    write_outputs(out)
    print("reg kernels %d, mem kernels %d, xforms %d, probes %d, asm lines %d" % (
        len(REG_KERNELS), len(MEM_KERNELS), len(XFORMS), len(PROBES), len(ASM)))


if __name__ == "__main__":
    main()
