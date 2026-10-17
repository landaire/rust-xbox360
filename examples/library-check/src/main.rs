//! Exercises each module of the `xbox360` crate and reports one line per
//! check to the debug output: `PASS name`, `FAIL name: detail` or
//! `SKIP name: reason`, then `DONE`. The title then ends through the
//! loader, because returning from the entry point stops the console.
//!
//! Reports do not allocate, so the checks run under Xenia, which lacks
//! `ExAllocatePoolWithTag`. Xenia also lacks `mcrf`, which core's u8, u16
//! and u64 formatting contain, so the title formats only 32-bit integers.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::cell::Cell;
use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;
use core::time::Duration;

use xbox360::atomic::AtomicU64;
use xbox360::console;
use xbox360::dbg_println;
use xbox360::module;
use xbox360::pool::PoolAllocator;
use xbox360::pool::PoolTag;
use xbox360::status::Hresult;
use xbox360::status::KernelError;
use xbox360::status::NtStatus;
use xbox360::status::Win32Error;
use xbox360::sync;
use xbox360::sync::CriticalSection;
use xbox360::sync::Event;
use xbox360::sync::EventType;
use xbox360::sync::InterruptGuard;
use xbox360::sync::Irql;
use xbox360::sync::IrqlGuard;
use xbox360::sync::SpinLock;
use xbox360::sync::WaitOutcome;
use xbox360::thread;
use xbox360::time;
use xbox360::title;
use xbox360::xam;
use xbox360::xam::UserIndex;

#[global_allocator]
static ALLOCATOR: PoolAllocator = PoolAllocator::new(TAG);

const TAG: PoolTag = PoolTag::new(*b"LChk");

/// Failure text in a fixed buffer, cut at its capacity.
struct Detail {
    bytes: [u8; 160],
    length: usize,
}

impl Detail {
    fn new(arguments: fmt::Arguments<'_>) -> Self {
        let mut detail = Self {
            bytes: [0; 160],
            length: 0,
        };
        // A full buffer ends the text; the start is enough to diagnose.
        let _ = detail.write_fmt(arguments);
        detail
    }

    fn as_str(&self) -> &str {
        match core::str::from_utf8(&self.bytes[..self.length]) {
            Ok(text) => text,
            // `write_str` stores whole characters only.
            Err(_) => unreachable!("the buffer holds whole characters"),
        }
    }
}

impl Write for Detail {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let room = self.bytes.len() - self.length;
        let mut count = text.len().min(room);
        while !text.is_char_boundary(count) {
            count -= 1;
        }
        self.bytes[self.length..self.length + count].copy_from_slice(&text.as_bytes()[..count]);
        self.length += count;
        if count < text.len() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

enum Outcome {
    Pass,
    Fail(Detail),
    Skip(&'static str),
}

/// Writes `count` copies of `byte`.
struct Repeat {
    byte: char,
    count: u32,
}

impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..self.count).try_for_each(|_| f.write_char(self.byte))
    }
}

/// A u64 as two 32-bit hexadecimal halves.
struct Hex64(u64);

impl fmt::Display for Hex64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x}_{:08x}", (self.0 >> 32) as u32, self.0 as u32)
    }
}

/// A kernel version with each part formatted as u32.
struct Version(console::KernelVersion);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = self.0;
        write!(
            f,
            "{}.{}.{}.{}",
            u32::from(version.major),
            u32::from(version.minor),
            u32::from(version.build),
            u32::from(version.qfe)
        )
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dbg_println!("FAIL panic: {info}");
    title::terminate()
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

/// Xenia does not provide `XboxKrnlBaseVersion`.
fn running_under_xenia() -> bool {
    console::base_kernel_version().is_none()
}

fn unless_xenia(xenia: bool, reason: &'static str, check: fn() -> Outcome) -> Outcome {
    if xenia {
        Outcome::Skip(reason)
    } else {
        check()
    }
}

fn check(name: &str, outcome: Outcome) {
    match outcome {
        Outcome::Pass => dbg_println!("PASS {name}"),
        Outcome::Fail(detail) => dbg_println!("FAIL {name}: {}", detail.as_str()),
        Outcome::Skip(reason) => dbg_println!("SKIP {name}: {reason}"),
    }
}

/// Return `Outcome::Fail` with the formatted text.
macro_rules! fail {
    ($($message:tt)*) => {
        return Outcome::Fail(Detail::new(format_args!($($message)*)))
    };
}

/// Fail with the formatted text unless `condition` holds.
macro_rules! ensure {
    ($condition:expr, $($message:tt)*) => {
        if !$condition {
            fail!($($message)*);
        }
    };
}

fn check_status() -> Outcome {
    ensure!(
        KernelError::from_status(NtStatus(0xc000_0034)) == Some(KernelError::ObjectNameNotFound),
        "0xc0000034 is not ObjectNameNotFound"
    );

    let Some(unlisted) = KernelError::from_status(NtStatus(0xc000_7777)) else {
        fail!("0xc0007777 is not a failure");
    };
    ensure!(
        unlisted.status() == NtStatus(0xc000_7777),
        "unlisted status changed: {unlisted:?}"
    );

    ensure!(
        Hresult::from(KernelError::NoMemory) == Hresult(0xd000_0017),
        "NoMemory result is {:?}",
        Hresult::from(KernelError::NoMemory)
    );
    ensure!(
        Hresult::from(Win32Error::INVALID_PARAMETER) == Hresult::INVALID_ARGUMENT,
        "INVALID_PARAMETER result is {:?}",
        Hresult::from(Win32Error::INVALID_PARAMETER)
    );

    let saved = xbox360::status::last_error();
    xbox360::status::set_last_error(KernelError::NoSuchFile);
    let translated = xbox360::status::last_error();
    xbox360::status::set_last_error_value(saved);
    let restored = xbox360::status::last_error();

    ensure!(
        translated.map(Win32Error::code) == Some(2),
        "STATUS_NO_SUCH_FILE became last error {translated:?}, expected 2"
    );
    ensure!(
        restored == saved,
        "last error {restored:?} after restoring {saved:?}"
    );

    Outcome::Pass
}

static SIGNAL: Event = Event::new(EventType::Notification, false);

extern "C" fn set_signal_later(_argument: u32) -> u32 {
    thread::sleep(Duration::from_millis(20));
    SIGNAL.set();
    0
}

xbox360::thread_entry!(SET_SIGNAL_LATER = set_signal_later);

fn check_event() -> Outcome {
    let event = Event::new(EventType::Notification, false);
    let outcome = event.wait_timeout(Duration::from_millis(10));
    ensure!(
        outcome == Ok(WaitOutcome::TimedOut),
        "unsignaled wait gave {outcome:?}"
    );

    event.set();
    let outcome = event.wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "signaled wait gave {outcome:?}"
    );

    let outcome = event.wait();
    ensure!(
        outcome == Ok(()),
        "notification event did not stay signaled: {outcome:?}"
    );

    event.reset();
    let outcome = event.wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::TimedOut),
        "reset event gave {outcome:?}"
    );

    let synchronization = Event::new(EventType::Synchronization, false);
    synchronization.set();
    let outcome = synchronization.wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "set synchronization event gave {outcome:?}"
    );

    let outcome = synchronization.wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::TimedOut),
        "synchronization event did not reset: {outcome:?}"
    );

    let setter = match thread::spawn_entry(SET_SIGNAL_LATER, 0) {
        Ok(setter) => setter,
        Err(error) => fail!("spawn failed: {error}"),
    };

    let outcome = SIGNAL.wait_timeout(Duration::from_secs(2));
    let joined = setter.join();
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "event set by another thread gave {outcome:?}"
    );
    ensure!(joined.is_ok(), "join failed: {joined:?}");

    Outcome::Pass
}

/// A moved event is set up again at its new address and keeps its state.
fn check_event_move() -> Outcome {
    let event = Event::new(EventType::Synchronization, false);
    event.set();
    let first = event.as_ptr() as usize;
    let moved = core::hint::black_box([event]);
    let second = moved[0].as_ptr() as usize;
    ensure!(first != second, "the event stayed at {first:#010x}");

    let outcome = moved[0].wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "moved event lost its state: {outcome:?}"
    );

    moved[0].set();
    let outcome = moved[0].wait_timeout(Duration::ZERO);
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "moved event was not set again: {outcome:?}"
    );

    Outcome::Pass
}

/// Start two threads at `entry` and wait for both. Reports the first
/// failure.
fn run_pair(entry: thread::ThreadEntry) -> Outcome {
    let workers = [thread::spawn_entry(entry, 0), thread::spawn_entry(entry, 1)];
    let mut outcome = Outcome::Pass;
    for worker in workers {
        let failure = match worker {
            Ok(worker) => match worker.join() {
                Ok(()) => None,
                Err(error) => Some(Detail::new(format_args!("join failed: {error}"))),
            },
            Err(error) => Some(Detail::new(format_args!("spawn failed: {error}"))),
        };
        if let (Outcome::Pass, Some(detail)) = (&outcome, failure) {
            outcome = Outcome::Fail(detail);
        }
    }
    outcome
}

static SECTION: CriticalSection<Cell<u32>> = CriticalSection::new(Cell::new(0));
const SECTION_ROUNDS: u32 = 2000;

extern "C" fn section_worker(_argument: u32) -> u32 {
    for _ in 0..SECTION_ROUNDS {
        let counter = SECTION.lock();
        counter.set(counter.get() + 1);
    }
    0
}

xbox360::thread_entry!(SECTION_WORKER = section_worker);

/// Set when the probe thread entered the section, which it must not.
static PROBE_ENTERED: AtomicBool = AtomicBool::new(false);
/// Set when the probe thread finished its attempt.
static PROBE_DONE: AtomicBool = AtomicBool::new(false);

extern "C" fn section_try_lock(_argument: u32) -> u32 {
    let acquired = SECTION.try_lock().is_some();
    PROBE_ENTERED.store(acquired, Ordering::Release);
    PROBE_DONE.store(true, Ordering::Release);
    0
}

xbox360::thread_entry!(SECTION_TRY_LOCK = section_try_lock);

fn check_critical_section() -> Outcome {
    {
        let _outer = SECTION.lock();
        {
            let inner = SECTION.lock();
            inner.set(0);
        }

        let probe = match thread::spawn_entry(SECTION_TRY_LOCK, 0) {
            Ok(probe) => probe,
            Err(error) => fail!("spawn failed: {error}"),
        };
        let joined = probe.join();
        ensure!(joined.is_ok(), "join failed: {joined:?}");
    }

    let done = PROBE_DONE.load(Ordering::Acquire);
    let entered = PROBE_ENTERED.load(Ordering::Acquire);

    ensure!(done, "the probe thread did not run");
    ensure!(!entered, "another thread entered a held section");

    if let Outcome::Fail(detail) = run_pair(SECTION_WORKER) {
        return Outcome::Fail(detail);
    }

    let total = SECTION.lock().get();
    ensure!(
        total == 2 * SECTION_ROUNDS,
        "counter is {total}, expected {}",
        2 * SECTION_ROUNDS
    );

    Outcome::Pass
}

static SPIN: SpinLock<u32> = SpinLock::new(0);
const SPIN_ROUNDS: u32 = 20_000;

extern "C" fn spin_worker(_argument: u32) -> u32 {
    for _ in 0..SPIN_ROUNDS {
        *SPIN.lock() += 1;
    }
    0
}

xbox360::thread_entry!(SPIN_WORKER = spin_worker);

fn check_spin_lock() -> Outcome {
    if let Outcome::Fail(detail) = run_pair(SPIN_WORKER) {
        return Outcome::Fail(detail);
    }

    let total = *SPIN.lock();
    ensure!(
        total == 2 * SPIN_ROUNDS,
        "counter is {total}, expected {}",
        2 * SPIN_ROUNDS
    );

    Outcome::Pass
}

fn check_spin_lock_irql() -> Outcome {
    let held = {
        let _guard = SPIN.lock();
        sync::current_irql()
    };
    let after = sync::current_irql();

    ensure!(
        held == Irql::DISPATCH,
        "IRQL while holding the lock is {}",
        u32::from(held.level())
    );
    ensure!(
        after == Irql::PASSIVE,
        "IRQL after the lock is {}",
        u32::from(after.level())
    );

    Outcome::Pass
}

fn check_irql() -> Outcome {
    let before = sync::current_irql();
    ensure!(
        before == Irql::PASSIVE,
        "title runs at IRQL {}",
        u32::from(before.level())
    );

    {
        let guard = IrqlGuard::raise(Irql::DISPATCH);
        let raised = sync::current_irql();
        let lower = IrqlGuard::raise(Irql::APC).is_none();

        ensure!(guard.is_some(), "raising to DISPATCH was refused");
        ensure!(
            raised == Irql::DISPATCH,
            "IRQL after raise is {}",
            u32::from(raised.level())
        );
        ensure!(lower, "lowering through raise was accepted");
    }

    let after = sync::current_irql();
    ensure!(
        after == Irql::PASSIVE,
        "IRQL after the guard is {}",
        u32::from(after.level())
    );

    Outcome::Pass
}

fn machine_state() -> u32 {
    let value: u32;
    // SAFETY: reads the machine state register.
    unsafe { core::arch::asm!("mfmsr {0}", out(reg) value) };
    value
}

fn check_interrupts() -> Outcome {
    let before = machine_state();
    let inside = {
        // SAFETY: nothing in this block waits or faults.
        let _guard = unsafe { InterruptGuard::new() };
        machine_state()
    };
    let after = machine_state();

    ensure!(
        before & 0x8000 != 0,
        "interrupts were disabled at the start (MSR {before:#010x})"
    );
    ensure!(
        inside & 0x8000 == 0,
        "interrupts stayed enabled under the guard (MSR {inside:#010x})"
    );
    ensure!(
        after & 0x8000 != 0,
        "interrupts stayed disabled after the guard (MSR {after:#010x})"
    );

    Outcome::Pass
}

static WIDE: AtomicU64 = AtomicU64::new(0);
const ATOMIC_ROUNDS: u64 = 10_000;
/// Changes both halves, so a torn update shows in the total.
const ATOMIC_STEP: u64 = 0x0000_0001_0000_0001;

extern "C" fn atomic_worker(_argument: u32) -> u32 {
    for _ in 0..ATOMIC_ROUNDS {
        let mut current = WIDE.load();
        while let Err(observed) = WIDE.compare_exchange(current, current.wrapping_add(ATOMIC_STEP))
        {
            current = observed;
        }
    }
    0
}

xbox360::thread_entry!(ATOMIC_WORKER = atomic_worker);

fn check_atomic() -> Outcome {
    let local = AtomicU64::new(0xffff_ffff);
    ensure!(
        local.fetch_add(1) == 0xffff_ffff,
        "fetch_add returned the wrong value"
    );
    ensure!(
        local.load() == 0x1_0000_0000,
        "carry into the high word lost: {}",
        Hex64(local.load())
    );
    ensure!(
        local.swap(7) == 0x1_0000_0000,
        "swap returned the wrong value"
    );
    ensure!(
        local.compare_exchange(8, 9) == Err(7),
        "compare_exchange succeeded on a mismatch"
    );
    ensure!(
        local.compare_exchange(7, 9) == Ok(7),
        "compare_exchange failed on a match"
    );

    local.store(u64::MAX);
    ensure!(
        local.load() == u64::MAX,
        "store then load gave {}",
        Hex64(local.load())
    );

    if let Outcome::Fail(detail) = run_pair(ATOMIC_WORKER) {
        return Outcome::Fail(detail);
    }

    let expected = 2 * ATOMIC_ROUNDS * ATOMIC_STEP;
    let total = WIDE.load();
    ensure!(
        total == expected,
        "total is {}, expected {}",
        Hex64(total),
        Hex64(expected)
    );

    Outcome::Pass
}

fn check_module() -> Outcome {
    let kernel = match module::SystemModule::Kernel.handle() {
        Ok(kernel) => kernel,
        Err(error) => fail!("handle of xboxkrnl.exe: {error}"),
    };

    let Some(ordinal) = module::Ordinal::new(0xad) else {
        fail!("ordinal 0xad rejected");
    };
    let bundle = match module::resolve_export(kernel, ordinal) {
        Ok(bundle) => bundle,
        Err(error) => fail!("resolve_export(KeTimeStampBundle): {error}"),
    };

    let Some(imported) = xbox360::sys::xboxkrnl::KeTimeStampBundle.get() else {
        fail!("KeTimeStampBundle is not resolved");
    };
    let imported = core::ptr::from_ref(imported) as u32;
    ensure!(
        bundle.raw() == imported,
        "KeTimeStampBundle resolves to {:#010x}, the import is {imported:#010x}",
        bundle.raw()
    );

    let Some(debug_print) = module::Ordinal::new(0x003) else {
        fail!("ordinal 3 rejected");
    };
    let resolved = module::resolve_export(kernel, debug_print);
    ensure!(
        matches!(resolved, Ok(address) if address.raw() != 0),
        "resolve_export(DbgPrint): {resolved:?}"
    );

    dbg_println!(
        "INFO module: xboxkrnl.exe {:#010x}, DbgPrint {:x?}",
        kernel.raw(),
        resolved
    );

    Outcome::Pass
}

static THREAD_RESULT: AtomicU32 = AtomicU32::new(0);
static THREAD_OBJECT: AtomicU32 = AtomicU32::new(0);

extern "C" fn thread_worker(argument: u32) -> u32 {
    THREAD_OBJECT.store(thread::current_object().raw(), Ordering::Relaxed);
    THREAD_RESULT.store(argument + 1, Ordering::Release);
    argument
}

xbox360::thread_entry!(THREAD_WORKER = thread_worker);

fn check_thread() -> Outcome {
    let worker = match thread::spawn_entry(THREAD_WORKER, 41) {
        Ok(worker) => worker,
        Err(error) => fail!("spawn_entry: {error}"),
    };

    let outcome = worker.wait_timeout(Duration::from_secs(2));
    ensure!(
        outcome == Ok(WaitOutcome::Signaled),
        "wait gave {outcome:?}"
    );

    let joined = worker.join();
    ensure!(joined.is_ok(), "join failed: {joined:?}");

    let result = THREAD_RESULT.load(Ordering::Acquire);
    ensure!(result == 42, "entry thread stored {result}, expected 42");

    let object = THREAD_OBJECT.load(Ordering::Relaxed);
    let own = thread::current_object().raw();
    ensure!(
        object != 0 && object != own,
        "thread object {object:#x} (own {own:#x})"
    );

    Outcome::Pass
}

fn check_thread_closure() -> Outcome {
    let shared = alloc::sync::Arc::new((
        Event::new(EventType::Notification, false),
        AtomicU32::new(0),
    ));
    let remote = alloc::sync::Arc::clone(&shared);
    let factor = 6u32;

    let Some(stack) = core::num::NonZeroU32::new(64 * 1024) else {
        fail!("stack size rejected");
    };

    let closure = thread::Builder::new().stack_size(stack).spawn(move || {
        remote.1.store(7 * factor, Ordering::Release);
        remote.0.set();
    });
    let closure = match closure {
        Ok(closure) => closure,
        Err(error) => fail!("spawn: {error}"),
    };

    let signaled = shared.0.wait_timeout(Duration::from_secs(2));
    let joined = closure.join();
    ensure!(
        signaled == Ok(WaitOutcome::Signaled),
        "closure thread did not signal: {signaled:?}"
    );
    ensure!(joined.is_ok(), "join failed: {joined:?}");

    let stored = shared.1.load(Ordering::Acquire);
    ensure!(stored == 42, "closure stored {stored}, expected 42");

    Outcome::Pass
}

fn check_debug() -> Outcome {
    dbg_println!(
        "INFO debug: {} {:#06x} {:?} {:>5}|",
        42,
        0x2a,
        "text",
        "pad"
    );

    // Longer than one DbgPrint piece.
    let long = Repeat {
        byte: 'x',
        count: 300,
    };
    let mut output = xbox360::debug::DebugOutput::new();
    let written = writeln!(output, "INFO debug long: {long}");
    drop(output);
    ensure!(written.is_ok(), "formatting failed");

    Outcome::Pass
}

fn check_time() -> Outcome {
    let frequency = time::TimebaseFrequency::query();
    let tick_before = time::TickCount::now();
    let system_before = time::FileTime::now();

    let mut previous = time::Timebase::now();
    for _ in 0..1000 {
        let now = time::Timebase::now();
        ensure!(
            now >= previous,
            "time base went back from {} to {}",
            Hex64(previous.ticks()),
            Hex64(now.ticks())
        );
        previous = now;
    }

    let requested = Duration::from_millis(50);
    let sleep_start = time::Timebase::now();
    time::sleep(requested);
    let slept = time::Timebase::now().duration_since(sleep_start, frequency);
    let system_elapsed = time::FileTime::now().duration_since(system_before);
    let ticks = time::TickCount::now().wrapping_duration_since(tick_before);

    dbg_println!(
        "INFO time: frequency {} Hz, slept {} us, system time advanced {:?} us, tick count advanced {} us",
        frequency.hertz(),
        micros(slept),
        system_elapsed.map(micros),
        micros(ticks)
    );

    // The time base and the sleep come from different clocks; allow 1 ms
    // for rounding between them.
    ensure!(
        slept + Duration::from_millis(1) >= requested,
        "slept {} us for 50000 us",
        micros(slept)
    );
    ensure!(
        slept < Duration::from_millis(500),
        "slept {} us for 50000 us",
        micros(slept)
    );
    ensure!(
        matches!(system_elapsed, Some(elapsed) if elapsed >= Duration::from_millis(30)),
        "system time advanced {:?} us",
        system_elapsed.map(micros)
    );
    ensure!(
        ticks >= Duration::from_millis(30),
        "tick count advanced {} us",
        micros(ticks)
    );

    Outcome::Pass
}

fn check_pool() -> Outcome {
    let mut lowest_alignment = u32::MAX;
    for size in [1u32, 7, 24, 100, 1000, 5000] {
        // SAFETY: the call takes no pointers.
        let block = unsafe { xbox360::sys::xboxkrnl::ExAllocatePoolWithTag(size, TAG) };
        ensure!(block != 0, "pool allocation of {size} bytes failed");
        lowest_alignment = lowest_alignment.min(1 << block.trailing_zeros());
        // SAFETY: `block` is the nonzero pool address allocated above,
        // freed once.
        unsafe { xbox360::sys::xboxkrnl::ExFreePool(block) };
    }

    dbg_println!("INFO pool: blocks aligned to at least {lowest_alignment} bytes");
    ensure!(
        lowest_alignment as usize >= xbox360::pool::POOL_ALIGNMENT,
        "pool block aligned to {lowest_alignment}, below {}",
        xbox360::pool::POOL_ALIGNMENT
    );

    for align in [4usize, 8, 16, 32, 64] {
        for size in [1usize, 13, 256, 4096] {
            let Ok(layout) = Layout::from_size_align(size, align) else {
                fail!("layout {size}/{align} rejected");
            };
            // SAFETY: the layout has a nonzero size.
            let block = unsafe { ALLOCATOR.alloc(layout) };
            ensure!(
                !block.is_null(),
                "allocation of {size} bytes aligned to {align} failed"
            );
            ensure!(
                (block as usize).is_multiple_of(align),
                "block {block:p} is not aligned to {align}"
            );

            // SAFETY: the block holds `size` bytes.
            let intact = unsafe {
                core::ptr::write_bytes(block, 0xa5, size);
                core::slice::from_raw_parts(block, size)
                    .iter()
                    .all(|&byte| byte == 0xa5)
            };
            // SAFETY: allocated above with the same layout.
            unsafe { ALLOCATOR.dealloc(block, layout) };

            ensure!(
                intact,
                "block of {size} bytes aligned to {align} did not keep its contents"
            );
        }
    }

    let values: Vec<u32> = (0..1000).collect();
    ensure!(
        values.iter().sum::<u32>() == 499_500,
        "vector contents changed"
    );

    Outcome::Pass
}

fn check_console() -> Outcome {
    let version = console::kernel_version();
    let flags = console::hardware_flags();
    dbg_println!(
        "INFO console: kernel {}, hardware flags {:#010x}, type {:?}, generation {}, hard disk {}",
        Version(version),
        flags.bits(),
        console::ConsoleType::from_hardware_flags(flags),
        u32::from(flags.generation().get()),
        flags.contains(console::HardwareFlags::HARD_DISK)
    );

    ensure!(
        version.major == 2,
        "kernel major version is {}",
        u32::from(version.major)
    );

    Outcome::Pass
}

fn check_base_version() -> Outcome {
    let Some(base) = console::base_kernel_version() else {
        return Outcome::Skip("XboxKrnlBaseVersion is not provided");
    };

    let running = console::kernel_version();
    dbg_println!("INFO console: base kernel {}", Version(base));

    ensure!(
        base.major == 2,
        "base kernel major version is {}",
        u32::from(base.major)
    );
    ensure!(
        base.build <= running.build,
        "base kernel {} is newer than kernel {}",
        Version(base),
        Version(running)
    );

    Outcome::Pass
}

fn check_input() -> Outcome {
    let user = UserIndex::User0;
    let state = xam::input::state(user);
    let capabilities = xam::input::capabilities(user);
    match (state, capabilities) {
        (Ok(None), Ok(None)) => {
            dbg_println!("INFO input: no controller on port 0");
            Outcome::Pass
        }
        (Ok(Some(state)), Ok(Some(capabilities))) => {
            dbg_println!(
                "INFO input: packet {}, buttons {:#06x}, device type {}, capability flags {:#06x}",
                state.packet_number.get(),
                u32::from(state.gamepad.buttons.bits()),
                u32::from(capabilities.device_type.raw()),
                u32::from(capabilities.flags.bits())
            );

            let vibration = xam::input::set_vibration(user, xam::input::Vibration::default());
            ensure!(vibration.is_ok(), "set_vibration: {vibration:?}");

            Outcome::Pass
        }
        (state, capabilities) => fail!(
            "state connected {:?}, capabilities connected {:?}",
            state.map(|state| state.is_some()),
            capabilities.map(|capabilities| capabilities.is_some())
        ),
    }
}

fn check_content() -> Outcome {
    use xam::content;

    let user = UserIndex::User0;
    let signin = xam::user::signin_state(user);
    if signin.kind() == Some(xam::user::SigninKind::NotSignedIn) {
        return Outcome::Skip("saved games need a signed-in user in slot 0");
    }

    // Device 1 is the hard disk under Xenia.
    let data = match content::ContentData::new(
        content::ContentDevice::new(1),
        content::ContentType::SavedGame,
        "library-check",
        b"librarycheck",
    ) {
        Ok(data) => data,
        Err(error) => fail!("content data: {error}"),
    };

    let root = match content::create(
        user,
        c"lcheck",
        &data,
        content::CreationDisposition::CreateAlways,
    ) {
        Ok(root) => root,
        Err(error) => fail!("create ({signin:?}): {error}"),
    };

    dbg_println!(
        "INFO content: {:?} as {:?}",
        root.disposition(),
        root.name()
    );

    let closed = root.close();
    ensure!(closed.is_ok(), "close: {closed:?}");

    let deleted = content::delete(user, &data);
    ensure!(deleted.is_ok(), "delete: {deleted:?}");

    Outcome::Pass
}

fn micros(duration: Duration) -> u32 {
    (duration.as_secs() as u32)
        .saturating_mul(1_000_000)
        .saturating_add(duration.subsec_micros())
}

extern "C" fn main(_context: u32) -> u32 {
    const NO_POOL: &str = "ExAllocatePoolWithTag is not provided";
    const NO_PCR_IRQL: &str = "Xenia does not keep the IRQL in the processor control region";

    let xenia = running_under_xenia();
    if xenia {
        dbg_println!("INFO running under Xenia");
    }

    check("status", check_status());
    check("event", check_event());
    check(
        "event_move",
        unless_xenia(
            xenia,
            "Xenia keeps event state outside guest memory",
            check_event_move,
        ),
    );
    check("critical_section", check_critical_section());
    check("spin_lock", check_spin_lock());
    check(
        "spin_lock_irql",
        unless_xenia(xenia, NO_PCR_IRQL, check_spin_lock_irql),
    );
    check(
        "irql",
        unless_xenia(xenia, "KfRaiseIrql is not provided", check_irql),
    );
    check("interrupts", check_interrupts());
    check("atomic_u64", check_atomic());
    check("module", check_module());
    check("thread", check_thread());
    check(
        "thread_closure",
        unless_xenia(xenia, NO_POOL, check_thread_closure),
    );
    check("debug", check_debug());
    check("time", check_time());
    check("pool", unless_xenia(xenia, NO_POOL, check_pool));
    check("console", check_console());
    check("console_base_version", check_base_version());
    check("input", check_input());
    check("content", check_content());
    check(
        "message_box",
        Outcome::Skip("XAM has no call that cancels a message box"),
    );

    dbg_println!("DONE");
    title::terminate()
}
