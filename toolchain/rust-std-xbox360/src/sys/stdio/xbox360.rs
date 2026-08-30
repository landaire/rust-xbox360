use crate::io::{self, IoSlice, IoSliceMut};
use crate::sys::pal::abi;

pub struct Stdin;
pub struct Stdout;
pub type Stderr = Stdout;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn read_vectored(&mut self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

/// Send `buf` to the kernel debug channel as null-terminated chunks. DbgPrint
/// is a printf-style C API, so `%` is doubled to print literally and each chunk
/// is passed as the format string (single fixed arg, no variadic ABI). Embedded
/// nul bytes end their chunk; text output is unaffected.
fn dbg_write(buf: &[u8]) {
    const CAP: usize = 255;
    let mut tmp = [0u8; CAP + 1];
    let mut n = 0;
    let mut flush = |tmp: &mut [u8; CAP + 1], n: &mut usize| {
        tmp[*n] = 0;
        unsafe { abi::DbgPrint(tmp.as_ptr()) };
        *n = 0;
    };
    for &b in buf {
        // Reserve room for a possible doubled '%' plus the nul terminator.
        if n >= CAP - 1 {
            flush(&mut tmp, &mut n);
        }
        tmp[n] = b;
        n += 1;
        if b == b'%' {
            tmp[n] = b'%';
            n += 1;
        }
    }
    if n > 0 {
        flush(&mut tmp, &mut n);
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        dbg_write(buf);
        Ok(buf.len())
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        let mut total = 0;
        for b in bufs {
            dbg_write(b);
            total += b.len();
        }
        Ok(total)
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stdout::new())
}
