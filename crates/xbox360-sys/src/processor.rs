//! Processor control region layout and access, barriers, and external
//! interrupt control.
//!
//! r13 holds the address of the current processor's control region (PCR).
//! The compiler reserves r13 under the SysV ABI, so it is never changed.
//!
//! [`Kpcr`] and [`Msr`] compile on every target. The functions exist only
//! for `target_os = "xbox360"`.

use core::sync::atomic::AtomicU8;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use crate::xboxkrnl::KernelU8;
use crate::xboxkrnl::KernelU32;

/// Hardware threads of the console: three cores with two threads each.
pub const HARDWARE_THREAD_COUNT: usize = 6;

/// Number of a hardware thread, below [`HARDWARE_THREAD_COUNT`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardwareThread(u8);

impl HardwareThread {
    /// `None` when `number` is not below [`HARDWARE_THREAD_COUNT`].
    pub const fn new(number: u8) -> Option<Self> {
        if (number as usize) < HARDWARE_THREAD_COUNT {
            Some(Self(number))
        } else {
            None
        }
    }

    /// A number the kernel stores, which it keeps below
    /// [`HARDWARE_THREAD_COUNT`].
    pub(crate) const fn from_kernel(number: u8) -> Self {
        Self(number)
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

flags! {
    /// Machine state register bits, as the low word of a saved MSR.
    pub struct Msr(u32) {
        /// Trace interrupt after each instruction.
        SINGLE_STEP = 0x400,
        /// External interrupts are enabled.
        EXTERNAL_INTERRUPT = 0x8000,
    }
}

/// PIX capture buffer of a hardware thread: the next free byte and the end.
/// The kernel writes records from the cursor up to the end, so both are
/// kernel state.
#[repr(C)]
pub struct PixBuffer {
    pub cursor: KernelU32,
    pub limit: KernelU32,
}

flags! {
    /// Flags word of a [`WaitStatistics`] block.
    pub struct WaitStatisticsFlags(u32) {
        /// The statistics are collected.
        ENABLED = 0x1,
    }
}

/// Critical section wait counts of the threads of one process type in a
/// [`WaitStatistics`] block.
#[repr(C)]
pub struct WaitCounts {
    /// Waits that began.
    pub begins: AtomicU32,
    /// Waits that ended.
    pub ends: AtomicU32,
}

/// Wait statistics block of a hardware thread ([`Kpcr::wait_statistics`]),
/// up to the last field in use. It is counted into while it is read.
#[repr(C)]
pub struct WaitStatistics {
    _reserved_000: [AtomicU8; 4],
    /// [`WaitStatisticsFlags`] word. Collection is started by setting bits
    /// 0 to 2 and stopped by clearing them; bits 1 and 2 have no known
    /// meaning.
    pub flags: KernelU32,
    /// Wait count, high word first.
    waits: [KernelU32; 2],
    _reserved_010: [AtomicU8; 8],
    /// Counts of title threads.
    pub title: WaitCounts,
    /// Counts of system threads.
    pub system: WaitCounts,
}

impl WaitStatistics {
    /// Wait count. The two words may come from different updates.
    pub fn waits(&self) -> u64 {
        let high = self.waits[0].load(Ordering::Relaxed);
        let low = self.waits[1].load(Ordering::Relaxed);

        (u64::from(high) << 32) | u64::from(low)
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(WaitStatistics, flags) == 0x4);
    assert!(offset_of!(WaitStatistics, waits) == 0x8);
    assert!(offset_of!(WaitStatistics, title) == 0x18);
    assert!(offset_of!(WaitStatistics, system) == 0x20);
    assert!(core::mem::size_of::<WaitStatistics>() == 0x28);
};

/// `KPRCB`: the processor control block at PCR +0x100 ([`Kpcr::prcb`]), up
/// to the last field in use. The kernel changes the fields at any time.
#[repr(C)]
pub struct Kprcb {
    /// Address of the running thread object ([`crate::xboxkrnl::Kthread`]).
    pub current_thread: KernelU32,
    _reserved_004: [AtomicU8; 4],
    /// Address of the idle thread object of the processor.
    pub idle_thread: KernelU32,
    processor_number: KernelU8,
    _reserved_00d: [AtomicU8; 7],
    /// Time the processor spent in DPCs.
    pub dpc_time: KernelU32,
    /// Time the processor spent in interrupts.
    pub interrupt_time: KernelU32,
    _reserved_01c: [AtomicU8; 0x34],
    /// Nonzero while a DPC runs on the processor.
    pub dpc_active: KernelU32,
}

impl Kprcb {
    /// Number of the hardware thread.
    pub fn processor_number(&self) -> HardwareThread {
        HardwareThread::from_kernel(self.processor_number.load(Ordering::Relaxed))
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(Kprcb, idle_thread) == 0x8);
    assert!(offset_of!(Kprcb, processor_number) == 0xc);
    assert!(offset_of!(Kprcb, dpc_time) == 0x14);
    assert!(offset_of!(Kprcb, interrupt_time) == 0x18);
    assert!(offset_of!(Kprcb, dpc_active) == 0x50);
    assert!(core::mem::size_of::<Kprcb>() == 0x54);
};

/// `KPCR`: the processor control region of a hardware thread, up to the
/// last field in use, with the processor control block ([`Kprcb`]) at
/// +0x100.
///
/// The kernel changes most fields at any time and acts on them, so they are
/// [`KernelU32`] and [`KernelU8`]. Under Xenia each thread has its own
/// region, which is freed when the thread ends. `Kpcr` is not `Sync`, so a
/// reference to it is not `Send` and stays on the thread that took it.
#[repr(C)]
pub struct Kpcr {
    _reserved_000: [AtomicU8; 0x18],
    /// Interrupt request level ([`crate::xboxkrnl::Irql`]).
    pub irql: KernelU8,
    /// Kernel byte whose meaning is not known (Xenia: `unk_19`).
    pub unknown_019: KernelU8,
    _reserved_01a: [AtomicU8; 0x56],
    /// Highest address of the running thread's stack.
    pub stack_base: KernelU32,
    /// Lowest address of the running thread's stack.
    pub stack_limit: KernelU32,
    _reserved_078: [AtomicU8; 0x88],
    /// The processor control block.
    pub prcb: Kprcb,
    _reserved_154: [AtomicU8; 0x154],
    /// Address of [`Kpcr::prcb`].
    _prcb_address: KernelU32,
    _reserved_2ac: [AtomicU8; 4],
    /// The two PIX capture buffers that the kernel and titles fill.
    pub pix_buffers: [PixBuffer; 2],
    /// PIX capture flags. The kernel reads them to decide whether to fill
    /// the buffers.
    pub pix_flags: KernelU32,
    _reserved_2c4: [AtomicU8; 4],
    /// Cycle count of the processor, high word first.
    cycles: [KernelU32; 2],
    /// Address of the wait statistics block of the processor, or 0.
    pub wait_statistics: KernelU32,
    _thread_bound: core::marker::PhantomData<*const ()>,
}

impl Kpcr {
    /// The control region of the processor that runs the caller at the
    /// time of the call. After the thread moves to another processor, the
    /// reference still names the region of the first one, which stays
    /// mapped.
    #[cfg(any(target_os = "xbox360", xbox360_docs))]
    #[inline]
    pub fn current() -> &'static Self {
        let address: u32;
        // SAFETY: reads r13.
        unsafe {
            core::arch::asm!("mr {0}, 13", out(reg) address, options(nomem, nostack, preserves_flags))
        };

        // SAFETY: r13 always holds the address of the control region of the
        // processor that runs this code. The kernel keeps the region mapped
        // while the thread runs, and a `&Kpcr` is not `Send`, so the
        // reference stays on this thread.
        unsafe { &*core::ptr::with_exposed_provenance::<Self>(address as usize) }
    }

    /// The control region of hardware thread `number`.
    ///
    /// # Safety
    ///
    /// The kernel must map the control regions of all hardware threads
    /// 0x1000 bytes apart, ordered by number, as the console kernel does.
    /// Xenia does not: it gives each thread its own region.
    #[cfg(any(target_os = "xbox360", xbox360_docs))]
    pub unsafe fn of_processor(number: HardwareThread) -> &'static Self {
        let current = Self::current();
        let first =
            core::ptr::from_ref(current).addr() - (current.processor_number().index() << 12);

        // SAFETY: the caller guarantees the layout of the regions, and
        // `number` names one of them.
        unsafe { &*core::ptr::with_exposed_provenance::<Self>(first + (number.index() << 12)) }
    }

    /// The control regions of every hardware thread, in number order.
    ///
    /// # Safety
    ///
    /// As for [`Kpcr::of_processor`].
    #[cfg(any(target_os = "xbox360", xbox360_docs))]
    pub unsafe fn all() -> impl Iterator<Item = &'static Self> {
        let current = Self::current();
        let first =
            core::ptr::from_ref(current).addr() - (current.processor_number().index() << 12);

        (0..HARDWARE_THREAD_COUNT).map(move |number| {
            // SAFETY: the caller guarantees the layout of the regions, and
            // `number` names one of them.
            unsafe { &*core::ptr::with_exposed_provenance::<Self>(first + (number << 12)) }
        })
    }

    /// Number of the hardware thread.
    pub fn processor_number(&self) -> HardwareThread {
        self.prcb.processor_number()
    }

    /// Cycle count of the processor. The kernel updates it while it is
    /// read, so the two words may come from different updates.
    pub fn cycles(&self) -> u64 {
        let high = self.cycles[0].load(Ordering::Relaxed);
        let low = self.cycles[1].load(Ordering::Relaxed);

        (u64::from(high) << 32) | u64::from(low)
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(Kpcr, irql) == 0x18);
    assert!(offset_of!(Kpcr, unknown_019) == 0x19);
    assert!(offset_of!(Kpcr, stack_base) == 0x70);
    assert!(offset_of!(Kpcr, stack_limit) == 0x74);
    assert!(offset_of!(Kpcr, prcb) == 0x100);
    assert!(offset_of!(Kpcr, _prcb_address) == 0x2a8);
    assert!(offset_of!(Kpcr, pix_buffers) == 0x2b0);
    assert!(offset_of!(Kpcr, pix_flags) == 0x2c0);
    assert!(offset_of!(Kpcr, cycles) == 0x2c8);
    assert!(offset_of!(Kpcr, wait_statistics) == 0x2d0);
    assert!(core::mem::size_of::<Kpcr>() == 0x2d4);
};

/// Interrupt request level of the current processor (PCR +0x18). It is one
/// load relative to r13. Through `Kpcr::current().irql`, r13 is first
/// copied to another register.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn current_irql() -> crate::xboxkrnl::Irql {
    let value: u8;
    // SAFETY: the processor control region in r13 is always mapped.
    unsafe { core::arch::asm!("lbz {0}, 0x18(13)", out(reg) value) };

    crate::xboxkrnl::Irql::new(value)
}

/// Address of the current thread object (PCR +0x100). It is one load
/// relative to r13, as for [`current_irql`].
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn current_thread() -> u32 {
    let value: u32;
    // SAFETY: the processor control region in r13 is always mapped.
    unsafe { core::arch::asm!("lwz {0}, 0x100(13)", out(reg) value) };

    value
}

/// `lwsync`: order earlier loads and stores before later stores, and
/// earlier loads before later loads. The compiler does not move memory
/// accesses across it either.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub fn lwsync() {
    // SAFETY: a barrier has no other effect.
    unsafe { core::arch::asm!("lwsync", options(nostack, preserves_flags)) };
}

/// `or 31, 31, 31`: lower the priority of this hardware thread for a spin
/// wait.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub fn low_priority_hint() {
    // SAFETY: changes only the issue priority of the hardware thread.
    unsafe { core::arch::asm!("or 31, 31, 31", options(nomem, nostack, preserves_flags)) };
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
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub unsafe fn disable_interrupts() {
    // SAFETY: the caller upholds the function's contract.
    unsafe { core::arch::asm!(".long 0x7da10164") };
}

/// Set the external interrupt enable bit of the MSR (`mtmsrd r11, 1` with
/// r11 = [`Msr::EXTERNAL_INTERRUPT`]).
///
/// # Safety
///
/// Interrupts must have been disabled by the caller, and the caller must
/// not hold state that an interrupt may observe in an inconsistent form.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub unsafe fn enable_interrupts() {
    // SAFETY: the caller upholds the function's contract.
    unsafe {
        core::arch::asm!(
            "lis 11, 0",
            "ori 11, 11, {ee}",
            ".long 0x7d610164",
            ee = const Msr::EXTERNAL_INTERRUPT.bits(),
            out("r11") _,
        )
    };
}
