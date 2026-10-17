//! Kernel status codes, Win32 error codes and result values.

/// A kernel status code as the kernel ABI and wire formats carry it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NtStatus(pub u32);

impl NtStatus {
    pub const SUCCESS: Self = Self(0);
    /// A wait ended because the object at index 0 was signaled.
    pub const WAIT_0: Self = Self(0);
    /// A wait ended because the object at index 1 was signaled.
    pub const WAIT_1: Self = Self(1);
    /// A wait ended because its timeout elapsed. A success status.
    pub const TIMEOUT: Self = Self(0x0000_0102);
    /// The operation completes later. The I/O status block and the event
    /// of the request report the result.
    pub const PENDING: Self = Self(0x0000_0103);
    /// Exception code a thread raises to give itself a name in the
    /// debugger.
    pub const THREAD_NAME: Self = Self(0x406d_1388);
    /// Exception code of a breakpoint trap.
    pub const BREAKPOINT: Self = Self(0x8000_0003);
    /// Exception code of a single-step trace interrupt.
    pub const SINGLE_STEP: Self = Self(0x8000_0004);
    /// Exception code of an access to an invalid address.
    pub const ACCESS_VIOLATION: Self = Self(0xc000_0005);
    /// `STATUS_ARRAY_BOUNDS_EXCEEDED`: exception code of a trap instruction
    /// whose condition held.
    pub const ARRAY_BOUNDS_EXCEEDED: Self = Self(0xc000_008c);
    /// Exception code of the divide-by-zero check trap.
    pub const INTEGER_DIVIDE_BY_ZERO: Self = Self(0xc000_0094);
    /// Exception code of the overflow check trap.
    pub const INTEGER_OVERFLOW: Self = Self(0xc000_0095);
    /// Exception code of a thread that ran past its stack limit.
    pub const STACK_OVERFLOW: Self = Self(0xc000_00fd);
    /// Exception code of a failed assertion trap.
    pub const ASSERTION_FAILURE: Self = Self(0xc000_0420);

    pub const fn is_success(self) -> bool {
        (self.0 as i32) >= 0
    }

    /// The severity in the top two bits.
    pub const fn severity(self) -> Severity {
        match self.0 >> 30 {
            0 => Severity::Success,
            1 => Severity::Informational,
            2 => Severity::Warning,
            _ => Severity::Error,
        }
    }

    /// `Ok` for a success or informational status, the failure otherwise.
    pub const fn result(self) -> Result<(), KernelError> {
        match KernelError::from_status(self) {
            None => Ok(()),
            Some(error) => Err(error),
        }
    }
}

value_enum! {
    /// The severity of an [`NtStatus`]. Warning and error statuses are
    /// failures.
    pub enum Severity: u8 {
        Success = 0,
        Informational = 1,
        Warning = 2,
        Error = 3,
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
    pub const INVALID_FUNCTION: Self = Self::known(1);
    pub const FILE_NOT_FOUND: Self = Self::known(2);
    pub const NOT_ENOUGH_MEMORY: Self = Self::known(8);
    pub const BAD_FORMAT: Self = Self::known(11);
    pub const INVALID_DATA: Self = Self::known(13);
    pub const BAD_LENGTH: Self = Self::known(24);
    pub const WRITE_FAULT: Self = Self::known(29);
    pub const NOT_SUPPORTED: Self = Self::known(50);
    pub const INVALID_PARAMETER: Self = Self::known(87);
    pub const BUFFER_OVERFLOW: Self = Self::known(111);
    pub const INVALID_NAME: Self = Self::known(123);
    pub const MOD_NOT_FOUND: Self = Self::known(126);
    pub const PROC_NOT_FOUND: Self = Self::known(127);
    pub const BUSY: Self = Self::known(170);
    /// A wait ended because its timeout elapsed. Equal to
    /// [`NtStatus::TIMEOUT`].
    pub const WAIT_TIMEOUT: Self = Self::known(258);
    pub const IO_PENDING: Self = Self::known(997);
    /// `ERROR_NO_LOG_SPACE`: no space could be allocated in a log.
    pub const NO_LOG_SPACE: Self = Self::known(1019);
    pub const SERVICE_NOT_ACTIVE: Self = Self::known(1062);
    pub const DEVICE_NOT_CONNECTED: Self = Self::known(1167);
    pub const NOT_FOUND: Self = Self::known(1168);
    pub const CANCELLED: Self = Self::known(1223);
    pub const REVISION_MISMATCH: Self = Self::known(1306);
    /// `WSAEINVAL`: a socket call received an invalid argument.
    pub const INVALID_SOCKET_ARGUMENT: Self = Self::known(10022);
    /// `WSAEWOULDBLOCK`: a non-blocking socket call would block.
    pub const WOULD_BLOCK: Self = Self::known(10035);
    /// `WSAEALREADY`: the socket operation is already in progress.
    pub const ALREADY_IN_PROGRESS: Self = Self::known(10037);
    /// `WSAENOBUFS`: the socket call found no buffer space. A table query
    /// with no buffer also reports it.
    pub const NO_BUFFER_SPACE: Self = Self::known(10055);
    /// `WSAETIMEDOUT`: a socket call timed out.
    pub const TIMED_OUT: Self = Self::known(10060);
    /// The HTTP stack found no header of the requested name.
    pub const HTTP_HEADER_NOT_FOUND: Self = Self::known(12150);

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
    /// `S_OK`.
    pub const SUCCESS: Self = Self(0);
    /// `S_FALSE`: success with a negative answer.
    pub const FALSE: Self = Self(1);
    /// `E_PENDING`: the data is not yet available.
    pub const PENDING: Self = Self(0x8000_000a);
    pub const NOT_IMPLEMENTED: Self = Self(0x8000_4001);
    pub const NO_INTERFACE: Self = Self(0x8000_4002);
    pub const ABORT: Self = Self(0x8000_4004);
    pub const FAIL: Self = Self(0x8000_4005);
    pub const UNEXPECTED: Self = Self(0x8000_ffff);
    pub const OUT_OF_MEMORY: Self = Self(0x8007_000e);
    pub const INVALID_ARGUMENT: Self = Self(0x8007_0057);

    /// A success result with `code` in `facility`.
    pub const fn success(facility: Facility, code: u16) -> Self {
        Self((facility.0 as u32) << 16 | code as u32)
    }

    /// A failure result with `code` in `facility`.
    pub const fn failure(facility: Facility, code: u16) -> Self {
        Self(HRESULT_SEVERITY_ERROR | Self::success(facility, code).0)
    }

    /// `HRESULT_FROM_WIN32`: a positive code moves its low word to the
    /// Win32 facility as a failure. Other codes are results already and
    /// pass unchanged.
    pub const fn from_win32(error: Win32Error) -> Self {
        let code = error.code();
        if (code as i32) <= 0 {
            Self(code)
        } else {
            Self::failure(Facility::WIN32, code as u16)
        }
    }

    /// `HRESULT_FROM_NT`: the status with the NT bit set. A success
    /// status gives a success result.
    pub const fn from_nt(status: NtStatus) -> Self {
        Self(status.0 | HRESULT_NT_STATUS)
    }

    /// The status of a result with the NT bit set, `None` for any other
    /// result.
    pub const fn nt_status(self) -> Option<NtStatus> {
        if self.0 & HRESULT_NT_STATUS != 0 {
            Some(NtStatus(self.0 & !HRESULT_NT_STATUS))
        } else {
            None
        }
    }

    /// The facility field, as `HRESULT_FACILITY` reads it.
    pub const fn facility(self) -> Facility {
        Facility((self.0 >> 16) as u16 & Facility::MASK)
    }

    /// The low word.
    pub const fn code(self) -> u16 {
        self.0 as u16
    }

    pub const fn is_failure(self) -> bool {
        self.0 & HRESULT_SEVERITY_ERROR != 0
    }
}

/// HRESULT severity bit: set for a failure.
const HRESULT_SEVERITY_ERROR: u32 = 0x8000_0000;
/// HRESULT bit that marks a value converted from an NT status.
const HRESULT_NT_STATUS: u32 = 0x1000_0000;

/// The facility field of an [`Hresult`]: bits 16 to 28 of the value. Bit
/// 28 is the NT bit, so a facility at or above 0x1000 marks a converted
/// NT status.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Facility(u16);

impl Facility {
    pub const WIN32: Self = Self::new(7);

    const MASK: u16 = 0x1fff;

    /// # Panics
    ///
    /// If `value` does not fit the 13-bit field.
    pub const fn new(value: u16) -> Self {
        assert!(value <= Self::MASK, "facility exceeds 13 bits");
        Self(value)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

/// A kernel status as a result: the status with the NT bit set.
impl From<KernelError> for Hresult {
    fn from(error: KernelError) -> Self {
        Self::from_nt(error.status())
    }
}

/// A Win32 error as a result, as [`Hresult::from_win32`] converts it.
impl From<Win32Error> for Hresult {
    fn from(error: Win32Error) -> Self {
        Self::from_win32(error)
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

    #[test]
    fn severity_is_the_top_two_bits() {
        assert_eq!(NtStatus(0x3fff_ffff).severity(), Severity::Success);
        assert_eq!(NtStatus(0x4000_0000).severity(), Severity::Informational);
        assert_eq!(NtStatus::THREAD_NAME.severity(), Severity::Informational);
        assert_eq!(NtStatus(0x7fff_ffff).severity(), Severity::Informational);
        assert_eq!(NtStatus(0x8000_0000).severity(), Severity::Warning);
        assert_eq!(NtStatus::SINGLE_STEP.severity(), Severity::Warning);
        assert_eq!(NtStatus(0xbfff_ffff).severity(), Severity::Warning);
        assert_eq!(NtStatus(0xc000_0000).severity(), Severity::Error);
        assert_eq!(NtStatus(0xffff_ffff).severity(), Severity::Error);
    }

    #[test]
    fn win32_codes_move_to_the_win32_facility() {
        assert_eq!(
            Hresult::from_win32(Win32Error::NOT_ENOUGH_MEMORY),
            Hresult(0x8007_0008)
        );
        assert_eq!(
            Hresult::from_win32(Win32Error::NO_BUFFER_SPACE),
            Hresult(0x8007_2747)
        );
        // A positive code keeps only its low word.
        assert_eq!(
            Hresult::from_win32(Win32Error::new(0x0001_0005).unwrap()),
            Hresult(0x8007_0005)
        );
        assert_eq!(
            Hresult::from_win32(Win32Error::new(0x7fff_ffff).unwrap()),
            Hresult(0x8007_ffff)
        );
        // A code with the sign bit set is a result already.
        assert_eq!(
            Hresult::from_win32(Win32Error::new(0x8000_0000).unwrap()),
            Hresult(0x8000_0000)
        );
        assert_eq!(
            Hresult::from_win32(Win32Error::new(0xd000_0017).unwrap()),
            Hresult(0xd000_0017)
        );
    }

    #[test]
    fn nt_statuses_carry_the_nt_bit() {
        let success = Hresult::from_nt(NtStatus::SUCCESS);
        assert_eq!(success, Hresult(0x1000_0000));
        assert!(!success.is_failure());
        assert_eq!(success.nt_status(), Some(NtStatus::SUCCESS));
        let failure = Hresult::from_nt(NtStatus::ACCESS_VIOLATION);
        assert_eq!(failure, Hresult(0xd000_0005));
        assert!(failure.is_failure());
        assert_eq!(failure.nt_status(), Some(NtStatus::ACCESS_VIOLATION));
        assert_eq!(Hresult::FAIL.nt_status(), None);
        assert_eq!(Hresult(0x8007_2747).nt_status(), None);
    }

    #[test]
    fn facility_and_code_split_the_value() {
        assert_eq!(
            Hresult::failure(Facility::new(0x31), 1),
            Hresult(0x8031_0001)
        );
        assert_eq!(
            Hresult::success(Facility::new(0x2da), 9),
            Hresult(0x02da_0009)
        );
        assert_eq!(
            Hresult::failure(Facility::WIN32, 0x18),
            Hresult::from_win32(Win32Error::BAD_LENGTH)
        );
        let result = Hresult(0x82da_0105);
        assert_eq!(result.facility(), Facility::new(0x2da));
        assert_eq!(result.code(), 0x105);
        // The field includes the NT bit but not bits 29 and 30.
        assert_eq!(Hresult(0xd000_0017).facility(), Facility::new(0x1000));
        assert_eq!(Hresult(0xe2da_0000).facility(), Facility::new(0x2da));
    }

    #[test]
    #[should_panic(expected = "facility exceeds 13 bits")]
    fn facility_rejects_values_above_13_bits() {
        Facility::new(0x2000);
    }
}
