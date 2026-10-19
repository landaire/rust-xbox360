//! Listing the entries of a directory (`NtOpenFile`, `NtQueryDirectoryFile`).
//!
//! The standard library of the console has no `read_dir`. Paths are object
//! paths such as `\Device\Harddisk0\Partition1\DEVKIT`.
//!
//! ```ignore
//! let mut directory = Directory::open(br"\Device\Harddisk0\Partition1\DEVKIT")?;
//! while let Some(entries) = directory.read()? {
//!     for entry in entries {
//!         // entry.name(), entry.is_directory()
//!     }
//! }
//! ```

use xbox360_status::KernelError;
use xbox360_sys::xboxkrnl;
use xbox360_sys::xboxkrnl::AccessMask;
use xbox360_sys::xboxkrnl::AnsiString;
use xbox360_sys::xboxkrnl::CreateOptions;
use xbox360_sys::xboxkrnl::FileDirectoryInformation;
use xbox360_sys::xboxkrnl::Handle;
use xbox360_sys::xboxkrnl::IoStatusBlock;
use xbox360_sys::xboxkrnl::ObjectAttributeFlags;
use xbox360_sys::xboxkrnl::ObjectAttributes;
use xbox360_sys::xboxkrnl::ShareAccess;

pub use xbox360_sys::xboxkrnl::FileAttributes;

/// Bytes of one query's entries.
const ENTRY_BUFFER_BYTES: usize = 0x1000;

/// An open directory, closed on drop.
pub struct Directory {
    handle: Handle,
    buffer: EntryBuffer,
}

impl Directory {
    /// Opens the directory at the object path `path`.
    ///
    /// # Errors
    ///
    /// The status of `NtOpenFile`. [`KernelError::InvalidParameter`] when
    /// `path` is longer than an object name can be.
    pub fn open(path: &[u8]) -> Result<Self, KernelError> {
        let name = AnsiString::new(path).ok_or(KernelError::InvalidParameter)?;
        let attributes = ObjectAttributes {
            root_directory: Handle::NULL,
            object_name: &name,
            attributes: ObjectAttributeFlags::CASE_INSENSITIVE,
        };
        let mut io_status = IoStatusBlock::default();
        let mut handle = Handle::NULL;

        // SAFETY: `attributes`, `name` and `path` live through the call, which
        // writes the live locals `handle` and `io_status`.
        let status = unsafe {
            xboxkrnl::NtOpenFile(
                &mut handle,
                AccessMask::LIST_DIRECTORY | AccessMask::SYNCHRONIZE,
                &attributes,
                &mut io_status,
                ShareAccess::READ | ShareAccess::WRITE,
                CreateOptions::DIRECTORY_FILE | CreateOptions::SYNCHRONOUS_IO_NONALERT,
            )
        };
        status.result()?;

        Ok(Self {
            handle,
            buffer: EntryBuffer([0; ENTRY_BUFFER_BYTES]),
        })
    }

    /// The entries of the next query, in the file system's order. `None`
    /// after the last entry.
    ///
    /// # Errors
    ///
    /// The status of `NtQueryDirectoryFile`.
    pub fn read(&mut self) -> Result<Option<Entries<'_>>, KernelError> {
        let mut io_status = IoStatusBlock::default();

        // The buffer is a small constant, and pointers are 32-bit.
        let capacity = ENTRY_BUFFER_BYTES as u32;
        // SAFETY: the handle is open for synchronous I/O, so the query
        // completes before the call returns, writing at most `capacity`
        // bytes to the buffer and the live local `io_status`. A null name
        // matches every entry.
        let status = unsafe {
            xboxkrnl::NtQueryDirectoryFile(
                self.handle,
                Handle::NULL,
                0,
                0,
                &mut io_status,
                self.buffer.0.as_mut_ptr(),
                capacity,
                core::ptr::null(),
                false,
            )
        };
        match status.result() {
            Ok(()) => {}
            Err(KernelError::NoMoreFiles | KernelError::NoSuchFile) => return Ok(None),
            Err(error) => return Err(error),
        }

        Ok(Some(Entries {
            buffer: &self.buffer.0,
            next: Some(0),
        }))
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        // SAFETY: `handle` is open and not used after this.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}

/// The entries of one query.
pub struct Entries<'a> {
    buffer: &'a [u8],
    /// Offset of the next entry, `None` after the last one.
    next: Option<usize>,
}

impl<'a> Iterator for Entries<'a> {
    type Item = DirectoryEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.next.take()?;
        let header_end = offset.checked_add(size_of::<FileDirectoryInformation>())?;
        let header = self.buffer.get(offset..header_end)?;

        // SAFETY: `header` holds `size_of::<FileDirectoryInformation>()`
        // bytes, and every bit pattern is a valid value of its integer
        // fields.
        let information = unsafe {
            header
                .as_ptr()
                .cast::<FileDirectoryInformation>()
                .read_unaligned()
        };

        let name_end = header_end.checked_add(information.file_name_length as usize)?;
        let name = self.buffer.get(header_end..name_end)?;
        if information.next_entry_offset != 0 {
            self.next = offset.checked_add(information.next_entry_offset as usize);
        }

        Some(DirectoryEntry {
            name,
            attributes: information.attributes,
            size: information.end_of_file,
        })
    }
}

/// An entry of a directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectoryEntry<'a> {
    name: &'a [u8],
    attributes: FileAttributes,
    size: u64,
}

impl<'a> DirectoryEntry<'a> {
    /// The entry's name in the file system's ANSI encoding.
    pub fn name(&self) -> &'a [u8] {
        self.name
    }

    pub fn attributes(&self) -> FileAttributes {
        self.attributes
    }

    pub fn is_directory(&self) -> bool {
        self.attributes.contains(FileAttributes::DIRECTORY)
    }

    /// File size in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }
}

/// Query buffer, aligned for the 64-bit fields of the entries.
#[repr(C, align(8))]
struct EntryBuffer([u8; ENTRY_BUFFER_BYTES]);
