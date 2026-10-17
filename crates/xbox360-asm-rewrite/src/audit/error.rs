use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("reading the scope file")]
    ReadScope(#[source] io::Error),
    #[error("scope line {line} is not `name<TAB>inline-asm|plain`")]
    MalformedScope { line: usize },
    #[error("reading the disassembly")]
    ReadDisassembly(#[source] io::Error),
}
