//! Console test of VMX128 code on the PPC64 toolchain.
//!
//! Every operation of `xbox360-vmx128`, vector code that the compiler
//! generates from `core::simd`, the use of v32-v127 and the callee-saved
//! v64-v127, and the vector calling convention against functions compiled
//! by the console compiler (`msvc/interop.c`) are compared with scalar
//! references. Each test prints `PASS name` or `FAIL name: first mismatch`;
//! the title then prints a summary line and `vmx128test: done`, writes the
//! same lines to `DEVKIT:\vmx128test\results.txt`, and exits.

#![no_main]
#![feature(asm_experimental_arch, portable_simd, simd_ffi)]
#![allow(improper_ctypes, improper_ctypes_definitions)]

use std::fmt::Write as _;
use std::io::Write as _;

mod model;
mod msvc;
mod ops;
mod regs;
mod simd;

core::arch::global_asm!(include_str!("interop.s"), options(raw));
core::arch::global_asm!(include_str!("msvc_runtime.s"), options(raw));
core::arch::global_asm!(include_str!("probes.s"), options(raw));

/// Outcome of one test: extra information on success, the first mismatch
/// on failure.
pub type Outcome = Result<String, String>;

#[derive(Default)]
pub struct Report {
    passed: u32,
    failed: u32,
    text: String,
}

impl Report {
    pub fn run(&mut self, name: &str, test: impl FnOnce() -> Outcome) {
        let line = match test() {
            Ok(info) if info.is_empty() => {
                self.passed += 1;
                format!("PASS {name}")
            }
            Ok(info) => {
                self.passed += 1;
                format!("PASS {name}: {info}")
            }
            Err(error) => {
                self.failed += 1;
                format!("FAIL {name}: {error}")
            }
        };
        println!("{line}");
        let _ = writeln!(self.text, "{line}");
    }
}

/// Fails with `context` unless `got == want`.
pub fn same<T: PartialEq + std::fmt::Debug>(got: T, want: T, context: impl FnOnce() -> String) -> Result<(), String> {
    if got == want {
        Ok(())
    } else {
        Err(format!("{}: got {got:x?}, want {want:x?}", context()))
    }
}

/// xorshift64*, fixed seeds so that every run sees the same inputs.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// A normal float with a random sign and mantissa and an exponent in
    /// `-range..=range`.
    pub fn float(&mut self, range: i32) -> f32 {
        let word = self.next_u32();
        let exponent = (word % (2 * range as u32 + 1)) as i32 - range;
        let bits = (self.next_u32() & 0x807F_FFFF) | (((exponent + 127) as u32) << 23);
        f32::from_bits(bits)
    }

    /// A small integer in `-range..=range` as a float.
    pub fn small(&mut self, range: i32) -> f32 {
        ((self.next_u32() % (2 * range as u32 + 1)) as i32 - range) as f32
    }
}

fn run_all() -> Report {
    let mut report = Report::default();
    simd::run_codegen(&mut report);
    msvc::run(&mut report);
    regs::run(&mut report);
    ops::run(&mut report);
    simd::run(&mut report);
    report
}

fn main_body() {
    // The title's main thread has a 16 KB stack; the tests use more.
    let report = std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(run_all)
        .expect("spawn")
        .join()
        .expect("join");
    let summary = format!("vmx128test: {} passed, {} failed", report.passed, report.failed);
    println!("{summary}");
    let directory = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\vmx128test";
    let written = (|| -> std::io::Result<()> {
        let mut file = std::fs::File::create(format!("{directory}\\results.txt"))?;
        file.write_all(report.text.as_bytes())?;
        writeln!(file, "{summary}")
    })();
    if let Err(error) = written {
        println!("vmx128test: writing results.txt failed: {error}");
    }
    println!("vmx128test: done");
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

extern "C" fn main(_context: u32) -> u32 {
    main_body();
    xbox360::title::terminate()
}
