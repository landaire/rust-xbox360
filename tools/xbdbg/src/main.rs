//! Launch a title on an XBDM console with a debugger registered, and stream
//! every notification across the title reboot via the console's dial-back
//! (`notify reconnectport`). A load failure or entry fault then surfaces as a
//! notification instead of silently falling back to the shell.
//!
//! Usage: xbdbg <console_host:port> <listen_port> <title> [directory] [secs]

use std::time::Duration;
use std::time::Instant;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use xeedee::Client;
use xeedee::commands::debug::Debugger;
use xeedee::transport::tokio::connect;

#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    let host = args.next().unwrap_or_else(|| "192.168.1.26:730".into());
    let listen_port: u16 = args.next().and_then(|s| s.parse().ok()).unwrap_or(52730);
    let title = args.next().expect("title path");
    let directory = args.next().unwrap_or_default();
    let secs: u64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(45);

    let start = Instant::now();

    // Listen for the console's dial-back notification connections.
    let listener = TcpListener::bind(("0.0.0.0", listen_port)).await.expect("bind");
    eprintln!("listening for dial-back on 0.0.0.0:{listen_port}");

    // Control connection: register as the debugger, arm stop conditions, and
    // ask the console to dial back on our listen port for notifications.
    let t = connect(&host).await.expect("connect");
    let mut c = Client::new(t).read_banner().await.expect("banner");
    eprintln!(
        "debugger connect: {:?}",
        c.run(Debugger { do_override: true, name: "xbdbg".into(), user: "xbdbg".into() }).await
    );
    for s in ["stopon fce", "stopon titleinit", "stopon titleexit", "stopon debugstr"] {
        eprintln!("{s}: {:?}", c.send_raw(s).await);
    }
    eprintln!(
        "notify reconnectport: {:?}",
        c.send_raw(&format!("notify reconnectport={listen_port}")).await
    );

    // Accept dial-back connections and print their notification lines.
    let host2 = host.clone();
    let printer = tokio::spawn(async move {
        loop {
            match tokio::time::timeout(Duration::from_secs(secs), listener.accept()).await {
                Ok(Ok((sock, peer))) => {
                    eprintln!("[{:>6}ms] dial-back from {peer}", start.elapsed().as_millis());
                    let host3 = host2.clone();
                    tokio::spawn(async move {
                        let mut lines = BufReader::new(sock).lines();
                        while let Ok(Some(ln)) = lines.next_line().await {
                            let ms = start.elapsed().as_millis();
                            println!("[{ms:>6}ms] {ln}");
                            // A first-chance exception halts a thread; on such a
                            // stop, dump the faulting thread's register context so
                            // we can see the faulting PC (Iar) and exception cause.
                            if ln.contains("exception") || ln.contains("stop") {
                                let tid = ln
                                    .split_whitespace()
                                    .find_map(|w| w.strip_prefix("thread="))
                                    .map(|s| s.to_string());
                                if let (Some(tid), Ok(t)) = (tid, connect(&host3).await) {
                                    if let Ok(mut cc) = Client::new(t).read_banner().await {
                                        let ctx = cc
                                            .send_raw(&format!("getcontext thread={tid} full"))
                                            .await;
                                        eprintln!("  getcontext {tid}: {ctx:?}");
                                    }
                                }
                            }
                        }
                        eprintln!("[{:>6}ms] dial-back closed", start.elapsed().as_millis());
                    });
                }
                _ => break,
            }
        }
    });

    // Control connection: issue the magicboot.
    let t = connect(&host).await.expect("connect ctrl");
    let mut ctrl = Client::new(t).read_banner().await.expect("banner ctrl");
    // The working launch on this console is `title name=...` then plain reboot;
    // `reboot title=` is ignored here.
    eprintln!("title: {:?}", ctrl.send_raw(&format!("title name=\"{title}\"")).await);
    let _ = directory;
    eprintln!("reboot: {:?}", ctrl.send_raw("reboot").await);

    let _ = tokio::time::timeout(Duration::from_secs(secs + 2), printer).await;
    eprintln!("done.");
}
