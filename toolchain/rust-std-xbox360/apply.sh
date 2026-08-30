#!/usr/bin/env bash
# Apply the xbox360 std + core-atomics backend into the active rustc's rust-src.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
rsrc="$(rustc --print sysroot)/lib/rustlib/src/rust"
std="$rsrc/library/std"
core="$rsrc/library/core"
[ -d "$std" ] || { echo "rust-src not found; run: rustup component add rust-src"; exit 1; }

# --- std sys backend files ---
mkdir -p "$std/src/sys/pal/xbox360"
cp "$here/src/sys/pal/xbox360/mod.rs"    "$std/src/sys/pal/xbox360/mod.rs"
cp "$here/src/sys/pal/xbox360/common.rs" "$std/src/sys/pal/xbox360/common.rs"
cp "$here/src/sys/pal/xbox360/abi.rs"    "$std/src/sys/pal/xbox360/abi.rs"
cp "$here/src/sys/alloc/xbox360.rs"      "$std/src/sys/alloc/xbox360.rs"
cp "$here/src/sys/stdio/xbox360.rs"      "$std/src/sys/stdio/xbox360.rs"

# --- std dispatch/build.rs arms (additive) ---
if ! grep -q 'target_os = "xbox360"' "$std/src/sys/alloc/mod.rs"; then
    ( cd "$rsrc" && patch -p1 < "$here/dispatch-edits.patch" )
else
    echo "std dispatch edits already present; skipping"
fi

# --- core protected-atomics module + helper redirects ---
cp "$here/src/core/sync/atomic_xbox360.rs" "$core/src/sync/atomic_xbox360.rs"
python3 - "$core/src/sync/atomic.rs" <<'PY'
import re, sys
p = sys.argv[1]
s = open(p).read()
if 'xbox360_prot' in s:
    print("core atomic edits already present; skipping")
    raise SystemExit(0)

# module declaration before the first helper (place before its attributes)
anchor = "#[inline]\n#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces\nunsafe fn atomic_store<T: Copy>"
decl = ('#[cfg(target_os = "xbox360")]\n#[path = "atomic_xbox360.rs"]\nmod xbox360_prot;\n\n')
assert anchor in s, "atomic_store anchor not found (version drift?)"
s = s.replace(anchor, decl + anchor, 1)

def inject(text, fn, code):
    m = re.search(r'unsafe fn ' + re.escape(fn) + r'<', text)
    if not m:
        raise SystemExit('helper not found: ' + fn)
    i = text.index('{', m.end())
    return text[:i+1] + '\n' + code + text[i+1:]

ops = {
    'atomic_swap': 'swap(dst, val)', 'atomic_add': 'add(dst, val)',
    'atomic_sub': 'sub(dst, val)', 'atomic_and': 'and(dst, val)',
    'atomic_nand': 'nand(dst, val)', 'atomic_or': 'or(dst, val)',
    'atomic_xor': 'xor(dst, val)', 'atomic_max': 'max(dst, val)',
    'atomic_min': 'min(dst, val)', 'atomic_umax': 'umax(dst, val)',
    'atomic_umin': 'umin(dst, val)',
}
for fn, call in ops.items():
    s = inject(s, fn, f'    #[cfg(target_os = "xbox360")]\n    return unsafe {{ xbox360_prot::{call} }};\n')
for fn in ('atomic_compare_exchange', 'atomic_compare_exchange_weak'):
    s = inject(s, fn,
        '    #[cfg(target_os = "xbox360")]\n'
        '    return {\n'
        '        let (v, ok) = unsafe { xbox360_prot::compare_exchange(dst, old, new) };\n'
        '        if ok { Ok(v) } else { Err(v) }\n'
        '    };\n')
open(p, 'w').write(s)
print("core atomic edits applied")
PY

echo "xbox360 std + core-atomics backend applied to $rsrc"
