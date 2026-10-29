#!/usr/bin/env bash
# Regenerates src/interop.s from interop.c with the console compiler
# (cl.exe 16.00.11886, /O2 /GS-) under Wine and tools/coff2gas.py.
#
#   WINE=wine SDK=/path/to/sdk examples/vmx128test/msvc/build.sh
#
# SDK is the directory that holds bin/win32/cl.exe and include/xbox.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../.." && pwd)
: "${SDK:?set SDK to the console SDK directory}"
WINE=${WINE:-wine}
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cp "$here/interop.c" "$work/"
include=$("$WINE" winepath -w "$SDK/include/xbox")
(cd "$work" && "$WINE" "$SDK/bin/win32/cl.exe" /nologo /c /O2 /GS- /I"$include" /Fointerop.obj interop.c)
python3 "$root/tools/coff2gas.py" "$work/interop.obj" -o "$work/interop.s"
{
  echo "# msvc/interop.c compiled by cl.exe 16.00.11886 /O2 /GS- and converted by"
  echo "# tools/coff2gas.py (msvc/build.sh). Every word is from interop.c; the"
  echo "# helpers it calls are in msvc_runtime.s."
  cat "$work/interop.s"
} >"$root/examples/vmx128test/src/interop.s"
