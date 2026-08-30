//! Multi-thread tokio on the Xbox 360: a work-stealing runtime with 2 workers
//! runs async tasks (each awaits a timer), and spawn_blocking does an HTTP GET
//! to the host web server over the xbox360 net backend.
#![no_main]
use std::io::{Read, Write};
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::time::Duration;
core::arch::global_asm!(".globl _start",".section .text._start,\"ax\"","_start:","bl real_main","1: b 1b");
#[unsafe(no_mangle)] pub static mut STAGE: u32 = 0;
#[unsafe(no_mangle)] pub static mut BYTES: u32 = 0;
fn stg(n:u32){ unsafe{core::ptr::write_volatile(&raw mut STAGE,n);} }
const HOST: &str = "192.168.1.74:8080";
fn http_get() -> std::io::Result<usize> {
    let mut s = std::net::TcpStream::connect(HOST)?;
    s.write_all(format!("GET / HTTP/1.0\r\nHost: {HOST}\r\nConnection: close\r\n\r\n").as_bytes())?;
    let mut b = Vec::new(); s.read_to_end(&mut b)?; Ok(b.len())
}
#[unsafe(no_mangle)] extern "C" fn real_main() {
    stg(0x01);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2).enable_time().build().unwrap();
    stg(0x02);
    rt.block_on(async {
        stg(0x03);
        // fan out async tasks across workers
        let sum = Arc::new(AtomicU32::new(0));
        let mut handles = Vec::new();
        for i in 1..=8u32 {
            let s = Arc::clone(&sum);
            handles.push(tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(20)).await;
                s.fetch_add(i*i, Ordering::SeqCst);
            }));
        }
        for h in handles { h.await.unwrap(); }
        stg(0x04 | (sum.load(Ordering::SeqCst) << 8)); // 0x04 | (204<<8) if all ran (1+4+..+64=204)
        // HTTP GET on the blocking pool (real threads)
        match tokio::task::spawn_blocking(http_get).await.unwrap() {
            Ok(n) => { unsafe{core::ptr::write_volatile(&raw mut BYTES, n as u32)}; stg(0x55); }
            Err(_) => stg(0xEE),
        }
    });
    loop { let mut t=0u32; while t<800_000 { unsafe{core::ptr::read_volatile(&t)}; t=t.wrapping_add(1);} }
}
