use core::marker::PhantomData;
use core::ops::Deref;
use core::sync::atomic::AtomicU32;

use xbox360_sys::xboxkrnl;

use super::anchor::anchor_at;

/// A recursive kernel critical section (`RTL_CRITICAL_SECTION`) around a
/// `T`. Holding it also holds a critical region, which disables normal
/// kernel APC delivery to the thread. The owning thread may enter it again,
/// so the guard gives shared access only. Use a `Cell` or `RefCell` for
/// changes.
pub struct CriticalSection<T: ?Sized = ()> {
    section: xboxkrnl::RtlCriticalSection,
    anchor: AtomicU32,
    value: T,
}

// SAFETY: one thread at a time holds the section and with it `&T`.
unsafe impl<T: ?Sized + Send> Sync for CriticalSection<T> {}

impl<T> CriticalSection<T> {
    pub const fn new(value: T) -> Self {
        Self {
            section: xboxkrnl::RtlCriticalSection::new(),
            anchor: AtomicU32::new(0),
            value,
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: ?Sized> CriticalSection<T> {
    fn section(&self) -> *mut xboxkrnl::RtlCriticalSection {
        let section = self.section.as_ptr();

        anchor_at(&self.anchor, section as usize as u32, || {
            // SAFETY: the storage belongs to `self`, and no other thread
            // uses it while the anchor is busy.
            unsafe { xboxkrnl::RtlInitializeCriticalSection(section) };
        });

        section
    }

    /// Enter the section, waiting for another owner to leave.
    pub fn lock(&self) -> CriticalSectionGuard<'_, T> {
        let section = self.section();

        // SAFETY: the section was initialized at its current address, and
        // the guard's Drop leaves it and the critical region.
        unsafe {
            xboxkrnl::KeEnterCriticalRegion();
            xboxkrnl::RtlEnterCriticalSection(section);
        }

        CriticalSectionGuard {
            lock: self,
            _thread: PhantomData,
        }
    }

    /// Enter the section if no other thread owns it.
    pub fn try_lock(&self) -> Option<CriticalSectionGuard<'_, T>> {
        let section = self.section();

        // SAFETY: the critical region is left below or by the guard's Drop.
        unsafe { xboxkrnl::KeEnterCriticalRegion() };

        // SAFETY: the section was initialized at its current address.
        if unsafe { xboxkrnl::RtlTryEnterCriticalSection(section) }.get() {
            return Some(CriticalSectionGuard {
                lock: self,
                _thread: PhantomData,
            });
        }

        // SAFETY: leaves the critical region entered above.
        unsafe { xboxkrnl::KeLeaveCriticalRegion() };

        None
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// Ownership of a [`CriticalSection`] by the current thread.
pub struct CriticalSectionGuard<'a, T: ?Sized> {
    lock: &'a CriticalSection<T>,
    /// The owning thread must leave the section.
    _thread: PhantomData<*const ()>,
}

impl<T: ?Sized> Deref for CriticalSectionGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.lock.value
    }
}

impl<T: ?Sized> Drop for CriticalSectionGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: the guard proves this thread entered the section and the
        // critical region; the section has not moved while borrowed.
        unsafe {
            xboxkrnl::RtlLeaveCriticalSection(self.lock.section.as_ptr());
            xboxkrnl::KeLeaveCriticalRegion();
        }
    }
}
