//! Kernel status codes, Win32 error codes and result values, from
//! `xbox360-sys`.
//!
//! [`last_error`], [`set_last_error`] and [`set_last_error_value`] read and
//! write the calling thread's last error and exist only for
//! `target_os = "xbox360"`.

pub use xbox360_sys::status::*;

/// PCR offset of the flag that is nonzero while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
const PCR_DPC_ACTIVE: usize = 0x150;
/// PCR offset of the current thread object.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
const PCR_CURRENT_THREAD: usize = 0x100;
/// KTHREAD offset of the Win32 last error.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
const KTHREAD_LAST_ERROR: usize = 0x160;

/// Address of the current thread's last-error field, or `None` while a
/// DPC runs, as stock XBDM finds it.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
fn last_error_slot() -> Option<*mut u32> {
    let region = xbox360_sys::processor::pcr() as usize;
    // SAFETY: both PCR fields stay valid for the life of the processor.
    unsafe {
        if core::ptr::read_volatile((region + PCR_DPC_ACTIVE) as *const u32) != 0 {
            return None;
        }
        let thread = core::ptr::read_volatile((region + PCR_CURRENT_THREAD) as *const u32) as usize;
        Some((thread + KTHREAD_LAST_ERROR) as *mut u32)
    }
}

/// The calling thread's Win32 last error. `None` for `ERROR_SUCCESS` and
/// while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn last_error() -> Option<Win32Error> {
    let slot = last_error_slot()?;
    // SAFETY: the slot is a field of the current thread object.
    Win32Error::new(unsafe { core::ptr::read_volatile(slot) })
}

/// Record a failed kernel call, translated by `RtlNtStatusToDosError`, as
/// the calling thread's last error. No effect while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn set_last_error(error: KernelError) {
    let code = xbox360_sys::xboxkrnl::RtlNtStatusToDosError(error.status()).0;
    if let Some(slot) = last_error_slot() {
        // SAFETY: the slot is a field of the current thread object.
        unsafe { core::ptr::write_volatile(slot, code) };
    }
}

/// Store a Win32 error, `None` for `ERROR_SUCCESS`, as the calling thread's
/// last error. No effect while a DPC runs.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub fn set_last_error_value(error: Option<Win32Error>) {
    if let Some(slot) = last_error_slot() {
        let code = Win32ErrorCode::from(error).0;
        // SAFETY: the slot is a field of the current thread object.
        unsafe { core::ptr::write_volatile(slot, code) };
    }
}
