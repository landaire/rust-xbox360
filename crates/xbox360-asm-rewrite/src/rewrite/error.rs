use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reading the assembly input")]
    Read(#[source] io::Error),
    #[error("writing the assembly output")]
    Write(#[source] io::Error),
    #[error("function `{name}` has no `{end_label}:` label")]
    UnterminatedFunction { name: String, end_label: String },
}
