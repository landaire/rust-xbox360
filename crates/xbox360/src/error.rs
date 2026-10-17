use crate::status::KernelError;
use crate::status::NtStatus;

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
