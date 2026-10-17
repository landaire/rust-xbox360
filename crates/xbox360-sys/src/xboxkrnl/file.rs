//! Files and file systems.

use super::device::FileDeviceType;
use super::handle::Handle;
use super::string::AnsiString;
use crate::status::NtStatus;

flags! {
    /// `FILE_SHARE_*`: what other opens of a file may do.
    pub struct ShareAccess(u32) {
        READ = 0x1,
        WRITE = 0x2,
        DELETE = 0x4,
    }
}

flags! {
    /// `FILE_ATTRIBUTE_*`.
    pub struct FileAttributes(u32) {
        READONLY = 0x1,
        HIDDEN = 0x2,
        SYSTEM = 0x4,
        DIRECTORY = 0x10,
        ARCHIVE = 0x20,
        DEVICE = 0x40,
        NORMAL = 0x80,
        TEMPORARY = 0x100,
        COMPRESSED = 0x800,
        ENCRYPTED = 0x4000,
    }
}

value_enum! {
    /// `NtCreateFile` create disposition: what to do when the file exists
    /// or does not.
    pub enum CreateDisposition: u32 {
        /// Replace the file if it exists, create it otherwise.
        Supersede = 0,
        /// Open the file; fail if it does not exist.
        Open = 1,
        /// Create the file; fail if it exists.
        Create = 2,
        /// Open the file, or create it if it does not exist.
        OpenIf = 3,
        /// Open and truncate the file; fail if it does not exist.
        Overwrite = 4,
        /// Open and truncate the file, or create it if it does not exist.
        OverwriteIf = 5,
    }
}

flags! {
    /// `FILE_*` create options.
    pub struct CreateOptions(u32) {
        DIRECTORY_FILE = 0x1,
        WRITE_THROUGH = 0x2,
        SEQUENTIAL_ONLY = 0x4,
        NO_INTERMEDIATE_BUFFERING = 0x8,
        SYNCHRONOUS_IO_ALERT = 0x10,
        SYNCHRONOUS_IO_NONALERT = 0x20,
        NON_DIRECTORY_FILE = 0x40,
        RANDOM_ACCESS = 0x800,
        DELETE_ON_CLOSE = 0x1000,
        OPEN_BY_FILE_ID = 0x2000,
        OPEN_FOR_BACKUP_INTENT = 0x4000,
    }
}

value_enum! {
    /// `FILE_INFORMATION_CLASS` of `NtQueryInformationFile` and
    /// `NtSetInformationFile`, with Xenia's values.
    #[non_exhaustive]
    pub enum FileInformationClass: u32 {
        Directory = 1,
        FullDirectory = 2,
        BothDirectory = 3,
        Basic = 4,
        Standard = 5,
        Internal = 6,
        Ea = 7,
        Access = 8,
        Name = 9,
        Rename = 10,
        Link = 11,
        Names = 12,
        Disposition = 13,
        Position = 14,
        FullEa = 15,
        Mode = 16,
        Alignment = 17,
        All = 18,
        Allocation = 19,
        EndOfFile = 20,
        AlternateName = 21,
        Stream = 22,
        MountPartition = 23,
        MountPartitions = 24,
        PipeRemote = 25,
        Sector = 26,
        XctdCompression = 27,
        Compression = 28,
        ObjectId = 29,
        Completion = 30,
        MoveCluster = 31,
        IoPriority = 32,
        ReparsePoint = 33,
        NetworkOpen = 34,
        AttributeTag = 35,
        Tracking = 36,
    }
}

/// Result of `NtQueryFullAttributesFile` and of `NtQueryInformationFile`
/// with [`FileInformationClass::NetworkOpen`]. Times are FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileNetworkOpenInformation {
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub allocation_size: u64,
    /// File size in bytes.
    pub end_of_file: u64,
    pub attributes: FileAttributes,
    _pad: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileNetworkOpenInformation, end_of_file) == 0x28);
    assert!(offset_of!(FileNetworkOpenInformation, attributes) == 0x30);
    assert!(core::mem::size_of::<FileNetworkOpenInformation>() == 0x38);
};

/// `FILE_DIRECTORY_INFORMATION`: an entry of `NtQueryDirectoryFile`. The
/// name, `file_name_length` bytes, follows at 0x40. Times are FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileDirectoryInformation {
    /// Offset of the next entry, or 0 for the last one.
    pub next_entry_offset: u32,
    pub file_index: u32,
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    /// File size in bytes.
    pub end_of_file: u64,
    pub allocation_size: u64,
    pub attributes: FileAttributes,
    pub file_name_length: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileDirectoryInformation, creation_time) == 0x08);
    assert!(offset_of!(FileDirectoryInformation, end_of_file) == 0x28);
    assert!(offset_of!(FileDirectoryInformation, attributes) == 0x38);
    assert!(offset_of!(FileDirectoryInformation, file_name_length) == 0x3c);
    assert!(core::mem::size_of::<FileDirectoryInformation>() == 0x40);
};

/// `FILE_BASIC_INFORMATION` of [`FileInformationClass::Basic`]. Times are
/// FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileBasicInformation {
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub attributes: FileAttributes,
    _pad: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileBasicInformation, attributes) == 0x20);
    assert!(core::mem::size_of::<FileBasicInformation>() == 0x28);
};

/// `FILE_DISPOSITION_INFORMATION` of [`FileInformationClass::Disposition`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileDispositionInformation {
    /// Delete the file when its last handle is closed.
    pub delete_file: bool,
}

const _: () = assert!(core::mem::size_of::<FileDispositionInformation>() == 1);

/// `FILE_POSITION_INFORMATION` of [`FileInformationClass::Position`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FilePositionInformation {
    pub current_byte_offset: u64,
}

/// `FILE_END_OF_FILE_INFORMATION` of [`FileInformationClass::EndOfFile`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileEndOfFileInformation {
    pub end_of_file: u64,
}

/// `FILE_RENAME_INFORMATION` of [`FileInformationClass::Rename`].
#[repr(C)]
pub struct FileRenameInformation {
    pub replace_if_exists: bool,
    /// Directory the name is relative to, or [`Handle::NULL`].
    pub root_directory: Handle,
    pub file_name: AnsiString,
}

#[cfg(target_pointer_width = "32")]
const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileRenameInformation, root_directory) == 0x04);
    assert!(offset_of!(FileRenameInformation, file_name) == 0x08);
    assert!(core::mem::size_of::<FileRenameInformation>() == 0x10);
};

value_enum! {
    /// `FS_INFORMATION_CLASS` of `NtQueryVolumeInformationFile`, with
    /// Xenia's values.
    #[non_exhaustive]
    pub enum FsInformationClass: u32 {
        Volume = 1,
        Size = 3,
        Device = 4,
        Attribute = 5,
    }
}

/// I/O status block.
#[repr(C)]
pub struct IoStatusBlock {
    pub status: NtStatus,
    pub information: u32,
}

impl Default for IoStatusBlock {
    fn default() -> Self {
        Self {
            status: NtStatus::SUCCESS,
            information: 0,
        }
    }
}

const _: () = assert!(core::mem::size_of::<IoStatusBlock>() == 8);

/// `FILE_FS_SIZE_INFORMATION`: what [`FsInformationClass::Size`] returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FsSizeInformation {
    pub total_allocation_units: u64,
    pub available_allocation_units: u64,
    pub sectors_per_allocation_unit: u32,
    pub bytes_per_sector: u32,
}

const _: () = assert!(core::mem::size_of::<FsSizeInformation>() == 0x18);

/// `FILE_FS_DEVICE_INFORMATION`: what [`FsInformationClass::Device`]
/// returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsDeviceInformation {
    pub device_type: FileDeviceType,
    pub characteristics: u32,
}

const _: () = assert!(core::mem::size_of::<FsDeviceInformation>() == 8);

/// `FILE_FS_ATTRIBUTE_INFORMATION`: what [`FsInformationClass::Attribute`]
/// returns. The file system name, `file_system_name_length` bytes, follows
/// at 0xc.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FsAttributeInformation {
    /// `FILE_*` file system flags.
    pub attributes: u32,
    pub maximum_component_name_length: u32,
    pub file_system_name_length: u32,
}

const _: () = assert!(core::mem::size_of::<FsAttributeInformation>() == 0xc);

flags! {
    /// `FILE_*` file system flags of [`FsAttributeInformation::attributes`].
    pub struct FileSystemAttributes(u32) {
        CASE_SENSITIVE_SEARCH = 0x1,
        CASE_PRESERVED_NAMES = 0x2,
        UNICODE_ON_DISK = 0x4,
        PERSISTENT_ACLS = 0x8,
        FILE_COMPRESSION = 0x10,
        VOLUME_IS_COMPRESSED = 0x8000,
        READ_ONLY_VOLUME = 0x8_0000,
    }
}
