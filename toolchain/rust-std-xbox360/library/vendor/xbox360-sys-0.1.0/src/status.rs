//! Kernel status codes, Win32 error codes and result values.

/// A kernel status code as the kernel ABI and wire formats carry it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NtStatus(pub u32);

impl NtStatus {
    pub const SUCCESS: Self = Self(0);
    /// A wait ended because its timeout elapsed. A success status.
    pub const TIMEOUT: Self = Self(0x0000_0102);

    pub const fn is_success(self) -> bool {
        (self.0 as i32) >= 0
    }

    /// `Ok` for a success or informational status, the failure otherwise.
    pub const fn result(self) -> Result<(), KernelError> {
        match KernelError::from_status(self) {
            None => Ok(()),
            Some(error) => Err(error),
        }
    }
}

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
    InvalidHandle = 0xc000_0008,
    InvalidParameter = 0xc000_000d,
    NoSuchDevice = 0xc000_000e,
    NoSuchFile = 0xc000_000f,
    EndOfFile = 0xc000_0011,
    NoMediaInDevice = 0xc000_0013,
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
    NotSameDevice = 0xc000_00d4,
    DirectoryNotEmpty = 0xc000_0101,
    NotADirectory = 0xc000_0103,
    CannotDelete = 0xc000_0121,
    OrdinalNotFound = 0xc000_0138,
    UnrecognizedVolume = 0xc000_014f,
    MappedAlignment = 0xc000_0220,
    EntryPointNotFound = 0xc000_0263,
}

/// A failure status without a [`KernelError`] variant. Only
/// [`KernelError::from_status`] creates it, so equal errors have equal
/// variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnlistedStatus(NtStatus);

/// A success or informational status, which is not a [`KernelError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotAFailure(pub NtStatus);

impl core::fmt::Display for NotAFailure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "kernel status {:#010x} is not a failure", self.0.0)
    }
}

impl core::error::Error for NotAFailure {}

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

impl core::fmt::Display for KernelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "kernel status {:#010x}", self.status().0)
    }
}

impl core::error::Error for KernelError {}

/// A Win32 error code, as the kernel's status translation, the last-error
/// value and XAM report it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32Error(core::num::NonZeroU32);

impl Win32Error {
    pub const INVALID_PARAMETER: Self = Self::known(87);
    pub const IO_PENDING: Self = Self::known(997);
    pub const DEVICE_NOT_CONNECTED: Self = Self::known(1167);
    pub const CANCELLED: Self = Self::known(1223);
    /// `WSAEWOULDBLOCK`: a non-blocking socket call would block.
    pub const WOULD_BLOCK: Self = Self::known(10035);

    const fn known(code: u32) -> Self {
        match Self::new(code) {
            Some(error) => error,
            None => panic!("ERROR_SUCCESS is not an error"),
        }
    }

    /// `None` for `ERROR_SUCCESS`.
    pub const fn new(code: u32) -> Option<Self> {
        match core::num::NonZeroU32::new(code) {
            Some(code) => Some(Self(code)),
            None => None,
        }
    }

    /// `Ok` for `ERROR_SUCCESS`, the error otherwise.
    pub const fn result(code: u32) -> Result<(), Self> {
        match Self::new(code) {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub const fn code(self) -> u32 {
        self.0.get()
    }
}

impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "win32 error {}", self.code())
    }
}

impl core::error::Error for Win32Error {}

/// A Win32 error code as an export returns it: `ERROR_SUCCESS` (0) or an
/// error. The default is `ERROR_SUCCESS`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Win32ErrorCode(pub u32);

impl Win32ErrorCode {
    pub const SUCCESS: Self = Self(0);

    /// `None` for `ERROR_SUCCESS`.
    pub const fn error(self) -> Option<Win32Error> {
        Win32Error::new(self.0)
    }

    /// `Ok` for `ERROR_SUCCESS`, the error otherwise.
    pub const fn result(self) -> Result<(), Win32Error> {
        Win32Error::result(self.0)
    }
}

impl From<Option<Win32Error>> for Win32ErrorCode {
    fn from(error: Option<Win32Error>) -> Self {
        match error {
            Some(error) => Self(error.code()),
            None => Self::SUCCESS,
        }
    }
}

/// A result value: success when the high bit is clear.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hresult(pub u32);

impl Hresult {
    pub const FAIL: Self = Self(0x8000_4005);
    pub const UNEXPECTED: Self = Self(0x8000_ffff);
    pub const OUT_OF_MEMORY: Self = Self(0x8007_000e);
    pub const INVALID_ARGUMENT: Self = Self(0x8007_0057);

    pub const fn is_failure(self) -> bool {
        self.0 & HRESULT_SEVERITY_ERROR != 0
    }
}

/// HRESULT severity bit: set for a failure.
const HRESULT_SEVERITY_ERROR: u32 = 0x8000_0000;
/// HRESULT bit that marks a value converted from an NT status.
const HRESULT_NT_STATUS: u32 = 0x1000_0000;
/// A failure in the Win32 facility (7), without the code.
const HRESULT_WIN32_FAILURE: u32 = 0x8007_0000;

/// A kernel status as a result: the status with the NT facility bit set.
impl From<KernelError> for Hresult {
    fn from(error: KernelError) -> Self {
        Self(error.status().0 | HRESULT_NT_STATUS)
    }
}

/// A Win32 error as a result: positive codes move to the Win32 facility,
/// others pass unchanged.
impl From<Win32Error> for Hresult {
    fn from(error: Win32Error) -> Self {
        let code = error.code();
        if (code as i32) <= 0 {
            Self(code)
        } else {
            Self(HRESULT_WIN32_FAILURE | (code & 0xffff))
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use std::string::ToString;

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

    #[test]
    fn success_and_informational_statuses_are_ok() {
        assert_eq!(NtStatus::SUCCESS.result(), Ok(()));
        assert_eq!(NtStatus::TIMEOUT.result(), Ok(()));
        assert_eq!(
            NtStatus(0x8000_0006).result(),
            Err(KernelError::NoMoreFiles)
        );
    }

    #[test]
    fn win32_success_is_not_an_error() {
        assert_eq!(Win32Error::new(0), None);
        assert_eq!(Win32Error::result(0), Ok(()));
        assert_eq!(
            Win32Error::result(1167),
            Err(Win32Error::DEVICE_NOT_CONNECTED)
        );
        assert_eq!(Win32ErrorCode::SUCCESS.error(), None);
        assert_eq!(Win32ErrorCode(997).error(), Some(Win32Error::IO_PENDING));
        assert_eq!(
            Win32ErrorCode::from(Some(Win32Error::CANCELLED)),
            Win32ErrorCode(1223)
        );
    }

    #[test]
    fn errors_convert_to_results() {
        assert_eq!(Hresult::from(KernelError::NoMemory), Hresult(0xd000_0017));
        assert_eq!(
            Hresult::from(Win32Error::INVALID_PARAMETER),
            Hresult::INVALID_ARGUMENT
        );
        assert_eq!(
            Hresult::from(Win32Error::new(0x8007_0005).unwrap()),
            Hresult(0x8007_0005)
        );
        assert!(Hresult::FAIL.is_failure());
        assert!(!Hresult(0x0000_0001).is_failure());
    }
}
