use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;

use xbox360_sys::xboxkrnl;

use super::Irql;

/// A kernel spin lock around a `T`. Holding it raises the IRQL to
/// [`Irql::DISPATCH`], so the holder must not wait, fault on paged memory,
/// or take the lock again.
pub struct SpinLock<T: ?Sized> {
    lock: xboxkrnl::KspinLock,
    value: UnsafeCell<T>,
}

// SAFETY: every access to `value` holds the spin lock.
unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            lock: xboxkrnl::KspinLock::new(),
            value: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // SAFETY: the lock word belongs to `self`, and the guard's Drop
        // releases it with the returned IRQL.
        let previous = unsafe { xboxkrnl::KfAcquireSpinLock(self.lock.as_ptr()) };

        SpinLockGuard {
            lock: self,
            previous,
            _thread: PhantomData,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

/// Ownership of a [`SpinLock`]. Releases the lock and restores the IRQL on
/// drop.
pub struct SpinLockGuard<'a, T: ?Sized> {
    lock: &'a SpinLock<T>,
    previous: Irql,
    _thread: PhantomData<*const ()>,
}

impl<T: ?Sized> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: the spin lock is held.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T: ?Sized> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the spin lock is held.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T: ?Sized> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: the guard proves this thread holds the lock, acquired at
        // `previous`.
        unsafe { xboxkrnl::KfReleaseSpinLock(self.lock.lock.as_ptr(), self.previous) };
    }
}
