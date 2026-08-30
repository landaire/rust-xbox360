//! Multithreading smoke test: 4 real threads (ExCreateThread) share an
//! Arc<Mutex>, each adds 250; join; STAGE = total (expect 1000 = 0x3E8).
#![no_main]
use std::sync::{Arc, Mutex};
use std::thread;
core::arch::global_asm!(".globl _start",".section .text._start,\"ax\"","_start:","bl real_main","1: b 1b");
#[unsafe(no_mangle)] pub static mut STAGE: u32 = 0;
fn stg(n:u32){ unsafe{core::ptr::write_volatile(&raw mut STAGE,n);} }
#[unsafe(no_mangle)] extern "C" fn real_main() {
    stg(0x01);
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..250 { *c.lock().unwrap() += 1; }
        }));
    }
    stg(0x02);
    for h in handles { h.join().unwrap(); }
    let total = *counter.lock().unwrap();
    stg(total); // 1000 = 0x3E8 if threads+mutex work
    loop { let mut t=0u32; while t<800_000 { unsafe{core::ptr::read_volatile(&t)}; t=t.wrapping_add(1);} }
}
