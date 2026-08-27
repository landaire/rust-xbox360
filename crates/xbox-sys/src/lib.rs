//! Xbox 360 kernel and Xam imports for the `powerpc-unknown-xbox360` target.
//!
//! Each import is a 16-byte thunk placed in the `.xthunks` section. The first
//! big-endian DWORD is the import record `(type<<24 | lib<<16 | ordinal)`;
//! `xex-link` collects these into the XEX `ImportLibraries` table, and at load
//! time Xenia (and a real console) rewrites the thunk to a syscall trap. The
//! remaining DWORDs (`nop; nop; blr`) are only a fallback if never patched.

#![no_std]

/// Library index shared with `xex-link`'s `LIBRARY_NAMES`.
pub mod lib_index {
    pub const XBOXKRNL: u32 = 0;
    pub const XAM: u32 = 1;
}

/// Define an import thunk: a naked 16-byte function whose first DWORD encodes
/// the import record. The signature is what callers use; the args are passed
/// through in r3.. to the resolved kernel/Xam export.
#[macro_export]
macro_rules! import_thunk {
    ($(#[$meta:meta])* $name:ident($($arg:ident: $argty:ty),* $(,)?) -> $ret:ty, lib = $lib:expr, ordinal = $ord:expr) => {
        $(#[$meta])*
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".xthunks")]
        pub extern "C" fn $name($(_: $argty),*) -> $ret {
            core::arch::naked_asm!(
                ".long {record}",
                ".long 0x60000000", // nop
                ".long 0x60000000", // nop
                ".long 0x4e800020", // blr
                record = const (1u32 << 24) | (($lib as u32) << 16) | ($ord as u32),
            )
        }
    };
}

import_thunk!(
    /// `DbgPrint(format, ...)` -- xboxkrnl.exe ordinal 0x03. Writes a debug
    /// string. Only the format pointer (r3) is required for a plain string.
    DbgPrint(format: *const u8) -> u32,
    lib = lib_index::XBOXKRNL,
    ordinal = 0x03
);

import_thunk!(
    /// `ExCreateThread` -- xboxkrnl.exe ordinal 0x0D. Creates a guest thread.
    /// With `xapi_thread_startup` = 0 the thread is raw: it enters
    /// `start_address` with r3 = `start_context` and its return value is the
    /// exit code. `creation_flags` = 0 starts it immediately (not suspended)
    /// and returns a handle via `handle_out`.
    ExCreateThread(
        handle_out: *mut u32,
        stack_size: u32,
        thread_id_out: *mut u32,
        xapi_thread_startup: u32,
        start_address: u32,
        start_context: u32,
        creation_flags: u32,
    ) -> u32,
    lib = lib_index::XBOXKRNL,
    ordinal = 0x0D
);

import_thunk!(
    /// `ExTerminateThread` -- xboxkrnl.exe ordinal 0x19. Exits the current
    /// thread with `exit_code`. Raw threads should call this rather than
    /// returning to the loader's exit sentinel.
    ExTerminateThread(exit_code: u32) -> u32,
    lib = lib_index::XBOXKRNL,
    ordinal = 0x19
);

import_thunk!(
    /// `NtWaitForSingleObjectEx` -- xboxkrnl.exe ordinal 0xFD. Waits on a
    /// handle. A null `timeout` waits forever.
    NtWaitForSingleObjectEx(
        handle: u32,
        wait_mode: u32,
        alertable: u32,
        timeout: *const u8,
    ) -> u32,
    lib = lib_index::XBOXKRNL,
    ordinal = 0xFD
);

import_thunk!(
    /// `NtClose` -- xboxkrnl.exe ordinal 0xCF. Closes a handle.
    NtClose(handle: u32) -> u32,
    lib = lib_index::XBOXKRNL,
    ordinal = 0xCF
);

/// `caller` value passed to the `NetDll_*` functions (XNCALLER_TITLE).
pub const XNCALLER_TITLE: u32 = 1;

import_thunk!(
    /// `NetDll_XNetStartup` -- xam.xex ordinal 0x33. Initializes the network
    /// stack (a no-op success in Xenia). `params` may be null.
    NetDll_XNetStartup(caller: u32, params: *const u8) -> u32,
    lib = lib_index::XAM,
    ordinal = 0x33
);

import_thunk!(
    /// `NetDll_WSAStartup` -- xam.xex ordinal 0x01. `data` receives WSADATA.
    NetDll_WSAStartup(caller: u32, version: u32, data: *mut u8) -> u32,
    lib = lib_index::XAM,
    ordinal = 0x01
);

import_thunk!(
    /// `NetDll_socket` -- xam.xex ordinal 0x03. Returns a socket handle or -1.
    NetDll_socket(caller: u32, af: u32, ty: u32, protocol: u32) -> u32,
    lib = lib_index::XAM,
    ordinal = 0x03
);

import_thunk!(
    /// `NetDll_sendto` -- xam.xex ordinal 0x18. `to` points at an XSOCKADDR_IN
    /// (16 bytes, big-endian family/port/addr). Returns bytes sent or -1.
    NetDll_sendto(
        caller: u32,
        socket_handle: u32,
        buf: *const u8,
        len: u32,
        flags: u32,
        to: *const u8,
        to_len: u32,
    ) -> u32,
    lib = lib_index::XAM,
    ordinal = 0x18
);

import_thunk!(
    /// `NetDll_closesocket` -- xam.xex ordinal 0x04.
    NetDll_closesocket(caller: u32, socket_handle: u32) -> u32,
    lib = lib_index::XAM,
    ordinal = 0x04
);
