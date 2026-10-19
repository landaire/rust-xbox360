//! Network configuration, link state, the Xbox LIVE logon, UPnP and
//! Windows Connect Now files, as the 2.0.6717 dashboard's network settings
//! use them.
//!
//! Checked against the call sites of the 2.0.6717 dashboard: the ordinals,
//! the caller value (`XNCALLER_TITLE`, 1, in every `NetDll_*` wrapper of
//! the dashboard), the argument order and the values passed. Not run on
//! hardware.

use core::ffi::CStr;
use core::pin::Pin;

use xbox360_status::Hresult;
use xbox360_status::NtStatus;
use xbox360_status::Win32Error;
use xbox360_sys::xam;
use xbox360_sys::xam::LogonFlags;
use xbox360_sys::xam::XnCaller;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::AccessMask;
use xbox360_sys::xboxkrnl::AnsiString;
use xbox360_sys::xboxkrnl::CreateOptions;
use xbox360_sys::xboxkrnl::Handle;
use xbox360_sys::xboxkrnl::IoStatusBlock;
use xbox360_sys::xboxkrnl::ObjectAttributeFlags;
use xbox360_sys::xboxkrnl::ObjectAttributes;
use xbox360_sys::xboxkrnl::ShareAccess;

use super::AsyncOperation;

pub use xbox360_sys::xam::EthernetLinkStatus;
pub use xbox360_sys::xam::XnetOption;

/// Size of the network configuration record of `NetDll_XnpLoadConfigParams`,
/// `NetDll_XnpSaveConfigParams` and `NetDll_XnpConfig`.
pub const CONFIG_PARAMS_BYTES: usize = core::mem::size_of::<xam::XnetConfigParams>();

/// Size of the `NetDll_XnpGetConfigStatus` record.
pub const CONFIG_STATUS_BYTES: usize = 0x3a;

/// Capacity the dashboard gives `XamGetWCNConfigFile`.
pub const WCN_PATH_BYTES: usize = 0x104;

/// The application and messages of the `XMsgInProcessCall` logon queries.
const LOGON_APP: u32 = 0xfc;
const LOGON_STATE_MESSAGE: u32 = 0x5_8003;
const NAT_TYPE_MESSAGE: u32 = 0x5_8006;

/// A failed `NetDll_XNetGetOpt`: its nonzero result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptionError(pub i32);

/// A value type of an XNet option.
pub trait OptionValue: sealed::Sealed + Copy + Default {}

impl OptionValue for u32 {}

impl OptionValue for u64 {}

impl OptionValue for i32 {}

/// `XNetLogonGetExtendedStatus` outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtendedLogonStatus {
    pub status: u32,
    pub extended: Hresult,
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for u32 {}

    impl Sealed for u64 {}

    impl Sealed for i32 {}
}

pub fn ethernet_link_status() -> EthernetLinkStatus {
    xam::NetDll_XNetGetEthernetLinkStatus(XnCaller::Title)
}

/// `NetDll_XNetGetOpt` of an option whose value is a `T`.
pub fn option<T>(option: XnetOption) -> Result<T, OptionError>
where
    T: OptionValue,
{
    let mut value = T::default();
    // Option values are a few bytes.
    let mut length = core::mem::size_of::<T>() as i32;

    // SAFETY: XAM writes at most `length` bytes to the live local `value`.
    let result = unsafe {
        xam::NetDll_XNetGetOpt(XnCaller::Title, option, (&raw mut value).cast(), &mut length)
    };
    if result != 0 {
        return Err(OptionError(result));
    }

    Ok(value)
}

/// `NetDll_XnpLoadConfigParams`: reads the saved configuration.
pub fn load_config_params(params: &mut [u8; CONFIG_PARAMS_BYTES]) -> Hresult {
    // SAFETY: XAM writes one record to `params`.
    unsafe { xam::NetDll_XnpLoadConfigParams(XnCaller::Title, params.as_mut_ptr()) }
}

/// `NetDll_XnpSaveConfigParams`: `E_PENDING` while the save runs; call it
/// again with the same record until it reports another result.
pub fn save_config_params(params: &[u8; CONFIG_PARAMS_BYTES]) -> Hresult {
    // SAFETY: XAM reads one record from `params`.
    unsafe { xam::NetDll_XnpSaveConfigParams(XnCaller::Title, params.as_ptr()) }
}

/// `NetDll_XnpConfig(params, 3)`, as the connection test applies a
/// configuration. Its progress is read with [`config_status`].
///
/// # Safety
///
/// `params` stays valid and unchanged until [`end_config`]: the dashboard
/// keeps its record until then, and whether XNet copies it is not known.
pub unsafe fn start_config(params: &[u8; CONFIG_PARAMS_BYTES]) -> i32 {
    const APPLY: u32 = 3;

    // SAFETY: the caller keeps `params` valid until `end_config`.
    unsafe { xam::NetDll_XnpConfig(XnCaller::Title, params.as_ptr(), APPLY) }
}

/// `NetDll_XnpConfig(NULL, 0)`: ends the configuration [`start_config`]
/// applied.
pub fn end_config() -> i32 {
    const END: u32 = 0;

    // SAFETY: a null record with flags 0 is the dashboard's end call.
    unsafe { xam::NetDll_XnpConfig(XnCaller::Title, core::ptr::null(), END) }
}

/// `NetDll_XnpGetConfigStatus`.
pub fn config_status(status: &mut [u8; CONFIG_STATUS_BYTES]) -> i32 {
    // SAFETY: XAM writes one status record to `status`.
    unsafe { xam::NetDll_XnpGetConfigStatus(XnCaller::Title, status.as_mut_ptr()) }
}

/// `XNetLogonClearTicketCaches(0)`.
pub fn clear_ticket_caches() {
    const UNKNOWN: u32 = 0;

    // SAFETY: the dashboard passes 0.
    unsafe { xam::XNetLogonClearTicketCaches(UNKNOWN) };
}

/// `XamUserLogon` with four zero XUIDs: every user slot is signed out, and
/// with the dashboard's test flags the Xbox LIVE logon starts. A call still
/// pending on `operation` is cancelled first. A failure is a negative
/// result; `ERROR_IO_PENDING` reads as a success.
pub fn logon_without_users(flags: LogonFlags, operation: Pin<&AsyncOperation>) -> Hresult {
    const USER_SLOTS: usize = 4;

    operation.reset();
    let xuids = [0u64; USER_SLOTS];

    // SAFETY: XAM copies the XUIDs during the call, and the pinned block
    // outlives the call (`AsyncOperation` cancels and waits on drop).
    unsafe { xam::XamUserLogon(xuids.as_ptr(), flags, operation.as_ptr()) }
}

/// `XamUserFlushLogonQueue`. A call still pending on `operation` is
/// cancelled first.
pub fn flush_logon_queue(operation: Pin<&AsyncOperation>) -> Hresult {
    operation.reset();

    // SAFETY: the pinned block outlives the call.
    unsafe { xam::XamUserFlushLogonQueue(operation.as_ptr()) }
}

pub fn logon_extended_status() -> ExtendedLogonStatus {
    let mut status = 0;
    let mut extended = 0;

    // SAFETY: both outputs are live locals.
    unsafe { xam::XNetLogonGetExtendedStatus(&mut status, &mut extended) };

    ExtendedLogonStatus {
        status,
        extended: Hresult(extended),
    }
}

/// The logon state of XAM (`XMsgInProcessCall(0xFC, 0x58003, 0, 0)`).
pub fn logon_state() -> Hresult {
    // SAFETY: the message takes no parameters.
    unsafe {
        xam::XMsgInProcessCall(
            LOGON_APP,
            LOGON_STATE_MESSAGE,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        )
    }
}

/// The NAT type XAM measured (`XMsgInProcessCall(0xFC, 0x58006, &type,
/// 0)`).
pub fn nat_type() -> Result<u32, Hresult> {
    let mut nat = 0;

    // SAFETY: the message writes one word to the live local `nat`.
    let result = unsafe { xam::XMsgInProcessCall(LOGON_APP, NAT_TYPE_MESSAGE, &mut nat, core::ptr::null_mut()) };
    if result.is_failure() {
        return Err(result);
    }

    Ok(nat)
}

pub fn upnp_startup() {
    // SAFETY: no arguments other than the caller.
    unsafe { xam::NetDll_UpnpStartup(XnCaller::Title) };
}

pub fn upnp_cleanup() {
    // SAFETY: no arguments other than the caller.
    unsafe { xam::NetDll_UpnpCleanup(XnCaller::Title) };
}

/// `XamGetWCNConfigFile`: the path of the Windows Connect Now configuration
/// file in `path`. `None` when no storage device holds one.
pub fn wcn_config_file(path: &mut [u8; WCN_PATH_BYTES]) -> Result<Option<&CStr>, Win32Error> {
    // The capacity is a small constant.
    let capacity = WCN_PATH_BYTES as u32;

    // SAFETY: XAM writes at most `capacity` bytes to `path`.
    match unsafe { xam::XamGetWCNConfigFile(path.as_mut_ptr(), capacity) }.result() {
        Ok(()) => {}
        Err(Win32Error::FILE_NOT_FOUND) => return Ok(None),
        Err(error) => return Err(error),
    }

    CStr::from_bytes_until_nul(&path[..])
        .map(Some)
        .map_err(|_| Win32Error::INVALID_DATA)
}

/// Reads the start of the file `path` (an ANSI object path without a NUL)
/// into `buffer` with one read, as the dashboard reads a WCN configuration
/// file. Returns the byte count; an empty file reads as 0 bytes. Errors are
/// `HRESULT_FROM_WIN32` of the kernel status, as the dashboard reports
/// them.
pub fn read_file_start(path: &[u8], buffer: &mut [u8]) -> Result<usize, Hresult> {
    let length = u16::try_from(path.len()).map_err(|_| Hresult::from_win32(Win32Error::INVALID_NAME))?;
    let name = AnsiString {
        length,
        maximum_length: length,
        buffer: path.as_ptr(),
    };
    let attributes = ObjectAttributes {
        root_directory: Handle::NULL,
        object_name: &name,
        attributes: ObjectAttributeFlags::CASE_INSENSITIVE,
    };
    let mut io_status = IoStatusBlock::default();
    let mut handle = Handle::NULL;

    // SAFETY: `attributes`, `name` and `path` live through the call, which
    // writes the live locals `handle` and `io_status`.
    let status = unsafe {
        xboxkrnl::NtOpenFile(
            &mut handle,
            AccessMask::GENERIC_READ | AccessMask::SYNCHRONIZE,
            &attributes,
            &mut io_status,
            ShareAccess::READ,
            CreateOptions::SYNCHRONOUS_IO_NONALERT | CreateOptions::NON_DIRECTORY_FILE,
        )
    };
    nt_result(status).map_err(Hresult::from_win32)?;

    // Pointers are 32-bit, so a slice length fits in `u32`.
    let capacity = buffer.len() as u32;
    // SAFETY: the handle is open for synchronous reads, so the read
    // completes before the call returns, writing at most `capacity` bytes.
    let status = unsafe {
        xboxkrnl::NtReadFile(
            handle,
            Handle::NULL,
            0,
            0,
            &mut io_status,
            buffer.as_mut_ptr(),
            capacity,
            core::ptr::null(),
        )
    };
    // SAFETY: `handle` is open and not used after this.
    unsafe { xboxkrnl::NtClose(handle) };

    match nt_result(status) {
        Ok(()) => Ok(io_status.information as usize),
        Err(error) if error.code() == HANDLE_EOF => Ok(0),
        Err(error) => Err(Hresult::from_win32(error)),
    }
}

/// `ERROR_HANDLE_EOF`, which the dashboard treats as a complete read.
const HANDLE_EOF: u32 = 0x26;

/// `RtlNtStatusToDosError` of a failed status.
fn nt_result(status: NtStatus) -> Result<(), Win32Error> {
    if status.is_success() {
        return Ok(());
    }

    xboxkrnl::RtlNtStatusToDosError(status).result()
}
