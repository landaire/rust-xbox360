use core::fmt;

use crate::status::NtStatus;

/// Generates [`KernelError`] with one variant per listed status.
macro_rules! kernel_errors {
    ($($(#[$meta:meta])* $name:ident = $value:literal,)*) => {
        /// A failed kernel call. A status without a variant keeps its value
        /// for exact passthrough. A later version may name more statuses, so
        /// match a status by its value with [`KernelError::status`] when it
        /// has no variant here.
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum KernelError {
            $($(#[$meta])* $name,)*
            Unlisted(UnlistedStatus),
        }

        impl KernelError {
            /// The error for a failure status, `None` for a success or
            /// informational status.
            pub const fn from_status(status: NtStatus) -> Option<Self> {
                if status.is_success() {
                    return None;
                }

                Some(match status.0 {
                    $($value => Self::$name,)*
                    _ => Self::Unlisted(UnlistedStatus(status)),
                })
            }

            pub const fn status(self) -> NtStatus {
                match self {
                    $(Self::$name => NtStatus($value),)*
                    Self::Unlisted(UnlistedStatus(status)) => status,
                }
            }
        }
    };
}

kernel_errors! {
    BufferOverflow = 0x8000_0005,
    NoMoreFiles = 0x8000_0006,
    NoMoreEntries = 0x8000_001a,
    Unsuccessful = 0xc000_0001,
    NotImplemented = 0xc000_0002,
    AccessViolation = 0xc000_0005,
    InvalidHandle = 0xc000_0008,
    InvalidParameter = 0xc000_000d,
    NoSuchDevice = 0xc000_000e,
    NoSuchFile = 0xc000_000f,
    InvalidDeviceRequest = 0xc000_0010,
    EndOfFile = 0xc000_0011,
    NoMediaInDevice = 0xc000_0013,
    /// A completion routine keeps the request: the I/O manager stops
    /// completing it.
    MoreProcessingRequired = 0xc000_0016,
    NoMemory = 0xc000_0017,
    AccessDenied = 0xc000_0022,
    BufferTooSmall = 0xc000_0023,
    ObjectNameInvalid = 0xc000_0033,
    ObjectNameNotFound = 0xc000_0034,
    ObjectNameCollision = 0xc000_0035,
    ObjectPathInvalid = 0xc000_0039,
    ObjectPathNotFound = 0xc000_003a,
    CrcError = 0xc000_003f,
    SectionTooBig = 0xc000_0040,
    DiskFull = 0xc000_007f,
    InsufficientResources = 0xc000_009a,
    DeviceNotConnected = 0xc000_009d,
    FileIsADirectory = 0xc000_00ba,
    DeviceDoesNotExist = 0xc000_00c0,
    NotSameDevice = 0xc000_00d4,
    DirectoryNotEmpty = 0xc000_0101,
    NotADirectory = 0xc000_0103,
    CannotDelete = 0xc000_0121,
    OrdinalNotFound = 0xc000_0138,
    UnrecognizedVolume = 0xc000_014f,
    MappedAlignment = 0xc000_0220,
    EntryPointNotFound = 0xc000_0263,
    DeviceRemoved = 0xc000_02b6,
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kernel status {:#010x}", self.status().0)
    }
}

impl core::error::Error for KernelError {}

impl TryFrom<NtStatus> for KernelError {
    type Error = NotAFailure;

    fn try_from(status: NtStatus) -> Result<Self, NotAFailure> {
        Self::from_status(status).ok_or(NotAFailure(status))
    }
}

impl From<KernelError> for NtStatus {
    fn from(error: KernelError) -> Self {
        error.status()
    }
}

/// A failure status without a [`KernelError`] variant. Only
/// [`KernelError::from_status`] creates it, so equal errors have equal
/// variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnlistedStatus(NtStatus);

/// A success or informational status, which is not a [`KernelError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotAFailure(pub NtStatus);

impl fmt::Display for NotAFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kernel status {:#010x} is not a failure", self.0.0)
    }
}

impl core::error::Error for NotAFailure {}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::ToString;

    use super::*;

    #[test]
    fn listed_status_maps_to_its_variant() {
        assert_eq!(
            KernelError::try_from(NtStatus(0xc000_0034)),
            Ok(KernelError::ObjectNameNotFound)
        );
        assert_eq!(
            NtStatus::from(KernelError::AccessDenied),
            NtStatus(0xc000_0022)
        );
    }

    #[test]
    fn unlisted_status_passes_through() {
        let error = KernelError::from_status(NtStatus(0xc000_9999)).unwrap();
        assert!(matches!(error, KernelError::Unlisted(_)));
        assert_eq!(error.status(), NtStatus(0xc000_9999));
        assert_eq!(error.to_string(), "kernel status 0xc0009999");
    }

    #[test]
    fn success_is_not_an_error() {
        assert_eq!(KernelError::from_status(NtStatus::SUCCESS), None);
        assert_eq!(KernelError::from_status(NtStatus::TIMEOUT), None);
        assert_eq!(
            KernelError::try_from(NtStatus::SUCCESS),
            Err(NotAFailure(NtStatus::SUCCESS))
        );
    }
}
