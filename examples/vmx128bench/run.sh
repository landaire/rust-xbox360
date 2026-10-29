#!/usr/bin/env bash
# Build vmx128bench for the PPC64 toolchain, run it on a devkit and fetch
# the results.
#
#   examples/vmx128bench/run.sh [OUTDIR] [CONFIG-LINES...]
#
# OUTDIR defaults to /tmp/vmx128bench. CONFIG-LINES are written to
# DEVKIT:\vmx128bench\config.txt, for example 'sections=reg,mem' 'hw=4'
# 'only=vaddfp|m_rd_'. Environment: XEEDEE (client binary), XBOX (console
# address), TIMEOUT (seconds to wait for done.txt, default 900), JOBS (Buck
# jobs, default 8). The console session holds /var/tmp/xbox360-console.lock
# (flock), which other users of the devkit share.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
XB=${XEEDEE:-$HOME/dev/xeedee/target/debug/xeedee}
# every console command has a deadline, so a hung console cannot hold the lock
X="timeout 30 $XB"
H=${XBOX:-192.168.1.27}
OUT=${1:-/tmp/vmx128bench}
shift || true
DIR='DEVKIT:\vmx128bench'
mkdir -p "$OUT"
cd "$ROOT"

XEX=$(bin/buck2 build -j "${JOBS:-8}" --target-platforms xbox360//platforms:ppc64 \
  //examples/vmx128bench:vmx128bench --show-simple-output)

exec 9>/var/tmp/xbox360-console.lock
flock 9

$X -H "$H" file mkdir "$DIR" >/dev/null 2>&1 || true
$X -H "$H" file rm "$DIR\\done.txt" >/dev/null 2>&1 || true
$X -H "$H" file rm "$DIR\\results.csv" >/dev/null 2>&1 || true
$X -H "$H" file rm "$DIR\\config.txt" >/dev/null 2>&1 || true
if [ $# -gt 0 ]; then
  printf '%s\n' "$@" >"$OUT/config.txt"
  $X -H "$H" --no-progress file put "$OUT/config.txt" "$DIR\\config.txt" >/dev/null
fi
$X -H "$H" --no-progress file put "$XEX" "$DIR\\default.xex" >/dev/null

# Debug output (one line per result) goes to debug.log as a second channel.
$XB -H "$H" pix-notify >"$OUT/debug.log" 2>&1 &
NOTIFY=$!
trap 'kill $NOTIFY 2>/dev/null || true' EXIT

$X -H "$H" magicboot --title "$DIR\\default.xex" --directory "$DIR" >/dev/null
start=$(date +%s)
bad=0
until $X -H "$H" file stat "$DIR\\done.txt" >/dev/null 2>&1; do
  if [ $(($(date +%s) - start)) -gt "${TIMEOUT:-900}" ]; then
    echo "timed out waiting for done.txt" >&2
    bad=1
    break
  fi
  if grep -aq 'exception code=' "$OUT/debug.log"; then
    echo "title stopped with an exception:" >&2
    grep -a 'exception code=' "$OUT/debug.log" >&2
    bad=1
    break
  fi
  sleep 5
done
rm -f "$OUT/results.csv"
$X -H "$H" --no-progress file get -o "$OUT/results.csv" "$DIR\\results.csv" >/dev/null || bad=1
if [ -f "$OUT/results.csv" ]; then
  echo "results: $OUT/results.csv ($(wc -l <"$OUT/results.csv") lines)"
  grep -E '^(check,(reg|ext),summary|end,)' "$OUT/results.csv" || true
fi
if [ "$bad" = 1 ]; then
  $X -H "$H" reboot >/dev/null || true
  exit 1
fi
grep -q '^end,failures,0$' "$OUT/results.csv"
