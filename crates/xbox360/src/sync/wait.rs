use super::WaitError;
use crate::status::KernelError;
use crate::status::NtStatus;

/// The `alertable` argument of a wait that APCs do not end.
pub(crate) const NOT_ALERTABLE: bool = false;

/// How a wait with a timeout ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WaitOutcome {
    Signaled,
    TimedOut,
}

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
