//! Ending the running title and launching another.
//!
//! A title must not return from its entry point: the kernel stops with
//! fatal error 0xE. End it with [`terminate`], [`return_to_dashboard`] or
//! [`launch`].

use core::ffi::CStr;

use xbox360_status::KernelError;
use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::AnsiString;
use xbox360_sys::xboxkrnl::ProcessType;

/// `XamLoaderLaunchTitle` flags for a plain launch.
const NO_LAUNCH_FLAGS: u32 = 0;

/// Create a drive mapping from `link`, such as `\??\content:`, to an object
/// path such as `\Device\Harddisk0\Partition1\content`.
///
/// # Errors
///
/// The status of `ObCreateSymbolicLink`. [`KernelError::InvalidParameter`]
/// when either name is longer than an object name can be.
pub fn map_drive(link: &[u8], target: &[u8]) -> Result<(), KernelError> {
    let link = AnsiString::new(link).ok_or(KernelError::InvalidParameter)?;
    let target = AnsiString::new(target).ok_or(KernelError::InvalidParameter)?;

    // SAFETY: both counted strings and their buffers live through the call.
    unsafe { xboxkrnl::ObCreateSymbolicLink(&link, &target) }.result()
}

/// Delete the drive mapping named by `link`.
///
/// # Errors
///
/// The status of `ObDeleteSymbolicLink`. [`KernelError::InvalidParameter`]
/// when `link` is longer than an object name can be.
pub fn unmap_drive(link: &[u8]) -> Result<(), KernelError> {
    let link = AnsiString::new(link).ok_or(KernelError::InvalidParameter)?;

    // SAFETY: the counted string and its buffer live through the call.
    unsafe { xboxkrnl::ObDeleteSymbolicLink(&link) }.result()
}

/// End the running title (`XamLoaderTerminateTitle`).
///
/// # Panics
///
/// Panics when the calling thread does not belong to the title process.
pub fn terminate() -> ! {
    assert_title_process();

    // SAFETY: the calling thread belongs to the title process (checked above).
    unsafe { xam::XamLoaderTerminateTitle() }
}

/// End the running title and return to the dashboard.
///
/// # Panics
///
/// Panics when the calling thread does not belong to the title process.
pub fn return_to_dashboard() -> ! {
    assert_title_process();

    // SAFETY: the calling thread belongs to the title process (checked above).
    // A null path selects the dashboard.
    unsafe { xam::XamLoaderLaunchTitle(core::ptr::null(), NO_LAUNCH_FLAGS) }
}

/// End the running title and launch the executable at `path`, such as
/// `game:\default.xex`.
///
/// # Panics
///
/// Panics when the calling thread does not belong to the title process.
pub fn launch(path: &CStr) -> ! {
    assert_title_process();

    // SAFETY: the calling thread belongs to the title process (checked above),
    // and `path` is a NUL-terminated string that outlives the call.
    unsafe { xam::XamLoaderLaunchTitle(path.as_ptr().cast(), NO_LAUNCH_FLAGS) }
}

fn assert_title_process() {
    assert!(
        xboxkrnl::KeGetCurrentProcessType() == ProcessType::TITLE,
        "the title loader was called from a thread outside the title process"
    );
}
