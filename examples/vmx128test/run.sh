#!/usr/bin/env bash
# Build and run the PPC64 SIMD and vector ABI tests on a development kit.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
X=${XEEDEE:-$ROOT/../xeedee/target/debug/xeedee}
H=${XBOX:-192.168.1.27}
OUT=${1:-/tmp/vmx128test-run}
DIR='DEVKIT:\vmx128test'
mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
cd "$ROOT"
XEX=$(bin/buck2 build -j "${JOBS:-8}" --target-platforms xbox360//platforms:ppc64 \
  //examples/vmx128test:vmx128test --show-simple-output)
exec 9>/var/tmp/xbox360-console.lock
flock 9
"$X" -H "$H" file mkdir "$DIR" >/dev/null 2>&1 || true
"$X" -H "$H" file rm "$DIR\results.txt" >/dev/null 2>&1 || true
"$X" -H "$H" --no-progress file put "$XEX" "$DIR\default.xex" >/dev/null
"$X" -H "$H" pix-notify >"$OUT/debug.log" 2>&1 &
notify=$!
trap 'kill "$notify" 2>/dev/null || true; wait "$notify" 2>/dev/null || true' EXIT
sleep 1
"$X" -H "$H" magicboot --title "$DIR\default.xex" --directory "$DIR" >/dev/null
start=$(date +%s)
until grep -aq 'vmx128test: done' "$OUT/debug.log"; do
  if grep -aq 'exception code=' "$OUT/debug.log" || \
     [ $(($(date +%s) - start)) -gt "${TIMEOUT:-180}" ]; then
    cat "$OUT/debug.log" >&2
    "$X" -H "$H" reboot >/dev/null || true
    exit 1
  fi
  sleep 1
done
"$X" -H "$H" --no-progress file get -o "$OUT/results.txt" "$DIR\results.txt" >/dev/null
cat "$OUT/results.txt"
grep -qE '^vmx128test: [0-9]+ passed, 0 failed$' "$OUT/results.txt"
