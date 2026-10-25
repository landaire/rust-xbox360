#!/usr/bin/env python3

import os
import shutil
import subprocess
import sys


def main() -> int:
    arguments = sys.argv[1:]
    separator = arguments.index("--profile-objcopy-end")
    objcopy = arguments[:separator]
    trailing = arguments[separator + 1:]
    binary, directory, name, symbols = trailing[:4]
    if len(trailing) == 5:
        shutil.copytree(trailing[4], directory, symlinks=True, dirs_exist_ok=True)
    output = os.path.join(directory, name)
    os.makedirs(directory, exist_ok=True)
    os.makedirs(os.path.dirname(symbols), exist_ok=True)
    subprocess.run(
        [*objcopy, "--only-keep-debug", binary, symbols],
        check=True,
    )
    subprocess.run(
        [
            *objcopy,
            "--strip-all",
            binary,
            output,
        ],
        check=True,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
