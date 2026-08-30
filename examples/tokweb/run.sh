#!/usr/bin/env bash
# Validate tokweb on deanxbox: start the host web server, launch the title,
# capture its output. Run after the console has been power-cycled to the dash.
set -uo pipefail
ROOT=/var/home/lander/dev/llvm-xbox-360
X=~/dev/xeedee/target/release/xeedee
H=192.168.1.26

echo "== starting host web server on :8080 =="
python3 "$ROOT/examples/tokweb/host_server.py" &
SRV=$!
trap 'kill $SRV 2>/dev/null' EXIT
sleep 1

echo "== console health check (plain-store heartbeat control) =="
# Optional: deploy+launch examples/stdhello first to confirm entry executes.

echo "== launch tokweb.xex =="
$X -H $H raw 'title name="E:\homebrew\tokweb.xex"' >/dev/null 2>&1
OUT=/tmp/tokweb_launch.log
"$ROOT/tools/xbdbg/target/release/xbdbg" $H:730 52730 'E:\homebrew\tokweb.xex' '' 30 >"$OUT" 2>&1 &
DBG=$!
sleep 28
echo "== tokweb output =="
grep -iE 'debugstr' "$OUT" | grep -iE 'tokweb|runtime|timer|HTTP|status|received|DONE|panic' | sed -E 's/.*string=//'
echo "== host server saw: =="
wait $DBG 2>/dev/null
