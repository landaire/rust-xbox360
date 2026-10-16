//! TLS keys for the Xbox 360 devkit, over the kernel `KeTls*` API.
//!
//! The kernel provides no per-key destructor, so registered destructors are not
//! run at thread exit (thread-local values with `Drop` leak when a thread ends).
//! Get/set are correct per-thread, which is what matters for correctness.
//!
//! The kernel gives a title as many slots as the TLS slot count in its XEX
//! header. A title without that header has none.

use crate::ptr;

use xbox360_sys::xboxkrnl::{self, TlsIndex};

/// The `TlsIndex` value: the racy `LazyKey` stores keys as integers.
pub type Key = u32;

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let key = xboxkrnl::KeTlsAlloc();
    if key == TlsIndex::OUT_OF_INDEXES {
        rtabort!("out of TLS keys");
    }
    key.0
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // SAFETY: `key` was returned by `create` and has not been destroyed.
    unsafe { xboxkrnl::KeTlsSetValue(TlsIndex(key), value.expose_provenance() as u32) };
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    ptr::with_exposed_provenance_mut(xboxkrnl::KeTlsGetValue(TlsIndex(key)) as usize)
}

#[inline]
pub unsafe fn destroy(key: Key) {
    // SAFETY: `key` was returned by `create` and is not used after this.
    unsafe { xboxkrnl::KeTlsFree(TlsIndex(key)) };
}
