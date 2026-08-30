#!/usr/bin/env bash
# Apply the xbox360 std backend into the active rustc's rust-src component.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
std="$(rustc --print sysroot)/lib/rustlib/src/rust/library/std"
[ -d "$std" ] || { echo "rust-src not found; run: rustup component add rust-src"; exit 1; }

mkdir -p "$std/src/sys/pal/xbox360"
cp "$here/src/sys/pal/xbox360/mod.rs"    "$std/src/sys/pal/xbox360/mod.rs"
cp "$here/src/sys/pal/xbox360/common.rs" "$std/src/sys/pal/xbox360/common.rs"
cp "$here/src/sys/pal/xbox360/abi.rs"    "$std/src/sys/pal/xbox360/abi.rs"
cp "$here/src/sys/alloc/xbox360.rs"      "$std/src/sys/alloc/xbox360.rs"
cp "$here/src/sys/stdio/xbox360.rs"      "$std/src/sys/stdio/xbox360.rs"

# Apply the additive dispatch/build.rs arms if not already present.
if ! grep -q 'target_os = "xbox360"' "$std/src/sys/alloc/mod.rs"; then
    ( cd "$std/../.." && patch -p1 < "$here/dispatch-edits.patch" )
else
    echo "dispatch edits already present; skipping patch"
fi
echo "xbox360 std backend applied to $std"
