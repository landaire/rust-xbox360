#![no_std]
#![no_main]

use core::ffi::CStr;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

/// `text` is the `DbgPrint` format, so it must not contain `%`.
fn dbg(text: &CStr) {
    // SAFETY: `text` is NUL-terminated, and every caller passes text
    // without `%`, so DbgPrint reads no further arguments.
    unsafe { xbox360_sys::xboxkrnl::DbgPrint(text.as_ptr().cast()) };
}

// Busy-wait roughly a second's worth of iterations. `read_volatile` keeps the
// loop from being optimized away; the exact wall time does not matter, only
// that output is spaced out enough to observe over the XBDM notify channel.
fn spin() {
    let mut count: u32 = 0;
    while count < 40_000_000 {
        // SAFETY: `count` is a live local.
        unsafe {
            core::ptr::read_volatile(&count);
        }
        count = count.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    dbg(c"blink: title alive on real hardware\n");

    let mut tick: u32 = 0;
    loop {
        // No heap, no writable globals: render the counter on the stack.
        let mut line = *b"blink: tick 000\n\0";
        line[12] = b'0' + ((tick / 100) % 10) as u8;
        line[13] = b'0' + ((tick / 10) % 10) as u8;
        line[14] = b'0' + (tick % 10) as u8;
        // SAFETY: `line` ends in its only NUL.
        dbg(unsafe { CStr::from_bytes_with_nul_unchecked(&line) });

        spin();
        tick = tick.wrapping_add(1);
    }
}
