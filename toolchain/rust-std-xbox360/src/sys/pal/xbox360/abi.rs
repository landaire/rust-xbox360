//! Xbox 360 kernel and networking imports.
//!
//! Each import is a 16-byte naked thunk placed in `.xthunks`. The first word
//! encodes `type(1) | library_index | ordinal`; the kernel loader resolves the
//! thunk at load time.
//! Library index 0 is `xboxkrnl.exe`, index 1 is `xam.xex` (the NetDll_* set).

#![allow(dead_code)]

/// Emit a naked import thunk. The body is inert until the loader patches it.
macro_rules! kimport {
    ($name:ident($($a:ident: $t:ty),*) $(-> $r:ty)?, $lib:expr, $ord:expr) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".xthunks")]
        pub extern "C" fn $name($($a: $t),*) $(-> $r)? {
            core::arch::naked_asm!(
                ".long {record}",
                ".long 0x60000000",
                ".long 0x60000000",
                ".long 0x4e800020",
                record = const 0x0100_0000u32 | (($lib as u32) << 16) | ($ord as u32),
            )
        }
    };
}

// Pool allocator (backs the global allocator).
kimport!(ExAllocatePoolWithTag(size: usize, tag: u32) -> *mut u8, 0, 10);
kimport!(ExFreePool(p: *mut u8), 0, 15);

// Debug output. Variadic printf in C, but we call it with a single
// pre-formatted (and %-escaped) string to avoid the variadic-call ABI.
kimport!(DbgPrint(fmt: *const u8) -> u32, 0, 3);
