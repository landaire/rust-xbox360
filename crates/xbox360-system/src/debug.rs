//! Debug output through `DbgPrint`.
//!
//! Titles that use the standard library write to the debug output with
//! `print!` and `eprint!`.

use core::fmt;

use xbox360_sys::lib_index::XBOXKRNL;

xbox360_sys::import_thunk!(
    /// `DbgPrint` with one string argument.
    dbg_print_string(format: *const u8, text: *const u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x003
);

/// Text bytes per `DbgPrint` call. The buffer holds one more for the NUL.
const CAPACITY: usize = 255;

/// A `core::fmt::Write` sink for the debug output. Text is passed to
/// `DbgPrint` as the argument of `%s` in pieces of at most 255 bytes, on
/// [`flush`](Self::flush) and on drop. NUL bytes cannot pass through
/// `%s` and are left out.
pub struct DebugOutput {
    buffer: [u8; CAPACITY + 1],
    length: usize,
}

impl DebugOutput {
    pub const fn new() -> Self {
        Self {
            buffer: [0; CAPACITY + 1],
            length: 0,
        }
    }

    /// Write the buffered text.
    pub fn flush(&mut self) {
        if self.length == 0 {
            return;
        }

        self.buffer[self.length] = 0;

        // SAFETY: the format is a NUL-terminated "%s" and the buffer is
        // NUL-terminated at `length`.
        unsafe { dbg_print_string(c"%s".as_ptr().cast(), self.buffer.as_ptr()) };

        self.length = 0;
    }
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for DebugOutput {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for &byte in text.as_bytes().iter().filter(|&&byte| byte != 0) {
            if self.length == CAPACITY {
                self.flush();
            }
            self.buffer[self.length] = byte;
            self.length += 1;
        }

        Ok(())
    }
}

impl Drop for DebugOutput {
    fn drop(&mut self) {
        self.flush();
    }
}

#[doc(hidden)]
pub fn print_arguments(arguments: fmt::Arguments<'_>) {
    let mut output = DebugOutput::new();

    // `DebugOutput` never fails; an error comes from a `Display` impl and
    // ends the message where it occurred.
    let _ = fmt::write(&mut output, arguments);
}

/// Format to the debug output.
#[macro_export]
macro_rules! dbg_print {
    ($($argument:tt)*) => {
        $crate::debug::print_arguments(::core::format_args!($($argument)*))
    };
}

/// Format to the debug output and end the line.
#[macro_export]
macro_rules! dbg_println {
    () => {
        $crate::debug::print_arguments(::core::format_args!("\n"))
    };
    ($($argument:tt)*) => {
        $crate::debug::print_arguments(::core::format_args!("{}\n", ::core::format_args!($($argument)*)))
    };
}
