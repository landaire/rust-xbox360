"""Runs a command and writes its standard output to a file.

Usage: write_stdout.py OUTPUT COMMAND [ARGUMENT...]

Exits with the command's status. Buck2 actions name their outputs as
arguments; this gives one to tools that only print.
"""

import subprocess
import sys


def main():
    if len(sys.argv) < 3:
        raise SystemExit(__doc__.strip())
    with open(sys.argv[1], "wb") as output:
        return subprocess.run(sys.argv[2:], stdout=output).returncode


if __name__ == "__main__":
    sys.exit(main())
