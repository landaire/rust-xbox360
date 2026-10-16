//! Futex for the Xbox 360 over a wait-on-address table.
//!
//! The kernel has no futex, keyed event or wait-on-address service. A waiter
//! links a node into the bucket that its futex address hashes to and blocks
//! on a kernel event in that node, which lives on the waiter's stack. A wake
//! unlinks matching nodes and sets their events while it holds the bucket
//! spin lock. A waiter whose wait ends without the event learns under the
//! same lock whether a wake took its node, so no event is set after its
//! frame is gone.

use crate::cell::{Cell, UnsafeCell};
use crate::ptr;
use crate::sync::atomic::Atomic;
use crate::sync::atomic::Ordering::Relaxed;
use crate::sys::pal::WaitInterval;
use crate::time::Duration;

use xbox360_sys::xboxkrnl::{
    self, DispatcherObject, EventType, Irql, Kevent, NtStatus, ProcessorMode, WaitReason,
};

/// An atomic for use as a futex that is at least 32-bits but may be larger
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex
pub type SmallPrimitive = u32;

const BUCKET_BITS: u32 = 6;
const BUCKET_COUNT: usize = 1 << BUCKET_BITS;

/// A blocked `futex_wait` call. It must not move while queued.
struct Waiter {
    event: Kevent,
    futex: *const Futex,
    previous: Cell<*const Waiter>,
    next: Cell<*const Waiter>,
    queued: Cell<bool>,
}

/// Waiters on the futex addresses that hash to one bucket, oldest first.
/// The 128-byte alignment keeps each spin lock on its own cache line.
#[repr(align(128))]
struct Bucket {
    /// Kernel spin lock (`KSPIN_LOCK`) that guards the other fields and the
    /// links of every queued waiter. Raw storage, because `KspinLock::new` is
    /// not const-stable within std.
    lock: UnsafeCell<u32>,
    head: Cell<*const Waiter>,
    tail: Cell<*const Waiter>,
}

// SAFETY: the queue fields are accessed only while the spin lock is held.
unsafe impl Sync for Bucket {}

impl Bucket {
    const fn new() -> Bucket {
        Bucket {
            lock: UnsafeCell::new(0),
            head: Cell::new(ptr::null()),
            tail: Cell::new(ptr::null()),
        }
    }

    fn lock(&self) -> Queue<'_> {
        // SAFETY: `self.lock` is a static spin lock, released only by `Queue::drop`.
        let previous_irql = unsafe { xboxkrnl::KfAcquireSpinLock(self.lock.get().cast()) };
        Queue { bucket: self, previous_irql }
    }
}

static BUCKETS: [Bucket; BUCKET_COUNT] = [const { Bucket::new() }; BUCKET_COUNT];

fn bucket(futex: &Futex) -> &'static Bucket {
    // Fibonacci hashing of the word address.
    let word = (ptr::from_ref(futex).addr() >> 2) as u32;
    let index = word.wrapping_mul(0x9E37_79B9) >> (u32::BITS - BUCKET_BITS);
    &BUCKETS[index as usize]
}

/// A bucket whose spin lock is held. The processor runs at DISPATCH_LEVEL
/// until the guard drops.
struct Queue<'a> {
    bucket: &'a Bucket,
    previous_irql: Irql,
}

impl Queue<'_> {
    fn push_back(&self, waiter: &Waiter) {
        let tail = self.bucket.tail.get();
        waiter.previous.set(tail);
        waiter.next.set(ptr::null());
        waiter.queued.set(true);
        // SAFETY: the tail is a queued waiter, which stays alive until it is unlinked.
        match unsafe { tail.as_ref() } {
            Some(tail) => tail.next.set(waiter),
            None => self.bucket.head.set(waiter),
        }
        self.bucket.tail.set(waiter);
    }

    fn remove(&self, waiter: &Waiter) {
        let previous = waiter.previous.get();
        let next = waiter.next.get();
        // SAFETY: queued waiters stay alive until they are unlinked, and their
        // links change only under this lock.
        match unsafe { previous.as_ref() } {
            Some(previous) => previous.next.set(next),
            None => self.bucket.head.set(next),
        }
        match unsafe { next.as_ref() } {
            Some(next) => next.previous.set(previous),
            None => self.bucket.tail.set(previous),
        }
        waiter.queued.set(false);
    }

    /// Unlink `waiter` and set its event. The waiter may return as soon as
    /// the event is set, so `waiter` must not be used afterwards.
    ///
    /// # Safety
    ///
    /// `waiter` must be queued in this bucket.
    unsafe fn wake(&self, waiter: &Waiter) {
        self.remove(waiter);
        // SAFETY: the waiter's event was initialized by `futex_wait`, and the waiter
        // stays alive while queued; it is unlinked above under the lock.
        unsafe { xboxkrnl::KeSetEvent(waiter.event.as_ptr(), xboxkrnl::EVENT_INCREMENT, false) };
    }
}

impl Drop for Queue<'_> {
    fn drop(&mut self) {
        // SAFETY: this guard holds the lock, acquired at `previous_irql` in `Bucket::lock`.
        unsafe { xboxkrnl::KfReleaseSpinLock(self.bucket.lock.get().cast(), self.previous_irql) };
    }
}

/// Wait while `futex` holds `expected`, for at most `timeout`. Returns
/// `false` only when the timeout expired.
pub fn futex_wait(futex: &Futex, expected: u32, timeout: Option<Duration>) -> bool {
    // A timeout beyond the interval range waits without limit.
    let interval = timeout.and_then(WaitInterval::relative);
    let waiter = Waiter {
        event: Kevent::new(),
        futex,
        previous: Cell::new(ptr::null()),
        next: Cell::new(ptr::null()),
        queued: Cell::new(false),
    };
    // SAFETY: `waiter.event` is live storage for a KEVENT, and `waiter` does not move
    // after this.
    unsafe { xboxkrnl::KeInitializeEvent(waiter.event.as_ptr(), EventType::Notification, false) };

    let bucket = bucket(futex);
    {
        let queue = bucket.lock();
        // A wake changes the value before it takes the lock, so the value
        // read under the lock is current or the wake will find the node.
        if futex.load(Relaxed) != expected {
            return true;
        }
        queue.push_back(&waiter);
    }

    // A null timeout waits without limit.
    // SAFETY: the event was initialized above and `waiter` outlives the wait; the
    // interval, if any, is a live local.
    let status = unsafe {
        xboxkrnl::KeWaitForSingleObject(
            waiter.event.as_ptr().cast::<DispatcherObject>(),
            WaitReason::Executive,
            ProcessorMode::Kernel,
            false,
            interval.as_ref().map_or(ptr::null(), WaitInterval::as_ptr),
        )
    };
    if status == NtStatus::SUCCESS {
        // Only a wake sets the event, and it unlinked the node first.
        return true;
    }

    let queue = bucket.lock();
    if waiter.queued.get() {
        queue.remove(&waiter);
        status != NtStatus::TIMEOUT
    } else {
        // A wake took the node after the wait ended and set the event while
        // it held the lock.
        true
    }
}

/// Wake one waiter on `futex`. Returns whether a waiter was woken.
pub fn futex_wake(futex: &Futex) -> bool {
    let queue = bucket(futex).lock();
    let mut node = queue.bucket.head.get();
    // SAFETY: queued waiters stay alive until they are unlinked.
    while let Some(waiter) = unsafe { node.as_ref() } {
        if ptr::eq(waiter.futex, futex) {
            // SAFETY: `waiter` is queued in this bucket and is not used again.
            unsafe { queue.wake(waiter) };
            return true;
        }
        node = waiter.next.get();
    }
    false
}

/// Wake every waiter on `futex`.
pub fn futex_wake_all(futex: &Futex) {
    let queue = bucket(futex).lock();
    let mut node = queue.bucket.head.get();
    // SAFETY: queued waiters stay alive until they are unlinked.
    while let Some(waiter) = unsafe { node.as_ref() } {
        node = waiter.next.get();
        if ptr::eq(waiter.futex, futex) {
            // SAFETY: `waiter` is queued in this bucket and is not used again.
            unsafe { queue.wake(waiter) };
        }
    }
}
