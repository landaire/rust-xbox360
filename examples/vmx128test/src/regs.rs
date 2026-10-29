//! Register use: code that needs more than 32 vector registers, values
//! kept in the callee-saved v64-v127 across calls into code that clobbers
//! vector registers, and callees that must preserve v64-v127.

use core::hint::black_box;
use core::simd::prelude::*;

use crate::msvc;
use crate::same;
use crate::Report;

pub const LIVE: usize = 96;

unsafe extern "C" {
    /// Overwrites v0-v63 (`probes.s`).
    pub fn vmx_clobber_volatile();
    /// Calls `f` with v64-v127 loaded from `pattern` and stores them to
    /// `out` after the call (`probes.s`).
    fn vmx_csr_probe(f: extern "C" fn(), pattern: *const f32x4, out: *mut f32x4);
}

/// Word `j` of vector `k` of the test input: small exact values.
pub const fn input_value(k: usize, j: usize) -> f32 {
    ((k * 4 + j) % 61) as f32 - 30.0 + 0.25 * j as f32
}

pub fn input() -> [f32x4; LIVE] {
    core::array::from_fn(|k| f32x4::from_array(core::array::from_fn(|j| input_value(k, j))))
}

macro_rules! live {
    ($($v:ident $i:literal)*) => {
        /// Keeps 96 vectors live through a loop: more than v0-v63 hold.
        #[inline(never)]
        pub fn pressure(input: &[f32x4; LIVE], rounds: u32) -> [f32x4; LIVE] {
            $( let mut $v = input[$i]; )*
            let step = f32x4::from_array([1.0, 2.0, 3.0, 4.0]);
            for _ in 0..rounds {
                $( $v = $v + step; )*
            }
            [$($v),*]
        }

        /// Keeps 96 opaque vectors live across `call`.
        #[inline(never)]
        pub fn keep_across(input: &[f32x4; LIVE], call: unsafe extern "C" fn()) -> [f32x4; LIVE] {
            $( let $v = black_box(input[$i]); )*
            // SAFETY: every callee passed here follows the C convention.
            unsafe { call() };
            [$($v),*]
        }

        #[inline(never)]
        pub fn keep_across_inline(input: &[f32x4; LIVE]) -> [f32x4; LIVE] {
            $( let $v = black_box(input[$i]); )*
            // SAFETY: the assembly callee follows the C convention.
            unsafe {
                core::arch::asm!(
                    "bl {callee}",
                    callee = sym vmx_clobber_volatile,
                    clobber_abi("C"),
                );
            }
            [$($v),*]
        }
    };
}

live!(
    v0 0 v1 1 v2 2 v3 3 v4 4 v5 5 v6 6 v7 7 v8 8 v9 9 v10 10 v11 11 v12 12 v13 13 v14 14 v15 15 v16
    16 v17 17 v18 18 v19 19 v20 20 v21 21 v22 22 v23 23 v24 24 v25 25 v26 26 v27 27 v28 28 v29 29
    v30 30 v31 31 v32 32 v33 33 v34 34 v35 35 v36 36 v37 37 v38 38 v39 39 v40 40 v41 41 v42 42 v43
    43 v44 44 v45 45 v46 46 v47 47 v48 48 v49 49 v50 50 v51 51 v52 52 v53 53 v54 54 v55 55 v56 56
    v57 57 v58 58 v59 59 v60 60 v61 61 v62 62 v63 63 v64 64 v65 65 v66 66 v67 67 v68 68 v69 69 v70
    70 v71 71 v72 72 v73 73 v74 74 v75 75 v76 76 v77 77 v78 78 v79 79 v80 80 v81 81 v82 82 v83 83
    v84 84 v85 85 v86 86 v87 87 v88 88 v89 89 v90 90 v91 91 v92 92 v93 93 v94 94 v95 95
);

/// Uses v0-v127 and returns; a callee for the preservation probes.
pub extern "C" fn rust_pressure() {
    let input = input();
    let result = pressure(black_box(&input), black_box(3));
    black_box(result);
}

/// Keeps vectors live across a call that clobbers v0-v63.
pub extern "C" fn rust_keep_across() {
    let input = input();
    let result = keep_across(black_box(&input), vmx_clobber_volatile);
    black_box(result);
}

pub fn run(report: &mut Report) {
    report.run("regs.inline_v127", || {
        let value: i32x4;
        // SAFETY: the declared output is the only register changed.
        unsafe {
            core::arch::asm!(
                "vspltisw128 127, -1",
                out("v127") value,
                options(nomem, nostack, preserves_flags),
            );
        }
        same(value.to_array(), [-1; 4], || "v127 output".into())?;
        Ok(String::new())
    });
    report.run("regs.inline_call_clobbers", || {
        let input = input();
        let got = keep_across_inline(black_box(&input));
        for k in 0..LIVE {
            same(got[k].to_bits().to_array(), input[k].to_bits().to_array(), || format!("vector {k}"))?;
        }
        Ok(String::new())
    });
    report.run("regs.pressure", || {
        let input = input();
        let got = pressure(black_box(&input), black_box(5));
        for k in 0..LIVE {
            let want: [u32; 4] = core::array::from_fn(|j| (input_value(k, j) + 5.0 * (j + 1) as f32).to_bits());
            same(got[k].to_bits().to_array(), want, || format!("vector {k}"))?;
        }
        Ok(String::new())
    });
    let callees: [(&str, unsafe extern "C" fn()); 5] = [
        ("asm_clobber_v0_v63", vmx_clobber_volatile),
        ("rust_pressure", rust_pressure),
        ("rust_keep_across", rust_keep_across),
        ("msvc_pressure", msvc::msvc_pressure),
        ("msvc_keep_across", msvc::msvc_keep_across),
    ];
    for (name, callee) in callees {
        report.run(&format!("regs.keep_across.{name}"), || {
            let input = input();
            let got = keep_across(black_box(&input), black_box(callee));
            for k in 0..LIVE {
                same(got[k].to_bits().to_array(), input[k].to_bits().to_array(), || format!("vector {k}"))?;
            }
            Ok(String::new())
        });
    }
    let preserving: [(&str, extern "C" fn()); 4] = [
        ("rust_pressure", rust_pressure),
        ("rust_keep_across", rust_keep_across),
        ("msvc_pressure", msvc::msvc_pressure),
        ("msvc_keep_across", msvc::msvc_keep_across),
    ];
    for (name, callee) in preserving {
        report.run(&format!("regs.preserves_v64_v127.{name}"), || {
            let pattern: [f32x4; 64] =
                core::array::from_fn(|k| f32x4::from_bits(u32x4::from_array(core::array::from_fn(|j| 0x5A00_0000 | (k * 4 + j) as u32))));
            let mut out = [f32x4::splat(0.0); 64];
            // SAFETY: both arrays hold 64 aligned vectors; the callee follows the C convention.
            unsafe { vmx_csr_probe(callee, pattern.as_ptr(), out.as_mut_ptr()) };
            for k in 0..64 {
                same(out[k].to_bits().to_array(), pattern[k].to_bits().to_array(), || format!("v{}", 64 + k))?;
            }
            Ok(String::new())
        });
    }
}
