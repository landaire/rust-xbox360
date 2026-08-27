#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

// The XEX entry point. Xenia calls it with r3 = start context and LR set to the
// 0xBCBCBCBC exit sentinel, so returning here exits the main thread with the
// value in r3. For the first bring-up we just return a recognizable exit code.
#[no_mangle]
pub extern "C" fn _start(_context: u32) -> u32 {
    0x1234
}
