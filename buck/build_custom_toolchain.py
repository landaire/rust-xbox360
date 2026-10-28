#!/usr/bin/env python3
"""Build the optional Rust and LLVM toolchain with buck2-rustc-bootstrap."""

import argparse
import hashlib
import os
import shutil
import subprocess
from pathlib import Path


def find_cxx_runtime() -> Path:
    output = subprocess.check_output(
        ["g++", "-print-file-name=libstdc++.so.6"],
        text=True,
    ).strip()
    runtime = Path(output)
    if not runtime.is_absolute() or not runtime.is_file():
        raise SystemExit("g++ could not locate libstdc++.so.6")
    return runtime.resolve()


def build(
    buck2: Path,
    bootstrap: Path,
    target: str,
    output: Path,
    jobs: int,
) -> None:
    shutil.rmtree(output, ignore_errors=True)
    command = [
        os.fspath(buck2),
        "--isolation-dir",
        "xbox360-toolchain",
        "build",
        "-j",
        str(jobs),
        "-c",
        f"xbox360.toolchain_jobs={jobs}",
        "--local-only",
        "--no-remote-cache",
        target,
        "--out",
        os.fspath(output),
    ]
    print("+", " ".join(command), flush=True)
    subprocess.run(command, cwd=bootstrap, check=True)


def replace(source: Path, destination: Path) -> None:
    shutil.rmtree(destination, ignore_errors=True)
    source.replace(destination)


def package_cxx_runtime(rust_dist: Path, llvm_dist: Path, runtime: Path) -> None:
    shutil.copy2(runtime, llvm_dist / "lib/libstdc++.so.6")

    rust_lib = rust_dist / "lib"
    for entry in rust_lib.iterdir():
        if entry.is_symlink():
            entry.unlink()
    shutil.copy2(runtime, rust_lib / "libstdc++.so.6")


def file_digest(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as file:
        for chunk in iter(lambda: file.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def git_revision(repository: Path) -> str | None:
    status = subprocess.check_output(
        ["git", "status", "--porcelain", "--untracked-files=no"],
        cwd=repository,
        text=True,
    )
    if status:
        return None
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"],
        cwd=repository,
        text=True,
    ).strip()


def toolchain_stamp(bootstrap: Path, rust: Path, runtime: Path) -> str | None:
    bootstrap_revision = git_revision(bootstrap)
    rust_revision = git_revision(rust)
    if bootstrap_revision is None or rust_revision is None:
        return None
    return "\n".join(
        [
            "installer=" + file_digest(Path(__file__)),
            "bootstrap=" + bootstrap_revision,
            "rust=" + rust_revision,
            "cxx-runtime=" + file_digest(runtime),
            "",
        ]
    )


def toolchain_ready(root: Path) -> bool:
    required = [
        root / "toolchain/local-llvm/bin/clang",
        root / "toolchain/local-llvm/bin/ld.lld",
        root / "toolchain/local-rust/bin/rustc",
        root / "toolchain/local-rust/lib/libstdc++.so.6",
        root / "toolchain/local-rust/lib/rustlib/x86_64-unknown-linux-gnu/lib",
    ]
    return all(path.exists() for path in required)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    if args.jobs < 1:
        parser.error("--jobs must be positive")

    root = Path(__file__).resolve().parent.parent
    bootstrap = root / "toolchain/buck2-rustc-bootstrap"
    rust = bootstrap / "rust"
    backtrace = rust / "library/backtrace/src/lib.rs"
    if not (bootstrap / ".buckroot").is_file() or not backtrace.is_file():
        raise SystemExit(
            "initialize the compiler sources first; see README.md's "
            "Experimental source-built compiler section"
        )

    buck2 = root / "bin/buck2"
    rust_staging = root / "toolchain/.local-rust.new"
    llvm_staging = root / "toolchain/.local-llvm.new"
    stamp_path = root / "toolchain/.local-toolchain-stamp"
    cxx_runtime = find_cxx_runtime()
    stamp = toolchain_stamp(bootstrap, rust, cxx_runtime)
    if stamp is not None and toolchain_ready(root):
        if stamp_path.is_file() and stamp_path.read_text(encoding="utf-8") == stamp:
            print("Custom toolchain is already current.", flush=True)
            return 0
    else:
        stamp_path.unlink(missing_ok=True)
    cxx_runtime_input = bootstrap / "stage0/host-libstdc++.so.6"
    cxx_runtime_input.unlink(missing_ok=True)
    shutil.copyfile(cxx_runtime, cxx_runtime_input)

    build(buck2, bootstrap, "stage1:dist", rust_staging, args.jobs)
    build(buck2, bootstrap, "stage0:ci_llvm", llvm_staging, args.jobs)
    package_cxx_runtime(rust_staging, llvm_staging, cxx_runtime)
    replace(rust_staging, root / "toolchain/local-rust")
    replace(llvm_staging, root / "toolchain/local-llvm")
    if stamp is not None:
        stamp_path.write_text(stamp, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
