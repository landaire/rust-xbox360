#!/usr/bin/env bash
# Build a C program into an Xbox 360 XEX, proving the toolchain is not
# Rust-specific: any LLVM frontend (clang here) emits a big-endian PowerPC ELF,
# which the single xex-link step turns into a devkit-signed XEX.
#
#   C --clang--> ppc BE ELF obj --rust-lld--> ELF exe --xex-link--> XEX
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
lld="$(ls ~/.rustup/toolchains/*/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld | head -1)"
out="${1:-$here/chello.xex}"

nix-shell -p llvmPackages.clang-unwrapped --run \
  "clang --target=powerpc-unknown-none-eabi -ffreestanding -nostdlib -fno-builtin -O2 -c '$here/hello.c' -o '$here/hello.o'"

"$lld" -flavor gnu "$here/hello.o" -T "$here/link.x" -o "$here/chello" --no-rosegment -e _start

"$root/target/debug/xex-link" "$here/chello" "$out" --manifest "$here/xex.toml"
echo "built $out"
