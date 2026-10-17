//! Multi-thread tokio on the Xbox 360: a work-stealing runtime with 2 workers
//! runs async tasks (each awaits a timer), and spawn_blocking does an HTTP GET
//! to the host web server over the xbox360 net backend.
#![no_main]

use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[unsafe(no_mangle)]
pub static mut STAGE: u32 = 0;
#[unsafe(no_mangle)]
pub static mut BYTES: u32 = 0;

core::arch::global_asm!(
    ".globl _start",
    ".section .text._start,\"ax\"",
    "_start:",
    "bl real_main",
    "1: b 1b"
);

fn set_stage(stage: u32) {
    unsafe {
        core::ptr::write_volatile(&raw mut STAGE, stage);
    }
}

fn http_get() -> std::io::Result<usize> {
    const HOST: &str = "192.168.1.74:8080";

    let mut stream = std::net::TcpStream::connect(HOST)?;
    stream.write_all(
        format!("GET / HTTP/1.0\r\nHost: {HOST}\r\nConnection: close\r\n\r\n").as_bytes(),
    )?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    Ok(response.len())
}

#[unsafe(no_mangle)]
extern "C" fn real_main() {
    set_stage(0x01);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_time()
        .build()
        .unwrap();
    set_stage(0x02);

    runtime.block_on(async {
        set_stage(0x03);

        // Fan out async tasks across workers.
        let sum = Arc::new(AtomicU32::new(0));
        let mut handles = Vec::new();
        for value in 1..=8u32 {
            let sum = Arc::clone(&sum);
            handles.push(tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(20)).await;
                sum.fetch_add(value * value, Ordering::SeqCst);
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // 0x04 | (204 << 8) when every task ran (1 + 4 + ... + 64 = 204).
        set_stage(0x04 | (sum.load(Ordering::SeqCst) << 8));

        // HTTP GET on the blocking pool (real threads).
        match tokio::task::spawn_blocking(http_get).await.unwrap() {
            Ok(len) => {
                unsafe { core::ptr::write_volatile(&raw mut BYTES, len as u32) };
                set_stage(0x55);
            }
            Err(_) => set_stage(0xEE),
        }
    });

    loop {
        let mut count = 0u32;
        while count < 800_000 {
            unsafe { core::ptr::read_volatile(&count) };
            count = count.wrapping_add(1);
        }
    }
}
