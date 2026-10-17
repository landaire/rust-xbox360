use std::num::ParseIntError;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("manifest {}: {kind}", path.display())]
pub struct ManifestError {
    pub path: PathBuf,
    pub kind: ManifestErrorKind,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestErrorKind {
    #[error("{}{message}", location_prefix(.location))]
    Parse {
        location: Option<Location>,
        message: String,
    },
    #[error("{location}: invalid {key} {value:?}: {reason}")]
    InvalidVersion {
        key: ManifestKey,
        value: String,
        location: Location,
        reason: VersionError,
    },
    #[error("{location}: invalid {key} entry {name:?}; expected a flag name or 0xHEX")]
    InvalidFlag {
        key: ManifestKey,
        name: String,
        location: Location,
    },
    #[error(
        "{location}: unknown export module {name:?}; expected xboxkrnl.exe, xam.xex, or xbdm.xex"
    )]
    UnknownExportModule { name: String, location: Location },
    #[error("{location}: invalid bounding path {value:?}; expected non-empty printable ASCII")]
    InvalidBoundingPath { value: String, location: Location },
    #[error(
        "{location}: load address {address:#010x} is not a multiple of the 0x{align:x} page size"
    )]
    UnalignedLoadAddress {
        address: u32,
        align: u32,
        location: Location,
    },
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

        Self {
            line: before.matches('\n').count() + 1,
            column: before[line_start..].chars().count() + 1,
        }
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

fn location_prefix(location: &Option<Location>) -> String {
    location
        .map(|location| format!("{location}: "))
        .unwrap_or_default()
}
