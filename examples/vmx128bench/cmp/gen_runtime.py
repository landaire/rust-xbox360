#!/usr/bin/env python3
"""Write msvc_runtime.s: the out-of-line helpers that cl.exe output calls,
written from observed compiler-generated call sites, not copied from an
SDK library.

  __savegprlr_N (N=14..31): std rK,-8-8*(32-K)(r1) for K=N..31, then
      stw r12,-8(r1) (r12 holds LR, set by the caller's mflr r12); blr.
  __restgprlr_N: ld rK from the same slots, lwz r12,-8(r1); mtlr r12; blr
      (the epilogue tail-branches to it, so it returns to the caller's
      caller).
  __savefpr_N / __restfpr_N: stfd/lfd fK,-8*(32-K)(r12) for K=N..31; blr.
      r12 is the base set by the caller (addi r12,r1,-(8+8*nGPR)).
  _blkmov(dst r3, src r4, bytes r5): forward copy, 4 bytes at a time when
      dst, src and bytes are multiples of 4, else bytewise. cl.exe relies
      on forward order: k_set_f32 stores v at d[0] and calls
      _blkmov(d+1, d, 4n-4) to replicate it. This is a project substitute;
      speed is not that of the SDK routine.

Each entry point falls through to the next one, so the helpers are one
straight sequence. 64-bit instructions are emitted as .long words so that
32-bit PowerPC assemblers accept the file.
"""
import sys


def ds(op, rs, d, ra):
    return (op << 26) | (rs << 21) | (ra << 16) | (d & 0xFFFC)


def d(op, rs, disp, ra):
    return (op << 26) | (rs << 21) | (ra << 16) | (disp & 0xFFFF)


def main(path):
    out = ["# MSVC-compatible runtime helpers for converted cl.exe code (gen_runtime.py).",
           "\t.section .text.msvc_runtime,\"ax\",@progbits", "\t.p2align 3"]
    # GPR save / restore
    for kind in ("save", "rest"):
        for n in range(14, 32):
            name = f"__{kind}gprlr_{n}"
            out += [f"\t.globl {name}", f"\t.type {name}, @function", f"{name}:"]
            off = -8 - 8 * (32 - n)
            word = ds(62, n, off, 1) if kind == "save" else ds(58, n, off, 1)
            mn = "std" if kind == "save" else "ld"
            out.append(f"\t.long 0x{word:08X}\t# {mn} r{n},{off}(r1)")
        if kind == "save":
            out += ["\tstw 12, -8(1)", "\tblr"]
        else:
            out += ["\tlwz 12, -8(1)", "\tmtlr 12", "\tblr"]
    # FPR save / restore
    for kind in ("save", "rest"):
        for n in range(14, 32):
            name = f"__{kind}fpr_{n}"
            out += [f"\t.globl {name}", f"\t.type {name}, @function", f"{name}:"]
            off = -8 * (32 - n)
            mn = "stfd" if kind == "save" else "lfd"
            out.append(f"\t{mn} {n}, {off}(12)")
        out.append("\tblr")
    out += [
        "\t.globl _blkmov", "\t.type _blkmov, @function", "_blkmov:",
        "\tcmplwi 5, 0", "\tbeqlr 0",
        "\tor 6, 3, 4", "\tor 6, 6, 5", "\tandi. 6, 6, 3", "\tbne 0, .Lblkmov_bytes",
        "\tsrwi 5, 5, 2", "\tmtctr 5", "\taddi 3, 3, -4", "\taddi 4, 4, -4",
        ".Lblkmov_words:", "\tlwzu 6, 4(4)", "\tstwu 6, 4(3)", "\tbdnz .Lblkmov_words", "\tblr",
        ".Lblkmov_bytes:", "\tmtctr 5", "\taddi 3, 3, -1", "\taddi 4, 4, -1",
        ".Lblkmov_b:", "\tlbzu 6, 1(4)", "\tstbu 6, 1(3)", "\tbdnz .Lblkmov_b", "\tblr",
        "",
    ]
    open(path, "w").write("\n".join(out))


if __name__ == "__main__":
    main(sys.argv[1])
