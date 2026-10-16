//! Kernel versions, hardware flags and firmware requests.

use xbox360_sys::ImportedVariable;
use xbox360_sys::xboxkrnl;

/// A kernel version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub qfe: u16,
}

impl core::fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.qfe
        )
    }
}

fn read_version(slot: &ImportedVariable) -> KernelVersion {
    let address = slot.address() as usize as *const u16;
    // SAFETY: the loader stored the address of a four-u16 kernel export.
    let part = |index| unsafe { core::ptr::read_volatile(address.add(index)) };
    KernelVersion {
        major: part(0),
        minor: part(1),
        build: part(2),
        qfe: part(3),
    }
}

/// Version of the running kernel (`XboxKrnlVersion`).
pub fn kernel_version() -> KernelVersion {
    read_version(&xboxkrnl::XboxKrnlVersion)
}

/// Version of the base kernel (`XboxKrnlBaseVersion`). `None` when the
/// loader left the import unresolved, as Xenia does: it stores
/// 0xDxxxBEEF in data imports it does not provide.
pub fn base_kernel_version() -> Option<KernelVersion> {
    let address = xboxkrnl::XboxKrnlBaseVersion.address();
    if address & 0xf000_ffff == 0xd000_beef {
        return None;
    }
    Some(read_version(&xboxkrnl::XboxKrnlBaseVersion))
}

pub use xbox360_sys::xboxkrnl::FirmwareRequest;
pub use xbox360_sys::xboxkrnl::HardwareFlags;
pub use xbox360_sys::xboxkrnl::HardwareGeneration;

/// Kind of console.
///
/// Stock XBDM reports further kinds, such as a review kit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConsoleType {
    DevelopmentKit,
    TestKit,
}

impl ConsoleType {
    /// Development kit or test kit, as stock XBDM decides from the hardware
    /// flags. XBDM also reports a test kit when its own setting says so.
    pub const fn from_hardware_flags(flags: HardwareFlags) -> Self {
        if flags.contains(HardwareFlags::NOT_DEBUGGABLE) {
            Self::TestKit
        } else {
            Self::DevelopmentKit
        }
    }
}

/// The hardware flags of the console.
pub fn hardware_flags() -> HardwareFlags {
    let address = xboxkrnl::XboxHardwareInfo.address();
    // SAFETY: the loader stored the address of the kernel export.
    HardwareFlags::from_bits_retain(unsafe {
        core::ptr::read_volatile(address as usize as *const u32)
    })
}

/// Hand the console to the firmware. Does not return.
pub fn return_to_firmware(request: FirmwareRequest) -> ! {
    // SAFETY: FirmwareRequest admits only the routines the kernel accepts;
    // the call takes no pointers.
    unsafe { xboxkrnl::HalReturnToFirmware(request) };
    // HalReturnToFirmware does not return.
    crate::thread::park()
}
