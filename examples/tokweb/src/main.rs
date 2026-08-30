//! tokio on the Xbox 360: a current-thread runtime awaits an async timer, then
//! does an HTTP GET to a web server on the host machine over the xbox360 net
//! backend (blocking std::net inside the async task).
#![no_main]

use std::io::{Read, Write};
use std::time::Duration;

core::arch::global_asm!(
    ".globl _start", ".section .text._start,\"ax\"", "_start:", "bl real_main", "1: b 1b",
);

const HOST: &str = "192.168.1.74:8080";

fn http_get(host: &str, path: &str) -> std::io::Result<String> {
    let mut s = std::net::TcpStream::connect(host)?;
    let req = format!("GET {path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    s.write_all(req.as_bytes())?;
    let mut buf = Vec::new();
    s.read_to_end(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

#[unsafe(no_mangle)]
extern "C" fn real_main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    rt.block_on(async {
        println!("tokweb: tokio current-thread runtime is running on the xbox 360");
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("tokweb: async tokio::time::sleep completed (timer driver works)");
        match http_get(HOST, "/") {
            Ok(body) => {
                let head = body.lines().next().unwrap_or("");
                println!("tokweb: HTTP response status line: {head}");
                println!("tokweb: received {} bytes from {HOST}", body.len());
            }
            Err(e) => println!("tokweb: HTTP GET failed: {e}"),
        }
    });
    println!("tokweb: DONE");
}
