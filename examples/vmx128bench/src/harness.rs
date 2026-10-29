//! Timing harness: calls a function through the `vb_call` trampoline, which
//! reads the timebase around the calls and preserves every register the
//! Rust code relies on, so the timed function may use any register except
//! r1, r2 and r13.
//!
//! [`time_call`] times one batch of calls; [`measure`] repeats it and
//! returns the median, minimum and maximum. [`Buffer`] is a 128-byte aligned
//! buffer in 64 KB pages. These are the entry points for timing
//! compiler-generated kernels: pass the function pointer, the integer
//! arguments (r3-r8) and the float arguments (f1, f2).

use core::ptr::NonNull;

use xbox360::sys::xboxkrnl;
use xbox360::sys::xboxkrnl::Kthread;
use xbox360::sys::xboxkrnl::MemoryRegion;
use xbox360::sys::xboxkrnl::PageProtection;

/// A function called through the trampoline. The real signature is set by
/// the arguments in [`CallCtx`].
pub type KernelFn = unsafe extern "C" fn();

/// Core clock cycles per timebase tick (3.2 GHz core, 50 MHz timebase). The
/// title prints the measured timebase frequency and scalar chain latencies
/// that confirm it.
pub const CYCLES_PER_TICK: f64 = 64.0;
pub const TB_HZ: f64 = 50_000_000.0;

/// The trampoline's argument block. Field offsets are fixed by the
/// generated `vb_call` (gen/gen_kernels.py).
#[repr(C, align(8))]
#[derive(Default)]
pub struct CallCtx {
    pub func: u32,
    pub reps: u32,
    /// r3..r8 of every call.
    pub args: [u32; 6],
    /// r3 after the last call.
    pub ret: u32,
    _pad: u32,
    /// f1, f2 of every call.
    pub fargs: [f64; 2],
    /// f1 after the last call.
    pub fret: f64,
    pub t0: u64,
    pub t1: u64,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(CallCtx, args) == 8);
    assert!(offset_of!(CallCtx, ret) == 32);
    assert!(offset_of!(CallCtx, fargs) == 40);
    assert!(offset_of!(CallCtx, fret) == 56);
    assert!(offset_of!(CallCtx, t0) == 64);
    assert!(offset_of!(CallCtx, t1) == 72);
};

unsafe extern "C" {
    fn vb_call(ctx: *mut CallCtx);
    fn vb_mftb(out: *mut u64);
}

/// The 64-bit timebase.
pub fn mftb() -> u64 {
    let mut value = 0u64;
    // SAFETY: stores one doubleword to `value`.
    unsafe { vb_mftb(&mut value) };
    value
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    pub ticks: u64,
    pub ret: u32,
    pub fret: f64,
}

/// Calls `f` `reps` times (at least once) and returns the timebase ticks of
/// the whole batch with the results of the last call.
///
/// # Safety
///
/// `f` must be safe to call with these arguments `reps` times in a row.
pub unsafe fn time_call(f: KernelFn, args: [u32; 6], fargs: [f64; 2], reps: u32) -> Sample {
    let mut ctx = CallCtx {
        func: f as usize as u32,
        reps: reps.max(1),
        args,
        fargs,
        ..Default::default()
    };
    // SAFETY: the caller guarantees the call; `ctx` is a live local.
    unsafe { vb_call(&mut ctx) };
    Sample {
        ticks: ctx.t1.wrapping_sub(ctx.t0),
        ret: ctx.ret,
        fret: ctx.fret,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub median: u64,
    pub min: u64,
    pub max: u64,
}

impl Stats {
    pub fn from(mut values: Vec<u64>) -> Self {
        values.sort_unstable();
        Self {
            median: values[values.len() / 2],
            min: values[0],
            max: values[values.len() - 1],
        }
    }

    pub fn scale(&self, by: f64) -> (f64, f64, f64) {
        (self.median as f64 * by, self.min as f64 * by, self.max as f64 * by)
    }
}

/// One warm-up batch, then `runs` timed batches of `reps` calls.
///
/// # Safety
///
/// As for [`time_call`].
pub unsafe fn measure(f: KernelFn, args: [u32; 6], fargs: [f64; 2], reps: u32, runs: usize) -> Stats {
    // SAFETY: forwarded from the caller.
    unsafe {
        time_call(f, args, fargs, reps);
        Stats::from((0..runs).map(|_| time_call(f, args, fargs, reps).ticks).collect())
    }
}

/// Finds a count for which `run(count)` takes at least `target` ticks,
/// doubling from 1 and then scaling.
pub fn calibrate(target: u64, max: u32, mut run: impl FnMut(u32) -> u64) -> u32 {
    let mut n = 1u32;
    loop {
        let t = run(n);
        if t >= target / 4 || n >= max {
            let scaled = (n as u64 * target / t.max(1)).clamp(1, max as u64);
            return scaled as u32;
        }
        n = n.saturating_mul(2).min(max);
    }
}

/// 128-byte aligned memory in 64 KB pages, from the title's physical
/// memory, or from the heap when that fails.
pub struct Buffer {
    ptr: NonNull<u8>,
    len: usize,
    physical: bool,
}

impl Buffer {
    pub fn new(len: usize) -> Self {
        let size = (len + 0xFFFF) & !0xFFFF;
        // SAFETY: the call has no preconditions; 0 reports failure.
        let address = unsafe {
            xboxkrnl::MmAllocatePhysicalMemoryEx(
                MemoryRegion::Title,
                size as u32,
                PageProtection::READWRITE | PageProtection::LARGE_PAGES,
                0,
                0xFFFF_FFFF,
                0x1_0000,
            )
        };
        if let Some(ptr) = NonNull::new(address as usize as *mut u8) {
            return Self { ptr, len: size, physical: true };
        }
        let layout = std::alloc::Layout::from_size_align(size, 4096).expect("layout");
        // SAFETY: the layout has a nonzero size.
        let ptr = unsafe { std::alloc::alloc(layout) };
        Self {
            ptr: NonNull::new(ptr).expect("out of memory"),
            len: size,
            physical: false,
        }
    }

    pub fn addr(&self) -> u32 {
        self.ptr.as_ptr() as usize as u32
    }

    pub fn is_physical(&self) -> bool {
        self.physical
    }

    pub fn bytes(&self) -> &[u8] {
        // SAFETY: the allocation is `len` bytes and owned by `self`.
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: the allocation is `len` bytes and owned by `self`.
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// The buffers are handed to one thread at a time.
unsafe impl Send for Buffer {}

/// Restricts the current thread to hardware thread `hw` (0-5) and returns
/// the hardware thread it runs on afterwards.
pub fn pin_current_thread(hw: u8) -> u8 {
    let thread = xbox360::sys::processor::current_thread();
    let mut previous = 0u32;
    // SAFETY: `thread` is the running thread object; `previous` is a live local.
    let _ = unsafe {
        xboxkrnl::KeSetAffinityThread(
            core::ptr::with_exposed_provenance_mut::<Kthread>(thread as usize),
            1u32 << hw,
            &mut previous,
        )
    };
    std::thread::yield_now();
    current_hw_thread()
}

pub fn current_hw_thread() -> u8 {
    xbox360::sys::processor::Kpcr::current().processor_number().get()
}
