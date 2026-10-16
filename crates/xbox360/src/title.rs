//! Ending the running title and launching another.
//!
//! A title must not return from its entry point: the kernel stops with
//! fatal error 0xE. End it with [`terminate`], [`return_to_dashboard`] or
//! [`launch`].

use core::ffi::CStr;

use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::ProcessType;

/// `XamLoaderLaunchTitle` flags for a plain launch.
const NO_LAUNCH_FLAGS: u32 = 0;

fn assert_title_process() {
    assert!(
        xboxkrnl::KeGetCurrentProcessType() == ProcessType::TITLE,
        "the title loader was called from a thread outside the title process"
    );
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
