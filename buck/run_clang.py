"""Runs clang from PATH.

Usage: run_clang.py [ARGUMENT...]

Exits with clang's status, or with a message if PATH has no clang.
"""

import shutil
import subprocess
import sys


def main():
    clang = shutil.which("clang")
    if clang is None:
        raise SystemExit(
            "run_clang.py: no clang on the PATH of the buck2 daemon. C titles require "
            "clang with the PowerPC target; run `buck2 kill` after changing PATH "
            "(see 'Compiling C' in the rust-xbox360 README)."
        )
    return subprocess.run([clang, *sys.argv[1:]]).returncode


if __name__ == "__main__":
    sys.exit(main())
