#!/usr/bin/env python3
"""Static instruction statistics of the compiler-comparison kernels in the
linked title.

    python3 gen/insn_stats.py ELF [OBJDUMP] > stats.csv

Disassembles ELF with OBJDUMP (default /var/tmp/llvm-vmx128-build/bin/llvm-objdump,
triple powerpc64-unknown-xbox360, +vmx128) and prints one CSV line per
kernel symbol (prefix msvc_, l*, c*, r*):

  name, insns, loop_insns, vec_classic, vec_128, loop_vec, max_vreg,
  lvsl_lvsr, vperm, lvlx_family, loop_stack_mem, scalar_fp, calls, unknown

loop_* count instructions inside backward-branch ranges. loop_stack_mem
counts loads and stores inside loops whose address is the stack pointer r1
or a register set from r1 with addi (the store-then-reload pattern).
max_vreg is the highest vector register named by a vector instruction.
"""

import re
import subprocess
import sys

PREFIX = re.compile(r'^(msvc|l2v|l3v|lfv|l3a|lfa|l3s|cv|ca|cs|rv|ra|rs)_(k|sp)_')
VEC_MEM = re.compile(r'^(lv|stv)')
SCALAR_FP = re.compile(r'^(lf|stf|f[a-z]+)')
LOADSTORE = re.compile(r'^(l[bhwdf]|st[bhwdf]|lv|stv|lwz|lhz|lha|lbz|ld|std|stw|sth|stb)')


def vector_regs(mn, ops):
    """Vector register numbers named by a vector instruction."""
    parts = [p.strip() for p in ops.split(',')]
    if VEC_MEM.match(mn):
        return [int(parts[0])] if parts and parts[0].isdigit() else []
    regs = []
    for p in parts:
        if p.isdigit():
            regs.append(int(p))
    # immediates of splat/shift/permute forms
    if mn.startswith(('vspltw', 'vsldoi', 'vcfp', 'vcsx', 'vcux', 'vctsxs', 'vctuxs', 'vcfsx', 'vcfux',
                      'vpermwi', 'vrlimi', 'vpkd3d', 'vupkd3d', 'vspltis', 'vsplth', 'vspltb')):
        n_regs = {'vspltisw128': 1, 'vspltisw': 1, 'vspltish': 1, 'vspltisb': 1}.get(mn, 2)
        regs = regs[:n_regs]
    if mn in ('vperm128',):
        regs = regs[:4]
    return regs


def main():
    elf = sys.argv[1]
    od = sys.argv[2] if len(sys.argv) > 2 else '/var/tmp/llvm-vmx128-build/bin/llvm-objdump'
    text = subprocess.run([od, '-d', '--no-show-raw-insn', '--triple=powerpc64-unknown-xbox360',
                           '--mattr=+vmx128', elf], capture_output=True, text=True, check=True).stdout
    # symbols sharing an address (identical functions folded by the
    # compiler) appear once in the disassembly; report every name
    nm = subprocess.run([od.replace('llvm-objdump', 'llvm-nm'), elf], capture_output=True, text=True,
                        check=True).stdout
    aliases = {}
    for line in nm.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[1] in 'Tt' and PREFIX.match(parts[2]):
            aliases.setdefault(int(parts[0], 16), []).append(parts[2])
    funcs = {}
    addr_of = {}
    cur = None
    for line in text.splitlines():
        m = re.match(r'^([0-9a-f]+) <([^>]+)>:', line)
        if m:
            names = aliases.get(int(m.group(1), 16), [])
            if not names and m.group(2).startswith('msvc___coff'):
                continue  # section-start symbol inside a converted SDK function
            cur = names[0] if names else None
            if cur:
                funcs[cur] = []
                addr_of[cur] = names
            continue
        if cur is None:
            continue
        m = re.match(r'^\s*([0-9a-f]+):\s+(\S+)\s*(.*)$', line)
        if m:
            funcs[cur].append((int(m.group(1), 16), m.group(2), m.group(3)))
    print('name,insns,loop_insns,vec_classic,vec_128,loop_vec,max_vreg,lvsl_lvsr,vperm,lvlx_family,'
          'loop_stack_mem,scalar_fp,calls,unknown')
    for name, ins in funcs.items():
        if not ins:
            continue
        lo, hi = ins[0][0], ins[-1][0]
        loops = []
        for pc, mn, ops in ins:
            if mn.startswith('b') and mn not in ('bl', 'blr', 'bctr', 'bctrl'):
                t = re.search(r'0x([0-9a-f]+)', ops)
                if t:
                    tgt = int(t.group(1), 16)
                    if lo <= tgt <= pc:
                        loops.append((tgt, pc))
        in_loop = lambda pc: any(a <= pc <= b for a, b in loops)
        stack_regs = {1}
        s = dict(insns=len(ins), loop_insns=0, vec_classic=0, vec_128=0, loop_vec=0, max_vreg=-1,
                 lvsl_lvsr=0, vperm=0, lvlx_family=0, loop_stack_mem=0, scalar_fp=0, calls=0, unknown=0)
        for pc, mn, ops in ins:
            loop = in_loop(pc)
            s['loop_insns'] += loop
            if mn == '<unknown>':
                s['unknown'] += 1
                continue
            m = re.match(r'addi (\d+), 1, ', mn + ' ' + ops)
            if m:
                stack_regs.add(int(m.group(1)))
            is_vec = (mn.startswith('v') or VEC_MEM.match(mn)) and mn not in ('vmr',)
            if is_vec:
                if mn.rstrip('.').endswith('128'):
                    s['vec_128'] += 1
                else:
                    s['vec_classic'] += 1
                s['loop_vec'] += loop
                regs = vector_regs(mn.rstrip('.'), ops)
                if regs:
                    s['max_vreg'] = max(s['max_vreg'], max(regs))
                base = mn.rstrip('.').replace('128', '')
                if base in ('lvsl', 'lvsr'):
                    s['lvsl_lvsr'] += 1
                if base == 'vperm':
                    s['vperm'] += 1
                if re.match(r'^(lv[lr]x|stv[lr]x)l?$', base):
                    s['lvlx_family'] += 1
            elif SCALAR_FP.match(mn):
                s['scalar_fp'] += 1
            if mn == 'bl':
                s['calls'] += 1
            if loop and LOADSTORE.match(mn):
                d = re.search(r'\((\d+)\)', ops)
                regs = [int(d.group(1))] if d else [int(x) for x in ops.split(',')[1:] if x.strip().isdigit()]
                if any(r in stack_regs for r in regs):
                    s['loop_stack_mem'] += 1
        for alias in addr_of[name]:
            print(','.join([alias] + [str(s[k]) for k in ('insns', 'loop_insns', 'vec_classic', 'vec_128',
                                                      'loop_vec', 'max_vreg', 'lvsl_lvsr', 'vperm',
                                                      'lvlx_family', 'loop_stack_mem', 'scalar_fp',
                                                      'calls', 'unknown')]))


if __name__ == '__main__':
    main()
