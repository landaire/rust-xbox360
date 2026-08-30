# Rust std backend for powerpc-unknown-xbox360

An overlay that teaches Rust std (1.98 rust-src) about the `xbox360` target OS,
so `-Zbuild-std=std,panic_abort` produces a working std for the Xbox 360 devkit.

std built with this overlay compiles, links, loads, and **runs on real hardware**.
Verified end-to-end: allocator, `println!` (DbgPrint), `Instant`/`SystemTime`,
TCP/UDP over NetDll, real `std::thread` + `Arc<Mutex>` (protected atomics + a
kernel-event futex), kernel-TLS `thread_local`, and `std::fs` over the NT file
API. tokio (single- and multi-thread) runs and does an HTTP GET to a host server.

Backend modules: `sys/{alloc,stdio,time,net,thread,fs}/xbox360.rs`,
`sys/pal/xbox360/{abi,futex}.rs`, `sys/thread_local/key/xbox360.rs`, and
`core/sync/atomic_xbox360.rs`.

## Protected atomics

The devkit kernel traps `lwarx`/`stwcx.` that are not in the interrupt-bracketed
"protected reservation" form existing titles use (the hypervisor cannot preserve
a reservation across an interrupt). LLVM emits bare lwarx/stwcx, so
`src/core/sync/atomic_xbox360.rs` provides a protected compare-and-swap in inline
asm and `core::sync::atomic`'s RMW/CAS helpers are redirected to it for xbox360.
The recognized form (matched from real titles) is:

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

Key constraints: mtmsrd(disable) immediately before lwarx, the restore
immediately after stwcx, and every lwarx path must reach a stwcx within a short
window. `max-atomic-width` stays 32 so `target_has_atomic` is kept and `Arc`/
`Mutex`/tokio still build; 64-bit atomics never reach these helpers.

## Layout

New backend files (copied verbatim into rust-src):
- `src/sys/pal/xbox360/{mod,common,abi}.rs` - PAL core + kernel/NetDll import thunks
- `src/sys/alloc/xbox360.rs` - global allocator over ExAllocatePoolWithTag/ExFreePool
- `src/sys/stdio/xbox360.rs` - stdout/stderr over DbgPrint
- `src/sys/time/xbox360.rs` - Instant via the PPC timebase, SystemTime via KeQuerySystemTime
- `src/sys/net/connection/xbox360.rs` - TcpStream/TcpListener/UdpSocket over NetDll winsock
- `src/core/sync/atomic_xbox360.rs` - protected lwarx/stwcx atomics

With these, `tokio` (current-thread runtime + time + macros) compiles for the
target; see `examples/tokweb` (async timer + HTTP GET to a host web server).

`dispatch-edits.patch` - the additive `cfg(target_os = "xbox360")` arms in the
std category dispatch files plus the `build.rs` restricted_std allowlist entry.
`apply.sh` also injects the core atomic-helper redirects programmatically.

## Apply

    ./apply.sh            # copies files + applies the patch into the active rustc's rust-src

Requires `rustup component add rust-src` on the pinned stable toolchain. The edits
are additive and inert for every other target.

## Build a title

    cd examples/stdhello
    RUSTC_BOOTSTRAP=1 cargo build --release -Zbuild-std=std,panic_abort -Zjson-target-spec

To force a std rebuild after editing the backend, delete the build-std cache:

    rm -rf "$CARGO_HOME/build/powerpc-unknown-xbox360"
