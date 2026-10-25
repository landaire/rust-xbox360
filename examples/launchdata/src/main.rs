//! Sets launch data and launches a title: reads `game:\launch.bin` (the
//! launch data bytes) and `game:\target.txt` (the title path), then sets
//! the launch data and calls `XamLoaderLaunchTitle`.

#![no_main]

use std::ffi::CString;
use std::fs::File;
use std::io::Read;

use xbox360::xam::system::LAUNCH_DATA_CAPACITY;

const LAUNCH_PATH: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\launchdata\\launch.bin";
const TARGET_PATH: &str = "\\Device\\Harddisk0\\Partition1\\DEVKIT\\launchdata\\target.txt";
/// `MAX_PATH`, the longest title path read.
const TARGET_CAPACITY: usize = 260;

/// Why [`read_bounded`] returned no data.
enum ReadError {
    Missing,
    TooLarge,
}

/// Reads the file at `path` into `buffer` and returns the bytes read.
/// Fails when the file holds more than `buffer.len()` bytes.
fn read_bounded<'a>(path: &str, buffer: &'a mut [u8]) -> Result<&'a [u8], ReadError> {
    let mut file = File::open(path).map_err(|_| ReadError::Missing)?;

    let mut filled = 0;
    while filled < buffer.len() {
        match file.read(&mut buffer[filled..]) {
            Ok(0) => return Ok(&buffer[..filled]),
            Ok(read) => filled += read,
            Err(_) => return Err(ReadError::Missing),
        }
    }

    let mut probe = [0u8; 1];
    match file.read(&mut probe) {
        Ok(0) => Ok(&buffer[..filled]),
        Ok(_) => Err(ReadError::TooLarge),
        Err(_) => Err(ReadError::Missing),
    }
}

fn main_body() {
    let mut data = [0u8; LAUNCH_DATA_CAPACITY];
    let data = match read_bounded(LAUNCH_PATH, &mut data) {
        Ok(data) => data,
        Err(ReadError::Missing) => {
            println!("launchdata: no game:\\launch.bin");
            return;
        }
        Err(ReadError::TooLarge) => {
            println!("launchdata: game:\\launch.bin exceeds {LAUNCH_DATA_CAPACITY} bytes");
            return;
        }
    };

    let mut target = [0u8; TARGET_CAPACITY];
    let target = match read_bounded(TARGET_PATH, &mut target) {
        Ok(target) => target,
        Err(ReadError::Missing) => {
            println!("launchdata: no game:\\target.txt");
            return;
        }
        Err(ReadError::TooLarge) => {
            println!("launchdata: game:\\target.txt exceeds {TARGET_CAPACITY} bytes");
            return;
        }
    };
    let Ok(target) = CString::new(target.trim_ascii()) else {
        return;
    };

    match xbox360::xam::system::set_launch_data(data) {
        Ok(()) => println!("launchdata: set {} bytes", data.len()),
        Err(error) => {
            println!("launchdata: {error}");
            return;
        }
    }
    // SAFETY: `target` is a NUL-terminated path.
    unsafe { xbox360::sys::xam::XamLoaderLaunchTitle(target.as_ptr().cast(), 0) }
}

xbox360::sys::bridged!(#[unsafe(no_mangle)] pub _start, main);

extern "C" fn main(_context: u32) -> u32 {
    main_body();
    xbox360::title::terminate()
}
