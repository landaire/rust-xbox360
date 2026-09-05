# Why ELF-to-XEX stays an external step

The toolchain compiles to a big-endian PowerPC ELF, then `xex-link` turns that
ELF into a devkit-signed XEX in one post-link step. A natural question is
whether the ELF-to-image conversion could move into LLVM itself, so rustc/clang
emit the final image directly. This documents the investigation and the answer:
it cannot be done cleanly, and it does not fit upstream LLVM. Keep it external.

## What "eliminating the ELF step" would mean

LLVM already emits a fully-linked ELF via rust-lld. Removing the separate
`xex-link` invocation means one of two things must gain Xbox 360 output:

- lld's COFF/PE backend emits a big-endian PowerPC PE directly, or
- llvm-objcopy converts the ELF into an Xbox PE (or XEX).

Both are blocked by the same facts.

## lld COFF backend

lld's PE linker supports x86, x64, ARM, ARM64 (and MIPS) only. There is no
PowerPC of any kind:

- `llvm::getMachineType()` (`llvm/lib/Object/WindowsMachineFlag.cpp`) is a
  `StringSwitch` over x64/x86/arm/arm64/arm64ec/arm64x/mips; anything else is
  `IMAGE_FILE_MACHINE_UNKNOWN`, and `/machine:powerpc` hits a fatal error in
  `lld/COFF/Driver.cpp`.
- The COFF header defines `IMAGE_FILE_MACHINE_POWERPC` (0x1F0) and `POWERPCFP`
  (0x1F1) in `llvm/include/llvm/BinaryFormat/COFF.h`, but there is no
  `POWERPCBE` (0x1F2) constant and no code consumes any PPC value.
- Relocation dispatch (`lld/COFF/Chunks.cpp`) switches on arch over
  x86_64/x86/thumb/aarch64/mipsel and `llvm_unreachable`s the default. There is
  no `IMAGE_REL_PPC_*` applier anywhere in `lld/COFF/`.
- PE writing is hardwired little-endian: `lld/COFF/Writer.cpp` uses `write32le`
  and `support::ulittle16_t/ulittle32_t`, and every on-disk PE/COFF struct in
  the shared headers is declared `support::ulittle*`. Emitting a big-endian PE
  means either a parallel big-endian struct set or templating every writer on
  endianness.

Adding BE PPC PE output would mean: a new machine constant; teaching
`getMachineType`/`machineToStr`/`getMachineArchType`; a full `applyRelPPC` for
the Xenon relocation set; big-endian header emission (the invasive part, since
the shared `llvm/Object` and `llvm/BinaryFormat` COFF structs assume
little-endian and are used by many tools); and then XEX import-record synthesis,
which is not a PE concept at all.

## llvm-objcopy

llvm-objcopy has no ELF-to-COFF path. `objcopy::executeObjcopyOnBinary`
(`llvm/lib/ObjCopy/ObjCopy.cpp`) dispatches on the input format and preserves it
on output: ELF in yields ELF out. The only cross-format outputs are `binary`,
`ihex`, and `srec`, and they exist only on the ELF path
(`llvm/lib/ObjCopy/ELF/ELFObjcopy.cpp`). Output-target selection only accepts
`elf*` prefixes. The COFF objcopy writer is little-endian as well. Adding
XEX/Xbox-PE output would require both a new writer and a new
format-crossing dispatch the tool is deliberately built not to have.

## The idiomatic pattern

Producing a proprietary, signed console image from an ELF via an external
post-link tool is exactly how LLVM-based toolchains do this. The only
final-image transforms LLVM ships are the raw dumps (`-O binary/ihex/srec`);
in-tree targets otherwise stop at standard containers (ELF/COFF/MachO/XCOFF/
Wasm). There is no in-tree precedent for a signed console container (no XEX,
NSO/NRO, DOL/REL, VITA SELF, etc.) -- those all live in external tools
(`make_fself`, `elf2rpl`, `elf2dol`). The whole `llvm/`+`lld/` tree has no Xbox
360 support; the only "xbox" hit is the original x86 `IMAGE_SUBSYSTEM_XBOX`
constant.

## Upstream acceptability

Adding this to lld or llvm-objcopy is effectively out of scope. The COFF backend
is a tightly scoped Windows PE linker with a single maintainer; a big-endian
PowerPC console variant with XEX semantics is far outside its charter. The real
cost is that BE PPC PE support would have to mutate the shared COFF structs used
by objcopy, llvm-readobj, MC, and LTO, purely to serve one dead console. XEX
also needs signing and crypto (the devkit key) that upstream would not host.

## Conclusion

Keep ELF-to-XEX as the external `xex-link` tool. It mirrors the universal
`ld -> objcopy/vendor tool -> sign` embedded/console pipeline. The one
upstream-shaped refinement worth keeping in mind: model `xex-link` after
`objcopy -O binary` semantics -- consume the `PT_LOAD` image, emit the container
-- rather than trying to fold it into LLVM. If tighter LLVM proximity is ever
wanted, the lowest-friction option is a standalone tool built on the
`llvm/Object` readers (as llvm-objcopy is a thin tool over the ObjCopy library),
which is a packaging choice, not elimination of the step.

Investigated against `~/src/llvm-project`. The claim that Microsoft's internal
Xbox 360 LLVM support was never upstreamed is context, not verifiable from the
tree; the tree today contains no remnant of it.
