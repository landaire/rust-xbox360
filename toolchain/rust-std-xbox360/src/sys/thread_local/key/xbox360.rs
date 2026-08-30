//! TLS keys for the Xbox 360 devkit, over the kernel `Tls*` API.
//!
//! The kernel provides no per-key destructor, so registered destructors are not
//! run at thread exit (thread-local values with `Drop` leak when a thread ends).
//! Get/set are correct per-thread, which is what matters for correctness.

use crate::sys::pal::abi;

pub type Key = u32;

const TLS_OUT_OF_INDEXES: u32 = 0xFFFF_FFFF;

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let key = unsafe { abi::TlsAlloc() };
    if key == TLS_OUT_OF_INDEXES {
        rtabort!("out of TLS keys");
    }
    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    unsafe { abi::TlsSetValue(key, value) };
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    unsafe { abi::TlsGetValue(key) }
}

#[inline]
pub unsafe fn destroy(key: Key) {
    unsafe { abi::TlsFree(key) };
}
