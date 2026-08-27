use rootcause::IntoReport;
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
    #[error("section {name:?} at rva {rva:#x} overlaps PE headers ({headers:#x})")]
    HeaderOverlap { name: String, rva: u32, headers: u32 },
    #[error("thunk at {va:#010x} has non-function record {record:#010x}")]
    BadThunk { va: u32, record: u32 },
    #[error("thunk references unknown import library index {0}")]
    UnknownLibrary(u8),
    #[error("building the XEX failed: {0}")]
    Xex(String),
    #[error("usage: xex-link <input.elf> <output.xex> [--title-id HEX]")]
    Usage,
}

pub type Report = rootcause::Report<XexLinkError>;
pub type Result<T> = std::result::Result<T, Report>;

pub(crate) fn io<T>(
    path: &str,
    r: std::result::Result<T, std::io::Error>,
) -> Result<T> {
    r.map_err(|source| {
        XexLinkError::Io {
            path: path.to_string(),
            source,
        }
        .into_report()
    })
}
