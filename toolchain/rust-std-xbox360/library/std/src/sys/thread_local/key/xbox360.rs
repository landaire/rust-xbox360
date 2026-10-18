//! TLS keys for the Xbox 360 devkit, over the kernel `KeTls*` API.
//!
//! The kernel provides no per-key destructor, so registered destructors are not
//! run at thread exit (thread-local values with `Drop` leak when a thread ends).
//! Get/set are correct per-thread, which is what matters for correctness.
//!
//! The kernel gives a title as many slots as the TLS slot count in its XEX
//! header. A title without that header has none.
//!
//! The system process never exits, so std built for a system module
//! (`--cfg xbox360_system`) records its keys, and `release_slots` returns them
//! when the module unloads.

use crate::ptr;
#[cfg(xbox360_system)]
use crate::sync::atomic::{AtomicU32, Ordering};

use xbox360_sys::xboxkrnl::{self, TlsIndex};

/// The `TlsIndex` value: the racy `LazyKey` stores keys as integers.
pub type Key = u32;

#[cfg(xbox360_system)]
const NO_KEY: Key = TlsIndex::OUT_OF_INDEXES.0;

// More entries than the system process has TLS slots, so `KeTlsAlloc` fails
// before the table is full.
#[cfg(xbox360_system)]
static KEYS: [AtomicU32; 64] = [const { AtomicU32::new(NO_KEY) }; 64];

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let key = xboxkrnl::KeTlsAlloc();
    if key == TlsIndex::OUT_OF_INDEXES {
        rtabort!("out of TLS keys");
    }
    #[cfg(xbox360_system)]
    record(key.0);
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
    #[cfg(xbox360_system)]
    unrecord(key);
    // SAFETY: `key` was returned by `create` and is not used after this.
    unsafe { xboxkrnl::KeTlsFree(TlsIndex(key)) };
}

#[cfg(xbox360_system)]
fn record(key: Key) {
    for entry in &KEYS {
        if entry.compare_exchange(NO_KEY, key, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            return;
        }
    }
    rtabort!("more TLS keys than the system process has");
}

#[cfg(xbox360_system)]
fn unrecord(key: Key) {
    for entry in &KEYS {
        if entry.compare_exchange(key, NO_KEY, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
            return;
        }
    }
}

/// Frees every key that `create` returned. The calling thread's values are
/// cleared first, since it is the loader thread that unloads the module.
///
/// # Safety
///
/// No thread may run code of the module during or after the call, and no
/// thread local of the module may be used after it.
#[cfg(xbox360_system)]
pub unsafe fn release_slots() {
    for entry in &KEYS {
        let key = entry.swap(NO_KEY, Ordering::AcqRel);
        if key == NO_KEY {
            continue;
        }

        // SAFETY: `key` came from `KeTlsAlloc` and was not freed, and the
        // caller guarantees that nothing uses it after this.
        unsafe {
            xboxkrnl::KeTlsSetValue(TlsIndex(key), 0);
            xboxkrnl::KeTlsFree(TlsIndex(key));
        }
    }
}
