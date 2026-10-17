//! Device registers at the fixed addresses where the console kernel maps
//! them.
//!
//! The console kernel maps these addresses in every process. Xenia does
//! not map all of them, so an access under Xenia can fault.
//!
//! Each access is one load or store instruction of the register width,
//! which reaches the device once and in program order.
//! [`Register::write`] is followed by `eieio`, which orders the store
//! before later device accesses on the bus.
//!
//! Reads are safe. Writes are unsafe: a device can take an address from a
//! register and write memory there.

use core::cell::UnsafeCell;

/// Order the preceding device accesses before the following ones.
#[inline(always)]
fn eieio() {
    // SAFETY: `eieio` has no effect other than ordering; without `nomem`
    // the compiler also keeps memory accesses on their side of it.
    unsafe { core::arch::asm!("eieio", options(nostack, preserves_flags)) };
}

mod sealed {
    pub trait Sealed {}
}

/// The value of a [`Register`]: a type that one load or store instruction
/// accesses. A `u64` is accessed with one `ld` or `std`, which the 32-bit
/// code of the target does not emit for a plain access.
pub trait RegisterValue: Copy + sealed::Sealed {
    #[doc(hidden)]
    /// # Safety
    ///
    /// `register` must be a mapped register of the size of `Self`,
    /// aligned to it.
    unsafe fn load(register: *const Self) -> Self;

    #[doc(hidden)]
    /// # Safety
    ///
    /// As for [`RegisterValue::load`], and the device must not write memory
    /// that other code uses as a result of the store.
    unsafe fn store(register: *mut Self, value: Self);
}

macro_rules! volatile_register_value {
    ($($type:ty),*) => {$(
        impl sealed::Sealed for $type {}

        impl RegisterValue for $type {
            #[inline(always)]
            unsafe fn load(register: *const Self) -> Self {
                // SAFETY: guaranteed by the caller. A volatile access of
                // at most 32 bits is one load instruction.
                unsafe { register.read_volatile() }
            }

            #[inline(always)]
            unsafe fn store(register: *mut Self, value: Self) {
                // SAFETY: as for `load`.
                unsafe { register.write_volatile(value) }
            }
        }
    )*};
}

volatile_register_value!(u8, u16, u32, GpuStatus);

impl sealed::Sealed for u64 {}

impl RegisterValue for u64 {
    #[inline(always)]
    unsafe fn load(register: *const Self) -> Self {
        // SAFETY: guaranteed by the caller; `load64` is one `ld`.
        unsafe { crate::interlocked::load64(register) }
    }

    #[inline(always)]
    unsafe fn store(register: *mut Self, value: Self) {
        // SAFETY: guaranteed by the caller; `store64` is one `std`.
        unsafe { crate::interlocked::store64(register, value) }
    }
}

/// A device register of type `T`.
#[repr(transparent)]
pub struct Register<T: RegisterValue>(UnsafeCell<T>);

// SAFETY: every access is a single load or store of a register, which the
// device serializes.
unsafe impl<T: RegisterValue> Sync for Register<T> {}

impl<T: RegisterValue> Register<T> {
    pub fn read(&self) -> T {
        // SAFETY: the register is mapped device memory for as long as
        // `self` is borrowed, aligned to its size.
        unsafe { T::load(self.0.get()) }
    }

    /// Store `value`, then `eieio`.
    ///
    /// # Safety
    ///
    /// The device must not write memory that other code uses as a result
    /// of the store.
    pub unsafe fn write(&self, value: T) {
        // SAFETY: guaranteed by the caller.
        unsafe { self.write_unordered(value) };
        eieio();
    }

    /// Store `value` without the trailing `eieio`.
    ///
    /// # Safety
    ///
    /// As for [`Register::write`].
    pub unsafe fn write_unordered(&self, value: T) {
        // SAFETY: the register is mapped device memory for as long as
        // `self` is borrowed; the caller answers for the device's effects.
        unsafe { T::store(self.0.get(), value) }
    }
}

/// A 32-bit register that the device stores byte-reversed (little-endian).
#[repr(transparent)]
pub struct ReversedRegister(Register<u32>);

impl ReversedRegister {
    pub fn read(&self) -> u32 {
        self.0.read().swap_bytes()
    }

    /// Store `value` byte-reversed, then `eieio`.
    ///
    /// # Safety
    ///
    /// As for [`Register::write`].
    pub unsafe fn write(&self, value: u32) {
        // SAFETY: guaranteed by the caller.
        unsafe { self.0.write(value.swap_bytes()) };
    }
}

flags! {
    /// The status register of [`GpuRegisters`].
    pub struct GpuStatus(u32) {
        /// The GPU is busy. A pending flip keeps it set.
        BUSY = 0x8000_0000,
    }
}

/// The GPU register window at 0x7fc8_0000: 0x1_0000 32-bit registers.
#[repr(C)]
pub struct GpuRegisters([Register<u32>; 0x1_0000]);

impl GpuRegisters {
    pub const ADDRESS: usize = 0x7fc8_0000;

    const STATUS: usize = 0x5d0;
    const DISPLAY_GRAPHICS_CONTROL: usize = 0x1841;
    const DISPLAY_PRIMARY_SURFACE: usize = 0x1844;

    pub fn get() -> &'static Self {
        // SAFETY: the console kernel maps the GPU registers at this address
        // in every process, and the window is only reached through
        // `Register`.
        unsafe { &*(Self::ADDRESS as *const Self) }
    }

    /// The register at word `index` of the window.
    pub fn register(&self, index: u16) -> &Register<u32> {
        &self.0[usize::from(index)]
    }

    pub fn status(&self) -> &Register<GpuStatus> {
        let register: *const Register<u32> = &self.0[Self::STATUS];
        // SAFETY: `GpuStatus` is a transparent `u32`, so the register has
        // the same layout and every value is valid.
        unsafe { &*register.cast() }
    }

    /// `D1GRPH_CONTROL` of the display controller.
    pub fn display_graphics_control(&self) -> &Register<u32> {
        &self.0[Self::DISPLAY_GRAPHICS_CONTROL]
    }

    /// `D1GRPH_PRIMARY_SURFACE_ADDRESS`: the front buffer of the display
    /// controller.
    pub fn display_primary_surface(&self) -> &Register<u32> {
        &self.0[Self::DISPLAY_PRIMARY_SURFACE]
    }
}

/// The memory controller performance counter block at 0x7fd2_0100. Every
/// register is byte-reversed.
#[repr(C)]
pub struct MemoryCounters {
    pub control: ReversedRegister,
    pub event: ReversedRegister,
    _reserved0: ReversedRegister,
    pub value: ReversedRegister,
    pub event2: ReversedRegister,
    _reserved1: ReversedRegister,
    pub value2: ReversedRegister,
}

impl MemoryCounters {
    pub const ADDRESS: usize = 0x7fd2_0100;
    /// [`MemoryCounters::control`] value that starts the counters.
    pub const START: u32 = 1;
    /// [`MemoryCounters::control`] value that stops the counters.
    pub const STOP: u32 = 2;

    pub fn get() -> &'static Self {
        // SAFETY: the console kernel maps the memory controller registers
        // at this address in every process.
        unsafe { &*(Self::ADDRESS as *const Self) }
    }
}

flags! {
    /// The ATA status register ([`SataRegisters::status`]).
    pub struct AtaStatus(u8) {
        BUSY = 0x80,
    }
}

flags! {
    /// The hot-plug register of a SATA channel ([`SataRegisters::hotplug`]).
    pub struct SataHotplug(u32) {
        /// The channel raised a hot-plug interrupt.
        INTERRUPT = 0x1,
        INSERTION = 0x4,
        REMOVAL = 0x8,
    }
}

/// The registers of a SATA channel: the ATA task file, then the SATA
/// status, error, control and hot-plug registers, which are byte-reversed.
#[repr(C)]
pub struct SataRegisters {
    _data: Register<u8>,
    pub features: Register<u8>,
    pub sector_count: Register<u8>,
    _lba: [Register<u8>; 3],
    pub device: Register<u8>,
    command: Register<u8>,
    _reserved: [Register<u8>; 8],
    pub sstatus: ReversedRegister,
    pub serror: ReversedRegister,
    pub scontrol: ReversedRegister,
    hotplug: ReversedRegister,
}

impl SataRegisters {
    /// The channel of the optical drive.
    pub const CDROM_ADDRESS: usize = 0x7fea_1200;

    /// The channel of the optical drive.
    pub fn cdrom() -> &'static Self {
        // SAFETY: the console kernel maps the SATA registers at this
        // address in every process.
        unsafe { &*(Self::CDROM_ADDRESS as *const Self) }
    }

    /// The ATA status register, which shares its address with the command
    /// register.
    pub fn status(&self) -> AtaStatus {
        AtaStatus::from_bits_retain(self.command.read())
    }

    /// Store an ATA command, then `eieio`.
    ///
    /// # Safety
    ///
    /// As for [`Register::write`].
    pub unsafe fn write_command(&self, command: u8) {
        // SAFETY: guaranteed by the caller.
        unsafe { self.command.write(command) };
    }

    pub fn hotplug(&self) -> SataHotplug {
        SataHotplug::from_bits_retain(self.hotplug.read())
    }

    /// Store the hot-plug register, then `eieio`.
    ///
    /// # Safety
    ///
    /// As for [`Register::write`].
    pub unsafe fn write_hotplug(&self, value: SataHotplug) {
        // SAFETY: guaranteed by the caller.
        unsafe { self.hotplug.write(value.bits()) };
    }
}

const _: () = {
    assert!(core::mem::size_of::<GpuRegisters>() == 0x4_0000);
    assert!(core::mem::offset_of!(MemoryCounters, event) == 4);
    assert!(core::mem::offset_of!(MemoryCounters, value) == 0xc);
    assert!(core::mem::offset_of!(MemoryCounters, event2) == 0x10);
    assert!(core::mem::offset_of!(MemoryCounters, value2) == 0x18);
    assert!(core::mem::offset_of!(SataRegisters, features) == 1);
    assert!(core::mem::offset_of!(SataRegisters, sector_count) == 2);
    assert!(core::mem::offset_of!(SataRegisters, device) == 6);
    assert!(core::mem::offset_of!(SataRegisters, command) == 7);
    assert!(core::mem::offset_of!(SataRegisters, sstatus) == 0x10);
    assert!(core::mem::offset_of!(SataRegisters, serror) == 0x14);
    assert!(core::mem::offset_of!(SataRegisters, scontrol) == 0x18);
    assert!(core::mem::offset_of!(SataRegisters, hotplug) == 0x1c);
};
