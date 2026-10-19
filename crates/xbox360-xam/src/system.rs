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
