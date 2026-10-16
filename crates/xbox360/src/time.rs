//! Time base, system time, tick count and sleep.
//!
//! Titles that use the standard library use `std::time::Instant`,
//! `std::time::SystemTime` and `std::thread::sleep`.

use core::num::NonZeroU32;
use core::time::Duration;

use xbox360_sys::xboxkrnl;

use crate::sync::NOT_ALERTABLE;
use crate::sync::ProcessorMode;

/// Byte offset of the millisecond tick count in `KeTimeStampBundle`.
const TICK_COUNT_OFFSET: usize = 0x10;

/// A kernel interval for `duration` from now: negative 100 ns units,
/// rounded up so the wait is never shorter than `duration`. Durations
/// beyond the range (about 29,000 years) wait the longest interval.
pub(crate) fn relative_interval(duration: Duration) -> i64 {
    let intervals = duration
        .as_secs()
        .checked_mul(10_000_000)
        .and_then(|intervals| {
            intervals.checked_add(u64::from(duration.subsec_nanos().div_ceil(100)))
        })
        .and_then(|intervals| i64::try_from(intervals).ok());
    -intervals.unwrap_or(i64::MAX)
}

/// A reading of the 64-bit time base register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timebase(u64);

impl Timebase {
    /// Read the time base. The value increases monotonically on each
    /// processor.
    pub fn now() -> Self {
        loop {
            let (upper, lower, check): (u32, u32, u32);
            // SAFETY: reads the time base registers; no memory access.
            unsafe {
                core::arch::asm!(
                    "mftbu {0}",
                    "mftb {1}",
                    "mftbu {2}",
                    out(reg) upper,
                    out(reg) lower,
                    out(reg) check,
                    options(nomem, nostack, preserves_flags),
                );
            }
            if upper == check {
                return Self((u64::from(upper) << 32) | u64::from(lower));
            }
        }
    }

    pub const fn ticks(self) -> u64 {
        self.0
    }

    /// Time from `earlier` to `self`, zero if `earlier` is later.
    pub fn duration_since(self, earlier: Self, frequency: TimebaseFrequency) -> Duration {
        let ticks = self.0.saturating_sub(earlier.0);
        let hertz = u64::from(frequency.hertz().get());
        let nanoseconds = (ticks % hertz) * 1_000_000_000 / hertz;
        Duration::new(ticks / hertz, nanoseconds as u32)
    }
}

/// Rate of the time base in ticks per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimebaseFrequency(NonZeroU32);

impl TimebaseFrequency {
    /// The kernel's performance counter frequency, which is the time base
    /// rate.
    ///
    /// # Panics
    ///
    /// If the kernel reports a frequency of zero.
    pub fn query() -> Self {
        match NonZeroU32::new(xboxkrnl::KeQueryPerformanceFrequency()) {
            Some(hertz) => Self(hertz),
            None => panic!("the kernel reported a time base frequency of zero"),
        }
    }

    pub const fn hertz(self) -> NonZeroU32 {
        self.0
    }
}

/// System time: 100 ns intervals since 1601-01-01 UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTime(u64);

impl FileTime {
    pub fn now() -> Self {
        let mut time = 0u64;
        // SAFETY: `time` is a live local u64.
        unsafe { xboxkrnl::KeQuerySystemTime(&mut time) };
        Self(time)
    }

    pub const fn from_intervals(intervals: u64) -> Self {
        Self(intervals)
    }

    pub const fn intervals(self) -> u64 {
        self.0
    }

    /// Time from `earlier` to `self`. `None` if `earlier` is later, as
    /// after a change of the system time.
    pub fn duration_since(self, earlier: Self) -> Option<Duration> {
        let intervals = self.0.checked_sub(earlier.0)?;
        Some(Duration::new(
            intervals / 10_000_000,
            ((intervals % 10_000_000) * 100) as u32,
        ))
    }
}

/// Milliseconds since boot from the kernel time stamp bundle. Wraps after
/// about 49.7 days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TickCount(u32);

impl TickCount {
    pub fn now() -> Self {
        let bundle = xboxkrnl::KeTimeStampBundle.address() as usize;
        // SAFETY: the loader stored the address of the kernel export; the
        // tick count is at TICK_COUNT_OFFSET.
        Self(unsafe { core::ptr::read_volatile((bundle + TICK_COUNT_OFFSET) as *const u32) })
    }

    pub const fn milliseconds(self) -> u32 {
        self.0
    }

    /// Time from `earlier` to `self`, assuming less than one wrap between
    /// them.
    pub const fn wrapping_duration_since(self, earlier: Self) -> Duration {
        Duration::from_millis(self.0.wrapping_sub(earlier.0) as u64)
    }
}

/// Suspend the calling thread for at least `duration`.
pub fn sleep(duration: Duration) {
    let interval = relative_interval(duration);
    // SAFETY: `interval` is a live local.
    unsafe { xboxkrnl::KeDelayExecutionThread(ProcessorMode::User, NOT_ALERTABLE, &interval) };
}
