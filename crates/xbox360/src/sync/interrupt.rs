use core::marker::PhantomData;

use xbox360_sys::processor;

/// External interrupts disabled on the current processor. Drop restores
/// the external interrupt and recoverable interrupt bits of the MSR as
/// they were.
pub struct InterruptGuard {
    saved: u32,
    _thread: PhantomData<*const ()>,
}

impl InterruptGuard {
    /// Disable external interrupts.
    ///
    /// # Safety
    ///
    /// Until the guard drops, the thread must not wait, call a kernel
    /// routine that may wait, or fault. Guards must drop in the reverse
    /// order of creation.
    pub unsafe fn new() -> Self {
        let saved: u32;
        // SAFETY: reads the machine state register.
        unsafe {
            core::arch::asm!("mfmsr {0}", out(reg) saved, options(nomem, nostack, preserves_flags))
        };

        // SAFETY: the caller upholds the contract until the guard drops.
        unsafe { processor::disable_interrupts() };

        Self {
            saved,
            _thread: PhantomData,
        }
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        // SAFETY: `mtmsrd r9, 1` changes only EE and RI, back to the values
        // saved on this processor; the guard cannot leave the thread. The
        // target CPU model has no `mtmsrd` mnemonic.
        unsafe { core::arch::asm!(".long 0x7d210164", in("r9") self.saved, options(nostack)) };
    }
}
