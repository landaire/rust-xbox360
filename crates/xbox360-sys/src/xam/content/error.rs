use super::DISPLAY_NAME_UNITS;
use super::FILE_NAME_BYTES;

/// Why [`ContentData::new`](super::ContentData::new) rejected a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentDataError {
    /// The display name needs more than 127 UTF-16 units.
    DisplayNameTooLong { units: usize },
    /// The file name is empty or longer than [`FILE_NAME_BYTES`].
    FileNameLength { bytes: usize },
    /// The file name contains a NUL byte.
    FileNameNul,
}

impl core::fmt::Display for ContentDataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DisplayNameTooLong { units } => {
                write!(
                    f,
                    "display name of {units} UTF-16 units exceeds {}",
                    DISPLAY_NAME_UNITS - 1
                )
            }
            Self::FileNameLength { bytes } => write!(
                f,
                "file name of {bytes} bytes is not 1 to {FILE_NAME_BYTES}"
            ),
            Self::FileNameNul => f.write_str("file name contains NUL"),
        }
    }
}

impl core::error::Error for ContentDataError {}
