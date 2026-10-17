//! Kernel status codes, Win32 error codes and result values, from
//! `xbox360-sys`.
//!
//! [`dos_error`], [`last_error`], [`set_last_error`] and
//! [`set_last_error_value`] exist only for `target_os = "xbox360"`.

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
fn last_error_thread() -> Option<crate::thread::ThreadObject<'static>> {
    let region = xbox360_sys::processor::Kpcr::current();
    if region.prcb.dpc_active.load(Ordering::Relaxed) != 0 {
        return None;
    }

    Some(crate::thread::current_object())
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

    Win32Error::new(thread.kthread().last_error.load(Ordering::Relaxed))
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
    thread.kthread().last_error.store(code, Ordering::Relaxed);
}
