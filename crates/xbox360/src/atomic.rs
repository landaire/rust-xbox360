//! Atomics in the interrupt-protected form the kernel loader accepts.
//!
//! The target's widest native atomic is 32 bits. `core::sync::atomic`
//! provides those, and the standard library overlay of this toolchain
//! routes their read-modify-write operations through the protected form as
//! `sync`/`isync` compare-and-swap loops, whatever the ordering argument.
//! [`Interlocked32`] gives 32-bit atomics single reservation loops without
//! barriers instead, and [`AtomicU64`] provides the 64-bit type.

use core::cell::UnsafeCell;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;
use core::sync::atomic::fence;

use xbox360_sys::interlocked;

/// Read-modify-write operations on [`AtomicU32`] through
/// [`xbox360_sys::interlocked`]: one reservation loop with no trailing
/// barrier. The operations of `AtomicU32` itself are `sync`/`isync`
/// compare-and-swap loops on this toolchain.
pub trait Interlocked32 {
    /// Add `delta`, wrapping on overflow. Returns the previous value.
    /// Relaxed: no barrier.
    fn fetch_add_relaxed(&self, delta: u32) -> u32;

    /// Store `value`. Returns the previous value. Relaxed: no barrier.
    fn swap_relaxed(&self, value: u32) -> u32;

    /// Store `new` if the value is `current`. `Ok` with the previous value
    /// on a store, `Err` with the observed value otherwise. Release: a
    /// leading `lwsync`, no barrier after the store.
    fn compare_exchange_release(&self, current: u32, new: u32) -> Result<u32, u32>;
}

impl Interlocked32 for AtomicU32 {
    #[inline]
    fn fetch_add_relaxed(&self, delta: u32) -> u32 {
        // SAFETY: the reference makes the word valid and 4-byte aligned.
        unsafe { interlocked::fetch_add32(self.as_ptr(), delta) }
    }

    #[inline]
    fn swap_relaxed(&self, value: u32) -> u32 {
        // SAFETY: the reference makes the word valid and 4-byte aligned.
        unsafe { interlocked::exchange32(self.as_ptr(), value) }
    }

    #[inline]
    fn compare_exchange_release(&self, current: u32, new: u32) -> Result<u32, u32> {
        // SAFETY: the reference makes the word valid and 4-byte aligned.
        let observed = unsafe { interlocked::compare_exchange32(self.as_ptr(), current, new) };
        if observed == current {
            Ok(observed)
        } else {
            Err(observed)
        }
    }
}

/// A `u64` shared between threads. The operations without an ordering
/// suffix are sequentially consistent. The `_relaxed` and `_release`
/// operations are one doubleword access or one reservation loop, with only
/// the barriers their documentation names.
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

    /// View the doubleword at `ptr` as an atomic, as
    /// `AtomicU32::from_ptr` does for words.
    ///
    /// # Safety
    ///
    /// `ptr` must be 8-byte aligned and valid for reads and writes for all
    /// of `'a`. During `'a`, every access to the doubleword must be atomic,
    /// and no access may use a different size.
    #[inline]
    pub const unsafe fn from_ptr<'a>(ptr: *mut u64) -> &'a Self {
        // SAFETY: `Self` has the size of `u64` and an alignment of 8; the
        // caller guarantees alignment, validity and atomic access for `'a`.
        unsafe { &*ptr.cast::<Self>() }
    }

    pub const fn as_ptr(&self) -> *mut u64 {
        self.value.get()
    }

    pub fn into_inner(self) -> u64 {
        self.value.into_inner()
    }

    pub fn get_mut(&mut self) -> &mut u64 {
        self.value.get_mut()
    }

    pub fn load(&self) -> u64 {
        fence(Ordering::SeqCst);
        let value = self.load_relaxed();
        fence(Ordering::SeqCst);
        value
    }

    /// Load in one doubleword access. Relaxed: no barrier.
    #[inline]
    pub fn load_relaxed(&self) -> u64 {
        // SAFETY: the value is valid and 8-byte aligned.
        unsafe { interlocked::load64(self.value.get()) }
    }

    pub fn store(&self, value: u64) {
        fence(Ordering::SeqCst);
        self.store_relaxed(value);
        fence(Ordering::SeqCst);
    }

    /// Store in one doubleword access. Relaxed: no barrier.
    #[inline]
    pub fn store_relaxed(&self, value: u64) {
        // SAFETY: the value is valid and 8-byte aligned.
        unsafe { interlocked::store64(self.value.get(), value) };
    }

    /// Store `new` if the value is `current`. `Ok` with the previous value
    /// on a store, `Err` with the observed value otherwise.
    pub fn compare_exchange(&self, current: u64, new: u64) -> Result<u64, u64> {
        fence(Ordering::SeqCst);
        let result = self.compare_exchange_release(current, new);
        fence(Ordering::SeqCst);
        result
    }

    /// Store `new` if the value is `current`. `Ok` with the previous value
    /// on a store, `Err` with the observed value otherwise. Release: a
    /// leading `lwsync`, no barrier after the store.
    #[inline]
    pub fn compare_exchange_release(&self, current: u64, new: u64) -> Result<u64, u64> {
        // SAFETY: the value is valid and 8-byte aligned.
        let observed = unsafe { interlocked::compare_exchange64(self.value.get(), current, new) };
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

    /// Add `delta`, wrapping on overflow, in one reservation loop. Returns
    /// the previous value. Relaxed: no barrier.
    #[inline]
    pub fn fetch_add_relaxed(&self, delta: u64) -> u64 {
        // SAFETY: the value is valid and 8-byte aligned.
        unsafe { interlocked::fetch_add64(self.value.get(), delta) }
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
