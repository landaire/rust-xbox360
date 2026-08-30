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

// Time sources. KeQueryPerformanceFrequency returns the timebase frequency by
// value; KeQuerySystemTime writes 100ns ticks since 1601 through a pointer.
kimport!(KeQueryPerformanceFrequency() -> u64, 0, 131);
kimport!(KeQuerySystemTime(time: *mut u64), 0, 132);

// Threads, waits, events, and TLS (xboxkrnl).
kimport!(ExCreateThread(handle: *mut u32, stack: u32, tid: *mut u32, xattr: u32, start: usize, ctx: usize, flags: u32) -> u32, 0, 13);
kimport!(ExTerminateThread(code: u32) -> u32, 0, 25);
kimport!(NtClose(handle: u32) -> u32, 0, 207);
kimport!(NtWaitForSingleObjectEx(handle: u32, mode: u32, alertable: u32, timeout: *const i64) -> u32, 0, 253);
kimport!(NtYieldExecution() -> u32, 0, 257);
kimport!(KeDelayExecutionThread(mode: u32, alertable: u32, interval: *const i64) -> u32, 0, 90);
kimport!(NtCreateEvent(handle: *mut u32, attr: *const u8, evtype: u32, initial: u32) -> u32, 0, 209);
kimport!(NtSetEvent(handle: u32, prev: *mut u32) -> u32, 0, 246);
kimport!(TlsAlloc() -> u32, 0, 338);
kimport!(TlsFree(idx: u32) -> u32, 0, 339);
kimport!(TlsGetValue(idx: u32) -> *mut u8, 0, 340);
kimport!(TlsSetValue(idx: u32, val: *mut u8) -> u32, 0, 341);

// NetDll_* winsock (library index 1 = xam.xex). Each takes a leading caller =
// XNCALLER_TITLE (1). Sockets are BSD/winsock-shaped.
kimport!(NetDll_XNetStartup(caller: u32, params: *const u8) -> i32, 1, 51);
kimport!(NetDll_WSAStartup(caller: u32, ver: u16, data: *mut u8) -> i32, 1, 1);
kimport!(NetDll_socket(caller: u32, af: i32, ty: i32, proto: i32) -> u32, 1, 3);
kimport!(NetDll_closesocket(caller: u32, s: u32) -> i32, 1, 4);
kimport!(NetDll_shutdown(caller: u32, s: u32, how: i32) -> i32, 1, 5);
kimport!(NetDll_ioctlsocket(caller: u32, s: u32, cmd: u32, argp: *mut u32) -> i32, 1, 6);
kimport!(NetDll_setsockopt(caller: u32, s: u32, level: i32, name: i32, val: *const u8, len: i32) -> i32, 1, 7);
kimport!(NetDll_getsockopt(caller: u32, s: u32, level: i32, name: i32, val: *mut u8, len: *mut i32) -> i32, 1, 8);
kimport!(NetDll_getsockname(caller: u32, s: u32, name: *mut u8, namelen: *mut i32) -> i32, 1, 9);
kimport!(NetDll_getpeername(caller: u32, s: u32, name: *mut u8, namelen: *mut i32) -> i32, 1, 10);
kimport!(NetDll_bind(caller: u32, s: u32, name: *const u8, namelen: i32) -> i32, 1, 11);
kimport!(NetDll_connect(caller: u32, s: u32, name: *const u8, namelen: i32) -> i32, 1, 12);
kimport!(NetDll_listen(caller: u32, s: u32, backlog: i32) -> i32, 1, 13);
kimport!(NetDll_accept(caller: u32, s: u32, name: *mut u8, namelen: *mut i32) -> u32, 1, 14);
kimport!(NetDll_recv(caller: u32, s: u32, buf: *mut u8, len: i32, flags: i32) -> i32, 1, 18);
kimport!(NetDll_recvfrom(caller: u32, s: u32, buf: *mut u8, len: i32, flags: i32, from: *mut u8, fromlen: *mut i32) -> i32, 1, 20);
kimport!(NetDll_send(caller: u32, s: u32, buf: *const u8, len: i32, flags: i32) -> i32, 1, 22);
kimport!(NetDll_sendto(caller: u32, s: u32, buf: *const u8, len: i32, flags: i32, to: *const u8, tolen: i32) -> i32, 1, 24);
kimport!(NetDll_WSAGetLastError(caller: u32) -> i32, 1, 27);
