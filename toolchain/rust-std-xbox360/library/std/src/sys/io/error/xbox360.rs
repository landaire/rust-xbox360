//! Raw OS error codes. A negative code is the failure status of a kernel
//! call, as the file system and thread backends report it. A positive code
//! is a Win32 or Winsock error, as `NetDll_WSAGetLastError` reports it. The
//! two ranges do not overlap, since every failure status has the high bit
//! set.
//!
//! Statuses are matched by value rather than by `KernelError` variant, so a
//! later `xbox360-sys` that names more statuses does not move them out of
//! these tables.

use crate::io::ErrorKind;

/// 0: no backend reports errors through the thread's last error.
pub fn errno() -> i32 {
    0
}

pub fn is_interrupted(_code: i32) -> bool {
    false
}

pub fn decode_error_kind(code: i32) -> ErrorKind {
    if code < 0 { status_kind(code as u32) } else { win32_error_kind(code as u32) }
}

pub fn error_string(code: i32) -> String {
    if code < 0 {
        let status = code as u32;
        match status_text(status) {
            Some(text) => format!("{text}, kernel status {status:#010x}"),
            None => format!("kernel status {status:#010x}"),
        }
    } else {
        win32_error_text(code as u32).unwrap_or("win32 error").to_string()
    }
}

mod status {
    pub const BUFFER_OVERFLOW: u32 = 0x8000_0005;
    pub const NO_MORE_FILES: u32 = 0x8000_0006;
    pub const NO_MORE_ENTRIES: u32 = 0x8000_001a;
    pub const UNSUCCESSFUL: u32 = 0xc000_0001;
    pub const NOT_IMPLEMENTED: u32 = 0xc000_0002;
    pub const INVALID_INFO_CLASS: u32 = 0xc000_0003;
    pub const ACCESS_VIOLATION: u32 = 0xc000_0005;
    pub const INVALID_HANDLE: u32 = 0xc000_0008;
    pub const INVALID_PARAMETER: u32 = 0xc000_000d;
    pub const NO_SUCH_DEVICE: u32 = 0xc000_000e;
    pub const NO_SUCH_FILE: u32 = 0xc000_000f;
    pub const INVALID_DEVICE_REQUEST: u32 = 0xc000_0010;
    pub const END_OF_FILE: u32 = 0xc000_0011;
    pub const NO_MEDIA_IN_DEVICE: u32 = 0xc000_0013;
    pub const NO_MEMORY: u32 = 0xc000_0017;
    pub const ACCESS_DENIED: u32 = 0xc000_0022;
    pub const BUFFER_TOO_SMALL: u32 = 0xc000_0023;
    pub const OBJECT_NAME_INVALID: u32 = 0xc000_0033;
    pub const OBJECT_NAME_NOT_FOUND: u32 = 0xc000_0034;
    pub const OBJECT_NAME_COLLISION: u32 = 0xc000_0035;
    pub const OBJECT_PATH_INVALID: u32 = 0xc000_0039;
    pub const OBJECT_PATH_NOT_FOUND: u32 = 0xc000_003a;
    pub const CRC_ERROR: u32 = 0xc000_003f;
    pub const SHARING_VIOLATION: u32 = 0xc000_0043;
    pub const DELETE_PENDING: u32 = 0xc000_0056;
    pub const DISK_FULL: u32 = 0xc000_007f;
    pub const INSUFFICIENT_RESOURCES: u32 = 0xc000_009a;
    pub const DEVICE_NOT_CONNECTED: u32 = 0xc000_009d;
    pub const MEDIA_WRITE_PROTECTED: u32 = 0xc000_00a2;
    pub const IO_TIMEOUT: u32 = 0xc000_00b5;
    pub const FILE_IS_A_DIRECTORY: u32 = 0xc000_00ba;
    pub const NOT_SUPPORTED: u32 = 0xc000_00bb;
    pub const DEVICE_DOES_NOT_EXIST: u32 = 0xc000_00c0;
    pub const NOT_SAME_DEVICE: u32 = 0xc000_00d4;
    pub const DIRECTORY_NOT_EMPTY: u32 = 0xc000_0101;
    pub const NOT_A_DIRECTORY: u32 = 0xc000_0103;
    pub const CANNOT_DELETE: u32 = 0xc000_0121;
    pub const UNRECOGNIZED_VOLUME: u32 = 0xc000_014f;
    pub const NOT_FOUND: u32 = 0xc000_0225;
    pub const DEVICE_REMOVED: u32 = 0xc000_02b6;
}

fn status_kind(code: u32) -> ErrorKind {
    match code {
        status::NO_SUCH_DEVICE
        | status::NO_SUCH_FILE
        | status::OBJECT_NAME_NOT_FOUND
        | status::OBJECT_PATH_NOT_FOUND
        | status::DEVICE_DOES_NOT_EXIST
        | status::NOT_FOUND => ErrorKind::NotFound,
        status::ACCESS_DENIED | status::CANNOT_DELETE => ErrorKind::PermissionDenied,
        status::OBJECT_NAME_COLLISION => ErrorKind::AlreadyExists,
        status::OBJECT_NAME_INVALID | status::OBJECT_PATH_INVALID => ErrorKind::InvalidFilename,
        status::INVALID_PARAMETER | status::INVALID_INFO_CLASS => ErrorKind::InvalidInput,
        status::NOT_IMPLEMENTED | status::INVALID_DEVICE_REQUEST | status::NOT_SUPPORTED => ErrorKind::Unsupported,
        status::NO_MEMORY | status::INSUFFICIENT_RESOURCES => ErrorKind::OutOfMemory,
        status::SHARING_VIOLATION => ErrorKind::ResourceBusy,
        status::DISK_FULL => ErrorKind::StorageFull,
        status::MEDIA_WRITE_PROTECTED => ErrorKind::ReadOnlyFilesystem,
        status::IO_TIMEOUT => ErrorKind::TimedOut,
        status::FILE_IS_A_DIRECTORY => ErrorKind::IsADirectory,
        status::NOT_A_DIRECTORY => ErrorKind::NotADirectory,
        status::DIRECTORY_NOT_EMPTY => ErrorKind::DirectoryNotEmpty,
        status::NOT_SAME_DEVICE => ErrorKind::CrossesDevices,
        _ => ErrorKind::Uncategorized,
    }
}

fn status_text(code: u32) -> Option<&'static str> {
    Some(match code {
        status::BUFFER_OVERFLOW => "buffer overflow",
        status::NO_MORE_FILES => "no more files",
        status::NO_MORE_ENTRIES => "no more entries",
        status::UNSUCCESSFUL => "unsuccessful",
        status::NOT_IMPLEMENTED => "not implemented",
        status::INVALID_INFO_CLASS => "invalid information class",
        status::ACCESS_VIOLATION => "access violation",
        status::INVALID_HANDLE => "invalid handle",
        status::INVALID_PARAMETER => "invalid parameter",
        status::NO_SUCH_DEVICE => "no such device",
        status::NO_SUCH_FILE => "no such file",
        status::INVALID_DEVICE_REQUEST => "invalid device request",
        status::END_OF_FILE => "end of file",
        status::NO_MEDIA_IN_DEVICE => "no media in device",
        status::NO_MEMORY => "no memory",
        status::ACCESS_DENIED => "access denied",
        status::BUFFER_TOO_SMALL => "buffer too small",
        status::OBJECT_NAME_INVALID => "object name invalid",
        status::OBJECT_NAME_NOT_FOUND => "object name not found",
        status::OBJECT_NAME_COLLISION => "object name collision",
        status::OBJECT_PATH_INVALID => "object path invalid",
        status::OBJECT_PATH_NOT_FOUND => "object path not found",
        status::CRC_ERROR => "CRC error",
        status::SHARING_VIOLATION => "sharing violation",
        status::DELETE_PENDING => "delete pending",
        status::DISK_FULL => "disk full",
        status::INSUFFICIENT_RESOURCES => "insufficient resources",
        status::DEVICE_NOT_CONNECTED => "device not connected",
        status::MEDIA_WRITE_PROTECTED => "media write protected",
        status::IO_TIMEOUT => "I/O timeout",
        status::FILE_IS_A_DIRECTORY => "file is a directory",
        status::NOT_SUPPORTED => "not supported",
        status::DEVICE_DOES_NOT_EXIST => "device does not exist",
        status::NOT_SAME_DEVICE => "not same device",
        status::DIRECTORY_NOT_EMPTY => "directory not empty",
        status::NOT_A_DIRECTORY => "not a directory",
        status::CANNOT_DELETE => "cannot delete",
        status::UNRECOGNIZED_VOLUME => "unrecognized volume",
        status::NOT_FOUND => "not found",
        status::DEVICE_REMOVED => "device removed",
        _ => return None,
    })
}

mod win32 {
    pub const FILE_NOT_FOUND: u32 = 2;
    pub const PATH_NOT_FOUND: u32 = 3;
    pub const ACCESS_DENIED: u32 = 5;
    pub const NOT_ENOUGH_MEMORY: u32 = 8;
    pub const OUTOFMEMORY: u32 = 14;
    pub const NOT_SAME_DEVICE: u32 = 17;
    pub const NOT_SUPPORTED: u32 = 50;
    pub const FILE_EXISTS: u32 = 80;
    pub const INVALID_PARAMETER: u32 = 87;
    pub const DISK_FULL: u32 = 112;
    pub const CALL_NOT_IMPLEMENTED: u32 = 120;
    pub const INVALID_NAME: u32 = 123;
    pub const DIR_NOT_EMPTY: u32 = 145;
    pub const BUSY: u32 = 170;
    pub const ALREADY_EXISTS: u32 = 183;
    pub const WAIT_TIMEOUT: u32 = 258;
    pub const DIRECTORY: u32 = 267;
    pub const NOT_FOUND: u32 = 1168;
    pub const TIMEOUT: u32 = 1460;
    pub const WSAEACCES: u32 = 10013;
    pub const WSAEINVAL: u32 = 10022;
    pub const WSAEMFILE: u32 = 10024;
    pub const WSAEWOULDBLOCK: u32 = 10035;
    pub const WSAEADDRINUSE: u32 = 10048;
    pub const WSAEADDRNOTAVAIL: u32 = 10049;
    pub const WSAENETDOWN: u32 = 10050;
    pub const WSAENETUNREACH: u32 = 10051;
    pub const WSAECONNABORTED: u32 = 10053;
    pub const WSAECONNRESET: u32 = 10054;
    pub const WSAENOTCONN: u32 = 10057;
    pub const WSAESHUTDOWN: u32 = 10058;
    pub const WSAETIMEDOUT: u32 = 10060;
    pub const WSAECONNREFUSED: u32 = 10061;
    pub const WSAEHOSTUNREACH: u32 = 10065;
}

fn win32_error_kind(code: u32) -> ErrorKind {
    match code {
        win32::FILE_NOT_FOUND | win32::PATH_NOT_FOUND | win32::NOT_FOUND => ErrorKind::NotFound,
        win32::ACCESS_DENIED | win32::WSAEACCES => ErrorKind::PermissionDenied,
        win32::NOT_ENOUGH_MEMORY | win32::OUTOFMEMORY => ErrorKind::OutOfMemory,
        win32::NOT_SAME_DEVICE => ErrorKind::CrossesDevices,
        win32::NOT_SUPPORTED | win32::CALL_NOT_IMPLEMENTED => ErrorKind::Unsupported,
        win32::FILE_EXISTS | win32::ALREADY_EXISTS => ErrorKind::AlreadyExists,
        win32::INVALID_PARAMETER | win32::WSAEINVAL => ErrorKind::InvalidInput,
        win32::DISK_FULL => ErrorKind::StorageFull,
        win32::INVALID_NAME => ErrorKind::InvalidFilename,
        win32::DIR_NOT_EMPTY => ErrorKind::DirectoryNotEmpty,
        win32::BUSY => ErrorKind::ResourceBusy,
        win32::WAIT_TIMEOUT | win32::TIMEOUT | win32::WSAETIMEDOUT => ErrorKind::TimedOut,
        win32::DIRECTORY => ErrorKind::NotADirectory,
        win32::WSAEMFILE => ErrorKind::TooManyOpenFiles,
        win32::WSAEWOULDBLOCK => ErrorKind::WouldBlock,
        win32::WSAEADDRINUSE => ErrorKind::AddrInUse,
        win32::WSAEADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        win32::WSAENETDOWN => ErrorKind::NetworkDown,
        win32::WSAENETUNREACH => ErrorKind::NetworkUnreachable,
        win32::WSAECONNABORTED => ErrorKind::ConnectionAborted,
        win32::WSAECONNRESET => ErrorKind::ConnectionReset,
        win32::WSAENOTCONN => ErrorKind::NotConnected,
        // Sending after the send half was shut down, which is EPIPE on Unix.
        win32::WSAESHUTDOWN => ErrorKind::BrokenPipe,
        win32::WSAECONNREFUSED => ErrorKind::ConnectionRefused,
        win32::WSAEHOSTUNREACH => ErrorKind::HostUnreachable,
        _ => ErrorKind::Uncategorized,
    }
}

fn win32_error_text(code: u32) -> Option<&'static str> {
    Some(match code {
        0 => "operation successful",
        win32::FILE_NOT_FOUND => "file not found",
        win32::PATH_NOT_FOUND => "path not found",
        win32::ACCESS_DENIED => "access denied",
        win32::NOT_ENOUGH_MEMORY => "not enough memory",
        win32::OUTOFMEMORY => "out of memory",
        win32::NOT_SAME_DEVICE => "not same device",
        win32::NOT_SUPPORTED => "not supported",
        win32::FILE_EXISTS => "file exists",
        win32::INVALID_PARAMETER => "invalid parameter",
        win32::DISK_FULL => "disk full",
        win32::CALL_NOT_IMPLEMENTED => "call not implemented",
        win32::INVALID_NAME => "invalid name",
        win32::DIR_NOT_EMPTY => "directory not empty",
        win32::BUSY => "device busy",
        win32::ALREADY_EXISTS => "already exists",
        win32::WAIT_TIMEOUT | win32::TIMEOUT => "timed out",
        win32::DIRECTORY => "not a directory",
        win32::NOT_FOUND => "element not found",
        win32::WSAEACCES => "socket access denied",
        win32::WSAEINVAL => "invalid socket argument",
        win32::WSAEMFILE => "too many open sockets",
        win32::WSAEWOULDBLOCK => "operation would block",
        win32::WSAEADDRINUSE => "address in use",
        win32::WSAEADDRNOTAVAIL => "address not available",
        win32::WSAENETDOWN => "network down",
        win32::WSAENETUNREACH => "network unreachable",
        win32::WSAECONNABORTED => "connection aborted",
        win32::WSAECONNRESET => "connection reset",
        win32::WSAENOTCONN => "socket not connected",
        win32::WSAESHUTDOWN => "socket shut down",
        win32::WSAETIMEDOUT => "connection timed out",
        win32::WSAECONNREFUSED => "connection refused",
        win32::WSAEHOSTUNREACH => "host unreachable",
        _ => return None,
    })
}
