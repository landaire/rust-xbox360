//! Errors of the command-line front end in `main.rs`.

use std::io;
use std::path::PathBuf;

use thiserror::Error;
use xbox360_asm_rewrite::audit;
use xbox360_asm_rewrite::reservation;
use xbox360_asm_rewrite::rewrite;

#[derive(Debug, Error)]
pub enum Error {
    #[error("opening {path}")]
    Open {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    Rewrite(#[from] rewrite::Error),
    #[error(transparent)]
    Audit(#[from] audit::Error),
    #[error(transparent)]
    Image(#[from] reservation::ElfError),
    #[error(
        "{count} unprotected reservations; LLVM emits them for `core::sync::atomic` read-modify-write operations unless the standard library carries the xbox360 atomics overlay (rust-xbox360 toolchain/rust-std-xbox360)"
    )]
    Unprotected { count: usize },
    #[error("{count} functions still hold rewritable sequences")]
    Rewritable { count: usize },
}
