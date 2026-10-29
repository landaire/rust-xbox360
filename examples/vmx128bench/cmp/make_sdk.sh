#!/usr/bin/env bash
# Compile our kernels with the SDK compiler under Wine and convert the COFF.
# Set SDK to the installation root or CLEXE to cl.exe; WINE selects Wine.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
dest=${1:-$here/../src/ext}
mkdir -p "$dest"
dest=$(cd "$dest" && pwd)
if [ -z "${CLEXE:-}" ]; then
  : "${SDK:?Set SDK to the Xbox 360 SDK installation root or set CLEXE}"
  CLEXE=$SDK/bin/win32/cl.exe
fi
WINE=${WINE:-wine}
export WINEDEBUG=-all
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
cp "$here/kernels.c" "$tmp/kernels.c"
cd "$tmp"
"$WINE" "$CLEXE" >version.txt 2>&1 || true
if ! "$WINE" "$CLEXE" /nologo /c /O2 /FAs /Fakernels.asm /Fokernels.obj kernels.c >compile.log 2>&1; then
  cat compile.log >&2
  exit 1
fi
python3 "$here/coff2gas.py" kernels.obj -o kernels.s --prefix msvc_
{
  python3 -c 'import pathlib, sys; print("# kernels.c: " + next(s for s in pathlib.Path(sys.argv[1]).read_text().splitlines() if "Compiler Version" in s) + " /O2")' version.txt
  cat kernels.s
} >"$dest/msvc_kernels.s"
python3 "$here/gen_runtime.py" "$dest/msvc_runtime.s"
