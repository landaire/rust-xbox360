//! Comprehensive std smoke test for the Xbox 360: allocator + println, real
//! threads sharing an Arc<Mutex>, and std::fs (create/write, open/read). STAGE
//! reaches 0x55 when all pass. Read STAGE once with `xeedee getmem <addr> 4`.
#![no_main]
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
core::arch::global_asm!(".globl _start",".section .text._start,\"ax\"","_start:","bl real_main","1: b 1b");
#[unsafe(no_mangle)] pub static mut STAGE: u32 = 0;
fn stg(n: u32) { unsafe { core::ptr::write_volatile(&raw mut STAGE, n) }; }
const FSPATH: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\homebrew\\fstest.txt";
const FSMSG: &[u8] = b"hello from the xbox 360 std::fs backend\n";
fn fs_test() -> std::io::Result<bool> {
    { File::create(FSPATH)?.write_all(FSMSG)?; }
    let mut buf = [0u8; 256];
    let n = File::open(FSPATH)?.read(&mut buf)?;
    Ok(&buf[..n] == FSMSG)
}
fn threads_test() -> bool {
    let c = Arc::new(Mutex::new(0u32));
    let hs: Vec<_> = (0..4).map(|_| { let c=Arc::clone(&c); thread::spawn(move|| for _ in 0..250 { *c.lock().unwrap()+=1; }) }).collect();
    for h in hs { h.join().unwrap(); }
    *c.lock().unwrap() == 1000
}
#[unsafe(no_mangle)] extern "C" fn real_main() {
    stg(0x01);
    let v: Vec<u32> = (1..=8).map(|i| i*i).collect();
    println!("stdhello: sum of squares = {}", v.iter().sum::<u32>());
    stg(0x11);
    let fs_ok = match fs_test() { Ok(b) => b, Err(e) => { stg(0xF0_0000 | (e.raw_os_error().unwrap_or(-1) as u32 & 0xffff)); return; } };
    stg(0x22);
    let th_ok = threads_test();
    stg(if fs_ok && th_ok { 0x55 } else { 0xE1 });
    loop { let mut t=0u32; while t<800_000 { unsafe{core::ptr::read_volatile(&t)}; t=t.wrapping_add(1);} }
}
