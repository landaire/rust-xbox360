#!/usr/bin/env python3
"""Convert an Xbox 360 cl.exe COFF object (machine 0x1F2, PPCBE) to GNU
assembly that LLVM's integrated assembler turns into an ELF32 PowerPC
relocatable object, or that a Rust title includes with `global_asm!`.

Every unrelocated word is emitted as `.long` with its original value, so
the machine code is byte-identical. Relocated words are emitted as the
instruction (or data word) whose encoding differs from the original only
in the relocated field:

  COFF IMAGE_REL_PPC_REL24 (b/bl)         -> `b`/`bl`/`ba`/`bla` sym+A  (R_PPC_REL24)
  COFF IMAGE_REL_PPC_ADDR32               -> `.long sym+A`             (R_PPC_ADDR32)
  COFF IMAGE_REL_PPC_REFHI + PAIR         -> D-form insn `sym+A@ha`    (R_PPC_ADDR16_HA)
  COFF IMAGE_REL_PPC_REFLO (+ PAIR)       -> D-form insn `sym+A@l`     (R_PPC_ADDR16_LO)

Addends, as observed in cl.exe output:
  REL24: A = sign-extended LI field + offset of the word in its section
         (cl.exe stores -offset, i.e. a branch to the section start).
  ADDR32: A = the 32-bit word.
  REFHI: A = (hi16 field << 16) + signed 16-bit displacement in the PAIR
         record's symbol index field.
  REFLO: A = sign-extended lo16 field.
  cl.exe emitted only zero REFHI/REFLO addends in every probe, so a
  nonzero one is refused (check_addend) instead of guessed.

Sections kept: .text*, .rdata*, .data*, .bss*. Dropped: .drectve,
.debug$*, .XBLD$W, .pdata (function tables for unwinding; our ELF
images do not carry them). COMDAT sections become ELF COMDAT groups
named after their first external symbol, so several converted objects
can share `__real@...` constants.

Usage: coff2gas.py in.obj -o out.s [--prefix P] [--check-dumpbin dis]
  --prefix P   prepend P to every symbol defined in or referenced by the
               object except runtime helpers (keeps names apart from the
               LLVM build of the same kernels).
"""
import argparse
import struct
import sys

MACHINE_PPCBE = 0x1F2

R_ABSOLUTE, R_ADDR32, R_REL24 = 0x00, 0x02, 0x06
R_REFHI, R_REFLO, R_PAIR = 0x10, 0x11, 0x12
RELNAMES = {0x00: "ABSOLUTE", 0x01: "ADDR64", 0x02: "ADDR32", 0x03: "ADDR24",
            0x04: "ADDR16", 0x05: "ADDR14", 0x06: "REL24", 0x07: "REL14",
            0x0A: "ADDR32NB", 0x0B: "SECREL", 0x0C: "SECTION", 0x0F: "SECREL16",
            0x10: "REFHI", 0x11: "REFLO", 0x12: "PAIR", 0x13: "SECRELLO",
            0x15: "GPREL", 0x16: "TOKEN"}

SCN_CNT_CODE = 0x20
SCN_CNT_UNINIT = 0x80
SCN_LNK_COMDAT = 0x1000
SCN_MEM_WRITE = 0x80000000

# Primary opcode -> (mnemonic, form). "d": rD, X(rA). "ai": rD, rA, X.
# "ds": DS-form, XO in the low 2 bits. "log": rA, rS, X (ori/oris...).
D_FORMS = {
    12: ("addic", "ai"), 13: ("addic.", "ai"), 14: ("addi", "ai"), 15: ("addis", "ai"),
    24: ("ori", "log"), 25: ("oris", "log"), 26: ("xori", "log"), 27: ("xoris", "log"),
    32: ("lwz", "d"), 33: ("lwzu", "d"), 34: ("lbz", "d"), 35: ("lbzu", "d"),
    36: ("stw", "d"), 37: ("stwu", "d"), 38: ("stb", "d"), 39: ("stbu", "d"),
    40: ("lhz", "d"), 41: ("lhzu", "d"), 42: ("lha", "d"), 43: ("lhau", "d"),
    44: ("sth", "d"), 45: ("sthu", "d"), 46: ("lmw", "d"), 47: ("stmw", "d"),
    48: ("lfs", "d"), 49: ("lfsu", "d"), 50: ("lfd", "d"), 51: ("lfdu", "d"),
    52: ("stfs", "d"), 53: ("stfsu", "d"), 54: ("stfd", "d"), 55: ("stfdu", "d"),
    58: ({0: "ld", 1: "ldu", 2: "lwa"}, "ds"), 62: ({0: "std", 1: "stdu"}, "ds"),
}

# Symbols supplied by the runtime support file, never prefixed.
RUNTIME_PREFIXES = ("__savegprlr_", "__restgprlr_", "__savefpr_", "__restfpr_",
                    "__savevmx_", "__restvmx_", "_blkmov", "_fltused", "memset",
                    "memcpy", "memmove", "_RtlCheckStack12", "__C1_", "__C2_")


class Section:
    def __init__(self, index, name, flags, data, relocs):
        self.index, self.name, self.flags, self.data, self.relocs = index, name, flags, data, relocs
        self.comdat = None  # (selection, associated section)


class Symbol:
    def __init__(self, index, name, value, secnum, typ, sclass, aux):
        self.index, self.name, self.value, self.secnum = index, name, value, secnum
        self.typ, self.sclass, self.aux = typ, sclass, aux


def parse(blob):
    machine, nsec, _, symptr, nsym, opthdr, _ = struct.unpack_from("<HHIIIHH", blob, 0)
    if machine != MACHINE_PPCBE:
        sys.exit(f"machine {machine:#x} is not PPCBE (0x1F2)")
    strtab = blob[symptr + 18 * nsym:]

    def name_of(raw):
        if raw[:4] == b"\0\0\0\0":
            off = struct.unpack_from("<I", raw, 4)[0]
            return strtab[off:strtab.index(b"\0", off)].decode()
        return raw.rstrip(b"\0").decode()

    sections = [None]
    for i in range(nsec):
        off = 20 + opthdr + 40 * i
        raw = blob[off:off + 8]
        (vsize, vaddr, rawsize, rawptr, relptr, _, nrel, _, flags) = struct.unpack_from("<IIIIIIHHI", blob, off + 8)
        name = raw.rstrip(b"\0").decode()
        if name.startswith("/"):
            o = int(name[1:])
            name = strtab[o:strtab.index(b"\0", o)].decode()
        data = b"" if flags & SCN_CNT_UNINIT else blob[rawptr:rawptr + rawsize]
        relocs = [struct.unpack_from("<IIH", blob, relptr + 10 * r) for r in range(nrel)]
        s = Section(i + 1, name, flags, data, relocs)
        s.size = rawsize
        sections.append(s)

    symbols = {}
    i = 0
    while i < nsym:
        off = symptr + 18 * i
        raw = blob[off:off + 8]
        value, secnum, typ, sclass, naux = struct.unpack_from("<IhHBB", blob, off + 8)
        aux = blob[off + 18:off + 18 + 18 * naux]
        symbols[i] = Symbol(i, name_of(raw), value, secnum, typ, sclass, aux)
        i += 1 + naux
    # COMDAT selection from the section-definition aux record.
    for s in symbols.values():
        if s.sclass == 3 and s.aux and s.secnum > 0 and s.value == 0:
            sec = sections[s.secnum]
            if sec.flags & SCN_LNK_COMDAT and s.name == sec.name:
                length, nrel, nline, csum, number, sel = struct.unpack_from("<IHHIHB", s.aux, 0)
                sec.comdat = (sel, number)
    return sections, symbols


def sext(v, bits):
    m = 1 << (bits - 1)
    return (v & (m - 1)) - (v & m)


def quote(name):
    """LLVM's PowerPC operand parser does not take quoted names with @ha/@l,
    so characters outside [A-Za-z0-9_.] map to '_' (`__real@3f800000`
    becomes `__real_3f800000`)."""
    return "".join(c if c.isalnum() or c in "_." else "_" for c in name)


def expr(sym, addend):
    return sym if addend == 0 else f"{sym}{addend:+d}"


def d_form(word, target, kind):
    """Return the instruction text for a D/DS-form word whose immediate is
    replaced by target@kind."""
    op = word >> 26
    rt, ra = (word >> 21) & 31, (word >> 16) & 31
    if op not in D_FORMS:
        raise ValueError(f"REFHI/REFLO on unsupported opcode {op} (word {word:08x})")
    mn, form = D_FORMS[op]
    imm = f"{target}@{kind}"
    if form == "ds":
        xo = word & 3
        mn = mn[xo]
        if kind != "l":
            raise ValueError("DS-form with @ha")
        return f"{mn} {rt}, {imm}({ra})"
    if form == "d":
        return f"{mn} {rt}, {imm}({ra})"
    if form == "ai":
        return f"{mn} {rt}, {ra}, {imm}"
    return f"{mn} {ra}, {rt}, {imm}"  # log: rA is bits 11-15, rS bits 6-10


def check_addend(kind, addend, s, pos):
    """cl.exe output in our probes always has zero REFHI/REFLO addends (it
    materialises sym+0 and adds offsets separately), so the nonzero addend
    formula above is unverified. Refuse rather than guess."""
    if addend != 0:
        raise ValueError(f"{kind} with nonzero addend {addend:#x} at {s.name}+{pos:#x}: "
                         "addend encoding not verified against the SDK linker")


def is_section_symbol(sym, sections):
    return sym.sclass == 3 and sym.secnum > 0 and sym.value == 0 and sym.name == sections[sym.secnum].name


def section_label(idx, prefix):
    """Local (not .L) label at the start of a kept section, so that it is in
    the ELF symbol table."""
    return f"{prefix}__coffsec{idx}"


def elf_name(sym, sections, prefix):
    """ELF name of a COFF symbol. Externals keep their name (plus prefix,
    except runtime helpers and __real@ constants); statics and labels get
    unique local names."""
    if is_section_symbol(sym, sections):
        return section_label(sym.secnum, prefix)
    if sym.sclass == 2 or sym.secnum == 0:
        n = sym.name
        if prefix and not n.startswith(RUNTIME_PREFIXES) and not n.startswith("__real@"):
            n = prefix + n
        return quote(n)
    return quote(f"{prefix}__coffstatic_{sym.name}_{sym.index}")


def convert(blob, prefix=""):
    sections, symbols = parse(blob)
    out = []
    keep = {}
    for s in sections[1:]:
        n = s.name
        if n.startswith(".text") or n.startswith(".rdata") or n.startswith(".data") or n.startswith(".bss"):
            keep[s.index] = s

    def symname(sym):
        return elf_name(sym, sections, prefix)

    # Externals defined per section, by offset.
    defs = {}
    for sym in symbols.values():
        if sym.secnum > 0 and sym.secnum in keep and sym.sclass in (2, 3, 6):
            if sym.sclass == 3 and sym.value == 0 and sym.name == sections[sym.secnum].name:
                continue
            defs.setdefault(sym.secnum, []).append(sym)
    referenced = set()
    stats = {}

    out.append(f"# Converted from cl.exe COFF by coff2gas.py; do not edit.")
    for idx, s in keep.items():
        syms = sorted(defs.get(idx, []), key=lambda x: x.value)
        ext = [x for x in syms if x.sclass == 2]
        code = bool(s.flags & SCN_CNT_CODE)
        bss = bool(s.flags & SCN_CNT_UNINIT)
        align = 1 << (((s.flags >> 20) & 0xF) - 1) if (s.flags >> 20) & 0xF else 16
        base = ".text" if code else ".bss" if bss else ".data" if s.flags & SCN_MEM_WRITE else ".rodata"
        label = ext[0].name if ext else f"sec{idx}"
        secname = f"{base}.{prefix}{label}" if prefix or not label.startswith("__real@") else f"{base}.{label}"
        stype = "@nobits" if bss else "@progbits"
        sflags = "a" + ("x" if code else "") + ("w" if (bss or s.flags & SCN_MEM_WRITE) else "")
        out.append("")
        if s.comdat and s.comdat[0] == 2 and ext:  # pick any
            out.append(f'\t.section "{secname}","{sflags}G",{stype},{symname(ext[0])},comdat')
        else:
            out.append(f'\t.section "{secname}","{sflags}",{stype}')
        out.append(f"\t.p2align {align.bit_length() - 1}")
        out.append(f"{section_label(idx, prefix)}:")
        by_off = {}
        for x in syms:
            by_off.setdefault(x.value, []).append(x)
        relocs = {}
        rl = s.relocs
        i = 0
        while i < len(rl):
            off, symi, typ = rl[i]
            pair = None
            if i + 1 < len(rl) and rl[i + 1][2] == R_PAIR:
                pair = rl[i + 1]
                i += 1
            if typ != R_PAIR:
                relocs[off] = (symi, typ, pair)
            i += 1
        if bss:
            pos = 0
            for off in sorted(by_off):
                if off > pos:
                    out.append(f"\t.zero {off - pos}")
                    pos = off
                for x in by_off[off]:
                    emit_def(out, x, symname, code)
            if s.size > pos:
                out.append(f"\t.zero {s.size - pos}")
            continue
        data = s.data
        pos = 0
        while pos < len(data):
            for x in by_off.get(pos, []):
                emit_def(out, x, symname, code)
            if pos in relocs:
                symi, typ, pair = relocs[pos]
                word = struct.unpack_from(">I", data, pos)[0]
                target = symname(symbols[symi])
                referenced.add((target, symbols[symi]))
                stats[RELNAMES.get(typ, hex(typ))] = stats.get(RELNAMES.get(typ, hex(typ)), 0) + 1
                if typ == R_REL24:
                    if word >> 26 != 18:
                        raise ValueError(f"REL24 on non-branch word {word:08x}")
                    addend = sext(word & 0x03FFFFFC, 26) + pos
                    mn = "b" + ("l" if word & 1 else "") + ("a" if word & 2 else "")
                    out.append(f"\t{mn} {expr(target, addend)}")
                elif typ == R_ADDR32:
                    out.append(f"\t.long {expr(target, sext(word, 32))}")
                elif typ == R_REFHI:
                    if pair is None:
                        raise ValueError("REFHI without PAIR")
                    addend = ((word & 0xFFFF) << 16) + sext(pair[1], 16)
                    addend = sext(addend & 0xFFFFFFFF, 32)
                    check_addend("REFHI", addend, s, pos)
                    out.append(f"\t{d_form(word, expr(target, addend), 'ha')}")
                elif typ == R_REFLO:
                    addend = sext(word & 0xFFFF, 16)
                    check_addend("REFLO", addend, s, pos)
                    out.append(f"\t{d_form(word, expr(target, addend), 'l')}")
                else:
                    raise ValueError(f"unsupported relocation {RELNAMES.get(typ, typ)} at {s.name}+{pos:#x}")
                pos += 4
                continue
            if pos + 4 <= len(data) and pos % 4 == 0:
                out.append(f"\t.long 0x{struct.unpack_from('>I', data, pos)[0]:08X}")
                pos += 4
            else:
                out.append(f"\t.byte 0x{data[pos]:02X}")
                pos += 1
        for x in by_off.get(len(data), []):
            emit_def(out, x, symname, code)
        for x in ext:
            if code and x.typ & 0x20:  # function
                out.append(f"\t.size {symname(x)}, {section_label(idx, prefix)}+{len(data)}-{symname(x)}")
    # Undefined externals and COFF common symbols.
    for target, sym in sorted(referenced, key=lambda t: t[0]):
        if sym.secnum == 0 and sym.value == 0 and sym.sclass == 2:
            out.append(f"\t.globl {target}")
    for sym in symbols.values():
        if sym.secnum == 0 and sym.sclass == 2 and sym.value > 0:
            out.append(f"\t.comm {symname(sym)}, {sym.value}, 16")
    out.append("")
    return "\n".join(out), stats


def emit_def(out, x, symname, code):
    n = symname(x)
    if x.sclass == 2:
        out.append(f"\t.globl {n}")
        if code and x.typ & 0x20:
            out.append(f"\t.type {n}, @function")
        elif not code:
            out.append(f"\t.type {n}, @object")
    if x.sclass in (2, 3, 6):
        out.append(f"{n}:")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("obj")
    ap.add_argument("-o", "--output", required=True)
    ap.add_argument("--prefix", default="")
    ap.add_argument("--list-relocs", action="store_true")
    a = ap.parse_args()
    blob = open(a.obj, "rb").read()
    if a.list_relocs:
        sections, symbols = parse(blob)
        for s in sections[1:]:
            for off, si, typ in s.relocs:
                name = symbols[si].name if typ != R_PAIR else f"(disp {sext(si, 16)})"
                print(f"{s.index:3X} {s.name:10s} {off:08X} {RELNAMES.get(typ, hex(typ)):8s} {name}")
    text, stats = convert(blob, a.prefix)
    open(a.output, "w").write(text)
    print(" ".join(f"{k}={v}" for k, v in sorted(stats.items())), file=sys.stderr)


if __name__ == "__main__":
    main()
