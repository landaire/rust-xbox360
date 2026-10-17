//! System management controller, processor registers and console hardware information.

use core::sync::atomic::AtomicU8;

use super::field::KernelU32;

value_enum! {
    /// `HalReturnToFirmware` routine.
    #[non_exhaustive]
    pub enum FirmwareRequest: u32 {
        /// `HalRebootRoutine`, the routine Xenia asserts on.
        Reboot = 1,
        /// Reset through the system management controller.
        SmcReset = 4,
        /// Restart the system software. Stock XBDM uses it for a reboot.
        RebootQuiesce = 6,
    }
}

value_enum! {
    /// Byte 0 of an [`SmcMessage`]: the command of a request to the system
    /// management controller, or the kind of a message it sends.
    #[non_exhaustive]
    pub enum SmcCommand: u8 {
        /// An I2C transfer.
        I2c = 0x11,
        /// The optical drive tray state. Byte 1 is 0x60 plus the state.
        TrayState = 0x83,
    }
}

/// A 16-byte message to or from the system management controller. Byte 0
/// is the [`SmcCommand`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SmcMessage(pub [u8; 16]);

impl SmcMessage {
    /// A message of `command` with the other bytes zero.
    pub const fn new(command: SmcCommand) -> Self {
        let mut bytes = [0; 16];
        bytes[0] = command.raw();
        Self(bytes)
    }

    /// `None` for a command without a variant.
    pub const fn command(&self) -> Option<SmcCommand> {
        SmcCommand::from_raw(self.0[0])
    }
}

/// Bus address of the device of an [`SmcCommand::I2c`] transfer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmcI2cDevice(pub u8);

impl SmcI2cDevice {
    /// The EEPROM that holds the debug monitor settings block.
    pub const SETTINGS_EEPROM: Self = Self(0xd4);
}

/// A special purpose register of `KeGetSpecialPurposeRegister` and
/// `KeSetSpecialPurposeRegister`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialPurposeRegister(pub u32);

impl SpecialPurposeRegister {
    /// Data address breakpoint register.
    pub const DABR: Self = Self(0x3f5);
    /// Performance monitor control: bit 1 freezes and bit 2 enables the
    /// counters; bits 10 to 15 select the event bus width.
    pub const MONITOR_CONTROL: Self = Self(0x3f6);
    /// Performance monitor event selection.
    pub const EVENT_SELECT: Self = Self(0x3fa);
}

/// A performance monitor bus register of `KeGetPMWRegister` and
/// `KeSetPMWRegister`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PmwRegister(pub u32);

impl PmwRegister {
    /// Lane sources of the bus units.
    pub const NCU0: Self = Self(0x480);
    pub const NCU1: Self = Self(0x580);
    pub const NCU2: Self = Self(0x680);
    pub const CIU: Self = Self(0x708);
    pub const BIU: Self = Self(0xb50);
    /// L2 event group selection.
    pub const L2_GROUP: Self = Self(0x310);
    pub const L2: Self = Self(0x318);
    /// L2 ECC error counts.
    pub const L2_ECC: Self = Self(0x860);
}

/// A performance monitor register of `KeSetPRVRegister`. Values are in
/// the high word.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrvRegister(pub u32);

impl PrvRegister {
    /// Counter run control.
    pub const RUN: Self = Self(0x1c0);
    /// Interval timer control.
    pub const INTERVAL_CONTROL: Self = Self(0x1c8);
    /// Processor whose events trigger the counters.
    pub const TRIGGER: Self = Self(0x1d8);
    pub const INTERVAL: Self = Self(0x1e0);
    pub const INTERVAL_RELOAD: Self = Self(0x1e8);

    /// The value register of counter `counter`. Panics unless `counter` is
    /// 0 to 15.
    pub const fn counter_value(counter: u8) -> Self {
        assert!(counter < 16, "the performance monitor has 16 counters");
        Self(0x200 + counter as u32 * 8)
    }

    /// The control register of counter `counter`. Panics unless `counter`
    /// is 0 to 15.
    pub const fn counter_control(counter: u8) -> Self {
        assert!(counter < 16, "the performance monitor has 16 counters");
        Self(0x280 + counter as u32 * 8)
    }
}

flags! {
    /// The flags word of `XboxHardwareInfo`. The top four bits hold the
    /// [`HardwareGeneration`].
    pub struct HardwareFlags(u32) {
        /// The automatic sign-in profile setting (configuration setting 3,
        /// 8) is available.
        AUTO_PROFILE = 0x10,
        HARD_DISK = 0x20,
        /// The kernel debugger is enabled at boot.
        KERNEL_DEBUGGER_ENABLED = 0x80,
        /// A kernel debugger is attached.
        KERNEL_DEBUGGER = 0x100,
        /// The console has 1 GB of memory, as some development kits do.
        ONE_GIGABYTE = 0x0001_0000,
        /// The console is a test kit.
        TEST_KIT = 0x0200_0000,
    }
}

impl HardwareFlags {
    /// The hardware generation in the top four bits.
    pub const fn generation(self) -> HardwareGeneration {
        HardwareGeneration((self.0 >> 28) as u8)
    }
}

/// Hardware generation number from [`HardwareFlags`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardwareGeneration(u8);

impl HardwareGeneration {
    /// The Trinity motherboard, the first Xbox 360 S.
    pub const TRINITY: Self = Self(4);

    pub const fn get(self) -> u8 {
        self.0
    }
}

flags! {
    /// Boot flags of [`HardwareInfo`].
    pub struct HardwareBootFlags(u16) {
        /// The console booted from a disc.
        FROM_DISC = 0x400,
    }
}

/// `XboxHardwareInfo`: console hardware information, up to the boot flags.
#[repr(C)]
pub struct HardwareInfo {
    /// [`HardwareFlags`] word. The debug monitor changes it while the
    /// system runs, and the kernel acts on it.
    pub flags: KernelU32,
    _reserved_004: [AtomicU8; 0xa],
    pub boot_flags: HardwareBootFlags,
}

const _: () = {
    assert!(core::mem::offset_of!(HardwareInfo, boot_flags) == 0xe);
    assert!(core::mem::size_of::<HardwareInfo>() == 0x10);
};
