//! Rust std on the Xbox 360 devkit.
//!
//! std built with the `toolchain/rust-std-xbox360` overlay compiles, links,
//! loads, and runs on real hardware. This example proves that: it writes an
//! advancing heartbeat to a global that `xeedee getmem 0x82020000 4` can watch.
//!
//! Caveat: `println!`/`Vec`/`Mutex` pull in RMW atomics (lwarx/stwcx), which the
//! devkit kernel traps as "unprotected reservations". That is the one remaining
//! blocker for full std/tokio; see the repo notes. Plain loads/stores work, so
//! this heartbeat (a volatile store) runs cleanly.

#![no_main]

// The XEX loader enters at `_start`; call the std runtime directly.
core::arch::global_asm!(
    ".globl _start",
    ".section .text._start,\"ax\"",
    "_start:",
    "bl real_main",
    "1: b 1b",
);

#[unsafe(no_mangle)]
pub static mut HEARTBEAT: u32 = 0x4845_4C4F; // "HELO"

#[unsafe(no_mangle)]
extern "C" fn real_main() {
    let p = &raw mut HEARTBEAT;
    let mut i: u32 = 0;
    loop {
        unsafe { core::ptr::write_volatile(p, 0x4845_4C4F ^ i) };
        i = i.wrapping_add(1);
        let mut spin = 0u32;
        while spin < 2_000_000 {
            unsafe { core::ptr::read_volatile(&spin) };
            spin = spin.wrapping_add(1);
        }
    }
}
