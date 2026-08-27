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
