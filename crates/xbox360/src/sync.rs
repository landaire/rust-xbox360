//! Kernel events, critical sections, spin locks, IRQL and interrupt control.
//!
//! Titles that use the standard library use `std::sync` for locks and
//! condition variables. The types here are for `no_std` code and for code
//! that shares kernel objects with the kernel or other modules.
//!
//! [`Event`] and [`CriticalSection`] hold kernel objects whose wait list
//! points into the object itself. Each records the address it was set up
//! at and repeats the setup on first use at a new address. A move needs
//! exclusive access, so no thread waits on or holds the object while it
//! moves.

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;
use core::time::Duration;

use xbox360_sys::processor;
use xbox360_sys::xboxkrnl;

use crate::status::KernelError;
use crate::status::NtStatus;

pub use xbox360_sys::xboxkrnl::EventType;
pub use xbox360_sys::xboxkrnl::Irql;
pub use xbox360_sys::xboxkrnl::ProcessorMode;
pub use xbox360_sys::xboxkrnl::WaitReason;

/// The `alertable` argument of a wait that APCs do not end.
pub(crate) const NOT_ALERTABLE: bool = false;
/// `KeSetEvent` `wait` argument: no wait call follows at raised IRQL.
const NO_WAIT_FOLLOWS: bool = false;
/// Anchor value while one thread sets up an object. Object addresses are
/// 4-byte aligned, so it never equals one.
const ANCHOR_BUSY: u32 = 1;

/// How a wait with a timeout ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaitOutcome {
    Signaled,
    TimedOut,
}

/// A wait that did not end in a way the caller asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WaitError {
    /// The wait failed.
    Kernel(KernelError),
    /// The wait ended with a success status that this wait does not
    /// produce, such as a timeout of a wait without one.
    Unexpected(NtStatus),
}

impl core::fmt::Display for WaitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Kernel(error) => error.fmt(f),
            Self::Unexpected(status) => write!(f, "wait ended with status {:#010x}", status.0),
        }
    }
}

impl core::error::Error for WaitError {}

/// The outcome of a non-alertable wait with a timeout.
pub(crate) fn wait_outcome(status: NtStatus) -> Result<WaitOutcome, WaitError> {
    match status {
        NtStatus::SUCCESS => Ok(WaitOutcome::Signaled),
        NtStatus::TIMEOUT => Ok(WaitOutcome::TimedOut),
        other => Err(match KernelError::from_status(other) {
            Some(error) => WaitError::Kernel(error),
            None => WaitError::Unexpected(other),
        }),
    }
}

/// The outcome of a non-alertable wait without a timeout.
pub(crate) fn wait_signaled(status: NtStatus) -> Result<(), WaitError> {
    match wait_outcome(status)? {
        WaitOutcome::Signaled => Ok(()),
        WaitOutcome::TimedOut => Err(WaitError::Unexpected(NtStatus::TIMEOUT)),
    }
}

/// Run `setup` once for the current address of an object, recorded in
/// `anchor`. Other users of the object at the same address wait for it.
fn anchor_at(anchor: &AtomicU32, address: u32, setup: impl FnOnce()) {
    loop {
        let current = anchor.load(Ordering::Acquire);
        if current == address {
            return;
        }
        if current == ANCHOR_BUSY {
            core::hint::spin_loop();
            continue;
        }
        if anchor
            .compare_exchange(current, ANCHOR_BUSY, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            setup();
            anchor.store(address, Ordering::Release);
            return;
        }
    }
}

/// A kernel event (`KEVENT`) in static or owned storage.
pub struct Event {
    /// Dispatcher header: type, signal state, wait list head. Kept as words
    /// so that `new` can set the type and state in a const context.
    header: UnsafeCell<[u32; 4]>,
    anchor: AtomicU32,
}

// SAFETY: the kernel synchronizes access to dispatcher objects, and
// `anchor_at` serializes the setup of the wait list head.
unsafe impl Sync for Event {}

impl Event {
    pub const fn new(kind: EventType, signaled: bool) -> Self {
        // The type is the first byte of the header word.
        Self {
            header: UnsafeCell::new([kind.raw() << 24, signaled as u32, 0, 0]),
            anchor: AtomicU32::new(0),
        }
    }

    /// The `KEVENT` for kernel calls that take one. It is valid while
    /// `self` is borrowed.
    pub fn as_ptr(&self) -> *mut xboxkrnl::Kevent {
        let header = self.header.get().cast::<u32>();
        anchor_at(&self.anchor, header as usize as u32, || {
            let list = header.wrapping_add(2) as usize as u32;
            // SAFETY: no other thread uses the event while the anchor is
            // busy, and the kernel sees the header only after the anchor
            // store that publishes it; the signal state is kept.
            unsafe {
                header.add(2).write(list);
                header.add(3).write(list);
            }
        });
        header.cast()
    }

    pub fn set(&self) {
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the call.
        unsafe { xboxkrnl::KeSetEvent(self.as_ptr(), xboxkrnl::EVENT_INCREMENT, NO_WAIT_FOLLOWS) };
    }

    pub fn reset(&self) {
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the call.
        unsafe { xboxkrnl::KeResetEvent(self.as_ptr()) };
    }

    /// Wait until the event is signaled.
    pub fn wait(&self) -> Result<(), WaitError> {
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the wait; a null timeout waits
        // without limit.
        wait_signaled(unsafe {
            xboxkrnl::KeWaitForSingleObject(
                self.as_ptr().cast(),
                WaitReason::Executive,
                ProcessorMode::Kernel,
                NOT_ALERTABLE,
                core::ptr::null(),
            )
        })
    }

    /// Wait until the event is signaled or `timeout` elapses. A zero
    /// timeout tests the state without waiting.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<WaitOutcome, WaitError> {
        let interval = crate::time::relative_interval(timeout);
        // SAFETY: `as_ptr` returns this event's header, set up at its
        // current address and borrowed for the wait; `interval` is a live
        // local.
        wait_outcome(unsafe {
            xboxkrnl::KeWaitForSingleObject(
                self.as_ptr().cast(),
                WaitReason::Executive,
                ProcessorMode::Kernel,
                NOT_ALERTABLE,
                &interval,
            )
        })
    }
}

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

/// The IRQL of the current processor.
pub fn current_irql() -> Irql {
    processor::current_irql()
}

/// A raised IRQL, lowered to the previous level on drop. Waiting while it
/// is above [`Irql::APC`] stops the system with a bug check. Drop guards
/// and spin lock guards in the reverse order of creation, or the IRQL ends
/// at the level of the guard dropped last.
pub struct IrqlGuard {
    previous: Irql,
    _thread: PhantomData<*const ()>,
}

impl IrqlGuard {
    /// Raise the IRQL to `level`. `None` if `level` is below the current
    /// IRQL, which the kernel does not allow.
    pub fn raise(level: Irql) -> Option<Self> {
        if level < current_irql() {
            return None;
        }
        // SAFETY: `level` is not below the current IRQL (checked above), and
        // Drop restores the previous level.
        let previous = unsafe { xboxkrnl::KfRaiseIrql(level) };
        Some(Self {
            previous,
            _thread: PhantomData,
        })
    }

    pub const fn previous(&self) -> Irql {
        self.previous
    }
}

impl Drop for IrqlGuard {
    fn drop(&mut self) {
        // SAFETY: restores the level that KfRaiseIrql returned.
        unsafe { xboxkrnl::KfLowerIrql(self.previous) };
    }
}

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

/// External interrupts disabled on the current processor. Drop restores
/// the external interrupt and recoverable interrupt bits of the MSR as
/// they were.
pub struct InterruptGuard {
    saved: u32,
    _thread: PhantomData<*const ()>,
}

impl InterruptGuard {
    /// Disable external interrupts.
    ///
    /// # Safety
    ///
    /// Until the guard drops, the thread must not wait, call a kernel
    /// routine that may wait, or fault. Guards must drop in the reverse
    /// order of creation.
    pub unsafe fn new() -> Self {
        let saved: u32;
        // SAFETY: reads the machine state register.
        unsafe {
            core::arch::asm!("mfmsr {0}", out(reg) saved, options(nomem, nostack, preserves_flags))
        };
        // SAFETY: the caller upholds the contract until the guard drops.
        unsafe { processor::disable_interrupts() };
        Self {
            saved,
            _thread: PhantomData,
        }
    }
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        // SAFETY: `mtmsrd r9, 1` changes only EE and RI, back to the values
        // saved on this processor; the guard cannot leave the thread. The
        // target CPU model has no `mtmsrd` mnemonic.
        unsafe { core::arch::asm!(".long 0x7d210164", in("r9") self.saved, options(nostack)) };
    }
}
