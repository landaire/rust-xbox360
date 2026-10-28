//! Comprehensive std smoke test for the Xbox 360: allocator + println, real
//! threads sharing an Arc<Mutex>, and std::fs (create/write, open/read). STAGE
//! reaches 0x55 when all pass. Read STAGE once with `xeedee getmem <addr> 4`.
#![no_main]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[unsafe(no_mangle)]
pub static mut STAGE: u32 = 0;

core::arch::global_asm!(
    ".globl _start",
    ".section .text._start,\"ax\"",
    "_start:",
    "bl real_main",
    "1: b 1b"
);

fn set_stage(stage: u32) {
    unsafe { core::ptr::write_volatile(&raw mut STAGE, stage) };
}

fn fs_test() -> std::io::Result<bool> {
    const FS_PATH: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\homebrew\\fstest.txt";
    const FS_MESSAGE: &[u8] = b"hello from the xbox 360 std::fs backend\n";

    {
        let mut file = File::create(FS_PATH)?;
        file.write_all(FS_MESSAGE)?;
    }

    let mut buf = [0u8; 256];
    let mut file = File::open(FS_PATH)?;
    let len = file.read(&mut buf)?;
    Ok(&buf[..len] == FS_MESSAGE)
}

fn threads_test() -> bool {
    let counter = Arc::new(Mutex::new(0u32));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..250 {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap() == 1000
}

#[unsafe(no_mangle)]
extern "C" fn real_main() {
    set_stage(0x01);

    let squares: Vec<u32> = (1..=8).map(|value| value * value).collect();
    set_stage(0x02);
    println!("stdhello: sum of squares = {}", squares.iter().sum::<u32>());
    set_stage(0x11);

    // An error without an OS code reports 0xFFFF in the low half of STAGE.
    let fs_ok = match fs_test() {
        Ok(matches) => matches,
        Err(e) => {
            set_stage(0xF0_0000 | (e.raw_os_error().unwrap_or(-1) as u32 & 0xffff));
            return;
        }
    };
    set_stage(0x22);

    let threads_ok = threads_test();
    set_stage(if fs_ok && threads_ok { 0x55 } else { 0xE1 });

    loop {
        let mut count = 0u32;
        while count < 800_000 {
            unsafe { core::ptr::read_volatile(&count) };
            count = count.wrapping_add(1);
        }
    }
}
