# Xbox 360 Rust and LLVM Toolchain

A toolchain that compiles Rust (and any other LLVM language, such as C) to a XEX
executable that runs on the Xbox 360. It replaces Microsoft's proprietary
toolchain. Compilation targets 32-bit big-endian PowerPC; a single post-link
step, `xex-link`, converts the resulting ELF into a devkit-signed XEX. Titles
are verified under the Xenia emulator.

The standard library is supported through an overlay that teaches Rust std about
the `xbox360` target. A title may therefore use `std::thread`, `std::net`,
`std::fs`, allocation, and `println!`.

## Requirements

- `rustup`, the Rust toolchain installer.
- The Rust 1.98.0 toolchain. The standard-library overlay is written against the
  1.98 standard-library source; other versions are not supported.
- The `rust-src` component for that toolchain.
- The `acceleration` repository, checked out as a sibling of this repository
  (that is, `../acceleration`). It supplies the crates that construct and sign
  the XEX; `xex-link` depends on them by path.
- Xenia, built separately, for verification. The helper script
  `tools/run-xenia.sh` invokes it.
- For C titles: `clang`. The example build script obtains it through `nix`.

## Getting started

This procedure assumes no prior experience modifying the Rust toolchain. It
proceeds from a machine with nothing installed to a running title. Commands are
run from the repository root unless stated otherwise.

### 1. Install Rust

Install `rustup` by following the instructions at https://rustup.rs. Then install
the required toolchain and components:

```
rustup toolchain install 1.98.0
rustup component add rust-src --toolchain 1.98.0
```

Direct this repository to use that toolchain. Run this once, in the repository
root:

```
rustup override set 1.98.0
```

### 2. Obtain the acceleration dependency

Check out the `acceleration` repository next to this one, so that the two
directories are siblings:

```
git clone <acceleration-url> ../acceleration
```

`xex-link` refers to it by the relative path `../acceleration`.

### 3. Install the standard-library overlay

The overlay copies the Xbox 360 backend into the toolchain's standard-library
source and applies additive dispatch edits. Apply it once (repeat it after every
`rustup update` or `rustup component add rust-src`, which restore the pristine
source):

```
./toolchain/rust-std-xbox360/apply.sh
```

The script edits only the `xbox360` target and is inert for all others. It
requires the `rust-src` component from step 1.

### 4. Build the linker

`xex-link` is a host program that converts an ELF into a XEX:

```
cargo build -p xex-link
```

The binary is written to `target/debug/xex-link`.

### 5. Compile and run a title without the standard library

Each example is a self-contained crate that selects the target and linker script
through its `.cargo/config.toml`. Because the target is a specification file and
the build compiles part of the library from source, the build requires
`RUSTC_BOOTSTRAP=1` and `-Zjson-target-spec`.

Compile the `hello` example to an ELF executable:

```
cd examples/hello
RUSTC_BOOTSTRAP=1 cargo build --release -Zjson-target-spec
```

The ELF is written to `target/powerpc-unknown-xbox360/release/hello`. Convert it
to a XEX with the metadata in the example's `xex.toml`:

```
../../target/debug/xex-link \
  target/powerpc-unknown-xbox360/release/hello hello.xex \
  --manifest xex.toml
```

Run it under Xenia:

```
../../tools/run-xenia.sh hello.xex --gpu=null --apu=nop
```

### 6. Compile and run a title with the standard library

Standard-library titles additionally build the standard library from the
overlaid source. Compile the `stdhello` example:

```
cd examples/stdhello
RUSTC_BOOTSTRAP=1 cargo build --release -Zbuild-std=std,panic_abort -Zjson-target-spec
```

The first build compiles the standard library and takes several minutes;
subsequent builds reuse the cached result. Convert and run it as in step 5.

If you edit the overlay after building, delete the standard-library build cache
before rebuilding; Cargo does not detect changes to the overlaid source:

```
rm -rf "${CARGO_HOME:-$HOME/.cargo}/build/powerpc-unknown-xbox360"
```

Re-run `./toolchain/rust-std-xbox360/apply.sh` to copy the edited files into the
toolchain source first.

## Repository layout

| Path | Contents |
| --- | --- |
| `crates/xex-link` | The ELF-to-XEX linker (the post-link step). |
| `crates/xbox-sys` | Kernel and Xam import thunks for `no_std` titles. |
| `targets/` | The custom rustc target specification. |
| `toolchain/rust-std-xbox360` | The standard-library overlay and `apply.sh`. |
| `examples/` | Sample titles (see below). |
| `tools/` | Helper scripts and the `xbdbg` devkit debugger. |
| `docs/` | Reference material. |

## Running under Xenia

```
tools/run-xenia.sh <title>.xex --gpu=null --apu=nop
```

The script launches a headless Xenia build and writes its log to standard
output. Xenia does not exit when the title's main thread returns, so runs end at
the timeout; inspect the log rather than the exit code. Set the Xenia path and
timeout at the top of the script if required.

## Compiling C

`xex-link` is language-agnostic: any LLVM front end that emits a big-endian
PowerPC ELF is supported. The C example demonstrates the full path:

```
examples/chello/build.sh chello.xex
```

The script compiles `hello.c` with `clang`, links it with `rust-lld`, and
converts the result with `xex-link`.

## Linker options

`xex-link` reads the XEX metadata from a TOML manifest:

```
usage: xex-link <input.elf> <output.xex> [--manifest PATH]
```

Every key is optional; an absent key, or an absent `--manifest`, takes the
default shown. Commented keys have no default. Unknown keys are rejected.

```toml
title_id = 0x00000000       # integer
media_id = 0x00000000       # integer
load_address = 0x82000000   # image base the ELF was linked at
version = "0.0.0.0"         # title and export table version: a.b.c.d or "0xHEX"
base_version = "0.0.0.0"    # minimum loader version, same format
image_flags = []            # ImageFlags names or "0xHEX", e.g. ["SMALL_PAGES"]
module_flags = ["TITLE"]    # ModuleFlags names or "0xHEX"
compress = false            # LZX-compress the image (Normal, 64 KB window)
# bounding_path = '\Device\Harddisk0\Partition1\DEVKIT\FlashEx\xbdm.xex'
# encrypt = "devkit"        # encrypt with the "devkit" or "retail" master key

[imports]
version = "2.0.21173.0"     # imported module version
min_version = "2.0.1861.0"  # minimum imported module version

# Emits an export table; module is required when the table is present.
# [exports]
# module = "xbdm.xex"       # xboxkrnl.exe, xam.xex, or xbdm.xex
# min_version = "2.0.1861.0" # oldest compatible version in the export table
```

The load address must match the image base the ELF was linked at and must be a
multiple of the page size: 64 KB, or 4 KB with `SMALL_PAGES`.

## Examples

| Example | Standard library | Description |
| --- | --- | --- |
| `hello` | No | Networking and threads; sends a UDP datagram from a worker thread. |
| `hello2` | No | Minimal title that calls `DbgPrint`. |
| `threads` | No | Spawns and joins a worker thread. |
| `net` | No | Sends a UDP datagram. |
| `noimp` | No | Title with no imports; writes to a memory heartbeat. |
| `stdhello` | Yes | Allocation, threads, and files. |
| `tokweb` | Yes | tokio async runtime with an HTTP request to a host server. |
| `chello` | No | C title, built with `clang`. |

Titles marked "No" build with step 5; titles marked "Yes" build with step 6.

## Further reading

- `ARCHITECTURE.md`: target ABI, the pipeline, imports, threading, and
  networking.
- `ROADMAP.md`: milestones and their verification status.
- `toolchain/rust-std-xbox360/README.md`: the standard-library overlay in
  detail, including the protected-atomics backend.
- `docs/llvm-integration.md`: why the ELF-to-XEX conversion is an external
  post-link step rather than part of LLVM.
- `docs/xenon-instructions.md`: Xenon instruction and kernel-ABI reference.
