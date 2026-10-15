"""Writes the Rust library source with the Xbox 360 overlay applied.

Usage: assemble_library_source.py OUTPUT RUST_SRC_ARCHIVE OVERLAY

OUTPUT receives the `library` directory of the rust-src component archive,
then every file under OVERLAY replaces or adds the file at the same
relative path.
"""

import shutil
import sys
import tarfile
from pathlib import Path, PurePosixPath

# Path of the library directory inside the rust-src component, below the
# archive's top-level directory.
LIBRARY_PREFIX = PurePosixPath("rust-src/lib/rustlib/src/rust/library")


def extract_library(archive, output):
    with tarfile.open(archive, mode="r|xz") as tar:
        for member in tar:
            parts = PurePosixPath(member.name).parts
            relative = PurePosixPath(*parts[1:])
            if relative == LIBRARY_PREFIX or LIBRARY_PREFIX not in relative.parents:
                continue
            member.name = str(relative.relative_to(LIBRARY_PREFIX))
            tar.extract(member, output, filter="data")


def apply_overlay(overlay, output):
    for source in sorted(overlay.rglob("*")):
        if source.is_dir():
            continue
        destination = output / source.relative_to(overlay)
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, destination)


def main():
    if len(sys.argv) != 4:
        raise SystemExit(__doc__.strip())
    output, archive, overlay = (Path(arg) for arg in sys.argv[1:])
    output.mkdir(parents=True)
    extract_library(archive, output)
    apply_overlay(overlay, output)


if __name__ == "__main__":
    main()
