//! Probes how the Xbox 360 CPU runs 64-bit instructions in the mode titles
//! use. Every probed instruction is in assembly; the harness only moves
//! words and never compares 64-bit values, so the results do not depend on
//! the code generation under test.
//!
//! Each probe loads A (r5), B (r6), XER (r7) and CTR (r9), clears CR0, runs
//! its instruction(s) with the result in r8, and stores r8, CR, XER, CTR and
//! r10 (aux). The table gives low-word and full-width comparison values.
//! Each probe also specifies whether its key follows MSR[SF] or always
//! uses the full-width value; both the key and result are checked.
//!
//! Results go to DEVKIT:\sfmode\results.csv and to the debug output;
//! DEVKIT:\sfmode\done.txt appears at the end. With `sf1=1` in
//! DEVKIT:\sfmode\config.txt the register probes run a second time with
//! MSR[SF] set and MSR[EE] clear around each probe.

#![no_main]

use std::fs::File;
use std::io::Write;

const DIR: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\sfmode";

#[repr(C, align(8))]
struct In {
    a: u64,
    b: u64,
    xer: u64,
    ctr: u64,
}

#[repr(C, align(8))]
#[derive(Default)]
struct Res {
    r: u64,
    cr: u32,
    xer: u32,
    ctr: u64,
    aux: u64,
}

type TestFn = unsafe extern "C" fn(*const In, *mut Res);

#[derive(Clone, Copy)]
enum Key {
    /// CR0 nibble: LT=8 GT=4 EQ=2 SO=1.
    Cr0,
    /// XER bits: SO=4 OV=2 CA=1.
    Xer,
    /// r10.
    Aux,
    /// MSR[SF] of the result.
    Sf,
    /// Only the result.
    Res,
}

/// Runs in both modes.
const BOTH: u32 = 0;
/// Runs only in the title's own mode (it would fault or run for minutes
/// with SF=1).
const SF0: u32 = 1;
/// A and the aux expectations are offset by the address of MAGIC.
const PTR: u32 = 2;

const LT: u64 = 8;
const GT: u64 = 4;
const EQ: u64 = 2;
const SO: u64 = 1;
const X_SO: u64 = 4;
const X_OV: u64 = 2;
const X_CA: u64 = 1;
/// XER value with CA set.
const CA_IN: u64 = 0x2000_0000;

#[derive(Clone, Copy)]
enum Expected {
    /// CR0, arithmetic carry/overflow and CTR tests follow MSR[SF].
    Mode,
    /// Explicit doubleword operations retain full-width semantics in SF=0.
    FullWidth,
}

struct T {
    name: &'static str,
    insn: &'static str,
    f: TestFn,
    flags: u32,
    a: u64,
    b: u64,
    xer: u64,
    ctr: u64,
    r: u64,
    key: Key,
    e32: u64,
    e64: u64,
    expected: Expected,
}

#[unsafe(no_mangle)]
static SF_MAGIC: u32 = 0xC0FF_EE11;

macro_rules! tests {
    ($($name:ident, $flags:expr, $insn:literal, $a:expr, $b:expr, $xer:expr, $ctr:expr,
       $r:expr, $key:ident, $e32:expr, $e64:expr $(, $expected:ident)?;)*) => {
        core::arch::global_asm!(
            ".macro probe_begin",
            "ld 5,0(3)",
            "ld 6,8(3)",
            "ld 7,16(3)",
            "ld 9,24(3)",
            "li 8,0",
            "li 10,0",
            "mtctr 9",
            "li 0,0",
            "mtcrf 0x80,0",
            "mtxer 7",
            ".endm",
            ".macro probe_end",
            "std 8,0(4)",
            "mfcr 0",
            "stw 0,8(4)",
            "mfxer 0",
            "stw 0,12(4)",
            "mfctr 0",
            "std 0,16(4)",
            "std 10,24(4)",
            "blr",
            ".endm",
            // sf1_call(in, res, f): runs f with MSR[SF]=1 and MSR[EE]=0.
            ".globl sf1_call",
            ".p2align 4",
            "sf1_call:",
            "mflr 0",
            "stwu 1,-64(1)",
            "stw 0,52(1)",
            "std 31,56(1)",
            "clrldi 3,3,32",
            "clrldi 4,4,32",
            "clrldi 5,5,32",
            "mtctr 5",
            "mfmsr 31",
            "li 0,1",
            "sldi 0,0,63",
            "or 0,31,0",
            "li 12,0",
            "ori 12,12,0x8000",
            "andc 0,0,12",
            "mtmsrd 0,0",
            "isync",
            "bctrl",
            "mtmsrd 31,0",
            "isync",
            "ld 31,56(1)",
            "lwz 0,52(1)",
            "mtlr 0",
            "addi 1,1,64",
            "blr",
            $(concat!(
                ".globl ", stringify!($name), "\n",
                ".p2align 4\n",
                stringify!($name), ":\n",
                "probe_begin\n",
                $insn, "\n",
                "probe_end\n",
            )),*
        );
        unsafe extern "C" {
            $(fn $name(i: *const In, o: *mut Res);)*
            fn sf1_call(i: *const In, o: *mut Res, f: TestFn);
        }
        static TESTS: &[T] = &[$(T {
            name: stringify!($name),
            insn: $insn,
            f: $name,
            flags: $flags,
            a: $a,
            b: $b,
            xer: $xer,
            ctr: $ctr,
            r: $r,
            key: Key::$key,
            e32: $e32,
            e64: $e64,
            expected: tests!(@expected $($expected)?),
        }),*];
    };
    (@expected) => { Expected::Mode };
    (@expected $expected:ident) => { Expected::$expected };
}

tests! {
    // name, flags, insn, A, B, XER, CTR, result, key, low-word value, full-width value,
    // optional expectation (default: Mode). Equal key values remain equal in both passes.
    msr, BOTH, "mfmsr 8", 0, 0, 0, 0, 0, Sf, 0, 1;

    // 64-bit record forms: CR0 from the low word (SF=0) or all 64 bits.
    rldicl_classify, BOTH, "rldicl. 8,5,52,1", 0x4010_0000_0000_0000, 0, 0, 0,
        0x0004_0100_0000_0000, Cr0, EQ, GT;
    rldicl_srdi4, BOTH, "rldicl. 8,5,60,4", 0x0000_0010_0000_0000, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    rldicr_sldi32, BOTH, "rldicr. 8,5,32,31", 1, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    rldic, BOTH, "rldic. 8,5,32,16", 1, 0, 0, 0, 0x0000_0001_0000_0000, Cr0, EQ, GT;
    rldimi, BOTH, "rldimi. 8,5,32,0", 1, 0, 0, 0, 0x0000_0001_0000_0000, Cr0, EQ, GT;
    rldcl, BOTH, "rldcl. 8,5,6,0", 1, 32, 0, 0, 0x0000_0001_0000_0000, Cr0, EQ, GT;
    sld, BOTH, "sld. 8,5,6", 1, 32, 0, 0, 0x0000_0001_0000_0000, Cr0, EQ, GT;
    srd, BOTH, "srd. 8,5,6", 0x8000_0000_0000_0000, 32, 0, 0,
        0x0000_0000_8000_0000, Cr0, LT, GT;
    sradi_rec, BOTH, "sradi. 8,5,4", 0x0000_0010_0000_0000, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    srad_rec, BOTH, "srad. 8,5,6", 0x0000_0010_0000_0000, 4, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    add_rec, BOTH, "add. 8,5,6", 0x7FFF_FFFF, 1, 0, 0, 0x8000_0000, Cr0, LT, GT;
    subf_rec, BOTH, "subf. 8,5,6", 1, 0x0000_0001_0000_0001, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    neg_rec, BOTH, "neg. 8,5", 0xFFFF_FFFF_0000_0000, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    and_rec, BOTH, "and. 8,5,6", 0xFFFF_FFFF_0000_0000, 0x7FF0_0000_0000_0000, 0, 0,
        0x7FF0_0000_0000_0000, Cr0, EQ, GT;
    andc_rec, BOTH, "andc. 8,5,6", 0x7FF0_0000_0000_0000, 0, 0, 0,
        0x7FF0_0000_0000_0000, Cr0, EQ, GT;
    or_rec, BOTH, "or. 8,5,6", 0x0000_0001_0000_0000, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    xor_rec, BOTH, "xor. 8,5,6", 0x0000_0001_8000_0000, 0x8000_0000, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    nor_rec, BOTH, "nor. 8,5,6", 0xFFFF_FFFE_FFFF_FFFF, 0, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    mulld_rec, BOTH, "mulld. 8,5,6", 0x1_0000, 0x1_0000, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    divd_rec, BOTH, "divd. 8,5,6", 0x0000_0002_0000_0000, 2, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    divdu_rec, BOTH, "divdu. 8,5,6", 0x0000_0002_0000_0000, 2, 0, 0,
        0x0000_0001_0000_0000, Cr0, EQ, GT;
    extsw_rec, BOTH, "extsw. 8,5", 0x8000_0000, 0, 0, 0, 0xFFFF_FFFF_8000_0000, Cr0, LT, LT;
    addic_rec, BOTH, "addic. 8,5,-1", 0x0000_0001_0000_0000, 0, 0, 0, 0xFFFF_FFFF, Cr0, LT, GT;
    andi_rec_zero, BOTH, "andi. 8,5,0xffff", 0xFFFF_0000_0000_0000, 0, 0, 0, 0, Cr0, EQ, EQ;
    andi_rec_pos, BOTH, "andi. 8,5,0xffff", 0x0000_0001_0000_8000, 0, 0, 0, 0x8000, Cr0, GT, GT;
    andis_rec_bit31, BOTH, "andis. 8,5,0x8000", 0x8000_0000, 0, 0, 0, 0x8000_0000, Cr0, LT, GT;
    andis_rec_zero, BOTH, "andis. 8,5,0xffff", 0xFFFF_FFFF_0000_FFFF, 0, 0, 0, 0, Cr0, EQ, EQ;
    rlwinm_rec_hi, BOTH, "rlwinm. 8,5,0,0,31", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, EQ;
    rlwinm_rec_bit31, BOTH, "rlwinm. 8,5,0,0,31", 0x8000_0000, 0, 0, 0, 0x8000_0000, Cr0, LT, GT;

    // L=1 compares use all 64 bits even with SF=0, unlike record-form CR0.
    cmpd_hi, BOTH, "cmpd 5,6", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, GT, FullWidth;
    cmpd_bit31, BOTH, "cmpd 5,6", 0x8000_0000, 0, 0, 0, 0, Cr0, LT, GT, FullWidth;
    cmpld_hi, BOTH, "cmpld 5,6", 0x0000_0001_0000_0000, 0xFFFF_FFFF, 0, 0, 0, Cr0, LT, GT, FullWidth;
    cmpdi_hi, BOTH, "cmpdi 5,0", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, GT, FullWidth;
    cmpldi_hi, BOTH, "cmpldi 5,0", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, GT, FullWidth;
    cmpw_hi, BOTH, "cmpw 5,6", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, EQ;
    cmplw_hi, BOTH, "cmplw 5,6", 0x0000_0001_0000_0000, 0, 0, 0, 0, Cr0, EQ, EQ;

    // Carry: from the low word (SF=0) or from all 64 bits.
    addc_lo, BOTH, "addc 8,5,6", 0xFFFF_FFFF, 1, 0, 0, 0x0000_0001_0000_0000, Xer, X_CA, 0;
    addc_hi, BOTH, "addc 8,5,6", 0xFFFF_FFFF_0000_0000, 0x0000_0001_0000_0000, 0, 0, 0, Xer, 0, X_CA;
    adde_lo, BOTH, "adde 8,5,6", 0xFFFF_FFFF, 0, CA_IN, 0, 0x0000_0001_0000_0000, Xer, X_CA, 0;
    adde_hi, BOTH, "adde 8,5,6", 0xFFFF_FFFF_0000_0000, 0x0000_0001_0000_0000, CA_IN, 0, 1, Xer, 0, X_CA;
    addic_ne, BOTH, "addic 8,5,-1", 0x0000_0001_0000_0000, 0, 0, 0, 0xFFFF_FFFF, Xer, 0, X_CA;
    subfc_hi, BOTH, "subfc 8,5,6", 1, 0x0000_0001_0000_0000, 0, 0, 0xFFFF_FFFF, Xer, 0, X_CA;
    subfe_hi, BOTH, "subfe 8,5,6", 1, 0x0000_0001_0000_0000, CA_IN, 0, 0xFFFF_FFFF, Xer, 0, X_CA;
    subfic_hi, BOTH, "subfic 8,5,0", 0x0000_0001_0000_0000, 0, 0, 0, 0xFFFF_FFFF_0000_0000, Xer, X_CA, 0;
    addze_lo, BOTH, "addze 8,5", 0xFFFF_FFFF, 0, CA_IN, 0, 0x0000_0001_0000_0000, Xer, X_CA, 0;
    addme_hi, BOTH, "addme 8,5", 0x0000_0001_0000_0000, 0, 0, 0, 0xFFFF_FFFF, Xer, 0, X_CA;
    subfze_hi, BOTH, "subfze 8,5", 0x0000_0001_0000_0000, 0, CA_IN, 0, 0xFFFF_FFFF_0000_0000, Xer, X_CA, 0;
    subfme_hi, BOTH, "subfme 8,5", 0xFFFF_FFFE_FFFF_FFFF, 0, 0, 0, 0xFFFF_FFFF, Xer, 0, X_CA;
    // Doubleword shift carry uses the full-width sign even in SF=0;
    // its record-form CR0 above still follows the mode.
    sradi_ca_pos, BOTH, "sradi 8,5,1", 0x8000_0001, 0, 0, 0, 0x4000_0000, Xer, X_CA, 0, FullWidth;
    sradi_ca_neg, BOTH, "sradi 8,5,1", 0xFFFF_FFFF_0000_0001, 0, 0, 0, 0xFFFF_FFFF_8000_0000, Xer, 0, X_CA, FullWidth;
    srad_ca_pos, BOTH, "srad 8,5,6", 0x8000_0001, 1, 0, 0, 0x4000_0000, Xer, X_CA, 0, FullWidth;
    srad_ca_neg, BOTH, "srad 8,5,6", 0xFFFF_FFFF_0000_0001, 1, 0, 0, 0xFFFF_FFFF_8000_0000, Xer, 0, X_CA, FullWidth;
    srawi_ca, BOTH, "srawi 8,5,1", 0x8000_0001, 0, 0, 0, 0xFFFF_FFFF_C000_0000, Xer, X_CA, X_CA;

    // Overflow.
    addo_lo, BOTH, "addo 8,5,6", 0x7FFF_FFFF, 1, 0, 0, 0x8000_0000, Xer, X_SO | X_OV, 0;
    addo_hi, BOTH, "addo 8,5,6", 0x7FFF_FFFF_FFFF_FFFF, 1, 0, 0, 0x8000_0000_0000_0000, Xer, 0, X_SO | X_OV;
    subfo_lo, BOTH, "subfo 8,5,6", 1, 0x8000_0000, 0, 0, 0x7FFF_FFFF, Xer, X_SO | X_OV, 0;
    nego_lo, BOTH, "nego 8,5", 0x8000_0000, 0, 0, 0, 0xFFFF_FFFF_8000_0000, Xer, X_SO | X_OV, 0;
    // Doubleword multiply overflow is full-width even in SF=0.
    mulldo_lo, BOTH, "mulldo 8,5,6", 0x1_0000, 0x1_0000, 0, 0, 0x0000_0001_0000_0000, Xer, X_SO | X_OV, 0, FullWidth;
    mullwo, BOTH, "mullwo 8,5,6", 0x1_0000, 0x1_0000, 0, 0, 0x0000_0001_0000_0000, Xer, X_SO | X_OV, X_SO | X_OV;
    addo_rec_lo, BOTH, "addo. 8,5,6", 0x7FFF_FFFF, 1, 0, 0, 0x8000_0000, Cr0, LT | SO, GT;

    // CTR: tested in the low word (SF=0) or in all 64 bits. aux=1: taken.
    bdnz_hi1, BOTH, "bdnz 2f\nb 3f\n2: li 10,1\n3:", 0, 0, 0, 0x0000_0001_0000_0001, 0, Aux, 0, 1;
    bdnz_hi0, BOTH, "bdnz 2f\nb 3f\n2: li 10,1\n3:", 0, 0, 0, 0x0000_0001_0000_0000, 0, Aux, 1, 1;
    bdnz_one, BOTH, "bdnz 2f\nb 3f\n2: li 10,1\n3:", 0, 0, 0, 1, 0, Aux, 0, 0;
    bdz_hi1, BOTH, "bdz 2f\nb 3f\n2: li 10,1\n3:", 0, 0, 0, 0x0000_0001_0000_0001, 0, Aux, 1, 0;
    bdz_hi0, BOTH, "bdz 2f\nb 3f\n2: li 10,1\n3:", 0, 0, 0, 0x0000_0001_0000_0000, 0, Aux, 0, 0;
    mfctr_full, BOTH, "mfctr 8", 0, 0, 0, 0x1234_5678_9ABC_DEF0, 0x1234_5678_9ABC_DEF0, Res, 0, 0;
    // aux = iterations.
    bdnz_loop, SF0, "2: addi 10,10,1\nbdnz 2b", 0, 0, 0, 0x0000_0001_0000_0003, 0, Aux,
        3, 0x0000_0001_0000_0003;

    // Effective addresses with high bits set. aux = RA after an update form.
    lwz_ea_hi, SF0 | PTR, "lwz 8,0(5)", 0x0000_0001_0000_0000, 0, 0, 0, 0xC0FF_EE11, Res, 0, 0;
    lwzx_ea_wrap, SF0 | PTR, "lwzx 8,5,6", 0, 0x0000_0001_0000_0000, 0, 0, 0xC0FF_EE11, Res, 0, 0;
    // Address translation truncates the EA in SF=0, but update retains full RA.
    lwzu_ra, SF0 | PTR, "lwzu 8,0(5)\nmr 10,5", 0x0000_0001_0000_0000, 0, 0, 0, 0xC0FF_EE11, Aux,
        0, 0x0000_0001_0000_0000, FullWidth;
}

fn hex32(s: &mut String, v: u32) {
    const D: &[u8; 16] = b"0123456789abcdef";
    for i in 0..8 {
        s.push(D[((v >> (28 - 4 * i)) & 15) as usize] as char);
    }
}

fn hi(v: u64) -> u32 {
    (v >> 32) as u32
}

fn lo(v: u64) -> u32 {
    v as u32
}

/// Compares halves so that no 64-bit compare is involved.
fn same(a: u64, b: u64) -> bool {
    hi(a) == hi(b) && lo(a) == lo(b)
}

fn hex64(s: &mut String, v: u64) {
    hex32(s, hi(v));
    s.push('_');
    hex32(s, lo(v));
}

struct Out {
    file: Option<File>,
    failures: u32,
}

impl Out {
    fn line(&mut self, text: &str) {
        println!("{text}");
        if let Some(f) = self.file.as_mut() {
            let _ = f.write_all(text.as_bytes());
            let _ = f.write_all(b"\n");
        }
    }
}

fn run(out: &mut Out, t: &T, sf1: bool) {
    let ptr = &raw const SF_MAGIC as u32 as u64;
    let off = if t.flags & PTR != 0 { ptr } else { 0 };
    let input = In { a: t.a | off, b: t.b, xer: t.xer, ctr: t.ctr };
    let mut res = Res::default();
    unsafe {
        if sf1 {
            sf1_call(&input, &mut res, t.f);
        } else {
            (t.f)(&input, &mut res);
        }
    }
    let obs = match t.key {
        Key::Cr0 => (res.cr >> 28) as u64,
        Key::Xer => ((res.xer >> 29) & 7) as u64,
        Key::Aux => res.aux,
        Key::Sf => (hi(res.r) >> 31) as u64,
        Key::Res => 0,
    };
    let (e32, e64) = (t.e32 | off, t.e64 | off);
    let verdict = match (same(obs, e32), same(obs, e64)) {
        _ if matches!(t.key, Key::Res) => "-",
        (true, true) => "same",
        (true, false) => "sf0",
        (false, true) => "sf1",
        (false, false) => "neither",
    };
    // The ordinary pass must start in SF=0, not whichever mode was observed.
    // In particular this checks MSR itself, as well as mode-sensitive probes.
    let want_key = match t.expected {
        Expected::Mode if !sf1 => e32,
        Expected::Mode | Expected::FullWidth => e64,
    };
    let key_ok = matches!(t.key, Key::Res) || same(obs, want_key);
    let expected = if matches!(t.key, Key::Res) {
        "-"
    } else if same(e32, e64) {
        "same"
    } else if matches!(t.expected, Expected::FullWidth) || sf1 {
        "sf1"
    } else {
        "sf0"
    };
    let want_r = if t.flags & PTR != 0 && !matches!(t.key, Key::Aux | Key::Res) { t.r | off } else { t.r };
    let res_ok = same(res.r, want_r) || matches!(t.key, Key::Sf);
    if !res_ok || !key_ok {
        out.failures += 1;
    }
    let mut s = String::new();
    s.push_str(if sf1 { "sf1," } else { "sf0," });
    s.push_str(t.name);
    s.push_str(",\"");
    s.push_str(&t.insn.replace('\n', "; "));
    s.push_str("\",a=");
    hex64(&mut s, input.a);
    s.push_str(",b=");
    hex64(&mut s, input.b);
    s.push_str(",xer_in=");
    hex32(&mut s, lo(input.xer));
    s.push_str(",ctr_in=");
    hex64(&mut s, input.ctr);
    s.push_str(",r=");
    hex64(&mut s, res.r);
    s.push_str(",cr=");
    hex32(&mut s, res.cr);
    s.push_str(",xer=");
    hex32(&mut s, res.xer);
    s.push_str(",ctr=");
    hex64(&mut s, res.ctr);
    s.push_str(",aux=");
    hex64(&mut s, res.aux);
    s.push_str(",key=");
    hex32(&mut s, lo(obs));
    s.push_str(",e_sf0=");
    hex32(&mut s, lo(e32));
    s.push_str(",e_sf1=");
    hex32(&mut s, lo(e64));
    s.push_str(",match=");
    s.push_str(verdict);
    s.push_str(if res_ok { ",result=ok" } else { ",result=BAD" });
    s.push_str(",expected=");
    s.push_str(expected);
    s.push_str(if key_ok { ",key_check=ok" } else { ",key_check=BAD" });
    out.line(&s);
}

fn main_body() {
    let sf1 = std::fs::read_to_string(format!("{DIR}\\config.txt"))
        .map(|c| c.lines().any(|l| l.trim() == "sf1=1"))
        .unwrap_or(false);
    let mut out = Out { file: File::create(format!("{DIR}\\results.csv")).ok(), failures: 0 };
    out.line("sfmode: start");
    for t in TESTS.iter().filter(|t| t.flags & PTR == 0) {
        run(&mut out, t, false);
    }
    for t in TESTS.iter().filter(|t| t.flags & PTR != 0) {
        run(&mut out, t, false);
    }
    if sf1 {
        for t in TESTS.iter().filter(|t| t.flags & SF0 == 0) {
            run(&mut out, t, true);
        }
    }
    let failures = out.failures;
    out.line(&format!("end,failures,{failures}"));
    drop(out);
    if let Ok(mut done) = File::create(format!("{DIR}\\done.txt")) {
        let _ = done.write_all(b"done\n");
    }
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

extern "C" fn main(_context: u32) -> u32 {
    main_body();
    xbox360::title::terminate()
}
