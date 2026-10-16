use rootcause::IntoReport;
use std::num::ParseIntError;
use std::path::PathBuf;
use thiserror::Error;

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
    HeaderOverlap { name: String, rva: u32, headers: u32 },
    #[error("section {name:?} file data {data_len:#x} exceeds virtual size {virtual_size:#x}")]
    SectionDataOverflow { name: String, data_len: usize, virtual_size: u32 },
    #[error("thunk at {va:#010x} has non-function record {record:#010x}")]
    BadThunk { va: u32, record: u32 },
    #[error("data import slot at {va:#010x} has invalid record {record:#010x}")]
    BadVariable { va: u32, record: u32 },
    #[error("record section {name:?} is {size:#x} bytes, not a multiple of the {record_size}-byte record")]
    PartialRecord { name: String, size: usize, record_size: usize },
    #[error("import slot at {va:#010x} is outside the image")]
    ImportSlotOutsideImage { va: u32 },
    #[error("export table needs a 64 KB aligned image base, got {base:#010x}")]
    UnalignedExportBase { base: u32 },
    #[error("export version {version} is below the export minimum version {minimum}")]
    ExportVersionBelowMinimum { version: xenon_types::Version, minimum: xenon_types::Version },
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

/// 1-based position of a value in a manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    /// Position of byte `offset` in `text`.
    pub fn at(text: &str, offset: usize) -> Self {
        let before = &text[..offset];
        let line_start = before.rfind('\n').map_or(0, |newline| newline + 1);
        Self { line: before.matches('\n').count() + 1, column: before[line_start..].chars().count() + 1 }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Manifest key holding a version or a flag list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestKey {
    Version,
    BaseVersion,
    ImportsVersion,
    ImportsMinVersion,
    ExportsMinVersion,
    ImageFlags,
    ModuleFlags,
}

impl std::fmt::Display for ManifestKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Version => "version",
            Self::BaseVersion => "base_version",
            Self::ImportsVersion => "imports.version",
            Self::ImportsMinVersion => "imports.min_version",
            Self::ExportsMinVersion => "exports.min_version",
            Self::ImageFlags => "image_flags",
            Self::ModuleFlags => "module_flags",
        })
    }
}

#[derive(Debug, Error)]
#[error("manifest {}: {kind}", path.display())]
pub struct ManifestError {
    pub path: PathBuf,
    pub kind: ManifestErrorKind,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestErrorKind {
    #[error("{}{message}", location_prefix(.location))]
    Parse { location: Option<Location>, message: String },
    #[error("{location}: invalid {key} {value:?}: {reason}")]
    InvalidVersion { key: ManifestKey, value: String, location: Location, reason: VersionError },
    #[error("{location}: invalid {key} entry {name:?}; expected a flag name or 0xHEX")]
    InvalidFlag { key: ManifestKey, name: String, location: Location },
    #[error("{location}: unknown export module {name:?}; expected xboxkrnl.exe, xam.xex, or xbdm.xex")]
    UnknownExportModule { name: String, location: Location },
    #[error("{location}: invalid bounding path {value:?}; expected non-empty printable ASCII")]
    InvalidBoundingPath { value: String, location: Location },
    #[error("{location}: load address {address:#010x} is not a multiple of the 0x{align:x} page size")]
    UnalignedLoadAddress { address: u32, align: u32, location: Location },
}

fn location_prefix(location: &Option<Location>) -> String {
    location.map(|location| format!("{location}: ")).unwrap_or_default()
}

/// Version text is dotted `major.minor.build.revision` or a packed `0xHEX`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VersionError {
    #[error("expected at most 4 dotted fields")]
    TooManyFields,
    #[error("dotted field {index}: {error}")]
    Field { index: usize, error: ParseIntError },
    #[error("major must be 0..15")]
    MajorRange,
    #[error("minor must be 0..15")]
    MinorRange,
    #[error("revision must be 0..255")]
    RevisionRange,
    #[error("packed version: {0}")]
    Packed(ParseIntError),
}

pub type Report = rootcause::Report<XexLinkError>;
pub type Result<T> = std::result::Result<T, Report>;

pub(crate) fn io<T>(path: &str, r: std::result::Result<T, std::io::Error>) -> Result<T> {
    r.map_err(|source| XexLinkError::Io { path: path.to_string(), source }.into_report())
}
