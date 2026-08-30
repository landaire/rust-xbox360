# Rust std backend for powerpc-unknown-xbox360

An overlay that teaches Rust std (1.98 rust-src) about the `xbox360` target OS,
so `-Zbuild-std=std,panic_abort` produces a working std for the Xbox 360 devkit.

std built with this overlay compiles, links, loads, and **runs on real hardware**
(heartbeat, allocation, DbgPrint stdout all verified). The one remaining blocker
is RMW atomics; see the repo notes on the Xbox 360 lwarx/stwcx trap.

## Layout

New backend files (copied verbatim into rust-src):
- `src/sys/pal/xbox360/{mod,common,abi}.rs` - PAL core + kernel/NetDll import thunks
- `src/sys/alloc/xbox360.rs` - global allocator over ExAllocatePoolWithTag/ExFreePool
- `src/sys/stdio/xbox360.rs` - stdout/stderr over DbgPrint

`dispatch-edits.patch` - the additive `cfg(target_os = "xbox360")` arms in the
category dispatch files plus the `build.rs` restricted_std allowlist entry.

## Apply

    ./apply.sh            # copies files + applies the patch into the active rustc's rust-src

Requires `rustup component add rust-src` on the pinned stable toolchain. The edits
are additive and inert for every other target.

## Build a title

    cd examples/stdhello
    RUSTC_BOOTSTRAP=1 cargo build --release -Zbuild-std=std,panic_abort -Zjson-target-spec

To force a std rebuild after editing the backend, delete the build-std cache:

    rm -rf "$CARGO_HOME/build/powerpc-unknown-xbox360"
