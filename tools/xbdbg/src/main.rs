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
use tokio::net::TcpStream;
use xeedee::Client;
use xeedee::commands::debug::Debugger;
use xeedee::transport::tokio::connect;

/// Print every notification line from one dial-back connection. On a stop or
/// exception, dump the reported thread's register context.
async fn print_notifications(socket: TcpStream, host: String, start: Instant) {
    let mut lines = BufReader::new(socket).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        let ms = start.elapsed().as_millis();
        println!("[{ms:>6}ms] {line}");

        // A first-chance exception halts a thread; on such a stop, dump the
        // faulting thread's register context so we can see the faulting PC
        // (Iar) and exception cause.
        if !line.contains("exception") && !line.contains("stop") {
            continue;
        }

        let thread = line
            .split_whitespace()
            .find_map(|word| word.strip_prefix("thread="))
            .map(|thread| thread.to_string());
        let transport = connect(&host).await;

        let Some(thread) = thread else {
            continue;
        };
        let Ok(transport) = transport else {
            continue;
        };
        let Ok(mut client) = Client::new(transport).read_banner().await else {
            continue;
        };

        let context = client
            .send_raw(&format!("getcontext thread={thread} full"))
            .await;
        eprintln!("  getcontext {thread}: {context:?}");
    }

    eprintln!("[{:>6}ms] dial-back closed", start.elapsed().as_millis());
}

/// Debugger registration used by every control connection.
fn debugger() -> Debugger {
    Debugger {
        do_override: true,
        name: "xbdbg".into(),
        user: "xbdbg".into(),
    }
}

/// Register as the debugger and clear any stop so the title free-runs.
async fn release_title(host: &str) {
    let Ok(transport) = connect(host).await else {
        return;
    };
    let Ok(mut client) = Client::new(transport).read_banner().await else {
        return;
    };

    let _ = client.run(debugger()).await;
    let _ = client.send_raw("stop off").await;
    let _ = client.send_raw("nostopon").await;
    let _ = client.send_raw("go").await;
}

#[tokio::main]
async fn main() {
    // Missing or unparsable optional arguments take the defaults for the
    // development console.
    let mut args = std::env::args().skip(1);
    let host = args.next().unwrap_or_else(|| "192.168.1.26:730".into());
    let listen_port: u16 = args
        .next()
        .and_then(|port| port.parse().ok())
        .unwrap_or(52730);
    let title = args.next().expect("title path");
    let directory = args.next().unwrap_or_default();
    let secs: u64 = args.next().and_then(|secs| secs.parse().ok()).unwrap_or(45);

    let start = Instant::now();

    // Listen for the console's dial-back notification connections.
    let listener = TcpListener::bind(("0.0.0.0", listen_port))
        .await
        .expect("bind");
    eprintln!("listening for dial-back on 0.0.0.0:{listen_port}");

    // Control connection: register as the debugger and ask the console to
    // dial back on our listen port for notifications.
    let transport = connect(&host).await.expect("connect");
    let mut client = Client::new(transport).read_banner().await.expect("banner");
    eprintln!("debugger connect: {:?}", client.run(debugger()).await);
    // Do NOT arm stop conditions: `stopon debugstr` etc. halt the title on every
    // notification (and reading memory keeps it halted), which looks like a hang.
    // The notify channel below streams debugstr while the title free-runs.
    eprintln!("nostopon: {:?}", client.send_raw("nostopon").await);
    eprintln!(
        "notify reconnectport: {:?}",
        client
            .send_raw(&format!("notify reconnectport={listen_port}"))
            .await
    );

    // Accept dial-back connections and print their notification lines.
    let printer_host = host.clone();
    let printer = tokio::spawn(async move {
        loop {
            let Ok(Ok((socket, peer))) =
                tokio::time::timeout(Duration::from_secs(secs), listener.accept()).await
            else {
                break;
            };

            eprintln!(
                "[{:>6}ms] dial-back from {peer}",
                start.elapsed().as_millis()
            );
            tokio::spawn(print_notifications(socket, printer_host.clone(), start));
        }
    });

    // Control connection: issue the magicboot.
    let transport = connect(&host).await.expect("connect ctrl");
    let mut ctrl = Client::new(transport)
        .read_banner()
        .await
        .expect("banner ctrl");
    // The working launch on this console is `title name=...` then plain reboot;
    // `reboot title=` is ignored here.
    // `title name=` persists the default title; the plain reboot then boots it.
    let _ = directory;
    eprintln!(
        "title: {:?}",
        ctrl.send_raw(&format!("title name=\"{title}\"")).await
    );
    eprintln!("reboot: {:?}", ctrl.send_raw("reboot").await);

    // A registered debugger makes the title break at its entry point; it stays
    // halted until told to `go`. Reboot drops our connections, so reconnect a
    // few times after the console comes back and clear any stop so the title
    // free-runs (and its DbgPrint output flows over the notify channel).
    let release_host = host.clone();
    tokio::spawn(async move {
        for _ in 0..12 {
            tokio::time::sleep(Duration::from_secs(2)).await;
            release_title(&release_host).await;
        }
    });

    let _ = tokio::time::timeout(Duration::from_secs(secs + 2), printer).await;
    eprintln!("done.");
}
