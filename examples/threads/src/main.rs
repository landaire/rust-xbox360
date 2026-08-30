#![no_std]
#![no_main]
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

// xboxkrnl imports (library index 0). The loader resolves these 16-byte records
// by ordinal.
macro_rules! import {
    ($name:ident($($a:ident: $t:ty),*) -> $r:ty, $ord:expr) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".xthunks")]
        pub extern "C" fn $name($(_: $t),*) -> $r {
            core::arch::naked_asm!(
                ".long {record}",
                ".long 0x60000000", ".long 0x60000000", ".long 0x4e800020",
                record = const 0x0100_0000u32 | ($ord as u32),
            )
        }
    };
}

import!(DbgPrint(fmt: *const u8) -> u32, 0x03);
import!(ExCreateThread(h: *mut u32, ss: u32, tid: *mut u32, xa: u32, start: u32, ctx: u32, flags: u32) -> u32, 0x0D);
import!(ExTerminateThread(code: u32) -> u32, 0x19);
import!(NtWaitForSingleObjectEx(h: u32, mode: u32, alert: u32, to: *const u8) -> u32, 0xFD);
import!(NtClose(h: u32) -> u32, 0xCF);

fn dbg(m: &[u8]) {
    DbgPrint(m.as_ptr());
}

// Worker thread: raw thread entered with r3=context; must ExTerminateThread
// rather than return to the loader sentinel.
extern "C" fn worker(_ctx: u32) -> u32 {
    dbg(b"threads: worker running on its own thread\n\0");
    ExTerminateThread(0);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_ctx: u32) -> u32 {
    dbg(b"threads: main starting, spawning worker\n\0");
    let mut handle: u32 = 0;
    ExCreateThread(
        &mut handle as *mut u32,
        0,
        core::ptr::null_mut(),
        0,
        worker as usize as u32,
        0,
        0,
    );
    NtWaitForSingleObjectEx(handle, 1, 0, core::ptr::null());
    NtClose(handle);
    dbg(b"threads: main joined worker, done\n\0");
    loop {
        dbg(b"threads: still alive\n\0");
        let mut i: u32 = 0;
        while i < 60_000_000 {
            unsafe {
                core::ptr::read_volatile(&i);
            }
            i = i.wrapping_add(1);
        }
    }
}
