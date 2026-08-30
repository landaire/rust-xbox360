#![no_main]
core::arch::global_asm!(".globl _start",".section .text._start,\"ax\"","_start:","bl real_main","1: b 1b");
#[unsafe(no_mangle)] pub static mut STAGE: u32 = 0;
fn stg(n:u32){ unsafe{core::ptr::write_volatile(&raw mut STAGE,n);} }
#[unsafe(no_mangle)] extern "C" fn real_main() {
    stg(0x11);
    println!("stdhello: println alive");
    stg(0x22);
    let v: Vec<u32> = (1..=8).map(|i| i*i).collect();   // allocator
    let sum: u32 = v.iter().sum();
    stg(0x33);
    let s = format!("stdhello: sum={sum} v={v:?}");       // heap + format
    println!("{s}");
    stg(0x44);
    // std sync/atomics: Mutex + Arc (protected lwarx/stwcx)
    use std::sync::{Arc, Mutex};
    let m = Arc::new(Mutex::new(0u32));
    for _ in 0..1000 { *m.lock().unwrap() += 1; }
    let total = *m.lock().unwrap();
    println!("stdhello: Arc<Mutex> counted to {total}");
    stg(if total==1000 {0x55} else {0xEE});
    loop { let mut t=0u32; while t<800_000 { unsafe{core::ptr::read_volatile(&t)}; t=t.wrapping_add(1);} }
}
