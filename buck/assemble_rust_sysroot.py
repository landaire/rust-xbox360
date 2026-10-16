"""Installs Rust component archives into one sysroot directory.

Usage: assemble_rust_sysroot.py OUTPUT ARCHIVE...

Equivalent to running each archive's install.sh: every line of a
component's manifest.in names a file or directory, relative to the
component, that belongs at the same path in the sysroot. rustc locates the
standard library and clippy-driver locates rustc's libraries relative to
their own executables, so the components must share one directory.
"""

import os
import shutil
import sys
import tarfile
from pathlib import Path


def move_entry(source, destination):
    """Moves a file or directory, merging into an existing directory."""
    if not destination.exists():
        destination.parent.mkdir(parents=True, exist_ok=True)
        os.replace(source, destination)
    elif source.is_dir() and destination.is_dir():
        for child in source.iterdir():
            move_entry(child, destination / child.name)
    else:
        raise SystemExit(f"error: {destination} is provided by more than one component")


def install_archive(archive, extract_dir, sysroot):
    with tarfile.open(archive, mode="r|xz") as tar:
        tar.extractall(extract_dir, filter="data")
    (package,) = extract_dir.iterdir()
    for component in (package / "components").read_text(encoding="utf-8").split():
        component_dir = package / component
        manifest = (component_dir / "manifest.in").read_text(encoding="utf-8")
        for line in manifest.splitlines():
            kind, _, relative = line.partition(":")
            if kind not in ("file", "dir"):
                raise SystemExit(f"error: {archive}: unknown manifest.in entry {line!r}")
            move_entry(component_dir / relative, sysroot / relative)
    shutil.rmtree(extract_dir)


def link_rust_lld_llvm(sysroot):
    """Makes the rustc component's libLLVM.dylib visible to rust-lld.

    On macOS rust-lld (lib/rustlib/<host>/bin) loads @rpath/libLLVM.dylib
    from lib/rustlib/<host>/lib, where only the llvm-tools component puts a
    copy; rustc keeps its own in lib/. rustup's proxies make rust-lld find
    it through DYLD_FALLBACK_LIBRARY_PATH instead.
    """
    library = sysroot / "lib" / "libLLVM.dylib"
    if not library.is_file():
        return
    for rust_lld in sorted(sysroot.glob("lib/rustlib/*/bin/rust-lld")):
        link = rust_lld.parent.parent / "lib" / "libLLVM.dylib"
        if link.exists():
            continue
        link.parent.mkdir(parents=True, exist_ok=True)
        link.symlink_to(os.path.relpath(library, link.parent))


def main():
    if len(sys.argv) < 3:
        raise SystemExit(__doc__.strip())
    sysroot = Path(sys.argv[1])
    sysroot.mkdir(parents=True)
    # Extraction stays inside the output so that moves are renames.
    extract_dir = sysroot / ".extract"
    for archive in sys.argv[2:]:
        install_archive(Path(archive), extract_dir, sysroot)
    link_rust_lld_llvm(sysroot)


if __name__ == "__main__":
    main()
