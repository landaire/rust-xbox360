#!/usr/bin/env python3
"""Package the prebuilt Xbox 360 Rust and LLVM toolchain for one Linux host."""

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path

HOST = "x86_64-unknown-linux-gnu"
ARCHIVE = f"xbox360-toolchain-{HOST}.tar.xz"
ROOT = f"xbox360-toolchain-{HOST}"
RUST_TOOLS = ("rustc", "rustdoc", "clippy-driver")
LLVM_REQUIRED_TOOLS = (
    "clang",
    "clang++",
    "ld.lld",
    "llvm-ar",
    "llvm-objcopy",
    "llvm-objdump",
)
LLVM_OPTIONAL_TOOLS = (
    "FileCheck",
    "clang-cpp",
    "llc",
    "lld",
    "llvm-addr2line",
    "llvm-dwarfdump",
    "llvm-mc",
    "llvm-nm",
    "llvm-ranlib",
    "llvm-readelf",
    "llvm-readobj",
    "llvm-size",
    "llvm-strings",
    "llvm-strip",
    "llvm-symbolizer",
    "opt",
)
DOTSLASH_TOOLS = {
    "xbox360-clang": "bin/clang",
    "xbox360-clang++": "bin/clang++",
    "xbox360-rustc": "bin/rustc",
}


def digest(path: Path, algorithm: str) -> str:
    value = hashlib.new(algorithm)
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def copy_binary(source: Path, destination: Path, selected: set[str]) -> None:
    if source.is_symlink():
        target = Path(os.readlink(source))
        if not target.is_absolute() and target.name in selected:
            destination.symlink_to(target.name)
            return
    shutil.copy2(source.resolve(), destination)


def copy_tool_set(source: Path, destination: Path, required: tuple[str, ...], optional: tuple[str, ...] = ()) -> list[str]:
    missing = [name for name in required if not (source / name).exists()]
    if missing:
        raise SystemExit(f"{source} is missing required tools: {', '.join(missing)}")

    names = list(required) + [name for name in optional if (source / name).exists()]
    selected = set(names)
    destination.mkdir(parents=True, exist_ok=True)
    for name in names:
        copy_binary(source / name, destination / name, selected)
    return names


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_dotslash(path: Path, *, archive: Path, blake3: str, url: str, executable: str) -> None:
    descriptor = {
        "name": path.name,
        "platforms": {
            "linux-x86_64": {
                "digest": blake3,
                "format": "tar.xz",
                "hash": "blake3",
                "path": f"{ROOT}/{executable}",
                "providers": [{"url": url}],
                "size": archive.stat().st_size,
            }
        },
    }
    path.write_text(
        "#!/usr/bin/env dotslash\n\n" + json.dumps(descriptor, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    path.chmod(0o755)


def write_buck_pin(path: Path, manifest: dict[str, object]) -> None:
    archive = manifest["archive"]
    path.write_text(
        '"""Generated pin for the latest tested prebuilt PPC64 toolchain."""\n\n'
        'DEFAULT_PPC64_TOOLCHAIN = "prebuilt"\n\n'
        "NIGHTLY_TOOLCHAIN = {\n"
        f'    "sha256": {json.dumps(archive["sha256"])},\n'
        f'    "source_commit": {json.dumps(manifest["source_commit"])},\n'
        f'    "strip_prefix": {json.dumps(archive["strip_prefix"])},\n'
        f'    "url": {json.dumps(archive["url"])},\n'
        "}\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rust-dist", type=Path, required=True)
    parser.add_argument("--llvm-dist", type=Path, required=True)
    parser.add_argument("--target-spec", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--repository", required=True, help="GitHub owner/repository")
    parser.add_argument("--release-tag", required=True)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--toolchain-inputs", type=Path, required=True)
    parser.add_argument("--source-date-epoch", type=int, required=True)
    parser.add_argument("--dotslash", type=Path, required=True)
    parser.add_argument("--runtime-library", type=Path, action="append", default=[])
    parser.add_argument("--compression-level", type=int, default=6)
    args = parser.parse_args()

    if not 0 <= args.compression_level <= 9:
        parser.error("--compression-level must be between 0 and 9")
    for directory in (args.rust_dist, args.llvm_dist):
        if not directory.is_dir():
            parser.error(f"not a distribution directory: {directory}")
    if not args.target_spec.is_file():
        parser.error(f"target specification does not exist: {args.target_spec}")
    if not args.dotslash.is_file():
        parser.error(f"DotSlash does not exist: {args.dotslash}")
    if not args.toolchain_inputs.is_file():
        parser.error(f"toolchain inputs do not exist: {args.toolchain_inputs}")

    toolchain_inputs = json.loads(args.toolchain_inputs.read_text(encoding="utf-8"))
    required_inputs = ("bootstrap_commit", "llvm_commit", "rust_commit", "toolchain_key")
    missing_inputs = [name for name in required_inputs if name not in toolchain_inputs]
    if missing_inputs:
        parser.error(f"toolchain inputs are missing: {', '.join(missing_inputs)}")

    args.out_dir.mkdir(parents=True, exist_ok=True)
    archive_path = args.out_dir / ARCHIVE
    download_url = (
        f"https://github.com/{args.repository}/releases/download/"
        f"{args.release_tag}/{ARCHIVE}"
    )

    with tempfile.TemporaryDirectory(prefix="xbox360-toolchain-") as temporary:
        root = Path(temporary) / ROOT
        bin_dir = root / "bin"
        rust_tools = copy_tool_set(args.rust_dist / "bin", bin_dir, RUST_TOOLS)
        llvm_tools = copy_tool_set(
            args.llvm_dist / "bin",
            bin_dir,
            LLVM_REQUIRED_TOOLS,
            LLVM_OPTIONAL_TOOLS,
        )
        rust_lld = bin_dir / "rust-lld"
        if not rust_lld.exists():
            rust_lld.symlink_to("ld.lld")
            llvm_tools.append("rust-lld")

        shutil.copytree(
            args.rust_dist / "lib" / "rustlib",
            root / "lib" / "rustlib",
            symlinks=False,
        )
        shutil.copytree(
            args.llvm_dist / "lib" / "clang",
            root / "lib" / "clang",
            symlinks=False,
        )
        for library in args.runtime_library:
            if not library.is_file():
                parser.error(f"runtime library does not exist: {library}")
            destination = root / "lib" / library.name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(library.resolve(), destination)

        target_dir = root / "share" / "xbox360" / "targets"
        target_dir.mkdir(parents=True)
        shutil.copy2(args.target_spec, target_dir / args.target_spec.name)

        package_metadata = {
            "bootstrap_commit": toolchain_inputs["bootstrap_commit"],
            "host": HOST,
            "llvm_commit": toolchain_inputs["llvm_commit"],
            "llvm_tools": sorted(llvm_tools),
            "rust_commit": toolchain_inputs["rust_commit"],
            "rust_tools": sorted(rust_tools),
            "source_commit": args.source_commit,
            "toolchain_key": toolchain_inputs["toolchain_key"],
            "target_spec": f"share/xbox360/targets/{args.target_spec.name}",
        }
        metadata_dir = root / "share" / "xbox360"
        write_json(metadata_dir / "toolchain.json", package_metadata)

        subprocess.run(
            [
                "tar",
                "--sort=name",
                f"--mtime=@{args.source_date_epoch}",
                "--owner=0",
                "--group=0",
                "--numeric-owner",
                "-I",
                f"xz -T1 -{args.compression_level}",
                "-cf",
                str(archive_path),
                "-C",
                temporary,
                ROOT,
            ],
            check=True,
        )

    sha256 = digest(archive_path, "sha256")
    blake3 = subprocess.check_output(
        [str(args.dotslash), "--", "b3sum", str(archive_path)], text=True
    ).strip()
    manifest = {
        "archive": {
            "blake3": blake3,
            "format": "tar.xz",
            "name": ARCHIVE,
            "sha256": sha256,
            "size": archive_path.stat().st_size,
            "strip_prefix": ROOT,
            "url": download_url,
        },
        **package_metadata,
        "toolchain_inputs": toolchain_inputs,
        "schema": 1,
    }
    write_json(args.out_dir / "xbox360-toolchain.json", manifest)
    write_buck_pin(args.out_dir / "nightly_toolchain.bzl", manifest)
    (args.out_dir / "xbox360-toolchain.buckconfig").write_text(
        "[xbox360]\n"
        "  ppc64_toolchain = prebuilt\n"
        f"  ppc64_toolchain_url = {download_url}\n"
        f"  ppc64_toolchain_sha256 = {sha256}\n"
        f"  ppc64_toolchain_strip_prefix = {ROOT}\n",
        encoding="utf-8",
    )
    for name, executable in DOTSLASH_TOOLS.items():
        write_dotslash(
            args.out_dir / name,
            archive=archive_path,
            blake3=blake3,
            url=download_url,
            executable=executable,
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
