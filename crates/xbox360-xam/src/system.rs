//! Launch data, language, region and video mode.
//!
//! Checked against the call sites of the 2.0.6717 dashboard and Xenia: the
//! ordinals (0x1A8 `XamLoaderGetLaunchData`, 0x3CC `XGetGameRegion`, 0x3D1
//! `XGetVideoMode`, 0x3D2 `XamGetLanguage`), the argument order and the
//! `XVIDEO_MODE` layout. Not run on hardware.

use xbox360_status::Win32Error;
use xbox360_sys::xam;

pub use xbox360_sys::xam::VideoMode;

/// Copies the launch data the previous title set to `buffer`. Fails when
/// no launch data is set.
pub fn launch_data(buffer: &mut [u8]) -> Result<(), Win32Error> {
    // Pointers are 32-bit, so a slice length fits in `u32`.
    let size = buffer.len() as u32;
    // SAFETY: XAM writes at most `size` bytes to `buffer`.
    unsafe { xam::XamLoaderGetLaunchData(buffer.as_mut_ptr(), size) }.result()
}

/// Largest launch data XAM accepts. In system software 2.0.21256.0 more
/// bytes reach a trap (`twui`).
pub const LAUNCH_DATA_CAPACITY: usize = 0x3FC;

/// Why [`set_launch_data`] failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LaunchDataError {
    /// The data exceeds [`LAUNCH_DATA_CAPACITY`]; XAM was not called.
    TooLarge { size: usize },
    /// XAM reported a failure.
    Xam(Win32Error),
}

impl core::fmt::Display for LaunchDataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooLarge { size } => {
                write!(f, "{size} bytes of launch data exceed {LAUNCH_DATA_CAPACITY}")
            }
            Self::Xam(error) => write!(f, "XAM failed with {error:?}"),
        }
    }
}

impl core::error::Error for LaunchDataError {}

/// Sets the launch data the next title reads; an empty `data` passes a
/// null pointer and clears it.
pub fn set_launch_data(data: &[u8]) -> Result<(), LaunchDataError> {
    if data.len() > LAUNCH_DATA_CAPACITY {
        return Err(LaunchDataError::TooLarge { size: data.len() });
    }

    let pointer = if data.is_empty() { core::ptr::null() } else { data.as_ptr() };
    // At most `LAUNCH_DATA_CAPACITY`, so the length fits in `u32`.
    let size = data.len() as u32;
    // SAFETY: XAM reads at most `size` bytes from `pointer`.
    unsafe { xam::XamLoaderSetLaunchData(pointer, size) }.result().map_err(LaunchDataError::Xam)
}

/// Console language setting.
pub fn language() -> u32 {
    xam::XamGetLanguage()
}

/// Game region of the console.
pub fn game_region() -> u32 {
    xam::XGetGameRegion()
}

/// Current video mode.
pub fn video_mode() -> VideoMode {
    let mut mode = VideoMode::default();
    // SAFETY: `mode` is a live local `XVIDEO_MODE`.
    unsafe { xam::XGetVideoMode(&mut mode) };
    mode
}
