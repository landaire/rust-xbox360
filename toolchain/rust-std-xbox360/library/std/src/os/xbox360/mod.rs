//! Xbox 360-specific definitions.

#![unstable(feature = "xbox360_ext", issue = "none")]
#![doc(cfg(target_os = "xbox360"))]
#![forbid(unsafe_op_in_unsafe_fn)]

/// Returns the thread-local storage slots that the standard library of this
/// system module took from the system process.
///
/// The system process never exits, so it gets the slots back only through this
/// call. Call it last in the module's entry point for `ProcessDetach`.
///
/// # Safety
///
/// No thread may run code of the module during or after the call, and no
/// thread local of the module may be used after it, `println!` included.
#[cfg(xbox360_system)]
pub unsafe fn release_tls_slots() {
    // SAFETY: the caller guarantees the conditions of `release_slots`.
    unsafe { crate::sys::thread_local::key::release_slots() }
}
