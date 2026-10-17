//! Link a fully-linked PPC ELF executable into an Xbox 360 XEX.
//!
//! This is the single post-link step of the toolchain: it takes the ELF that
//! rustc/clang + rust-lld produce and emits a devkit-signed XEX that Xenia (and
//! the devkit) load directly. The intermediate PE image is built in memory; no
//! separate ELF-to-EXE or EXE-to-XEX tool is required.

mod args;
mod elf;
mod error;
mod exports;
mod imports;
mod layout;
mod link;
mod manifest;
mod pe;

use std::process::ExitCode;

use crate::args::Config;
use crate::args::USAGE;
use crate::link::link;

fn main() -> ExitCode {
    let result = Config::from_args().and_then(link);
    let Err(report) = result else {
        return ExitCode::SUCCESS;
    };

    eprintln!("xex-link: {report:?}\n\n{USAGE}");

    ExitCode::FAILURE
}
