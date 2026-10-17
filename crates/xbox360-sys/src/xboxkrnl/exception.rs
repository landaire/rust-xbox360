//! Exception records, thread contexts and bug checks.

use crate::processor::Msr;
use crate::status::NtStatus;

flags! {
    /// `EXCEPTION_*` flags of an [`ExceptionRecord`].
    pub struct ExceptionFlags(u32) {
        /// Execution cannot continue after the exception.
        NONCONTINUABLE = 0x1,
    }
}

/// `EXCEPTION_RECORD`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionRecord {
    pub code: NtStatus,
    pub flags: ExceptionFlags,
    /// Address of the record of an exception raised while this one was
    /// handled, or 0.
    pub next_record: u32,
    /// Address of the instruction that raised the exception.
    pub address: u32,
    /// Number of [`ExceptionRecord::parameters`] in use.
    pub parameter_count: u32,
    pub parameters: [u32; 15],
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(ExceptionRecord, address) == 0xc);
    assert!(offset_of!(ExceptionRecord, parameters) == 0x14);
    assert!(core::mem::size_of::<ExceptionRecord>() == 0x50);
};

flags! {
    /// `CONTEXT_*`: the register groups of a [`Context`] that hold values.
    pub struct ContextFlags(u32) {
        /// MSR, IAR, LR and CTR.
        CONTROL = 0x1,
        /// FPR and FPSCR.
        FLOATING_POINT = 0x2,
        /// GPR, CR and XER.
        INTEGER = 0x4,
        /// VR and VSCR.
        VECTOR = 0x10,
    }
}

impl ContextFlags {
    /// Every register group, as `RtlCaptureContext` requests.
    pub const FULL: Self = Self::CONTROL
        .union(Self::FLOATING_POINT)
        .union(Self::INTEGER)
        .union(Self::VECTOR);
}

/// `CONTEXT`: the registers of a thread. The vector registers need the
/// 16-byte alignment.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub context_flags: ContextFlags,
    pub msr: Msr,
    pub iar: u32,
    pub lr: u32,
    pub ctr: u64,
    pub gpr: [u64; 32],
    pub cr: u32,
    pub xer: u32,
    pub fpscr: u64,
    /// Floating point registers as IEEE bits.
    pub fpr: [u64; 32],
    _reserved_228: [u32; 2],
    pub vscr: [u32; 4],
    pub vr: [[u32; 4]; 128],
}

impl Context {
    /// A context with every register 0 that requests the groups of
    /// `flags`.
    pub const fn new(flags: ContextFlags) -> Self {
        Self {
            context_flags: flags,
            msr: Msr::empty(),
            iar: 0,
            lr: 0,
            ctr: 0,
            gpr: [0; 32],
            cr: 0,
            xer: 0,
            fpscr: 0,
            fpr: [0; 32],
            _reserved_228: [0; 2],
            vscr: [0; 4],
            vr: [[0; 4]; 128],
        }
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(Context, msr) == 0x4);
    assert!(offset_of!(Context, iar) == 0x8);
    assert!(offset_of!(Context, lr) == 0xc);
    assert!(offset_of!(Context, ctr) == 0x10);
    assert!(offset_of!(Context, gpr) == 0x18);
    assert!(offset_of!(Context, cr) == 0x118);
    assert!(offset_of!(Context, xer) == 0x11c);
    assert!(offset_of!(Context, fpscr) == 0x120);
    assert!(offset_of!(Context, fpr) == 0x128);
    assert!(offset_of!(Context, vscr) == 0x230);
    assert!(offset_of!(Context, vr) == 0x240);
    assert!(core::mem::size_of::<Context>() == 0xa40);
};

/// `KTRAP_FRAME`: the registers the kernel saves on an interrupt or
/// exception, up to the last field in use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KtrapFrame {
    _reserved_000: [u32; 0x14],
    pub gpr: [u64; 32],
    _reserved_150: [u32; 0x16],
    pub lr: u32,
    pub iar: u32,
    _msr_high: u32,
    /// Low word of the saved MSR.
    pub msr: Msr,
    pub xer: u32,
    _reserved_1bc: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(KtrapFrame, gpr) == 0x50);
    assert!(offset_of!(KtrapFrame, iar) == 0x1ac);
    assert!(offset_of!(KtrapFrame, lr) == 0x1a8);
    assert!(offset_of!(KtrapFrame, msr) == 0x1b4);
    assert!(offset_of!(KtrapFrame, xer) == 0x1b8);
    assert!(core::mem::size_of::<KtrapFrame>() == 0x1c0);
};

/// `KSWITCHFRAME`: the nonvolatile registers a thread saves on its stack
/// when it stops running, up to the last field in use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KswitchFrame {
    _reserved_000: [u32; 0x14],
    /// r14 to r31.
    pub gpr: [u64; 18],
    pub cr: u32,
    pub iar: u32,
    pub lr: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(KswitchFrame, gpr) == 0x50);
    assert!(offset_of!(KswitchFrame, cr) == 0xe0);
    assert!(offset_of!(KswitchFrame, iar) == 0xe4);
    assert!(offset_of!(KswitchFrame, lr) == 0xe8);
};

/// A `KeBugCheck` code. Codes that are not named here are module-specific.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BugCheckCode(pub u32);

impl BugCheckCode {
    pub const KMODE_EXCEPTION_NOT_HANDLED: Self = Self(0x1e);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_context_is_the_capture_request() {
        // RtlCaptureContext stores 0x17 in the flags word.
        assert_eq!(ContextFlags::FULL.bits(), 0x17);
    }
}
