#!/usr/bin/env python3
"""Compute the inputs that require a new prebuilt Xbox 360 toolchain."""

import argparse
import configparser
import hashlib
import json
import re
from pathlib import Path

FINGERPRINT_FILES = (
    "buck/custom_toolchain.bzl",
    "buck/toolchains.bzl",
    "targets/powerpc64-unknown-xbox360.json",
    "tools/package_toolchain.py",
    "tools/toolchain_fingerprint.py",
)
SOURCE_PINS = {
    "llvm_commit": ("stage0/custom_llvm.bzl", r'_LLVM_COMMIT = "([0-9a-f]{40})"'),
    "rust_commit": ("sources/BUCK", r'_RUST_COMMIT = "([0-9a-f]{40})"'),
}


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bootstrap-root", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    config = configparser.ConfigParser()
    config.read(".buckconfig")
    bootstrap_commit = config["external_cell_rustc_bootstrap"]["commit_hash"]

    inputs = {
        "bootstrap_commit": bootstrap_commit,
        "files": {path: digest(Path(path)) for path in FINGERPRINT_FILES},
        "schema": 1,
    }
    for name, (relative_path, pattern) in SOURCE_PINS.items():
        path = args.bootstrap_root / relative_path
        match = re.search(pattern, path.read_text(encoding="utf-8"))
        if not match:
            parser.error(f"could not read {name} from {path}")
        inputs[name] = match.group(1)

    canonical = json.dumps(inputs, separators=(",", ":"), sort_keys=True).encode()
    inputs["toolchain_key"] = hashlib.sha256(canonical).hexdigest()
    content = json.dumps(inputs, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(content, encoding="utf-8")
    else:
        print(content, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
