//! VMX128 and memory benchmarks for the Xbox 360 CPU.
//!
//! Measures latency and throughput of classic AltiVec and VMX128 forms of
//! vector operations, the VMX128-only operations against classic
//! sequences, a 4x4 transform with 32 and 128 registers, and load, store
//! and copy bandwidth by working-set size. Every kernel is first checked
//! against the result computed by the generator's instruction model
//! (register kernels) or a scalar reference (buffer kernels).
//!
//! Results go to DEVKIT:\vmx128bench\results.csv and to the debug output;
//! DEVKIT:\vmx128bench\done.txt appears at the end. An optional
//! DEVKIT:\vmx128bench\config.txt holds `key=value` lines: `sections=`
//! (comma list of info,scan,clock,check,reg,xform,mem,smt,ext,insn; insn
//! runs only with `only=` and is not in the default list), `hw=`
//! (hardware thread, default 1: the quietest in the `scan` section), `runs=`, `target_us=`, `only=` (substring
//! filter on kernel names, `|` separates alternatives), `stagger=` (ext
//! section: buffer j starts j * stagger bytes, at most 32768, into its
//! 64 KB-aligned allocation; default 4224, because streams whose addresses
//! agree in the low bits run several times slower).

#![no_main]

mod ext;
mod harness;
mod kernels;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use harness::Buffer;
use harness::CYCLES_PER_TICK;
use harness::KernelFn;
use harness::Stats;
use harness::TB_HZ;
use kernels::EXPECT;
use kernels::INPUT;

core::arch::global_asm!(include_str!("kernels.s"), options(raw));

const DIR: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\vmx128bench";

const MEM_SIZES: &[usize] = &[
    4 << 10,
    8 << 10,
    16 << 10,
    24 << 10,
    32 << 10,
    48 << 10,
    64 << 10,
    96 << 10,
    128 << 10,
    256 << 10,
    384 << 10,
    512 << 10,
    768 << 10,
    1 << 20,
    3 << 19,
    2 << 20,
    4 << 20,
    8 << 20,
    16 << 20,
    32 << 20,
];
const MEM_MAX: usize = 32 << 20;
const SLACK: usize = 4096;
const XFORM_VECS: usize = 480;

struct Config {
    sections: Vec<String>,
    hw: u8,
    runs: usize,
    target: u64,
    only: String,
    /// ext section: buffer j starts j * stagger bytes into its allocation.
    stagger: usize,
}

impl Config {
    fn load() -> Self {
        let mut config = Self {
            sections: "info,scan,clock,check,reg,xform,mem,smt,ext".split(',').map(String::from).collect(),
            hw: 1,
            runs: 7,
            target: 200_000,
            only: String::new(),
            stagger: 4224,
        };
        // The devkit disk only reads into image or stack memory, and only
        // reads that cover the rest of the file.
        let mut buf = [0u8; 4096];
        let Ok(mut file) = File::open(format!("{DIR}\\config.txt")) else {
            return config;
        };
        let len = file.read(&mut buf).unwrap_or(0);
        for line in String::from_utf8_lossy(&buf[..len]).lines() {
            let Some((key, value)) = line.trim().split_once('=') else {
                continue;
            };
            match key.trim() {
                "sections" => config.sections = value.split(',').map(|s| s.trim().to_string()).collect(),
                "hw" => config.hw = value.trim().parse().unwrap_or(1),
                "runs" => config.runs = value.trim().parse().unwrap_or(7),
                "target_us" => config.target = value.trim().parse::<u64>().map(|us| us * 50).unwrap_or(200_000),
                "only" => config.only = value.trim().to_string(),
                "stagger" => config.stagger = value.trim().parse::<usize>().unwrap_or(0).min(32768) & !15,
                _ => {}
            }
        }
        config
    }

    fn has(&self, section: &str) -> bool {
        self.sections.iter().any(|s| s == section)
    }

    fn selected(&self, name: &str) -> bool {
        self.only.is_empty() || self.only.split('|').any(|p| name.contains(p))
    }
}

/// Result lines, written to the results file and the debug output.
struct Out {
    file: Option<File>,
    pending: String,
    failures: u32,
}

impl Out {
    fn new() -> Self {
        let file = File::create(format!("{DIR}\\results.csv")).ok();
        Self { file, pending: String::new(), failures: 0 }
    }

    fn line(&mut self, text: String) {
        println!("{text}");
        self.pending.push_str(&text);
        self.pending.push('\n');
        if self.pending.len() > 16 * 1024 {
            self.flush();
        }
    }

    fn flush(&mut self) {
        if let Some(file) = &mut self.file {
            // Write through a stack copy: the disk driver transfers from
            // image or stack memory.
            let mut chunk = [0u8; 4096];
            for piece in self.pending.as_bytes().chunks(chunk.len()) {
                chunk[..piece.len()].copy_from_slice(piece);
                let _ = file.write_all(&chunk[..piece.len()]);
            }
            let _ = file.flush();
        }
        self.pending.clear();
    }
}

fn kernel_addr(f: KernelFn) -> KernelFn {
    f
}

fn ptr_u32<T>(p: *const T) -> u32 {
    p as usize as u32
}

/// Aligned scratch for register kernel inputs and outputs.
#[repr(C, align(128))]
struct Scratch([u32; 4 * 128]);

fn run_regk(k: &kernels::RegKernel, iters: u32, out: &mut Scratch, runs: usize) -> Stats {
    let input = ptr_u32(INPUT.0[k.input..].as_ptr());
    let args = [iters, input, ptr_u32(out.0.as_mut_ptr()), 0, 0, 0];
    // SAFETY: the kernel reads its inputs from INPUT and writes `outputs`
    // vectors to `out`.
    unsafe { harness::measure(kernel_addr(k.f), args, [0.0; 2], 1, runs) }
}

fn call_regk(k: &kernels::RegKernel, iters: u32, out: &mut Scratch) -> u64 {
    let input = ptr_u32(INPUT.0[k.input..].as_ptr());
    let args = [iters, input, ptr_u32(out.0.as_mut_ptr()), 0, 0, 0];
    // SAFETY: as in run_regk.
    unsafe { harness::time_call(k.f, args, [0.0; 2], 1).ticks }
}

fn compare_words(got: &[u32], want: &[u32], tol: f32) -> bool {
    got.iter().zip(want).all(|(&g, &w)| {
        if tol == 0.0 {
            g == w
        } else {
            let (g, w) = (f32::from_bits(g), f32::from_bits(w));
            (g - w).abs() <= tol * w.abs().max(1e-30) || g == w
        }
    })
}

fn hex(words: &[u32]) -> String {
    words.iter().map(|w| format!("{w:08X}")).collect::<Vec<_>>().join(" ")
}

fn check_reg(out: &mut Out, cfg: &Config) -> Vec<bool> {
    let mut scratch = Scratch([0; 512]);
    let mut ok_all = Vec::new();
    let mut fails = 0;
    for k in kernels::REG_KERNELS {
        if !cfg.selected(k.name) {
            ok_all.push(true);
            continue;
        }
        scratch.0.fill(0xDEAD_BEEF);
        call_regk(k, k.check_iters, &mut scratch);
        let n = 4 * k.outputs;
        let want = &EXPECT.0[k.expect..k.expect + n];
        let got = &scratch.0[..n];
        let ok = compare_words(got, want, k.tol);
        if !ok {
            fails += 1;
            out.line(format!("check,reg,{},FAIL,got {},want {}", k.name, hex(&got[..n.min(8)]), hex(&want[..n.min(8)])));
        }
        ok_all.push(ok);
    }
    out.failures += fails;
    out.line(format!("check,reg,summary,{},{} kernels,{} failed", if fails == 0 { "ok" } else { "FAIL" }, ok_all.len(), fails));
    ok_all
}

/// Executes single memory-form instructions (`only=` selects them). An
/// instruction the CPU does not implement stops the title with an
/// illegal-instruction exception after its `start` line.
fn insn(out: &mut Out, cfg: &Config) {
    #[repr(C, align(16))]
    struct Bytes([u8; 64]);
    let input = Bytes(core::array::from_fn(|i| 0x10 + i as u8));
    let mut output = Bytes([0xEE; 64]);
    for q in kernels::QPROBES {
        if cfg.only.is_empty() || !cfg.selected(q.name) {
            continue;
        }
        out.line(format!("insn,{},start", q.name));
        out.flush();
        output.0.fill(0xEE);
        let args = [1, ptr_u32(input.0.as_ptr()), ptr_u32(output.0.as_mut_ptr()), 0, 0, 0];
        // SAFETY: the probe reads 32 bytes of input and writes at most 32 of output.
        unsafe { harness::time_call(q.f, args, [0.0; 2], 1) };
        let words: Vec<u32> = output.0[..32].chunks(4).map(|c| u32::from_be_bytes(c.try_into().unwrap())).collect();
        out.line(format!("insn,{},executed,{}", q.name, hex(&words)));
        out.flush();
    }
}

fn probes(out: &mut Out) {
    let mut scratch = Scratch([0; 512]);
    for p in kernels::PROBES {
        scratch.0.fill(0xDEAD_BEEF);
        let input = ptr_u32(INPUT.0[p.input..].as_ptr());
        let args = [1, input, ptr_u32(scratch.0.as_mut_ptr()), 0, 0, 0];
        // SAFETY: the probe reads `inputs` vectors and writes one.
        unsafe { harness::time_call(p.f, args, [0.0; 2], 1) };
        let got = &scratch.0[..4];
        let want = &EXPECT.0[p.expect..p.expect + 4];
        let inputs = &INPUT.0[p.input + 4..p.input + 4 * p.inputs];
        out.line(format!(
            "probe,{},{},in {},got {},model {},{}",
            p.name,
            if got == want { "matches-model" } else { "differs" },
            hex(inputs),
            hex(got),
            hex(want),
            p.desc
        ));
    }
}

fn info(out: &mut Out, cfg: &Config) {
    let freq = xbox360::sys::xboxkrnl::KeQueryPerformanceFrequency();
    out.line(format!("info,KeQueryPerformanceFrequency,{freq}"));
    let mut st0 = 0u64;
    let mut st1 = 0u64;
    // SAFETY: KeQuerySystemTime stores 8 bytes.
    unsafe { xbox360::sys::xboxkrnl::KeQuerySystemTime(&mut st0) };
    let tb0 = harness::mftb();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    // SAFETY: as above.
    unsafe { xbox360::sys::xboxkrnl::KeQuerySystemTime(&mut st1) };
    let tb1 = harness::mftb();
    let seconds = (st1 - st0) as f64 * 100e-9;
    out.line(format!(
        "info,timebase_vs_systemtime,{:.4} MHz,{} ticks in {:.6} s",
        (tb1 - tb0) as f64 / seconds / 1e6,
        tb1 - tb0,
        seconds
    ));
    out.line(format!("info,bench_hw_thread,{},runs {},target {} ticks", harness::current_hw_thread(), cfg.runs, cfg.target));
}

fn clocks(out: &mut Out, cfg: &Config) {
    #[repr(C, align(16))]
    struct In([f64; 4]);
    let input = In([0.0, 1.0, 1.0, 0.0]);
    let mut result = In([0.0; 4]);
    for c in kernels::CLOCKS {
        let run = |iters: u32, res: &mut In| {
            let args = [iters, ptr_u32(input.0.as_ptr()), ptr_u32(res.0.as_mut_ptr()), 0, 0, 0];
            // SAFETY: the clock kernels read 24 bytes of input and write 8.
            unsafe { harness::time_call(c.f, args, [0.0; 2], 1).ticks }
        };
        let iters = harness::calibrate(cfg.target, 1 << 28, |n| run(n, &mut result));
        let args = [iters, ptr_u32(input.0.as_ptr()), ptr_u32(result.0.as_mut_ptr()), 0, 0, 0];
        // SAFETY: as above.
        let stats = unsafe { harness::measure(c.f, args, [0.0; 2], 1, cfg.runs) };
        let total = iters as u64 * c.ops.max(1) as u64;
        let bits = result.0[0].to_bits();
        let ok = match c.name {
            "c_add" | "c_add_ind" | "c_lhs_gpr" => bits == total,
            "c_mulld" => bits == 3,
            "c_lhz_chain" | "c_lha_chain" | "c_lhz_extsh_chain" | "c_lwz_chain" => bits == 16,
            "c_fadd" | "c_fmadd" => result.0[0] == total as f64,
            _ => bits == 0,
        };
        if !ok {
            out.failures += 1;
        }
        let (m, lo, hi) = stats.scale(1.0 / total as f64);
        out.line(format!(
            "clock,{},{},{},ticks_per_op,{:.6},{:.6},{:.6},cycles_at_64,{:.3},{}",
            c.name,
            c.desc,
            iters,
            m,
            lo,
            hi,
            m * CYCLES_PER_TICK,
            if ok { "ok" } else { "FAIL" }
        ));
    }
}

fn reg_timing(out: &mut Out, cfg: &Config, checks: &[bool]) {
    let mut scratch = Scratch([0; 512]);
    for (k, &ok) in kernels::REG_KERNELS.iter().zip(checks) {
        if k.kind == "chk" || !cfg.selected(k.name) {
            continue;
        }
        let iters = harness::calibrate(cfg.target, 1 << 24, |n| call_regk(k, n, &mut scratch));
        let stats = run_regk(k, iters, &mut scratch, cfg.runs);
        let per = CYCLES_PER_TICK / (iters as f64 * k.body as f64);
        let (m, lo, hi) = stats.scale(per);
        out.line(format!(
            "reg,{},{},{},{},{},{},{},{},cpi,{:.3},{:.3},{:.3},per_step,{:.3},{}",
            k.op,
            k.group,
            k.variant,
            k.kind,
            k.chains,
            k.seq,
            k.per,
            k.body,
            m,
            lo,
            hi,
            m * k.seq as f64,
            if ok { "ok" } else { "FAIL" }
        ));
    }
}

/// 4x4 matrix A (out_i = sum_k A[i][k] v[k]) and input vectors with small
/// integer values, so every form computes exact results.
fn xform_setup(src: &mut Buffer, rowmajor: bool) -> Vec<[f32; 4]> {
    // Positive values: vmsum4fp128 is not exact when products have mixed
    // signs (probe dot4_neg).
    let a = |i: usize, k: usize| ((i * 4 + k) % 7) as f32 + 1.0;
    let v = |n: usize, k: usize| ((n * 5 + k * 3) % 11) as f32 + 1.0;
    let bytes = src.bytes_mut();
    let mut put = |off: usize, x: f32| bytes[off..off + 4].copy_from_slice(&x.to_bits().to_be_bytes());
    for i in 0..4 {
        for k in 0..4 {
            // column k at 16*k holds A[*][k]; row i at 16*i holds A[i][*]
            if rowmajor {
                put(16 * i + 4 * k, a(i, k));
            } else {
                put(16 * k + 4 * i, a(i, k));
            }
        }
    }
    let mut expect = Vec::with_capacity(XFORM_VECS);
    for n in 0..XFORM_VECS {
        let mut o = [0f32; 4];
        for k in 0..4 {
            put(64 + 16 * n + 4 * k, v(n, k));
        }
        for (i, oi) in o.iter_mut().enumerate() {
            *oi = (0..4).map(|k| a(i, k) * v(n, k)).sum();
        }
        expect.push(o);
    }
    expect
}

fn xforms(out: &mut Out, cfg: &Config, src: &mut Buffer, dst: &mut Buffer) {
    for x in kernels::XFORMS {
        if !cfg.selected(x.name) {
            continue;
        }
        let expect = xform_setup(src, x.rowmajor);
        dst.bytes_mut()[..XFORM_VECS * 16].fill(0xEE);
        let args = |passes: u32| [passes, src.addr(), dst.addr(), XFORM_VECS as u32, 0, 0];
        // SAFETY: the kernel reads 64 + 16*XFORM_VECS bytes of src and writes 16*XFORM_VECS to dst.
        unsafe { harness::time_call(x.f, args(1), [0.0; 2], 1) };
        let got = dst.bytes();
        let ok = expect.iter().enumerate().all(|(n, o)| {
            (0..4).all(|i| {
                let off = 16 * n + 4 * i;
                u32::from_be_bytes(got[off..off + 4].try_into().unwrap()) == o[i].to_bits()
            })
        });
        if !ok {
            out.failures += 1;
        }
        // SAFETY: as above.
        let passes = harness::calibrate(cfg.target, 1 << 20, |p| unsafe { harness::time_call(x.f, args(p), [0.0; 2], 1).ticks });
        // SAFETY: as above.
        let stats = unsafe { harness::measure(x.f, args(passes), [0.0; 2], 1, cfg.runs) };
        let (m, lo, hi) = stats.scale(CYCLES_PER_TICK / (passes as f64 * XFORM_VECS as f64));
        out.line(format!(
            "xform,{},{},{},{},cycles_per_vector,{:.3},{:.3},{:.3},{}",
            x.name,
            x.form,
            x.unroll,
            x.regs,
            m,
            lo,
            hi,
            if ok { "ok" } else { "FAIL" }
        ));
    }
}

fn fill_src(src: &mut Buffer) {
    for (i, chunk) in src.bytes_mut().chunks_exact_mut(4).enumerate() {
        let w = (i as u32).wrapping_mul(0x9E37_79B9) ^ 0x5A5A_1234;
        chunk.copy_from_slice(&w.to_be_bytes());
    }
}

fn xor_words(bytes: &[u8]) -> u32 {
    bytes.chunks_exact(4).fold(0, |a, c| a ^ u32::from_be_bytes(c.try_into().unwrap()))
}

fn run_mem(k: &kernels::MemKernel, passes: u32, src: u32, dst: u32, res: u32, size: usize) -> u64 {
    let out = if k.group == "read" { res } else { dst + k.dst_off as u32 };
    let args = [passes, src + k.src_off as u32, out, size as u32, 0, 0];
    // SAFETY: the kernel touches size (+ one line of slack) bytes at src and dst, 128 bytes at res.
    unsafe { harness::time_call(k.f, args, [0.0; 2], 1).ticks }
}

/// Runs `k` once over `size` bytes and checks the result.
fn check_mem(k: &kernels::MemKernel, size: usize, src: &Buffer, dst: &mut Buffer, res: &mut Buffer) -> bool {
    dst.bytes_mut()[..size + SLACK].fill(0xEE);
    res.bytes_mut()[..256].fill(0xEE);
    run_mem(k, 1, src.addr(), dst.addr(), res.addr(), size);
    let s = &src.bytes()[k.src_off..k.src_off + size];
    let d = &dst.bytes()[k.dst_off..k.dst_off + size];
    let r = res.bytes();
    match k.kind {
        "read" => xor_words(&r[..64]) == xor_words(s),
        "readgpr" => xor_words(&r[..32]) == xor_words(s),
        "readlast" => r[..128] == s[size - 128..],
        "readlastf" => r[64..128] == s[size - 64..],
        "write" if k.pattern == 0 => d.iter().all(|&b| b == 0),
        "write" => d.chunks(k.pattern).all(|c| c == &src.bytes()[..c.len()]),
        "copy" => d == s,
        _ => false,
    }
}

fn mem(out: &mut Out, cfg: &Config, src: &mut Buffer, dst: &mut Buffer, res: &mut Buffer) {
    fill_src(src);
    let mut ok = Vec::new();
    for k in kernels::MEM_KERNELS {
        let good = [4096usize, 65536].iter().all(|&size| check_mem(k, size, src, dst, res));
        if !good {
            out.failures += 1;
        }
        out.line(format!("check,mem,{},{}", k.name, if good { "ok" } else { "FAIL" }));
        ok.push(good);
    }
    for &size in MEM_SIZES {
        for (k, &good) in kernels::MEM_KERNELS.iter().zip(&ok) {
            if !cfg.selected(k.name) {
                continue;
            }
            let (s, d, r) = (src.addr(), dst.addr(), res.addr());
            let passes = harness::calibrate(cfg.target, 1 << 20, |p| run_mem(k, p, s, d, r, size));
            let out_addr = if k.group == "read" { r } else { d + k.dst_off as u32 };
            let args = [passes, s + k.src_off as u32, out_addr, size as u32, 0, 0];
            // SAFETY: as in run_mem.
            let stats = unsafe { harness::measure(k.f, args, [0.0; 2], 1, cfg.runs) };
            // Traffic bandwidth counts both the read and write of a copy.
            let bytes = size as f64 * passes as f64 * if k.kind == "copy" { 2.0 } else { 1.0 };
            let bpc = |t: u64| bytes / (t as f64 * CYCLES_PER_TICK);
            let gbs = |t: u64| bytes / (t as f64 / TB_HZ) / 1e9;
            out.line(format!(
                "mem,{},{},{},{},bytes_per_cycle,{:.3},{:.3},{:.3},GBps,{:.3},{:.3},{:.3},{}",
                k.name,
                k.group,
                k.variant,
                size,
                bpc(stats.median),
                bpc(stats.max),
                bpc(stats.min),
                gbs(stats.median),
                gbs(stats.max),
                gbs(stats.min),
                if good { "ok" } else { "FAIL" }
            ));
        }
        out.flush();
    }
}

fn scan(out: &mut Out, cfg: &Config, src: &mut Buffer, dst: &mut Buffer, res: &mut Buffer) {
    fill_src(src);
    let lat = kernels::REG_KERNELS.iter().find(|k| k.name == "k_vaddfp_cls_lat1").unwrap();
    let thr = kernels::REG_KERNELS.iter().find(|k| k.name == "k_vaddfp_128lo_thr16").unwrap();
    let rd = kernels::MEM_KERNELS.iter().find(|k| k.name == "m_rd_lvx128hi").unwrap();
    let mut scratch = Scratch([0; 512]);
    for hw in 0..6u8 {
        let actual = harness::pin_current_thread(hw);
        let mut cols = Vec::new();
        for k in [lat, thr] {
            let iters = harness::calibrate(cfg.target, 1 << 24, |n| call_regk(k, n, &mut scratch));
            let st = run_regk(k, iters, &mut scratch, cfg.runs);
            let (m, lo, hi) = st.scale(CYCLES_PER_TICK / (iters as f64 * k.body as f64));
            cols.push(format!("{},cpi,{m:.3},{lo:.3},{hi:.3}", k.name));
        }
        for size in [16usize << 10, 8 << 20] {
            let (s, d, r) = (src.addr(), dst.addr(), res.addr());
            let passes = harness::calibrate(cfg.target, 1 << 20, |p| run_mem(rd, p, s, d, r, size));
            let args = [passes, s, r, size as u32, 0, 0];
            // SAFETY: as in run_mem.
            let st = unsafe { harness::measure(rd.f, args, [0.0; 2], 1, cfg.runs) };
            let bytes = size as f64 * passes as f64;
            cols.push(format!(
                "{}@{},bpc,{:.3},{:.3},{:.3}",
                rd.name,
                size,
                bytes / (st.median as f64 * CYCLES_PER_TICK),
                bytes / (st.max as f64 * CYCLES_PER_TICK),
                bytes / (st.min as f64 * CYCLES_PER_TICK)
            ));
        }
        out.line(format!("scan,{hw},{actual},{}", cols.join(",")));
    }
    let actual = harness::pin_current_thread(cfg.hw);
    out.line(format!("scan,back_to,{},{}", cfg.hw, actual));
    let _ = dst;
}

/// One call batch for the SMT tests.
#[derive(Clone, Copy)]
struct Job {
    f: KernelFn,
    args: [u32; 6],
}

unsafe impl Send for Job {}

fn run_pair(a: (u8, Job), b: (u8, Job), runs: usize) -> (Stats, Stats) {
    let gate = Arc::new(AtomicU32::new(0));
    let spawn = |hw: u8, job: Job, gate: Arc<AtomicU32>| {
        std::thread::spawn(move || {
            harness::pin_current_thread(hw);
            gate.fetch_add(1, Ordering::SeqCst);
            while gate.load(Ordering::SeqCst) < 2 {
                core::hint::spin_loop();
            }
            // SAFETY: the job's arguments were valid for a single-thread run.
            unsafe { harness::measure(job.f, job.args, [0.0; 2], 1, runs) }
        })
    };
    let ta = spawn(a.0, a.1, gate.clone());
    let tb = spawn(b.0, b.1, gate);
    (ta.join().unwrap(), tb.join().unwrap())
}

fn smt(out: &mut Out, cfg: &Config, bufs: [&mut Buffer; 3], extra: [&mut Buffer; 3]) {
    let [src, dst, res] = bufs;
    let [src2, dst2, res2] = extra;
    fill_src(src);
    fill_src(src2);
    // Each thread needs its own scratch for register kernels. Both live
    // until the end of this function, after every job has joined.
    let mut scratch_a = Box::new(Scratch([0; 512]));
    let mut scratch_b = Box::new(Scratch([0; 512]));
    let mut jobs: Vec<(String, Job, Job, f64)> = Vec::new();
    for name in [
        "k_vaddfp_cls_lat1",
        "k_vaddfp_128lo_thr16",
        "k_vmaddfp_acc_128lo_thr16",
        "k_vperm_128lo_thr16",
        "k_dot4_128lo_lat1",
        "k_dot4_128lo_thr16",
        "k_vxor_128lo_thr16",
    ] {
        let Some(k) = kernels::REG_KERNELS.iter().find(|k| k.name == name) else {
            continue;
        };
        let input = ptr_u32(INPUT.0[k.input..].as_ptr());
        let iters = harness::calibrate(cfg.target, 1 << 24, |n| call_regk(k, n, &mut scratch_a));
        let mk = |s: &mut Scratch| Job { f: k.f, args: [iters, input, ptr_u32(s.0.as_mut_ptr()), 0, 0, 0] };
        let work = iters as f64 * k.body as f64;
        jobs.push((name.to_string(), mk(&mut scratch_a), mk(&mut scratch_b), work));
    }
    for (name, size) in [("m_rd_lvx128hi", 16usize << 10), ("m_rd_lvx128hi", 8 << 20), ("m_cp_dcbz", 8 << 20), ("m_wr_dcbz_stvx128", 8 << 20)] {
        let k = kernels::MEM_KERNELS.iter().find(|k| k.name == name).unwrap();
        let (s, d, r) = (src.addr(), dst.addr(), res.addr());
        let passes = harness::calibrate(cfg.target, 1 << 20, |p| run_mem(k, p, s, d, r, size));
        let mk = |s: u32, d: u32, r: u32| Job {
            f: k.f,
            args: [passes, s, if k.group == "read" { r } else { d }, size as u32, 0, 0],
        };
        jobs.push((
            format!("{name}@{size}"),
            mk(s, d, r),
            mk(src2.addr(), dst2.addr(), res2.addr()),
            size as f64 * passes as f64 * if k.kind == "copy" { 2.0 } else { 1.0 },
        ));
    }
    for (name, a, b, work) in &jobs {
        // SAFETY: the job was calibrated with these arguments.
        let single = unsafe { harness::measure(a.f, a.args, [0.0; 2], 1, cfg.runs) };
        let sib = cfg.hw ^ 1;
        let other_core = if cfg.hw >= 2 { cfg.hw - 2 } else { cfg.hw + 2 };
        let (pa, pb) = run_pair((cfg.hw, *a), (sib, *b), cfg.runs);
        let (ca, cb) = run_pair((cfg.hw, *a), (other_core, *b), cfg.runs);
        let unit = if name.starts_with("m_") { "bytes_per_cycle" } else { "insn_per_cycle" };
        let rate = |t: u64| work / (t as f64 * CYCLES_PER_TICK);
        out.line(format!(
            "smt,{name},{unit},single,{:.3},smt_pair_{}_{},{:.3},{:.3},sum,{:.3},cross_core_{}_{},{:.3},{:.3},sum,{:.3}",
            rate(single.median),
            cfg.hw,
            sib,
            rate(pa.median),
            rate(pb.median),
            rate(pa.median) + rate(pb.median),
            cfg.hw,
            other_core,
            rate(ca.median),
            rate(cb.median),
            rate(ca.median) + rate(cb.median),
        ));
    }
    let _ = (dst2, res2);
}

fn fnv(hash: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *hash ^= b as u64;
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}

fn ext_bytes(n: u32) -> usize {
    // Strided streams need 2n words; a matrix needs at least 16 words.
    (n as usize * 8).max(64)
}

fn reset_ext(k: &ext::ExtKernel, bufs: &mut [Buffer; 3], a: &mut ext::ExtArgs, stagger: usize, mis: u32) {
    for (j, b) in bufs.iter_mut().enumerate() {
        let start = j * stagger + mis as usize;
        b.bytes_mut()[..start].fill(0xA5);
        k.input.fill(&mut b.bytes_mut()[start..start + ext_bytes(a.n)], j);
    }
    k.reset_globals();
    a.args = [0; 6];
    a.fargs = [0.0; 2];
    (k.setup)(a);
}

fn ext_args(bufs: &[Buffer; 3], n: u32, stagger: usize, mis: u32) -> ext::ExtArgs {
    ext::ExtArgs {
        bufs: core::array::from_fn(|j| bufs[j].addr() + (j * stagger) as u32 + mis),
        n,
        args: [0; 6],
        fargs: [0.0; 2],
    }
}

fn ext_hash(k: &ext::ExtKernel, bufs: &[Buffer; 3], a: &ext::ExtArgs, first: &harness::Sample) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for (j, b) in bufs.iter().enumerate() {
        let start = (a.bufs[j] - b.addr()) as usize;
        fnv(&mut hash, &b.bytes()[..start + ext_bytes(a.n)]);
    }
    if let Some(globals) = k.globals {
        for addr in globals() {
            // SAFETY: each generated global is an initialized 1024-word array.
            fnv(&mut hash, unsafe { core::slice::from_raw_parts(addr as usize as *const u8, 4096) });
        }
    }
    match k.ret {
        ext::Ret::None => {}
        ext::Ret::Int => fnv(&mut hash, &first.ret.to_be_bytes()),
        ext::Ret::Float => fnv(&mut hash, &(first.fret as f32).to_bits().to_be_bytes()),
    }
    hash
}

fn ext(out: &mut Out, cfg: &Config) {
    if ext::EXT_KERNELS.is_empty() {
        out.line("ext,none".to_string());
        return;
    }
    let mut bufs = [Buffer::new(ext::EXT_BUFFER_BYTES), Buffer::new(ext::EXT_BUFFER_BYTES), Buffer::new(ext::EXT_BUFFER_BYTES)];
    let mut refs = [Buffer::new(ext::EXT_BUFFER_BYTES), Buffer::new(ext::EXT_BUFFER_BYTES), Buffer::new(ext::EXT_BUFFER_BYTES)];
    let mut checked = 0;
    let mut passed = 0;
    out.line(format!(
        "ext,buffers,{:08X},{:08X},{:08X},stagger,{}",
        bufs[0].addr(),
        bufs[1].addr() + cfg.stagger as u32,
        bufs[2].addr() + 2 * cfg.stagger as u32,
        cfg.stagger
    ));
    for k in ext::EXT_KERNELS {
        if !cfg.selected(k.name) {
            continue;
        }
        let reference = ext::EXT_KERNELS.iter().find(|r| r.name == k.reference);
        for &n in ext::EXT_COUNTS {
            if n < k.min_n || k.fixed_n.is_some_and(|fixed| fixed != n) {
                continue;
            }
            for mis in [0u32, 4] {
                if mis != 0 && k.aligned_only {
                    continue;
                }
                let mut a = ext_args(&bufs, n, cfg.stagger, mis);
                // Do not count a reference comparing to a second invocation
                // of itself as validation. Every row has the scalar oracle.
                let expected = reference.filter(|r| r.name != k.name).map(|r| {
                    let mut ra = ext_args(&refs, n, cfg.stagger, mis);
                    reset_ext(r, &mut refs, &mut ra, cfg.stagger, mis);
                    // SAFETY: designated references accept this same count/offset.
                    let first = unsafe { harness::time_call(r.f, ra.args, ra.fargs, 1) };
                    (ext_hash(r, &refs, &ra, &first), first, r.check_globals(first.fret))
                });
                let mut oracle_args = ext_args(&refs, n, cfg.stagger, mis);
                reset_ext(k, &mut refs, &mut oracle_args, cfg.stagger, mis);
                let [y, x, b] = &mut refs;
                let scalar = k.scalar([
                    &mut y.bytes_mut()[mis as usize..],
                    &mut x.bytes_mut()[cfg.stagger + mis as usize..],
                    &mut b.bytes_mut()[2 * cfg.stagger + mis as usize..],
                ], n as usize);
                reset_ext(k, &mut bufs, &mut a, cfg.stagger, mis);
                // SAFETY: setup and typed initialization describe a valid call.
                let first = unsafe { harness::time_call(k.f, a.args, a.fargs, 1) };
                let hash = ext_hash(k, &bufs, &a, &first);
                let oracle_good = scalar.is_some_and(|(expected_int, expected_float)| {
                    let same_buffers = bufs.iter().zip(&refs).enumerate().all(|(j, (b, r))| {
                        let start = j * cfg.stagger + mis as usize;
                        let end = start + ext_bytes(n);
                        b.bytes()[..end] == r.bytes()[..end]
                    });
                    let same_return = match k.ret {
                        ext::Ret::None => true,
                        ext::Ret::Int => first.ret == expected_int,
                        ext::Ret::Float => (first.fret as f32).to_bits() == (expected_float as f32).to_bits(),
                    };
                    k.check_globals(first.fret) && same_buffers && same_return
                });
                let designated_good = reference.is_some_and(|r| {
                    r.name == k.name || expected.is_some_and(|(expected_hash, _, reference_good)| {
                        reference_good && hash == expected_hash
                    })
                });
                let good = oracle_good && designated_good;
                checked += 1;
                if good {
                    passed += 1;
                } else {
                    out.failures += 1;
                }
                let expected_hash = expected.map(|(hash, _, _)| format!("{hash:016x}")).unwrap_or_else(|| {
                    if reference.is_some() { "identity" } else { "missing" }.to_string()
                });
                out.line(format!(
                    "check,ext,{},{},{},{},reference,{},checksum,{:016x},expected,{},oracle,{},designated,{}",
                    k.name, n, mis, if good { "ok" } else { "FAIL" }, k.reference, hash, expected_hash,
                    if oracle_good { "ok" } else { "FAIL" },
                    if designated_good { "ok" } else { "FAIL" }
                ));
                // Bound repeated in-place growth to finite f32 values. Every
                // calibration/sample batch starts from the same typed inputs.
                let max_reps = match k.base {
                    "k_scale_inplace" => 64,
                    "k_prefix_f32" => 4,
                    _ => 1 << 24,
                };
                let reps = harness::calibrate(cfg.target, max_reps, |r| {
                    reset_ext(k, &mut bufs, &mut a, cfg.stagger, mis);
                    // SAFETY: the bounded batch has valid arguments and inputs.
                    unsafe { harness::time_call(k.f, a.args, a.fargs, r).ticks }
                });
                reset_ext(k, &mut bufs, &mut a, cfg.stagger, mis);
                // SAFETY: as above; untimed warm-up batch.
                unsafe { harness::time_call(k.f, a.args, a.fargs, reps) };
                let st = Stats::from((0..cfg.runs).map(|_| {
                    reset_ext(k, &mut bufs, &mut a, cfg.stagger, mis);
                    // SAFETY: as above; resetting is outside the timed region.
                    unsafe { harness::time_call(k.f, a.args, a.fargs, reps).ticks }
                }).collect());
                let (m, lo, hi) = st.scale(CYCLES_PER_TICK / (reps as f64 * n as f64));
                out.line(format!(
                    "ext,{},{},{},cycles_per_element,{:.3},{:.3},{:.3},checksum,{:016x}",
                    k.name, n, mis, m, lo, hi, hash
                ));
            }
        }
        out.flush();
    }
    out.line(format!("check,ext,summary,total,{checked},passed,{passed},failed,{}", checked - passed));
}

fn main_body() {
    let cfg = Config::load();
    let mut out = Out::new();
    out.line(format!("start,vmx128bench,sections,{}", cfg.sections.join(" ")));
    let pinned = harness::pin_current_thread(cfg.hw);
    out.line(format!("info,pinned,{},{}", cfg.hw, pinned));
    let mut src = Buffer::new(MEM_MAX + SLACK);
    let mut dst = Buffer::new(MEM_MAX + SLACK);
    let mut res = Buffer::new(65536);
    out.line(format!(
        "info,buffers,src {:08X},dst {:08X},physical 64KB pages {}",
        src.addr(),
        dst.addr(),
        src.is_physical() && dst.is_physical()
    ));
    if cfg.has("info") {
        info(&mut out, &cfg);
        out.flush();
    }
    if cfg.has("scan") {
        scan(&mut out, &cfg, &mut src, &mut dst, &mut res);
        out.flush();
    }
    if cfg.has("clock") {
        clocks(&mut out, &cfg);
        out.flush();
    }
    let checks = check_reg(&mut out, &cfg);
    probes(&mut out);
    out.flush();
    if cfg.has("reg") {
        reg_timing(&mut out, &cfg, &checks);
        out.flush();
    }
    if cfg.has("xform") {
        xforms(&mut out, &cfg, &mut src, &mut dst);
        out.flush();
    }
    if cfg.has("mem") {
        mem(&mut out, &cfg, &mut src, &mut dst, &mut res);
        out.flush();
    }
    if cfg.has("smt") {
        let mut src2 = Buffer::new((8 << 20) + SLACK);
        let mut dst2 = Buffer::new((8 << 20) + SLACK);
        let mut res2 = Buffer::new(65536);
        smt(&mut out, &cfg, [&mut src, &mut dst, &mut res], [&mut src2, &mut dst2, &mut res2]);
        out.flush();
    }
    if cfg.has("ext") {
        ext(&mut out, &cfg);
    }
    if cfg.has("insn") {
        insn(&mut out, &cfg);
    }
    let failures = out.failures;
    out.line(format!("end,failures,{failures}"));
    out.flush();
    drop(out);
    if let Ok(mut done) = File::create(format!("{DIR}\\done.txt")) {
        let msg = *b"done\n";
        let _ = done.write_all(&msg);
    }
    println!("vmx128bench: done, {failures} failures");
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

extern "C" fn main(_context: u32) -> u32 {
    // The title's main thread has a 16 KB stack, so the benchmarks run on
    // a thread with a larger one.
    let worker = std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(main_body)
        .expect("spawn benchmark thread");
    let _ = worker.join();
    xbox360::title::terminate()
}
