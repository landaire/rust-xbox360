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
use crate::hash::{Hash, Hasher};
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
pub use crate::sys::fs::common::Dir;
use crate::sync::atomic::{AtomicU32, Ordering};
use crate::sys::pal::abi;
use crate::sys::time::{SystemTime, UNIX_EPOCH};
use crate::sys::unsupported;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const FILE_APPEND_DATA: u32 = 0x0000_0004;
const SYNCHRONIZE: u32 = 0x0010_0000;
const FILE_SHARE_READ: u32 = 1;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x20;
const FILE_NON_DIRECTORY_FILE: u32 = 0x40;

const FILE_SUPERSEDE_OPEN: u32 = 1; // FILE_OPEN
const FILE_CREATE: u32 = 2;
const FILE_OPEN_IF: u32 = 3;
const FILE_OVERWRITE: u32 = 4;
const FILE_OVERWRITE_IF: u32 = 5;

const FILE_STANDARD_INFORMATION: u32 = 5;
const FILE_POSITION_INFORMATION: u32 = 14;

// LARGE_INTEGER sentinel telling NtRead/WriteFile to use & advance the file pointer.
#[allow(dead_code)]
const USE_FILE_POINTER: i64 = -2;

#[repr(C)]
struct AnsiString {
    length: u16,
    maximum_length: u16,
    buffer: *const u8,
}

#[repr(C)]
struct ObjectAttributes {
    root_directory: u32,
    object_name: *const AnsiString,
    attributes: u32,
}

#[repr(C)]
struct IoStatusBlock {
    status: u32,
    information: u32,
}

fn nt_err(status: u32) -> io::Error {
    io::Error::from_raw_os_error(status as i32)
}

pub struct File {
    handle: u32,
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

    fn access(&self) -> u32 {
        let mut a = SYNCHRONIZE;
        if self.read {
            a |= GENERIC_READ | 0x1 /*FILE_READ_DATA*/ | 0x80 /*FILE_READ_ATTRIBUTES*/;
        }
        if self.write {
            a |= GENERIC_WRITE;
        }
        if self.append {
            a |= FILE_APPEND_DATA;
        }
        a
    }

    fn disposition(&self) -> u32 {
        if self.create_new {
            FILE_CREATE
        } else if self.create && self.truncate {
            FILE_OVERWRITE_IF
        } else if self.create {
            FILE_OPEN_IF
        } else if self.truncate {
            FILE_OVERWRITE
        } else {
            FILE_SUPERSEDE_OPEN
        }
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let bytes = path.as_os_str().as_encoded_bytes();
        let name = AnsiString {
            length: bytes.len() as u16,
            maximum_length: bytes.len() as u16,
            buffer: bytes.as_ptr(),
        };
        let oa = ObjectAttributes {
            root_directory: 0,
            object_name: &name,
            attributes: 0x40, // OBJ_CASE_INSENSITIVE
        };
        let mut iosb = IoStatusBlock { status: 0, information: 0 };
        let mut handle: u32 = 0;
        let status = unsafe {
            abi::NtCreateFile(
                &mut handle,
                opts.access(),
                &oa as *const ObjectAttributes as *const u8,
                &mut iosb as *mut IoStatusBlock as *mut u8,
                crate::ptr::null(),
                FILE_ATTRIBUTE_NORMAL,
                FILE_SHARE_READ,
                opts.disposition(),
                FILE_SYNCHRONOUS_IO_NONALERT | FILE_NON_DIRECTORY_FILE,
            )
        };
        if status != 0 {
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
        let mut iosb = IoStatusBlock { status: 0, information: 0 };
        let status = unsafe {
            abi::NtQueryInformationFile(
                self.handle,
                &mut iosb as *mut IoStatusBlock as *mut u8,
                buf.as_mut_ptr(),
                buf.len() as u32,
                FILE_STANDARD_INFORMATION,
            )
        };
        if status != 0 {
            return Err(nt_err(status));
        }
        let end_of_file = u64::from_be_bytes(buf[8..16].try_into().unwrap());
        let is_dir = buf[21] != 0; // Directory flag
        Ok(FileAttr { size: end_of_file, is_dir })
    }

    pub fn fsync(&self) -> io::Result<()> {
        let mut iosb = IoStatusBlock { status: 0, information: 0 };
        let status =
            unsafe { abi::NtFlushBuffersFile(self.handle, &mut iosb as *mut IoStatusBlock as *mut u8) };
        if status != 0 { Err(nt_err(status)) } else { Ok(()) }
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
        // The devkit HDD wants the read length to reach EOF and DMAs into title
        // memory, so callers should pass a stack buffer at least the file size
        // (read/read_exact). read_to_end (small growing heap buffers) is not
        // supported on this device.
        let mut iosb = IoStatusBlock { status: 0, information: 0 };
        let off = self.pos.load(Ordering::Relaxed) as i64;
        let status = unsafe {
            abi::NtReadFile(
                self.handle, 0, 0, 0,
                &mut iosb as *mut IoStatusBlock as *mut u8,
                buf.as_mut_ptr(), buf.len() as u32, &off, crate::ptr::null(),
            )
        };
        if status == 0xC000_0011 {
            return Ok(0); // END_OF_FILE
        }
        if status != 0 {
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
        let mut iosb = IoStatusBlock { status: 0, information: 0 };
        let off = self.pos.load(Ordering::Relaxed) as i64;
        let status = unsafe {
            abi::NtWriteFile(
                self.handle, 0, 0, 0,
                &mut iosb as *mut IoStatusBlock as *mut u8,
                buf.as_ptr(), buf.len() as u32, &off, crate::ptr::null(),
            )
        };
        if status != 0 {
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
        unsafe { abi::NtClose(self.handle) };
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
        write!(f, "File(handle={})", self.handle)
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
