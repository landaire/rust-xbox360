#!/usr/bin/env bash
# Regenerates the compiler-comparison assembly in ../src/ext/:
#   c_<p>.s  clang output of kernels.c (the 44 auto-vectorization kernels)
#            or speed.c, one file per configuration (see gen/gen_ext.py)
#   r_<p>.s  rustc output of kernels.rs (Buck libraries kernels_<p>,
#            subtarget [static][asm])
# Symbols carry the prefix "<p>_". make_sdk.sh compiles the same kernels.c
# with the SDK compiler and generates project-written runtime helpers.
# Set SDK (installation root), WINE and optionally WINEPREFIX for that step.
#
# Every file goes through fixup.py so local labels get a per-file prefix.
# All included files share one assembler scope.
#
#   CLANG=/path/to/clang JOBS=8 examples/vmx128bench/cmp/make.sh
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../.." && pwd)
dest=$here/../src/ext
CLANG=${CLANG:-/var/tmp/llvm-vmx128-build/bin/clang}
mkdir -p "$dest"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

base=(--target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions
  -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0)

# prefix : source : flags
configs=(
  "l2v:kernels.c:-O2"
  "l3v:kernels.c:-O3"
  "lfv:kernels.c:-O3 -ffast-math"
  "l3a:kernels.c:-O3 -mno-vmx128"
  "lfa:kernels.c:-O3 -ffast-math -mno-vmx128"
  "l3s:kernels.c:-O3 -mno-altivec"
  "cv:speed.c:-O3 -ffast-math"
  "ca:speed.c:-O3 -ffast-math -mno-vmx128"
  "cs:speed.c:-O3 -ffast-math -mno-altivec"
)

for c in "${configs[@]}"; do
  IFS=: read -r p src flags <<<"$c"
  # rename every external kernel and global to <p>_<name>
  renames=$(grep -oE '^(void|float|int) ((k|sp)_[a-z0-9_]+)\(|^ALIGN16 [a-z]+ (g_[a-z]+)\[|^MAT4_BLOCK\(([0-9]+)\)' "$here/$src" |
    sed -E 's/^MAT4_BLOCK\(([0-9]+)\)/sp_mat4_v\1/' | grep -oE '(k|sp|g)_[a-z0-9_]+' | sort -u | sed "s/.*/-D&=${p}_&/")
  # shellcheck disable=SC2086
  "$CLANG" "${base[@]}" $flags $renames -S -o "$tmp/c_$p.s" "$here/$src"
  python3 "$here/fixup.py" "$p" "$tmp/c_$p.s" "$dest/c_$p.s" \
    "$src: $("$CLANG" --version | head -1)" "flags: ${base[*]} $flags"
done

for p in rv ra rs; do
  asm=$(cd "$root" && bin/buck2 build -j "${JOBS:-8}" --target-platforms xbox360//platforms:xbox360-ppc64-ilp32-std \
    "//examples/vmx128bench:kernels_$p[static][asm]" --show-simple-output 2>/dev/null)
  python3 "$here/fixup.py" "$p" "$root/$asm" "$dest/r_$p.s" \
    "kernels.rs: rustc of the PPC64 toolchain (rustc_bootstrap cell)" "Buck target //examples/vmx128bench:kernels_$p"
done

"$here/make_sdk.sh" "$dest"
