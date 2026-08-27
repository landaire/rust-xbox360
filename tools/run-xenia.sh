#!/usr/bin/env bash
# Run a XEX in headless Xenia under a virtual X display.
#
# Xenia here is built with a mixed nix/host toolchain, so we launch it through
# the host dynamic linker against host libraries, and provide a virtual display
# via Xvfb (from nix) because the Linux app still initializes GTK.
#
# Usage: run-xenia.sh <path-to.xex> [extra xenia args...]
set -u

XEX="${1:?usage: run-xenia.sh <xex> [args...]}"
shift || true
XENIA="$HOME/src/xenia/build/bin/Linux/Release/xenia"
HOSTLD=/lib64/ld-linux-x86-64.so.2
DISPLAY_NUM=:99
TIMEOUT="${XENIA_TIMEOUT:-45}"

pkill -f 'Xvfb :99' 2>/dev/null
pkill -f 'Release/xenia' 2>/dev/null

exec nix-shell -p xorg.xorgserver --run "
  Xvfb ${DISPLAY_NUM} -screen 0 1280x720x24 >/dev/null 2>&1 &
  XV=\$!
  sleep 2
  env -u LD_LIBRARY_PATH DISPLAY=${DISPLAY_NUM} timeout ${TIMEOUT} \
    ${HOSTLD} --library-path /usr/lib64 \
    ${XENIA} --headless=true --log_file=stdout ${*} \"${XEX}\"
  RC=\$?
  kill \$XV 2>/dev/null
  exit \$RC
"
