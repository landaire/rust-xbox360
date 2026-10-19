//! Reads the two resources that BUCK embeds and reports their sizes and
//! first bytes to the debug output, then that a missing name and a name
//! that differs only in case are absent. Ends with `DONE`.

#![no_std]
#![no_main]

use core::ffi::CStr;
use core::panic::PanicInfo;

use xbox360::dbg_println;
use xbox360::module;
use xbox360::title;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dbg_println!("FAIL panic: {info}");
    title::terminate()
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

fn report(module: module::ModuleHandle<'static>, name: &CStr) {
    let label = name.to_str().unwrap_or("?");
    match module::resource(module, name) {
        Ok(Some(bytes)) => {
            let first = &bytes[..bytes.len().min(8)];
            dbg_println!(
                "resource {label}: {} bytes at {:#010x}, first bytes {first:02x?}",
                bytes.len(),
                bytes.as_ptr() as usize
            );
        }
        Ok(None) => dbg_println!("resource {label}: absent"),
        Err(error) => dbg_println!("resource {label}: {error}"),
    }
}

extern "C" fn main(_context: u32) -> u32 {
    let module = module::executable_module();

    report(module, c"greeting");
    report(module, c"counter");
    report(module, c"missing");
    report(module, c"GREETING");

    dbg_println!("DONE");
    title::terminate()
}
