//! Titles played, achievements, aggregate content enumerations and title
//! queries.
//!
//! Checked against the call sites of the 2.0.6717 dashboard, not run on
//! hardware: 0x2F3 `XamUserCreateTitlesPlayedEnumerator`, 0x2EE
//! `XamUserCreateAchievementEnumerator`, 0x279
//! `XamContentAggregateCreateEnumerator`, 0x250 `XamEnumerate`, 0x1BF
//! `XamIsXbox1TitleId`, 0x1CF `XamGetCurrentTitleId`, 0x2B6
//! `XamGetCachedTitleName`, 0x1E8 `XamFormatDateString` and 0x1C5
//! `XamGetDefaultSystemImage`.

use core::num::NonZeroU32;

use xbox360_status::Win32Error;
use xbox360_status::Win32ErrorCode;
use xbox360_sys::xam;
use xbox360_sys::xam::Overlapped;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

use super::UserIndex;

/// An open XAM enumeration handle, closed on drop.
pub struct Enumerator {
    handle: Handle,
    /// The buffer size XAM asks for, when the create call reports one.
    buffer_size: Option<NonZeroU32>,
}

impl Enumerator {
    /// `XamUserCreateTitlesPlayedEnumerator(title, user, xuid, first,
    /// count)`. `xuid` 0 names the user.
    pub fn titles_played(title: u32, user: UserIndex, xuid: u64, first: u32, count: u32) -> Result<Self, Win32Error> {
        let mut size = 0;
        let mut handle = Handle::NULL;
        // SAFETY: `size` and `handle` are live locals.
        unsafe { xam::XamUserCreateTitlesPlayedEnumerator(title, user, xuid, first, count, &mut size, &mut handle) }
            .result()?;
        Ok(Self { handle, buffer_size: NonZeroU32::new(size) })
    }

    /// `XamUserCreateAchievementEnumerator(title, user, xuid, details,
    /// first, count)`. `xuid` 0 names the user.
    pub fn achievements(
        title: u32,
        user: UserIndex,
        xuid: u64,
        details: u32,
        first: u32,
        count: u32,
    ) -> Result<Self, Win32Error> {
        let mut size = 0;
        let mut handle = Handle::NULL;
        // SAFETY: `size` and `handle` are live locals.
        unsafe {
            xam::XamUserCreateAchievementEnumerator(title, user, xuid, details, first, count, &mut size, &mut handle)
        }
        .result()?;
        Ok(Self { handle, buffer_size: NonZeroU32::new(size) })
    }

    /// `XamContentAggregateCreateEnumerator(0, device, content_type,
    /// title)`. `device` 0 names every device, `title` 0 every title.
    pub fn aggregate_content(device: u32, content_type: u32, title: u32) -> Result<Self, Win32Error> {
        const RESERVED: u32 = 0;

        let mut handle = Handle::NULL;
        // SAFETY: `handle` is a live local.
        unsafe { xam::XamContentAggregateCreateEnumerator(RESERVED, device, content_type, title, &mut handle) }
            .result()?;
        Ok(Self { handle, buffer_size: None })
    }

    /// Takes ownership of an open enumeration `handle`.
    pub(crate) fn from_handle(handle: Handle) -> Self {
        Self { handle, buffer_size: None }
    }

    /// The buffer size the create call reported, `None` when it reports none.
    pub fn buffer_size(&self) -> Option<NonZeroU32> {
        self.buffer_size
    }

    /// `XamEnumerate(handle, flags, buffer, size, NULL, overlapped)`: the
    /// next records. Returns `ERROR_IO_PENDING` while the call runs; the
    /// block then holds the record count in its length.
    ///
    /// # Safety
    ///
    /// `buffer` holds `size` bytes, and `buffer` and `overlapped` stay valid
    /// until the call completes or is cancelled.
    pub unsafe fn enumerate(&self, flags: u32, buffer: *mut u8, size: u32, overlapped: *mut Overlapped) -> Win32ErrorCode {
        // SAFETY: the caller keeps the addresses valid; `handle` is open.
        unsafe { xam::XamEnumerate(self.handle, flags, buffer, size, core::ptr::null_mut(), overlapped) }
    }

    /// `XamEnumerate(handle, flags, buffer, size, items, NULL)`: the next
    /// records, waiting for them. XAM stores the record count at `items`.
    ///
    /// # Safety
    ///
    /// `buffer` holds `size` bytes.
    pub unsafe fn enumerate_counted(&self, flags: u32, buffer: *mut u8, size: u32, items: &mut u32) -> Win32ErrorCode {
        // SAFETY: the call completes before it returns; `handle` is open.
        unsafe { xam::XamEnumerate(self.handle, flags, buffer, size, items, core::ptr::null_mut()) }
    }
}

impl Drop for Enumerator {
    fn drop(&mut self) {
        // SAFETY: `handle` is open and not used after this.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}

/// Whether `title` is an original Xbox title.
pub fn is_xbox1_title(title: u32) -> bool {
    xam::XamIsXbox1TitleId(title).get()
}

/// The running title, `None` for none.
pub fn current_title() -> Option<NonZeroU32> {
    NonZeroU32::new(xam::XamGetCurrentTitleId())
}

/// The cached name of `title` in `buffer`, NUL-terminated by XAM. Returns
/// the units before the terminator.
pub fn cached_title_name(title: u32, buffer: &mut [u16]) -> Result<usize, Win32Error> {
    let mut length = u32::try_from(buffer.len()).map_err(|_| Win32Error::INVALID_PARAMETER)?;
    // SAFETY: XAM writes at most `length` units to `buffer`.
    unsafe { xam::XamGetCachedTitleName(title, buffer.as_mut_ptr(), &mut length) }.result()?;
    Ok(buffer.iter().position(|&unit| unit == 0).unwrap_or(buffer.len()))
}

/// `XamFormatDateString(user, time, buffer)` of the local FILETIME `time`.
/// Returns the units before the terminator.
pub fn format_date(user: u32, time: u64, buffer: &mut [u16]) -> usize {
    // Pointers are 32-bit, so a slice length fits in `u32`.
    let capacity = buffer.len() as u32;
    // SAFETY: XAM writes at most `capacity` units to `buffer`.
    unsafe { xam::XamFormatDateString(user, time, buffer.as_mut_ptr(), capacity) };
    buffer.iter().position(|&unit| unit == 0).unwrap_or(buffer.len())
}

/// The PNG of the default gamer picture, in XAM memory. `None` when XAM
/// reports none.
pub fn default_system_image() -> Option<&'static [u8]> {
    let mut image = core::ptr::null();
    let mut size = 0;
    // SAFETY: `image` and `size` are live locals.
    unsafe { xam::XamGetDefaultSystemImage(&mut image, &mut size) };
    if image.is_null() || size == 0 {
        return None;
    }
    // SAFETY: XAM keeps the image resident for the life of the system.
    // [INFERENCE] The dashboard keeps the address without copying it.
    Some(unsafe { core::slice::from_raw_parts(image, size as usize) })
}
