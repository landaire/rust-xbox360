#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }
fn dbg(msg: &[u8]) { xbox_sys::DbgPrint(msg.as_ptr()); }
extern "C" fn child(_c: u32) -> u32 {
    dbg(b"child: running\n\0");
    xbox_sys::ExTerminateThread(0);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn _start(_context: u32) -> u32 {
    dbg(b"main: creating a thread\n\0");
    let mut handle: u32 = 0;
    xbox_sys::ExCreateThread(&mut handle as *mut u32, 0, core::ptr::null_mut(), 0, child as usize as u32, 0, 0);
    dbg(b"main: waiting for child\n\0");
    xbox_sys::NtWaitForSingleObjectEx(handle, 1, 0, core::ptr::null());
    xbox_sys::NtClose(handle);
    dbg(b"main: child joined\n\0");
    0x1234
}
