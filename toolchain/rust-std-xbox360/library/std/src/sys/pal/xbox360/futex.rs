//! Futex for the Xbox 360 devkit.
//!
//! There is no kernel futex, so this is built on a single global auto-reset
//! kernel event used as a wakeup channel. `futex_wait` re-checks the futex word
//! and waits with a short bounded timeout, so a wake targeting a different futex
//! address (or a lost wake) is recovered within ~1ms. This is correct -- futex
//! semantics permit spurious wakeups and require the caller to re-check -- and
//! good enough for std's Mutex/Condvar/RwLock/Once/parking.

use crate::sync::atomic::Atomic;
use crate::sync::atomic::{AtomicU32, Ordering};
use crate::sys::pal::abi;
use crate::time::{Duration, Instant};

pub type Futex = Atomic<u32>;
pub type Primitive = u32;
pub type SmallFutex = Atomic<u32>;
pub type SmallPrimitive = u32;

/// Global auto-reset event handle (0 = not yet created).
static EVENT: AtomicU32 = AtomicU32::new(0);

const POLL: Duration = Duration::from_millis(1);

fn event() -> u32 {
    let h = EVENT.load(Ordering::Acquire);
    if h != 0 {
        return h;
    }
    let mut nh: u32 = 0;
    // NtCreateEvent(handle*, attr=null, type=1 SynchronizationEvent/auto-reset, initial=0)
    unsafe { abi::NtCreateEvent(&mut nh, crate::ptr::null(), 1, 0) };
    match EVENT.compare_exchange(0, nh, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => nh,
        Err(existing) => {
            unsafe { abi::NtClose(nh) };
            existing
        }
    }
}

/// Wait on `ev` for at most `dur`. NT timeouts are 100ns ticks; negative =
/// relative, null = infinite.
fn wait_event(ev: u32, dur: Duration) {
    let ticks = (dur.as_nanos() / 100) as i64;
    let timeout = -ticks.max(1);
    unsafe { abi::NtWaitForSingleObjectEx(ev, 1, 0, &timeout) };
}

pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    let ev = event();
    let deadline = timeout.map(|d| Instant::now() + d);
    loop {
        if futex.load(Ordering::Acquire) != expected {
            return true;
        }
        let wait = match deadline {
            Some(dl) => {
                let now = Instant::now();
                if now >= dl {
                    return false;
                }
                (dl - now).min(POLL)
            }
            None => POLL,
        };
        wait_event(ev, wait);
    }
}

#[inline]
pub fn futex_wake(_futex: &Atomic<u32>) -> bool {
    let ev = EVENT.load(Ordering::Acquire);
    if ev != 0 {
        unsafe { abi::NtSetEvent(ev, crate::ptr::null_mut()) };
    }
    true
}

#[inline]
pub fn futex_wake_all(futex: &Atomic<u32>) {
    futex_wake(futex);
}
