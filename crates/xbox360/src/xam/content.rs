//! Content packages, such as saved games.
//!
//! Checked only against Xenia: the ordinals (0x258 `XamContentCreate`,
//! 0x25A `XamContentClose`, 0x25B `XamContentDelete`), the arguments, the
//! `XCONTENT_DATA` layout, the disposition values, and device id 1 for the
//! hard disk. Calls here pass no `XOVERLAPPED` and complete before they
//! return.

use core::ffi::CStr;

use xbox360_sys::xam;
use xbox360_sys::xam::ContentCreateFlags;

use super::UserIndex;
use crate::status::Win32Error;

pub use xbox360_sys::xam::ContentData;
pub use xbox360_sys::xam::ContentDataError;
pub use xbox360_sys::xam::ContentDevice;
pub use xbox360_sys::xam::ContentType;
pub use xbox360_sys::xam::CreationDisposition;
pub use xbox360_sys::xam::DISPLAY_NAME_UNITS;
pub use xbox360_sys::xam::Disposition;
pub use xbox360_sys::xam::DispositionKind;
pub use xbox360_sys::xam::FILE_NAME_BYTES;

/// A package mapped to a drive. Closes the drive on drop.
#[derive(Debug)]
pub struct ContentRoot<'a> {
    root: &'a CStr,
    disposition: Disposition,
}

impl ContentRoot<'_> {
    /// The drive name; files are under `name:\`.
    pub fn name(&self) -> &CStr {
        self.root
    }

    pub const fn disposition(&self) -> Disposition {
        self.disposition
    }

    /// Close the drive and report the result.
    pub fn close(self) -> Result<(), Win32Error> {
        let root = self.root;
        core::mem::forget(self);
        // SAFETY: `root` is NUL-terminated and names the drive that `create`
        // mapped; `forget` keeps Drop from closing it again.
        unsafe { xam::XamContentClose(root.as_ptr().cast(), core::ptr::null_mut()) }.result()
    }
}

impl Drop for ContentRoot<'_> {
    fn drop(&mut self) {
        // SAFETY: `root` is NUL-terminated and names the drive that `create`
        // mapped; a null overlapped block completes the call before return.
        unsafe { xam::XamContentClose(self.root.as_ptr().cast(), core::ptr::null_mut()) };
    }
}

/// Create or open the package `data` for `user` and map it to the drive
/// `root`, such as `c"save"`.
pub fn create<'a>(
    user: UserIndex,
    root: &'a CStr,
    data: &ContentData,
    disposition: CreationDisposition,
) -> Result<ContentRoot<'a>, Win32Error> {
    let mut taken = Disposition::default();
    let mut license_mask = 0u32;
    // SAFETY: `root` is NUL-terminated, `data` is a live `XCONTENT_DATA`,
    // the outputs are live locals, and a null overlapped block completes
    // the call before it returns.
    unsafe {
        xam::XamContentCreate(
            user,
            root.as_ptr().cast(),
            data,
            ContentCreateFlags::new(disposition),
            &mut taken,
            &mut license_mask,
            core::ptr::null_mut(),
        )
    }
    .result()?;
    Ok(ContentRoot {
        root,
        disposition: taken,
    })
}

/// Delete the package `data` of `user`. It must not be mapped.
pub fn delete(user: UserIndex, data: &ContentData) -> Result<(), Win32Error> {
    // SAFETY: `data` is a live `XCONTENT_DATA`, and a null overlapped block
    // completes the call before it returns.
    unsafe { xam::XamContentDelete(user, data, core::ptr::null_mut()) }.result()
}
