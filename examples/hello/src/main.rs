#![no_std]
#![no_main]

use core::ffi::CStr;
use core::panic::PanicInfo;

use xbox360_sys::xam;
use xbox360_sys::xam::XnCaller;
use xbox360_sys::xboxkrnl;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

/// `text` is the `DbgPrint` format, so it must not contain `%`.
fn dbg(text: &CStr) {
    // SAFETY: `text` is NUL-terminated, and every caller passes a literal
    // without `%`, so DbgPrint reads no further arguments.
    unsafe { xboxkrnl::DbgPrint(text.as_ptr().cast()) };
}

// The loader and the thread start code call these with the console ABI, so
// they enter through the bridge that preserves r14-r31 in full.
xbox360_sys::bridged!(#[unsafe(no_mangle)] pub _start, main);
xbox360_sys::bridged!(sender_entry, sender);

// Worker thread: sends a UDP datagram. Receives the socket handle in r3.
extern "C" fn sender(socket: u32) -> u32 {
    let destination = xam::SockAddrIn {
        family: xam::AddressFamily::Inet.raw() as u16,
        port: 9999,
        address: 0x7F00_0001,
        zero: [0; 8],
    };
    let message = b"hello from xbox 360 rust over udp";
    // SAFETY: `message` and `destination` are live for the call and their
    // lengths are passed with them; `socket` is the socket that main keeps
    // open until this thread ends.
    let sent = unsafe {
        xam::NetDll_sendto(
            XnCaller::Title,
            xam::Socket::from_raw(socket),
            message.as_ptr(),
            message.len() as i32,
            xam::MessageFlags::empty(),
            &destination as *const xam::SockAddrIn as *const u8,
            size_of::<xam::SockAddrIn>() as i32,
        )
    };
    if sent == xam::SOCKET_ERROR {
        dbg(c"sender: sendto failed\n");
    } else {
        dbg(c"sender: datagram sent\n");
    }

    // SAFETY: no frame of this thread owns a value that must be dropped.
    unsafe { xboxkrnl::ExTerminateThread(0) }
}

extern "C" fn main(_ctx: u32) -> u32 {
    dbg(c"main: starting network\n");
    let mut wsadata = xam::WsaData::new();
    // SAFETY: a null parameter block selects the default settings, and
    // `wsadata` is live local storage.
    unsafe {
        xam::NetDll_XNetStartup(XnCaller::Title, core::ptr::null());
        xam::NetDll_WSAStartup(
            XnCaller::Title,
            xam::WinsockVersion::new(2, 2),
            &mut wsadata,
        );
    }

    // SAFETY: the network stack was started above; the call takes no
    // pointers.
    let socket = unsafe {
        xam::NetDll_socket(
            XnCaller::Title,
            xam::AddressFamily::Inet,
            xam::SocketType::Datagram,
            xam::Protocol::Udp,
        )
    };
    if socket == xam::Socket::INVALID {
        dbg(c"main: socket failed\n");
        // SAFETY: main runs on the title's main thread.
        unsafe { xam::XamLoaderTerminateTitle() }
    }

    dbg(c"main: socket ready, spawning sender thread\n");

    let mut handle: u32 = 0;
    // SAFETY: `sender_entry` is a bridged entry for an
    // `extern "C" fn(u32) -> u32`, `handle` is a live local, and a null id
    // pointer is accepted.
    unsafe {
        xboxkrnl::ExCreateThread(
            &mut handle as *mut u32,
            0,
            core::ptr::null_mut(),
            0,
            sender_entry as usize as u32,
            socket.raw(),
            xboxkrnl::ThreadCreationFlags::empty(),
        )
    };
    let handle = xboxkrnl::Handle::from_raw(handle);

    // SAFETY: `handle` is the thread handle created above, closed once
    // after the wait; `socket` is closed after the thread has ended.
    unsafe {
        xboxkrnl::NtWaitForSingleObjectEx(
            handle,
            xboxkrnl::ProcessorMode::User,
            false,
            core::ptr::null(),
        );
        xboxkrnl::NtClose(handle);
        xam::NetDll_closesocket(XnCaller::Title, socket);
    }
    dbg(c"main: done\n");

    // A title must not return from its entry point: the kernel stops with
    // fatal error 0xE.
    // SAFETY: main runs on the title's main thread.
    unsafe { xam::XamLoaderTerminateTitle() }
}
