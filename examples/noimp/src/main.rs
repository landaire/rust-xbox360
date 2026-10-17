#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Observable state written to a writable global. With no kernel imports, the
// title cannot DbgPrint, but XBDM can read guest memory: a host reading this
// counter over `getmem` sees it advancing, proving the title runs on hardware.
#[unsafe(no_mangle)]
pub static mut HEARTBEAT: [u32; 2] = [0x4845_4C4F, 0]; // "HELO" magic, then counter

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    let mut count: u32 = 0;
    loop {
        unsafe {
            core::ptr::write_volatile(&raw mut HEARTBEAT[1], count);
        }
        count = count.wrapping_add(1);
    }
}
