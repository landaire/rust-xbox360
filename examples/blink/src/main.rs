#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn dbg(m: &[u8]) {
    xbox_sys::DbgPrint(m.as_ptr());
}

// Busy-wait roughly a second's worth of iterations. `read_volatile` keeps the
// loop from being optimized away; the exact wall time does not matter, only
// that output is spaced out enough to observe over the XBDM notify channel.
fn spin() {
    let mut i: u32 = 0;
    while i < 40_000_000 {
        unsafe {
            core::ptr::read_volatile(&i);
        }
        i = i.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    dbg(b"blink: title alive on real hardware\n\0");
    let mut n: u32 = 0;
    loop {
        // No heap, no writable globals: render the counter on the stack.
        let mut line = *b"blink: tick 000\n\0";
        line[12] = b'0' + ((n / 100) % 10) as u8;
        line[13] = b'0' + ((n / 10) % 10) as u8;
        line[14] = b'0' + (n % 10) as u8;
        dbg(&line);
        spin();
        n = n.wrapping_add(1);
    }
}
