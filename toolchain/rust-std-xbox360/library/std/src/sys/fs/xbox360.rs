//! Filesystem for the Xbox 360 devkit, over the kernel NT file API.
//!
//! Paths are passed straight through as ANSI object names, so callers use Xbox
//! device paths, e.g. `\Device\Harddisk0\Partition1\DEVKIT\file.txt`. Files are
//! opened for synchronous IO, so the kernel maintains the file position and
//! read/write/seek need no interior mutability. Directory enumeration and the
//! metadata-mutating operations are not implemented.

use crate::ffi::OsString;
use crate::fmt;
use crate::fs::TryLockError;
use crate::hash::Hash;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
pub use crate::sys::fs::common::Dir;
use crate::sync::atomic::{AtomicU32, Ordering};
use crate::sys::time::{SystemTime, UNIX_EPOCH};
use crate::sys::unsupported;

use xbox360_sys::status::KernelError;
use xbox360_sys::xboxkrnl::{
    self, AccessMask, AnsiString, CreateDisposition, CreateOptions, FileAttributes,
    FileInformationClass, Handle, IoStatusBlock, NtStatus, ObjectAttributeFlags, ObjectAttributes,
    ShareAccess,
};

// LARGE_INTEGER sentinel telling NtRead/WriteFile to use & advance the file pointer.
#[allow(dead_code)]
const USE_FILE_POINTER: i64 = -2;

fn nt_err(status: NtStatus) -> io::Error {
    io::Error::from_raw_os_error(status.0 as i32)
}

pub struct File {
    handle: Handle,
    pos: AtomicU32,
}


unsafe impl Send for File {}
unsafe impl Sync for File {}

#[derive(Clone)]
pub struct FileAttr {
    size: u64,
    is_dir: bool,
}

pub struct ReadDir(!);
pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FileTimes {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    readonly: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FileType {
    is_dir: bool,
}

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn perm(&self) -> FilePermissions {
        FilePermissions { readonly: false }
    }
    pub fn file_type(&self) -> FileType {
        FileType { is_dir: self.is_dir }
    }
    pub fn modified(&self) -> io::Result<SystemTime> {
        Ok(UNIX_EPOCH)
    }
    pub fn accessed(&self) -> io::Result<SystemTime> {
        Ok(UNIX_EPOCH)
    }
    pub fn created(&self) -> io::Result<SystemTime> {
        Ok(UNIX_EPOCH)
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.readonly
    }
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
    }
}

impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;
    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }
    pub fn file_name(&self) -> OsString {
        self.0
    }
    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0
    }
    pub fn file_type(&self) -> io::Result<FileType> {
        self.0
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }
    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

    fn access(&self) -> AccessMask {
        let mut a = AccessMask::SYNCHRONIZE;
        if self.read {
            a |= AccessMask::GENERIC_READ | AccessMask::READ_DATA | AccessMask::READ_ATTRIBUTES;
        }
        if self.write {
            a |= AccessMask::GENERIC_WRITE;
        }
        if self.append {
            a |= AccessMask::APPEND_DATA;
        }
        a
    }

    fn disposition(&self) -> CreateDisposition {
        if self.create_new {
            CreateDisposition::Create
        } else if self.create && self.truncate {
            CreateDisposition::OverwriteIf
        } else if self.create {
            CreateDisposition::OpenIf
        } else if self.truncate {
            CreateDisposition::Overwrite
        } else {
            CreateDisposition::Open
        }
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let bytes = path.as_os_str().as_encoded_bytes();
        let Some(name) = AnsiString::new(bytes) else {
            return Err(io::const_error!(io::ErrorKind::InvalidFilename, "path is too long"));
        };
        let oa = ObjectAttributes {
            root_directory: Handle::NULL,
            object_name: &name,
            attributes: ObjectAttributeFlags::CASE_INSENSITIVE,
        };
        let mut iosb = IoStatusBlock::default();
        let mut handle = Handle::NULL;
        // SAFETY: every pointer refers to a live local or to `name`, which outlives the call.
        let status = unsafe {
            xboxkrnl::NtCreateFile(
                &mut handle,
                opts.access(),
                &oa,
                &mut iosb,
                crate::ptr::null(),
                FileAttributes::NORMAL,
                ShareAccess::READ,
                opts.disposition(),
                CreateOptions::SYNCHRONOUS_IO_NONALERT | CreateOptions::NON_DIRECTORY_FILE,
            )
        };
        if status != NtStatus::SUCCESS {
            return Err(nt_err(status));
        }
        let f = File { handle, pos: AtomicU32::new(0) };
        if opts.append {
            let end = f.file_attr().map(|a| a.size as u32).unwrap_or(0);
            f.pos.store(end, Ordering::Relaxed);
        }
        Ok(f)
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        // FILE_STANDARD_INFORMATION: AllocationSize(i64), EndOfFile(i64) at +8, ...
        let mut buf = [0u8; 24];
        let mut iosb = IoStatusBlock::default();
        // SAFETY: `self.handle` is an open file handle; `buf` and `iosb` are live locals
        // and `buf.len()` is the size of `buf`.
        let status = unsafe {
            xboxkrnl::NtQueryInformationFile(
                self.handle,
                &mut iosb,
                buf.as_mut_ptr(),
                buf.len() as u32,
                FileInformationClass::Standard,
            )
        };
        if status != NtStatus::SUCCESS {
            return Err(nt_err(status));
        }
        let end_of_file = u64::from_be_bytes(buf[8..16].try_into().unwrap());
        let is_dir = buf[21] != 0; // Directory flag
        Ok(FileAttr { size: end_of_file, is_dir })
    }

    pub fn fsync(&self) -> io::Result<()> {
        let mut iosb = IoStatusBlock::default();
        // SAFETY: `self.handle` is an open file handle and `iosb` is a live local.
        let status = unsafe { xboxkrnl::NtFlushBuffersFile(self.handle, &mut iosb) };
        if status != NtStatus::SUCCESS { Err(nt_err(status)) } else { Ok(()) }
    }
    pub fn datasync(&self) -> io::Result<()> {
        self.fsync()
    }
    pub fn lock(&self) -> io::Result<()> {
        Ok(())
    }
    pub fn lock_shared(&self) -> io::Result<()> {
        Ok(())
    }
    pub fn try_lock(&self) -> Result<(), TryLockError> {
        Ok(())
    }
    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        Ok(())
    }
    pub fn unlock(&self) -> io::Result<()> {
        Ok(())
    }
    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut iosb = IoStatusBlock::default();
        let off = u64::from(self.pos.load(Ordering::Relaxed));
        // SAFETY: `self.handle` is an open synchronous file handle; `buf`, `iosb` and
        // `off` stay live until the call completes.
        let status = unsafe {
            xboxkrnl::NtReadFile(
                self.handle,
                Handle::NULL,
                0,
                0,
                &mut iosb,
                buf.as_mut_ptr(),
                buf.len() as u32,
                &off,
            )
        };
        if status == KernelError::EndOfFile.status() {
            return Ok(0);
        }
        if status != NtStatus::SUCCESS {
            return Err(nt_err(status));
        }
        let n = iosb.information;
        self.pos.fetch_add(n, Ordering::Relaxed);
        Ok(n as usize)
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        let mut tmp = [0u8; 4096];
        let want = cursor.capacity().min(tmp.len());
        if want == 0 {
            return Ok(());
        }
        let n = self.read(&mut tmp[..want])?;
        cursor.append(&tmp[..n]);
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match bufs.iter_mut().find(|b| !b.is_empty()) {
            Some(b) => self.read(b),
            None => Ok(0),
        }
    }
    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut iosb = IoStatusBlock::default();
        let off = u64::from(self.pos.load(Ordering::Relaxed));
        // SAFETY: `self.handle` is an open synchronous file handle; `buf`, `iosb` and
        // `off` stay live until the call completes.
        let status = unsafe {
            xboxkrnl::NtWriteFile(
                self.handle,
                Handle::NULL,
                0,
                0,
                &mut iosb,
                buf.as_ptr(),
                buf.len() as u32,
                &off,
            )
        };
        if status != NtStatus::SUCCESS {
            return Err(nt_err(status));
        }
        let n = iosb.information;
        self.pos.fetch_add(n, Ordering::Relaxed);
        Ok(n as usize)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match bufs.iter().find(|b| !b.is_empty()) {
            Some(b) => self.write(b),
            None => Ok(0),
        }
    }
    pub fn is_write_vectored(&self) -> bool {
        false
    }
    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let target: i64 = match pos {
            SeekFrom::Start(n) => n as i64,
            SeekFrom::End(n) => self.file_attr()?.size as i64 + n,
            SeekFrom::Current(n) => self.pos.load(Ordering::Relaxed) as i64 + n,
        };
        if target < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "negative seek"));
        }
        self.pos.store(target as u32, Ordering::Relaxed);
        Ok(target as u64)
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        Some(self.file_attr().map(|a| a.size))
    }

    pub fn tell(&self) -> io::Result<u64> {
        Ok(self.pos.load(Ordering::Relaxed) as u64)
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }
    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        Ok(())
    }
    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        unsupported()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        // SAFETY: `self.handle` is owned by this `File` and is closed only here.
        unsafe { xboxkrnl::NtClose(self.handle) };
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }
    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "File(handle={})", self.handle.raw())
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}
pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}
pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}
pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    unsupported()
}
pub fn set_times(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}
pub fn set_times_nofollow(_p: &Path, _times: FileTimes) -> io::Result<()> {
    unsupported()
}
pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}
pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}
pub fn exists(path: &Path) -> io::Result<bool> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    match File::open(path, &opts) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}
pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}
pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}
pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    File::open(p, &opts)?.file_attr()
}
pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
}
pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    crate::sys::fs::common::copy(from, to)
}
