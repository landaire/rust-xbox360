#![no_main]
use std::io::{Read, Write};
use std::time::Duration;
core::arch::global_asm!(".globl _start",".section .text._start,\"ax\"","_start:","bl real_main","1: b 1b");
#[unsafe(no_mangle)] pub static mut STAGE: u32 = 0;
#[unsafe(no_mangle)] pub static mut ERRCODE: i32 = 0;
fn stg(n:u32){ unsafe{core::ptr::write_volatile(&raw mut STAGE,n);} }
const HOST: &str = "192.168.1.74:8080";
fn http_get(host: &str) -> std::io::Result<usize> {
    let mut s = std::net::TcpStream::connect(host)?;
    s.write_all(format!("GET / HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n").as_bytes())?;
    let mut buf = Vec::new(); s.read_to_end(&mut buf)?; Ok(buf.len())
}
#[unsafe(no_mangle)] extern "C" fn real_main() {
    stg(0x11);
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    stg(0x22);
    rt.block_on(async {
        stg(0x33);
        tokio::time::sleep(Duration::from_millis(150)).await;
        stg(0x44);
        match http_get(HOST) {
            Ok(n) => { unsafe{core::ptr::write_volatile(&raw mut ERRCODE, n as i32)}; stg(0x55); }
            Err(e) => { unsafe{core::ptr::write_volatile(&raw mut ERRCODE, e.raw_os_error().unwrap_or(-1))}; stg(0xEE); }
        }
    });
    loop { let mut t=0u32; while t<800_000 { unsafe{core::ptr::read_volatile(&t)}; t=t.wrapping_add(1);} }
}
