//! Exercises 64-bit atomics from several threads and prints the results:
//! loads and stores, swap, compare-exchange, fetch_add/sub/and/or/xor and
//! fetch_max/min, against values computed without atomics.

#![no_main]

use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

const THREADS: u64 = 4;
const ADDS_PER_THREAD: u64 = 100_000;
/// Crosses the 32-bit halves on every add.
const STEP: u64 = 0x0000_0001_0000_0001;

fn main_body() {
    let value = AtomicU64::new(0x1122_3344_5566_7788);
    let mut ok = value.load(SeqCst) == 0x1122_3344_5566_7788;
    value.store(0xffff_ffff_0000_0000, SeqCst);
    ok &= value.swap(1, SeqCst) == 0xffff_ffff_0000_0000;
    ok &= value.compare_exchange(1, 0x8000_0000_0000_0001, SeqCst, SeqCst) == Ok(1);
    ok &= value.compare_exchange(1, 2, SeqCst, SeqCst) == Err(0x8000_0000_0000_0001);
    ok &= value.fetch_and(0xf000_0000_0000_000f, SeqCst) == 0x8000_0000_0000_0001;
    ok &= value.fetch_or(0x0000_0001_0000_0000, SeqCst) == 0x8000_0000_0000_0001;
    ok &= value.fetch_xor(u64::MAX, SeqCst) == 0x8000_0001_0000_0001;
    ok &= value.load(SeqCst) == 0x7fff_fffe_ffff_fffe;
    ok &= value.fetch_max(u64::MAX, SeqCst) == 0x7fff_fffe_ffff_fffe && value.load(SeqCst) == u64::MAX;
    let signed = AtomicI64::new(-5);
    ok &= signed.fetch_min(-9, SeqCst) == -5 && signed.load(SeqCst) == -9;
    ok &= signed.fetch_max(3, SeqCst) == -9 && signed.load(SeqCst) == 3;
    println!("atomics64: single-thread checks {}", if ok { "pass" } else { "FAIL" });

    let counter = Arc::new(AtomicU64::new(0));
    let threads: Vec<_> = (0..THREADS)
        .map(|_| {
            let counter = Arc::clone(&counter);
            std::thread::spawn(move || {
                for _ in 0..ADDS_PER_THREAD {
                    counter.fetch_add(STEP, SeqCst);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().expect("thread");
    }
    let expected = THREADS * ADDS_PER_THREAD * STEP;
    let total = counter.load(SeqCst);
    println!("atomics64: {} threads x {} adds: {:#x}, expected {:#x}: {}", THREADS, ADDS_PER_THREAD, total, expected, if total == expected { "pass" } else { "FAIL" });
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

extern "C" fn main(_context: u32) -> u32 {
    main_body();
    xbox360::title::terminate()
}
