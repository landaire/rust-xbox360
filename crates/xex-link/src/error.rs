use rootcause::IntoReport;
use thiserror::Error;

use crate::manifest::ManifestError;

pub type Report = rootcause::Report<XexLinkError>;
pub type Result<T, E = Report> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum XexLinkError {
    #[error("I/O error on {path}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse ELF")]
    Elf(#[source] object::read::Error),
    #[error("no loadable sections in ELF")]
    NoSections,
    #[error("ELF entry {entry:#010x} is below image base {base:#010x}")]
    EntryBelowBase { entry: u32, base: u32 },
    #[error("section {name:?} address {addr:#010x} is below image base {base:#010x}")]
    SectionBelowBase { name: String, addr: u32, base: u32 },
    #[error("section {name:?} at rva {rva:#x} overlaps PE headers ({headers:#x})")]
    HeaderOverlap {
        name: String,
        rva: u32,
        headers: u32,
    },
    #[error("section {name:?} file data {data_len:#x} exceeds virtual size {virtual_size:#x}")]
    SectionDataOverflow {
        name: String,
        data_len: usize,
        virtual_size: u32,
    },
    #[error("thunk at {va:#010x} has non-function record {record:#010x}")]
    BadThunk { va: u32, record: u32 },
    #[error("data import slot at {va:#010x} has invalid record {record:#010x}")]
    BadVariable { va: u32, record: u32 },
    #[error(
        "record section {name:?} is {size:#x} bytes, not a multiple of the {record_size}-byte record"
    )]
    PartialRecord {
        name: String,
        size: usize,
        record_size: usize,
    },
    #[error("import slot at {va:#010x} is outside the image")]
    ImportSlotOutsideImage { va: u32 },
    #[error("export table needs a 64 KB aligned image base, got {base:#010x}")]
    UnalignedExportBase { base: u32 },
    #[error("export version {version} is below the export minimum version {minimum}")]
    ExportVersionBelowMinimum {
        version: xenon_types::Version,
        minimum: xenon_types::Version,
    },
    #[error("export section length {len:#x} is not a whole number of 8-byte records")]
    BadExportSection { len: usize },
    #[error("export of {va:#010x} has invalid ordinal {ordinal}")]
    BadExportOrdinal { ordinal: u32, va: u32 },
    #[error("export ordinal {ordinal} address {va:#010x} is outside the image")]
    BadExportAddress { ordinal: u16, va: u32 },
    #[error("export ordinal {ordinal} is declared more than once")]
    DuplicateExport { ordinal: u16 },
    #[error("{count} export(s) declared but the manifest has no [exports] module")]
    ExportsWithoutModule { count: usize },
    #[error("thunk references unknown import library index {0}")]
    UnknownLibrary(u8),
    #[error("building the XEX failed: {0}")]
    Xex(String),
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error("missing value for {0}")]
    MissingValue(String),
    #[error("unexpected argument {0:?}")]
    UnexpectedArg(String),
    #[error("missing required input and output paths")]
    MissingPaths,
}

pub(crate) fn io<T>(path: &str, result: std::result::Result<T, std::io::Error>) -> Result<T> {
    result.map_err(|source| {
        XexLinkError::Io {
            path: path.to_string(),
            source,
        }
        .into_report()
    })
}
