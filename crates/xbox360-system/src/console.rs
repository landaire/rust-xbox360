//! Kernel versions, hardware flags and firmware requests.

use core::sync::atomic::Ordering;

use xbox360_sys::xboxkrnl;

pub use xbox360_sys::xboxkrnl::FirmwareRequest;
pub use xbox360_sys::xboxkrnl::HardwareFlags;
pub use xbox360_sys::xboxkrnl::HardwareGeneration;
pub use xbox360_sys::xboxkrnl::KernelVersion;

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
    /// Development kit or test kit, from [`HardwareFlags::TEST_KIT`]. A
    /// debug monitor can also report a test kit from its own setting.
    pub const fn from_hardware_flags(flags: HardwareFlags) -> Self {
        if flags.contains(HardwareFlags::TEST_KIT) {
            Self::TestKit
        } else {
            Self::DevelopmentKit
        }
    }
}

/// Version of the running kernel (`XboxKrnlVersion`).
///
/// # Panics
///
/// If the loader did not resolve the import.
pub fn kernel_version() -> KernelVersion {
    match xboxkrnl::XboxKrnlVersion.get() {
        Some(version) => *version,
        None => xbox360_status::unresolved_import("XboxKrnlVersion"),
    }
}

/// Version of the base kernel (`XboxKrnlBaseVersion`). `None` when the
/// loader left the import unresolved, as Xenia does.
pub fn base_kernel_version() -> Option<KernelVersion> {
    xboxkrnl::XboxKrnlBaseVersion.get().copied()
}

/// The hardware flags of the console.
///
/// # Panics
///
/// If the loader did not resolve `XboxHardwareInfo`.
pub fn hardware_flags() -> HardwareFlags {
    match xboxkrnl::XboxHardwareInfo.get() {
        Some(info) => HardwareFlags::from_bits_retain(info.flags.load(Ordering::Relaxed)),
        None => xbox360_status::unresolved_import("XboxHardwareInfo"),
    }
}

/// Hand the console to the firmware. Does not return.
pub fn return_to_firmware(request: FirmwareRequest) -> ! {
    // SAFETY: FirmwareRequest admits only the routines the kernel accepts;
    // the call takes no pointers.
    unsafe { xboxkrnl::HalReturnToFirmware(request) };

    // HalReturnToFirmware does not return.
    xbox360_thread::park()
}

