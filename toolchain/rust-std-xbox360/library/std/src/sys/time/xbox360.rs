//! Time for the Xbox 360 devkit.
//!
//! `Instant` reads the PowerPC timebase register (monotonic) and scales it by
//! the timebase frequency. `SystemTime` reads the kernel wall clock
//! (`KeQuerySystemTime`, 100ns ticks since 1601) rebased to the Unix epoch.

use crate::sync::atomic::{AtomicU32, Ordering};
use crate::time::Duration;

use xbox360_sys::xboxkrnl;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

/// 100ns ticks between 1601-01-01 and 1970-01-01.
const EPOCH_DIFF_100NS: u64 = 116_444_736_000_000_000;

#[inline]
fn timebase() -> u64 {
    // The 32-bit target exposes only 32-bit GPR operands, so read the timebase
    // upper/lower halves separately and retry if the upper half rolled over.
    loop {
        let hi: u32;
        let lo: u32;
        let hi2: u32;
        unsafe {
            core::arch::asm!(
                "mftbu {hi}",
                "mftb  {lo}",
                "mftbu {hi2}",
                hi = out(reg) hi,
                lo = out(reg) lo,
                hi2 = out(reg) hi2,
                options(nomem, nostack, preserves_flags),
            );
        }
        if hi == hi2 {
            return ((hi as u64) << 32) | lo as u64;
        }
    }
}

fn tb_freq() -> u64 {
    // 0 marks a frequency not yet read.
    static FREQ: AtomicU32 = AtomicU32::new(0);
    let f = FREQ.load(Ordering::Relaxed);
    if f != 0 {
        return u64::from(f);
    }
    let f = xboxkrnl::KeQueryPerformanceFrequency();
    if f == 0 {
        rtabort!("KeQueryPerformanceFrequency returned 0");
    }
    FREQ.store(f, Ordering::Relaxed);
    u64::from(f)
}

impl Instant {
    pub fn now() -> Instant {
        let tb = timebase();
        let f = tb_freq();
        let secs = tb / f;
        let nanos = ((tb % f).wrapping_mul(1_000_000_000) / f) as u32;
        Instant(Duration::new(secs, nanos))
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(*other)?))
    }
}

impl SystemTime {
    pub const MAX: SystemTime = SystemTime(Duration::MAX);
    pub const MIN: SystemTime = SystemTime(Duration::ZERO);

    pub fn now() -> SystemTime {
        let mut t: u64 = 0;
        // SAFETY: `t` is a live local.
        unsafe { xboxkrnl::KeQuerySystemTime(&mut t) };
        let unix = t.saturating_sub(EPOCH_DIFF_100NS);
        let secs = unix / 10_000_000;
        let nanos = ((unix % 10_000_000) * 100) as u32;
        SystemTime(Duration::new(secs, nanos))
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
