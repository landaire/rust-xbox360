use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElfError {
    #[error("reading the image")]
    Read(#[source] io::Error),
    #[error("the image is not a 32-bit big-endian ELF file")]
    NotElf32BigEndian,
    #[error("section header entry size {size} is smaller than 40")]
    SectionHeaderSize { size: u16 },
}
