//! Processor control region access and external interrupt control.
//!
//! r13 holds the address of the current processor's control region (PCR).
//! The compiler reserves r13 under the SysV ABI, so it is never changed.

/// Address of the current processor's control region.
pub fn pcr() -> u32 {
    let value: u32;
    // SAFETY: reads r13.
    unsafe { core::arch::asm!("mr {0}, 13", out(reg) value) };
    value
}

/// Interrupt request level of the current processor (PCR +0x18).
pub fn current_irql() -> crate::xboxkrnl::Irql {
    let value: u8;
    // SAFETY: the processor control region in r13 is always mapped.
    unsafe { core::arch::asm!("lbz {0}, 0x18(13)", out(reg) value) };
    crate::xboxkrnl::Irql::new(value)
}

/// Address of the current thread object (PCR +0x100).
pub fn current_thread() -> u32 {
    let value: u32;
    // SAFETY: the processor control region in r13 is always mapped.
    unsafe { core::arch::asm!("lwz {0}, 0x100(13)", out(reg) value) };
    value
}

/// Clear the external interrupt enable bit of the MSR.
///
/// `mtmsrd r13, 1` changes only the EE and RI bits. The PCR address in r13
/// has the EE bit (0x8000) clear, so no register needs to be loaded. The
/// target CPU model has no `mtmsrd` mnemonic, so the word is encoded.
///
/// # Safety
///
/// The caller must re-enable interrupts with [`enable_interrupts`] and must
/// not wait or fault while they are disabled.
#[inline(always)]
pub unsafe fn disable_interrupts() {
    // SAFETY: the caller upholds the function's contract.
    unsafe { core::arch::asm!(".long 0x7da10164") };
}

/// Set the external interrupt enable bit of the MSR (`mtmsrd r11, 1` with
/// r11 = 0x8000).
///
/// # Safety
///
/// Interrupts must have been disabled by the caller, and the caller must
/// not hold state that an interrupt may observe in an inconsistent form.
#[inline(always)]
pub unsafe fn enable_interrupts() {
    // SAFETY: the caller upholds the function's contract.
    unsafe {
        core::arch::asm!("lis 11, 0", "ori 11, 11, 0x8000", ".long 0x7d610164", out("r11") _)
    };
}
