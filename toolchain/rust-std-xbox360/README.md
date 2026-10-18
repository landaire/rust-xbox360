# Rust std backend for powerpc-unknown-xbox360

An overlay that teaches Rust std (1.98 rust-src) about the `xbox360` target OS, so `-Zbuild-std=std,panic_abort` produces a working std for the Xbox 360 devkit.

std built with this overlay runs on real hardware: allocator, `println!` (DbgPrint), `Instant`/`SystemTime`, TCP/UDP over NetDll, `std::thread`, `Mutex`/`Condvar`/`RwLock`/`Once`/parking, kernel-TLS `thread_local`, and `std::fs` over the NT file API.

## Kernel and XAM calls

std depends on [`xbox360-sys`](../../crates/xbox360-sys) the way it depends on `libc` on other targets (`rustc-dep-of-std`, declared in `library/std/Cargo.toml` for `cfg(target_os = "xbox360")`). Backend code calls `xbox360_sys::xboxkrnl::*` and `xbox360_sys::xam::*` directly. The import stubs of that crate give each callee the console calling convention: a home area for r3-r10, stack arguments beyond the eighth (`NtCreateFile`), and 32-bit arguments and results only.

The kernel enters a thread start routine with the console calling convention, so `thread_start` is entered through `xbox360_sys::bridged!`, which preserves r14-r31 in full 64 bits. `ExCreateThread` is called with `xapi_thread_startup` = 0, so returning from `thread_start` ends the thread with the returned value as its exit code. std makes no `ExTerminateThread` call.

A title needs a XEX TLS_INFO header with a nonzero slot count. Without it the kernel gives the title no TLS slots, `KeTlsAlloc` fails, and std aborts with "out of TLS keys". `xex-link` writes 64 slots for modules with the TITLE flag unless the manifest sets `tls_slots`.

## System modules

std built with `--cfg xbox360_system` is the standard library of system modules, which run in the system process:
- `ExCreateThread` gets `SYSTEM_PROCESS`. Without that flag the kernel starts a thread in the title process, whichever process the creator is in, and ends it when the title exits.
- Sockets open as `XnCaller::SystemApp` instead of `XnCaller::Title`, so they do not belong to the running title. `XNET_STARTUP_BYPASS_SECURITY` does not cover them. On a development kit their datagrams were accepted and never sent. Each socket that `socket()` opens therefore gets SOL_SOCKET option 0x5801, after which UDP and TCP to ordinary hosts work. Connections that a marked listener accepts worked without the option.

TLS keys come from `KeTlsAlloc` in both builds. The system process has no TLS_INFO header to size it. On a development kit, 24 of its slots were free after one load of a module that uses `std::thread` and `println!`, and each further load of that module took 5. The kernel returns slots only through `KeTlsFree`, which std never calls, so the slots of a system module stay taken after it unloads. When the slots run out, `KeTlsAlloc` fails and std aborts with "out of TLS keys", which stopped the console with a crash dump.

Buck2 builds this variant as `xbox360//sysroot:std-system` (`std = "system"` of `xbox360_title`). With Cargo, add `"--cfg", "xbox360_system"` to the target's `rustflags`, which build-std passes to std.

## Synchronization

std uses its futex-based `Mutex`, `Condvar`, `RwLock`, `Once` and thread parker for xbox360. The kernel has no futex, keyed event or wait-on-address service, so `sys/pal/xbox360/futex.rs` provides a wait-on-address table:
- A futex address hashes to one of 64 buckets. Each bucket holds a kernel spin lock and a FIFO list of waiters.
- A waiter initializes a kernel event on its own stack (`KeInitializeEvent`), checks the futex value and links itself under the bucket lock, then blocks in `KeWaitForSingleObject` with the requested timeout.
- A wake unlinks matching waiters and sets their events (`KeSetEvent`) while it holds the bucket lock. `futex_wake` reports whether it woke a waiter.
- A waiter whose wait times out takes the bucket lock. If it is still linked it unlinks itself, else a wake already set its event, so no event is set after the waiter's frame is gone.

Wakes are exact and nothing polls.

## Protected atomics

The devkit kernel traps `lwarx`/`stwcx.` that are not in the interrupt-bracketed "protected reservation" form existing titles use (the hypervisor cannot preserve a reservation across an interrupt). LLVM emits bare lwarx/stwcx, so `src/core/sync/atomic_xbox360.rs` provides a protected compare-and-swap in inline asm and `core::sync::atomic`'s RMW/CAS helpers are redirected to it for xbox360. The recognized form (matched from real titles) is:

    mfmsr rS                 ; once, before the loop
  loop:
    mtmsrd rDisable, 1       ; clear MSR[EE] (L=1)
    lwarx  cur, 0, ptr
    cmpw   cur, old
    bne-   mismatch
    stwcx. new, 0, ptr
    mtmsrd rS, 1             ; restore EE right after stwcx
    bne-   loop              ; retry if reservation lost
    b      done
  mismatch:
    stwcx. cur, 0, ptr       ; release the reservation (store unchanged)
    mtmsrd rS, 1
  done:

Key constraints: mtmsrd(disable) immediately before lwarx, the restore immediately after stwcx, and every lwarx path must reach a stwcx within a short window. `max-atomic-width` stays 32 so `target_has_atomic` is kept and `Arc`/`Mutex`/tokio still build. 64-bit atomics never reach these helpers.

## Layout

Backend files, under `library/`:
- `std/Cargo.toml` - the `xbox360-sys` dependency
- `Cargo.lock` and `vendor/xbox360-sys-<version>` - its lock entry and the published crate. rust-src resolves std's dependencies from `library/vendor` only, so `-Zbuild-std` needs the vendored copy. Buck downloads the crate from crates.io instead (`sysroot/crates.bzl`).
- `std/src/sys/pal/xbox360/{mod,common,futex}.rs` - PAL core, kernel wait intervals, and the futex
- `std/src/sys/alloc/xbox360.rs` - global allocator over ExAllocatePoolWithTag/ExFreePool
- `std/src/sys/stdio/xbox360.rs` - stdout/stderr over DbgPrint
- `std/src/sys/time/xbox360.rs` - Instant via the PPC timebase, SystemTime via KeQuerySystemTime
- `std/src/sys/thread/xbox360.rs` - threads over ExCreateThread
- `std/src/sys/thread_local/key/xbox360.rs` - TLS keys over KeTls*
- `std/src/sys/fs/xbox360.rs` - files over the NT file API
- `std/src/sys/net/connection/xbox360.rs` - TcpStream/TcpListener/UdpSocket over NetDll winsock
- `core/src/sync/atomic_xbox360.rs` - protected lwarx/stwcx atomics

`library/` also holds the complete files that carry the additive `cfg(target_os = "xbox360")` arms in the std category dispatch files, the `build.rs` restricted_std allowlist entry, and the core atomic-helper redirects.

## Apply

Buck2 builds the standard library from the release's `rust-src` archive with these files in place (`sysroot/BUCK`). For Cargo, copy them over the active toolchain's `rust-src` from the repository root:

    cp -R toolchain/rust-std-xbox360/library/. "$(rustc --print sysroot)/lib/rustlib/src/rust/library/"

Requires `rustup component add rust-src` on the pinned stable toolchain. The edits are additive and inert for every other target.

To move std to a new `xbox360-sys` release: change the version in `library/std/Cargo.toml`, replace `library/vendor/xbox360-sys-*` with the output of `cargo vendor --versioned-dirs` for that release, resolve a scratch copy of the overlaid library to update `library/Cargo.lock`, and run `buck/update_sysroot.py` to regenerate `sysroot/crates.bzl`.

To build against an unpublished `xbox360-sys`, copy the library to a scratch directory instead, point the dependency at `crates/xbox360-sys` with `path = "..."`, and set `__CARGO_TESTS_ONLY_SRC_ROOT` to that `library` directory. Cargo adds the lock entry when it resolves the scratch library.

## Build a title

    cd examples/stdhello
    RUSTC_BOOTSTRAP=1 cargo build --release -Zbuild-std=std,panic_abort -Zjson-target-spec

Cargo does not rebuild std when the library sources change. To force a std rebuild after editing the backend, delete the build-std cache:

    rm -rf "$CARGO_HOME/build/powerpc-unknown-xbox360"
