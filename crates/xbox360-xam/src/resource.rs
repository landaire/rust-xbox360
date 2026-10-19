//! Locators of system resources that XAM holds for titles.
//!
//! Ordinal 0x31C `XamBuildLegacySystemResourceLocator` (system software
//! 2.0.21256.0, from the code, not checked on hardware) writes
//! `section://<xam.xex module handle>,shrdres#<name>`, a file of the XUIZ
//! archive in resource `shrdres` of `xam.xex`, or a `memory://` locator for
//! eight legacy names. It asserts (`twi 31`) unless the running title is one
//! of two system titles, which halts other titles under the debug monitor.

use xbox360_status::Hresult;
use xbox360_sys::xam;

use super::WideCStr;

/// Units of the locator buffer, terminator included. The 2.0.6717
/// dashboard passes this capacity.
pub const LOCATOR_CAPACITY: usize = 128;

/// A locator that XAM built.
pub struct SystemResourceLocator {
    units: [u16; LOCATOR_CAPACITY],
    length: usize,
}

impl SystemResourceLocator {
    /// The locator without its terminator.
    pub fn units(&self) -> &[u16] {
        &self.units[..self.length]
    }
}

/// Why [`legacy_system_resource_locator`] returned no locator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LocatorError {
    /// XAM reported a failure.
    Xam(Hresult),
    /// The locator does not fit [`LOCATOR_CAPACITY`] units with its
    /// terminator.
    Truncated,
}

impl core::fmt::Display for LocatorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Xam(result) => write!(f, "XAM failed with {:#010x}", result.0),
            Self::Truncated => write!(f, "the locator exceeds {LOCATOR_CAPACITY} units"),
        }
    }
}

impl core::error::Error for LocatorError {}

/// `XamBuildLegacySystemResourceLocator`: the locator of the system
/// resource `name`, such as `A-Button.png`. XAM does not check that the
/// resource exists.
pub fn legacy_system_resource_locator(
    name: &WideCStr,
) -> Result<SystemResourceLocator, LocatorError> {
    let mut units = [0u16; LOCATOR_CAPACITY];
    // SAFETY: `name` is NUL-terminated, and XAM writes at most
    // `LOCATOR_CAPACITY` units to `units`.
    let result = unsafe {
        xam::XamBuildLegacySystemResourceLocator(
            name.as_ptr(),
            units.as_mut_ptr(),
            LOCATOR_CAPACITY as u32,
        )
    };
    if result.is_failure() {
        return Err(LocatorError::Xam(result));
    }

    let length = units
        .iter()
        .position(|&unit| unit == 0)
        .ok_or(LocatorError::Truncated)?;

    Ok(SystemResourceLocator { units, length })
}
