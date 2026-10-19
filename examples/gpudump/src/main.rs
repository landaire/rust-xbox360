//! System module that captures the GPU command stream of the running title:
//! the command processor ring, every indirect buffer reachable from it and
//! every shader that a 0x27 packet loads, written as records
//! `[tag, physical address, dwords, data...]` (tag 1 ring, 2 indirect
//! buffer, 3 shader) to `DEVKIT:\gpudump.bin`. Load it beside a title with
//! XBDM `dbgextld`.

#![no_main]
#![feature(xbox360_ext)]

use std::io::Write;
use std::os::xbox360::release_tls_slots;
use std::thread;

use xbox360::sys::mmio::GpuRegisters;
use xbox360::sys::xboxkrnl::Boolean;
use xbox360::sys::xboxkrnl::DllEntryReason;

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, module_main);

extern "C" fn module_main(_module: u32, reason: u32, _reserved: u32) -> Boolean {
    let loaded = match DllEntryReason::from_raw(reason) {
        Some(DllEntryReason::ProcessAttach) => thread::Builder::new().stack_size(64 * 1024).spawn(worker).is_ok(),
        Some(DllEntryReason::ProcessDetach) => {
            unsafe { release_tls_slots() };
            true
        }
        _ => true,
    };
    Boolean::from(loaded)
}

fn read_phys(physical: u32) -> u32 {
    unsafe { (0xC000_0000u32.wrapping_add(physical) as *const u32).read_volatile() }
}

fn record(out: &mut Vec<u32>, tag: u32, address: u32, dwords: u32) {
    out.extend([tag, address, dwords]);
    for i in 0..dwords {
        out.push(read_phys(address + i * 4));
    }
}

fn worker() {
    let gpu = GpuRegisters::get();
    let base = gpu.register(0x1c0).read();
    let ring_dwords = 1u32 << ((gpu.register(0x1c1).read() & 0x3f) + 1);
    println!(
        "gpudump: rptr {:#x} wptr {:#x} status {:08x} scanout {:#x} update {:#x}",
        gpu.register(0x1c4).read(),
        gpu.register(0x1c5).read(),
        gpu.register(0x5d0).read(),
        gpu.register(0x1844).read(),
        gpu.register(0x1851).read()
    );
    let mut out = Vec::new();
    record(&mut out, 1, base, ring_dwords);
    let mut seen = Vec::new();
    let mut shaders: Vec<(u32, u32)> = Vec::new();
    let mut queue: Vec<(u32, u32)> = Vec::new();
    for i in 0..ring_dwords {
        if read_phys(base + i * 4) == 0xc001_3f00 {
            queue.push((read_phys(base + ((i + 1) % ring_dwords) * 4), read_phys(base + ((i + 2) % ring_dwords) * 4) & 0xfffff));
        }
    }
    while let Some((address, dwords)) = queue.pop() {
        if dwords == 0 || dwords > 0x40000 || address >= 0x2000_0000 || seen.contains(&address) {
            continue;
        }
        seen.push(address);
        let start = out.len() + 3;
        record(&mut out, 2, address, dwords);
        for j in 0..dwords as usize {
            if out[start + j] == 0xc001_3f00 && j + 2 < dwords as usize {
                queue.push((out[start + j + 1], out[start + j + 2] & 0xfffff));
            }
            if out[start + j] == 0xc001_2700 && j + 2 < dwords as usize {
                shaders.push((out[start + j + 1] & !3, out[start + j + 2]));
            }
        }
    }
    shaders.sort();
    shaders.dedup();
    for (address, dwords) in shaders {
        if dwords < 0x1000 {
            record(&mut out, 3, address, dwords & 0xfff);
        }
    }
    let bytes: Vec<u8> = out.iter().flat_map(|w| w.to_be_bytes()).collect();
    let result = std::fs::File::create("\\Device\\Harddisk0\\Partition1\\DEVKIT\\gpudump.bin").and_then(|mut f| f.write_all(&bytes));
    println!("gpudump: {} ibs {} bytes {:?}", seen.len(), bytes.len(), result.map_err(|e| e.raw_os_error()));
}
