#![no_std]
#![no_main]

use core::panic::PanicInfo;

// xboxkrnl imports (library index 0). The loader resolves these 16-byte records
// by ordinal.
macro_rules! import {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $ordinal:expr) => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        #[unsafe(link_section = ".xthunks")]
        pub extern "C" fn $name($(_: $arg_ty),*) -> $ret {
            core::arch::naked_asm!(
                ".long {record}",
                ".long 0x60000000", ".long 0x60000000", ".long 0x4e800020",
                record = const 0x0100_0000u32 | ($ordinal as u32),
            )
        }
    };
}

import!(DbgPrint(fmt: *const u8) -> u32, 0x03);
import!(
    ExCreateThread(
        handle: *mut u32,
        stack_size: u32,
        thread_id: *mut u32,
        xapi_startup: u32,
        start: u32,
        context: u32,
        flags: u32,
    ) -> u32,
    0x0D
);
import!(ExTerminateThread(code: u32) -> u32, 0x19);
import!(
    NtWaitForSingleObjectEx(handle: u32, mode: u32, alertable: u32, timeout: *const u8) -> u32,
    0xFD
);
import!(NtClose(handle: u32) -> u32, 0xCF);

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn dbg(message: &[u8]) {
    DbgPrint(message.as_ptr());
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

        let mut count: u32 = 0;
        while count < 60_000_000 {
            unsafe {
                core::ptr::read_volatile(&count);
            }
            count = count.wrapping_add(1);
        }
    }
}
