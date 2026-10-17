#![no_std]
#![no_main]

use core::panic::PanicInfo;

// A single import thunk for DbgPrint (xboxkrnl.exe ordinal 3). xex-link rewrites
// this 16-byte record into a jump stub through the IAT. Record encodes
// type<<24 | library_index<<16 | ordinal; library 0 == xboxkrnl.exe.
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".xthunks")]
pub extern "C" fn DbgPrint(_fmt: *const u8) -> u32 {
    core::arch::naked_asm!(
        ".long 0x01000003", // type 1, lib 0, ordinal 3
        ".long 0x60000000",
        ".long 0x60000000",
        ".long 0x4e800020",
    )
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn spin() {
    let mut count: u32 = 0;
    while count < 40_000_000 {
        unsafe {
            core::ptr::read_volatile(&count);
        }
        count = count.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    loop {
        DbgPrint(b"hello2: alive from rust homebrew on real xbox 360\n\0".as_ptr());
        spin();
    }
}
