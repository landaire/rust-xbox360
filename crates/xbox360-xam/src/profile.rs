//! Profile enumeration and profile settings drives.
//!
//! Checked against the call sites of the 2.0.6717 dashboard and Xenia: the
//! ordinals (0x231 `XamProfileCreateEnumerator`, 0x232 `XamProfileEnumerate`,
//! 0x237 `XamProfileOpen`, 0x238 `XamProfileClose`), the argument order and
//! the 0x188-byte enumeration record. The dashboard enumerates with an
//! overlapped block and polls it from a timer, one profile per poll; this
//! module does the same.

use core::cell::UnsafeCell;
use core::ffi::CStr;
use core::marker::PhantomPinned;
use core::pin::Pin;

use xbox360_status::Hresult;
use xbox360_status::KernelError;
use xbox360_status::Win32Error;
use xbox360_status::Win32ErrorCode;
use xbox360_sys::xam;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::Handle;

use super::overlapped::OverlappedCall;

pub use xbox360_sys::xam::ProfileEnumResult;

/// Why a profile enumeration did not start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EnumerationError {
    /// `XamProfileCreateEnumerator` failed.
    Xam(Win32Error),
    /// The completion event could not be created.
    Event(KernelError),
}

/// Result of one [`ProfileEnumeration::poll`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumerationStep {
    /// The request has not completed yet.
    Pending,
    /// The next profile, by its offline XUID.
    Profile { xuid_offline: u64 },
    /// No profile remains.
    Finished,
    /// A request completed without pending; the dashboard ends the
    /// enumeration without taking the count.
    Stopped,
    /// The request completed with this extended error; the next poll issues
    /// another request.
    Failed(Hresult),
}

/// An enumeration of the profiles on all storage devices, advanced by
/// polling as the dashboard's profile count timer (0x921A5ED0) does. XAM
/// writes into the value while a request is pending, so it is pinned;
/// dropping it waits for a pending request.
pub struct ProfileEnumeration {
    handle: Handle,
    call: OverlappedCall,
    record: UnsafeCell<ProfileEnumResult>,
    pending: bool,
    _pinned: PhantomPinned,
}

impl ProfileEnumeration {
    /// `XamProfileCreateEnumerator(0, &handle)`: device 0 enumerates every
    /// device, as the dashboard does.
    pub fn start() -> Result<Self, EnumerationError> {
        const ALL_DEVICES: u32 = 0;

        let call = OverlappedCall::new().map_err(EnumerationError::Event)?;
        let mut handle = Handle::NULL;
        // SAFETY: `handle` is a live local.
        unsafe { xam::XamProfileCreateEnumerator(ALL_DEVICES, &mut handle) }
            .result()
            .map_err(EnumerationError::Xam)?;

        Ok(Self {
            handle,
            call,
            record: UnsafeCell::new(ProfileEnumResult::new()),
            pending: false,
            _pinned: PhantomPinned,
        })
    }

    /// Issues the next request, or checks the pending one without waiting.
    /// A completed request is judged by its extended error, as the
    /// dashboard reads it: 0 is a profile, `HRESULT_FROM_WIN32(
    /// ERROR_NO_MORE_FILES)` the end. Its result field then holds
    /// `ERROR_FUNCTION_FAILED` for every failure.
    pub fn poll(self: Pin<&mut Self>) -> EnumerationStep {
        // The dashboard's flags argument.
        const FLAGS: u32 = 2;
        const NO_MORE_FILES: Hresult = Hresult::from_win32(Win32Error::NO_MORE_FILES);

        // SAFETY: nothing is moved out of the pinned value.
        let this = unsafe { self.get_unchecked_mut() };

        if !this.pending {
            this.call.reset();
            // SAFETY: `handle` is the open enumerator, and `record` and the
            // overlapped block live at pinned addresses until the request
            // completes (see `Drop`).
            let code = unsafe {
                xam::XamProfileEnumerate(this.handle, FLAGS, this.record.get(), this.call.as_ptr())
            };
            if code.error() != Some(Win32Error::IO_PENDING) {
                return EnumerationStep::Stopped;
            }
            this.pending = true;
            return EnumerationStep::Pending;
        }

        let block = this.call.as_ptr();
        // SAFETY: the block stays valid while pinned; XAM writes the result
        // last, so a value other than pending means it is done.
        let result = unsafe { core::ptr::read_volatile(&raw const (*block).result) };
        if result.error() == Some(Win32Error::IO_PENDING) {
            return EnumerationStep::Pending;
        }
        // Orders the record reads after the result XAM wrote last.
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        this.pending = false;
        // SAFETY: the request completed, so XAM no longer writes the block.
        let extended =
            Hresult(unsafe { core::ptr::read_volatile(&raw const (*block).extended_error) });

        match extended {
            Hresult::SUCCESS => {
                // SAFETY: the request completed, so XAM no longer writes
                // `record`.
                EnumerationStep::Profile {
                    xuid_offline: unsafe { (*this.record.get()).xuid_offline },
                }
            }
            NO_MORE_FILES => EnumerationStep::Finished,
            error => EnumerationStep::Failed(error),
        }
    }
}

impl Drop for ProfileEnumeration {
    fn drop(&mut self) {
        if self.pending {
            // XAM still owns `record` and the block; wait for it.
            let _ = self
                .call
                .complete(Win32ErrorCode(Win32Error::IO_PENDING.code()));
        }
        // SAFETY: `handle` is open and not used after this.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}

/// The settings of one profile mounted as a drive. Unmounted on drop.
pub struct ProfileDrive<'a> {
    name: &'a CStr,
}

impl ProfileDrive<'_> {
    pub fn name(&self) -> &CStr {
        self.name
    }
}

/// `XamProfileOpen`: mounts the settings of the profile `xuid` as the drive
/// `name`, such as `DASHUSER`.
pub fn open(xuid: u64, name: &CStr) -> Result<ProfileDrive<'_>, Win32Error> {
    // SAFETY: `name` is NUL-terminated; the dashboard passes 0 and null for
    // the remaining arguments.
    unsafe { xam::XamProfileOpen(xuid, name.as_ptr().cast(), 0, core::ptr::null_mut()) }
        .result()?;
    Ok(ProfileDrive { name })
}

impl Drop for ProfileDrive<'_> {
    fn drop(&mut self) {
        // SAFETY: `name` is NUL-terminated and names the mounted drive.
        let _ = unsafe { xam::XamProfileClose(self.name.as_ptr().cast()) };
    }
}
