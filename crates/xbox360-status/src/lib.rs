//! Kernel status codes, Win32 error codes and result values, from
//! `xbox360-sys`.
//!
//! [`dos_error`], [`last_error`], [`set_last_error`] and
//! [`set_last_error_value`] exist only for `target_os = "xbox360"`.

#![no_std]

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use core::sync::atomic::Ordering;

pub use xbox360_sys::status::Facility;
pub use xbox360_sys::status::Hresult;
pub use xbox360_sys::status::KernelError;
pub use xbox360_sys::status::NotAFailure;
pub use xbox360_sys::status::NtStatus;
pub use xbox360_sys::status::Severity;
pub use xbox360_sys::status::UnlistedStatus;
pub use xbox360_sys::status::Win32Error;
pub use xbox360_sys::status::Win32ErrorCode;

/// The thread whose last error the functions of this module use: the
/// calling thread, or `None` while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
fn last_error_thread() -> Option<&'static xbox360_sys::xboxkrnl::Kthread> {
    let region = xbox360_sys::processor::Kpcr::current();
    if region.prcb.dpc_active.load(Ordering::Relaxed) != 0 {
        return None;
    }

    // SAFETY: the processor control region names the running thread, whose
    // object stays allocated while it runs.
    Some(unsafe { &*(xbox360_sys::processor::current_thread() as usize as *const xbox360_sys::xboxkrnl::Kthread) })
}

/// The Win32 error that `RtlNtStatusToDosError` makes of `error`. `None`
/// when the kernel translates it to `ERROR_SUCCESS`.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn dos_error(error: KernelError) -> Option<Win32Error> {
    xbox360_sys::xboxkrnl::RtlNtStatusToDosError(error.status()).error()
}

/// The calling thread's Win32 last error. `None` for `ERROR_SUCCESS` and
/// while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn last_error() -> Option<Win32Error> {
    let thread = last_error_thread()?;

    Win32Error::new(thread.last_error.load(Ordering::Relaxed))
}

/// Record a failed kernel call, translated by [`dos_error`], as the calling
/// thread's last error. No effect while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn set_last_error(error: KernelError) {
    set_last_error_value(dos_error(error));
}

/// Store a Win32 error, `None` for `ERROR_SUCCESS`, as the calling thread's
/// last error. No effect while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn set_last_error_value(error: Option<Win32Error>) {
    let Some(thread) = last_error_thread() else {
        return;
    };

    let code = Win32ErrorCode::from(error).0;
    thread.last_error.store(code, Ordering::Relaxed);
}

/// A wait that did not end in a way the caller asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WaitError {
    /// The wait failed.
    Kernel(KernelError),
    /// The wait ended with a success status that this wait does not
    /// produce, such as a timeout of a wait without one.
    Unexpected(NtStatus),
}

impl core::fmt::Display for WaitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Kernel(error) => error.fmt(f),
            Self::Unexpected(status) => write!(f, "wait ended with status {:#010x}", status.0),
        }
    }
}

impl core::error::Error for WaitError {}

/// Stop on a data import that the loader left unresolved. One shared copy
/// keeps the panic code out of the callers.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[cold]
#[inline(never)]
pub fn unresolved_import(name: &str) -> ! {
    panic!("the loader did not resolve {name}")
}
