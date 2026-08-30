//! Rust std on the Xbox 360 devkit.
//!
//! Built with the `toolchain/rust-std-xbox360` overlay (std sys backend +
//! protected lwarx/stwcx atomics). std compiles, links, loads, and runs on real
//! hardware. `println!`, `Vec`, and `format!` exercise the allocator, DbgPrint
//! stdout, and the protected atomics used by `OnceLock`/`Arc`.

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
extern "C" fn real_main() {
    println!("stdhello: rust std println is alive on the xbox 360");
    let v: Vec<u32> = (1..=8).map(|i| i * i).collect();
    println!("stdhello: sum of squares = {}", v.iter().sum::<u32>());
    println!("{}", format!("stdhello: format! + heap ok, v = {v:?}"));
    println!("stdhello: DONE");
}
