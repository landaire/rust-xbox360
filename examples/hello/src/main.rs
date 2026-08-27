#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }

fn dbg(m: &[u8]) { xbox_sys::DbgPrint(m.as_ptr()); }

// Guest sockaddr_in: 16 bytes, big-endian family/port/addr (native on PPC).
#[repr(C)]
struct SockAddrIn {
    family: u16,
    port: u16,
    addr: u32,
    zero: [u8; 8],
}

// Worker thread: sends a UDP datagram. Receives the socket handle in r3.
extern "C" fn sender(sock: u32) -> u32 {
    let dst = SockAddrIn {
        family: 2,            // AF_INET
        port: 9999,
        addr: 0x7F00_0001,
        zero: [0; 8],
    };
    let msg = b"hello from xbox 360 rust over udp";
    let sent = xbox_sys::NetDll_sendto(
        xbox_sys::XNCALLER_TITLE,
        sock,
        msg.as_ptr(),
        msg.len() as u32,
        0,
        &dst as *const SockAddrIn as *const u8,
        16,
    );
    if sent == 0xFFFF_FFFF {
        dbg(b"sender: sendto failed\n\0");
    } else {
        dbg(b"sender: datagram sent\n\0");
    }
    xbox_sys::ExTerminateThread(0);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    dbg(b"main: starting network\n\0");
    xbox_sys::NetDll_XNetStartup(xbox_sys::XNCALLER_TITLE, core::ptr::null());
    let mut wsadata = [0u8; 512];
    xbox_sys::NetDll_WSAStartup(xbox_sys::XNCALLER_TITLE, 0x0202, wsadata.as_mut_ptr());

    let sock = xbox_sys::NetDll_socket(xbox_sys::XNCALLER_TITLE, 2, 2, 17);
    if sock == 0xFFFF_FFFF {
        dbg(b"main: socket failed\n\0");
        return 1;
    }
    dbg(b"main: socket ready, spawning sender thread\n\0");

    let mut handle: u32 = 0;
    xbox_sys::ExCreateThread(
        &mut handle as *mut u32,
        0,
        core::ptr::null_mut(),
        0,
        sender as usize as u32,
        sock,
        0,
    );
    xbox_sys::NtWaitForSingleObjectEx(handle, 1, 0, core::ptr::null());
    xbox_sys::NtClose(handle);
    xbox_sys::NetDll_closesocket(xbox_sys::XNCALLER_TITLE, sock);
    dbg(b"main: done\n\0");
    0x1234
}
