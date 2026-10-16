use crate::io as std_io;
use crate::time::Duration;

// SAFETY: must be called only once during runtime initialization.
pub unsafe fn init(_argc: isize, _argv: *const *const u8, _sigpipe: u8) {}

// SAFETY: must be called only once during runtime cleanup.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

pub fn abort_internal() -> ! {
    core::intrinsics::abort();
}

/// A relative interval in the form kernel waits and delays take: negative,
/// in 100 ns units.
pub struct WaitInterval(i64);

impl WaitInterval {
    /// The longest relative interval, about 29,000 years.
    pub const LONGEST: WaitInterval = WaitInterval(-i64::MAX);

    /// `duration` rounded up to 100 ns, or `None` when it exceeds
    /// [`WaitInterval::LONGEST`].
    pub fn relative(duration: Duration) -> Option<WaitInterval> {
        let ticks = i64::try_from(duration.as_secs())
            .ok()?
            .checked_mul(10_000_000)?
            .checked_add(i64::from(duration.subsec_nanos().div_ceil(100)))?;
        Some(WaitInterval(-ticks))
    }

    pub fn as_ptr(&self) -> *const i64 {
        &self.0
    }
}
