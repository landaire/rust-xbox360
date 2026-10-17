#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C)]
struct SockAddrIn {
    family: u16,
    port: u16,
    addr: u32,
    zero: [u8; 8],
}

// Import thunk: record = type<<24 | library_index<<16 | ordinal. Library 0 is
// xboxkrnl.exe, library 1 is xam.xex (where NetDll_* live). xex-link rewrites
// these into IAT jump stubs.
macro_rules! import {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $library:expr, $ordinal:expr) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".xthunks")]
        pub extern "C" fn $name($(_: $arg_ty),*) -> $ret {
            core::arch::naked_asm!(
                ".long {record}",
                ".long 0x60000000", ".long 0x60000000", ".long 0x4e800020",
                record = const 0x0100_0000u32 | (($library as u32) << 16) | ($ordinal as u32),
            )
        }
    };
}

import!(DbgPrint(fmt: *const u8) -> u32, 0, 0x03);
// xam.xex NetDll_* take a leading `caller` = XNCALLER_TITLE (1).
import!(
    NetDll_XNetStartup(caller: u32, params: *const u8) -> u32,
    1,
    0x33
);
import!(
    NetDll_WSAStartup(caller: u32, ver: u32, data: *mut u8) -> u32,
    1,
    0x01
);
import!(
    NetDll_socket(caller: u32, af: u32, ty: u32, proto: u32) -> u32,
    1,
    0x03
);
import!(
    NetDll_sendto(
        caller: u32,
        socket: u32,
        buf: *const u8,
        len: u32,
        flags: u32,
        to: *const u8,
        tolen: u32,
    ) -> u32,
    1,
    0x18
);
import!(NetDll_closesocket(caller: u32, socket: u32) -> u32, 1, 0x04);

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn dbg(message: &[u8]) {
    DbgPrint(message.as_ptr());
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    const CALLER_TITLE: u32 = 1;

    dbg(b"net: XNetStartup (bypass security)\n\0");

    // XNetStartupParams: [0]=cfgSizeOfStruct(0x0D), [1]=cfgFlags. Devkit honors
    // XNET_STARTUP_BYPASS_SECURITY (0x01), letting us talk to a normal host.
    let mut startup_params = [0u8; 16];
    startup_params[0] = 0x0D;
    startup_params[1] = 0x01;
    NetDll_XNetStartup(CALLER_TITLE, startup_params.as_ptr());
    let mut wsadata = [0u8; 512];
    NetDll_WSAStartup(CALLER_TITLE, 0x0202, wsadata.as_mut_ptr());

    let socket = NetDll_socket(CALLER_TITLE, 2, 2, 17); // AF_INET, SOCK_DGRAM, UDP
    if socket == 0xFFFF_FFFF {
        dbg(b"net: socket failed\n\0");
        loop {}
    }

    // 192.168.1.74:9999 (host). Fields are network byte order = native on PPC.
    let destination = SockAddrIn {
        family: 2,
        port: 9999,
        addr: 0xC0A8_014A,
        zero: [0; 8],
    };
    let message = b"hello from rust homebrew on real xbox 360 over udp";

    loop {
        let sent = NetDll_sendto(
            CALLER_TITLE,
            socket,
            message.as_ptr(),
            message.len() as u32,
            0,
            &destination as *const SockAddrIn as *const u8,
            16,
        );
        if sent == 0xFFFF_FFFF {
            dbg(b"net: sendto failed\n\0");
        } else {
            dbg(b"net: datagram sent to host\n\0");
        }

        let mut count: u32 = 0;
        while count < 120_000_000 {
            unsafe {
                core::ptr::read_volatile(&count);
            }
            count = count.wrapping_add(1);
        }
    }
}
