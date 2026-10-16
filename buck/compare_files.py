"""Fails unless two files are identical; on success writes OUTPUT.

Usage: compare_files.py OUTPUT EXPECTED ACTUAL

OUTPUT is an empty file that Buck2 actions name as their result.
"""

import filecmp
import sys


def main():
    if len(sys.argv) != 4:
        raise SystemExit(__doc__.strip())
    output, expected, actual = sys.argv[1:]
    if not filecmp.cmp(expected, actual, shallow=False):
        print(f"error: {actual} differs from {expected}", file=sys.stderr)
        return 1
    with open(output, "wb"):
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main())
