//! 64-bit atomics in the interrupt-protected form the kernel loader accepts.
//!
//! The target's widest native atomic is 32 bits. `core::sync::atomic`
//! provides those, and the standard library overlay of this toolchain
//! routes their read-modify-write operations through the protected form,
//! so 8-, 16- and 32-bit atomics need nothing from this module.

use core::cell::UnsafeCell;
use core::sync::atomic::Ordering;
use core::sync::atomic::fence;

use xbox360_sys::interlocked;

/// A `u64` shared between threads. Every operation is sequentially
/// consistent.
#[repr(C, align(8))]
pub struct AtomicU64 {
    value: UnsafeCell<u64>,
}

// SAFETY: every access is a single doubleword access or a reservation
// sequence.
unsafe impl Sync for AtomicU64 {}

impl AtomicU64 {
    pub const fn new(value: u64) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> u64 {
        self.value.into_inner()
    }

    pub fn get_mut(&mut self) -> &mut u64 {
        self.value.get_mut()
    }

    pub fn load(&self) -> u64 {
        fence(Ordering::SeqCst);
        // SAFETY: the value is valid and 8-byte aligned.
        let value = unsafe { interlocked::load64(self.value.get()) };
        fence(Ordering::SeqCst);
        value
    }

    pub fn store(&self, value: u64) {
        fence(Ordering::SeqCst);
        // SAFETY: the value is valid and 8-byte aligned.
        unsafe { interlocked::store64(self.value.get(), value) };
        fence(Ordering::SeqCst);
    }

    /// Store `new` if the value is `current`. `Ok` with the previous value
    /// on a store, `Err` with the observed value otherwise.
    pub fn compare_exchange(&self, current: u64, new: u64) -> Result<u64, u64> {
        fence(Ordering::SeqCst);
        // SAFETY: the value is valid and 8-byte aligned.
        let observed = unsafe { interlocked::compare_exchange64(self.value.get(), current, new) };
        fence(Ordering::SeqCst);
        if observed == current {
            Ok(observed)
        } else {
            Err(observed)
        }
    }

    /// Store the result of `update` applied to the value. Returns the
    /// previous value.
    fn fetch_update(&self, update: impl Fn(u64) -> u64) -> u64 {
        let mut current = self.load();
        loop {
            match self.compare_exchange(current, update(current)) {
                Ok(previous) => return previous,
                Err(observed) => current = observed,
            }
        }
    }

    /// Add `delta`, wrapping on overflow. Returns the previous value.
    pub fn fetch_add(&self, delta: u64) -> u64 {
        self.fetch_update(|value| value.wrapping_add(delta))
    }

    /// Store `value`. Returns the previous value.
    pub fn swap(&self, value: u64) -> u64 {
        self.fetch_update(|_| value)
    }
}

impl core::fmt::Debug for AtomicU64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.load().fmt(f)
    }
}
