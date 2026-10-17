//! Loaded modules and images.

use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;

use super::field::KernelU32;
use super::field::ListEntry;
use super::string::UnicodeString;

value_enum! {
    /// Reason the loader passes to the entry point of a module.
    #[non_exhaustive]
    pub enum DllEntryReason: u32 {
        /// The module is unloaded.
        ProcessDetach = 0,
        /// The module is loaded.
        ProcessAttach = 1,
    }
}

flags! {
    /// XEX module flags of `XexLoadImage`, with the names of Xenia's
    /// `xex2_module_flags`.
    pub struct ImageFlags(u32) {
        TITLE = 0x1,
        EXPORTS_TO_TITLE = 0x2,
        SYSTEM_DEBUGGER = 0x4,
        DLL = 0x8,
        MODULE_PATCH = 0x10,
        PATCH_FULL = 0x20,
        PATCH_DELTA = 0x40,
        USER_MODE = 0x80,
    }
}

/// A packed XEX version: 4, 4, 16 and 8 bits of major, minor, build and
/// qfe from the top. As a minimum version, [`ImageVersion::ANY`] accepts
/// every image.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageVersion(u32);

impl ImageVersion {
    pub const ANY: Self = Self(0);

    /// `None` if `major` or `minor` exceeds 15.
    pub const fn new(major: u8, minor: u8, build: u16, qfe: u8) -> Option<Self> {
        if major > 0xf || minor > 0xf {
            return None;
        }

        Some(Self(
            ((major as u32) << 28) | ((minor as u32) << 24) | ((build as u32) << 8) | qfe as u32,
        ))
    }

    pub const fn from_packed(packed: u32) -> Self {
        Self(packed)
    }

    pub const fn packed(self) -> u32 {
        self.0
    }
}

/// `LDR_DATA_TABLE_ENTRY`: the loader's record of a loaded module, up to
/// the last field in use. The address of the entry is the module handle.
/// The loader changes the links and the flags under its lock.
#[repr(C)]
pub struct LdrDataTableEntry {
    /// Link in the loaded module list.
    pub in_load_order_links: ListEntry,
    _reserved_008: [AtomicU8; 0x10],
    /// Address the image is loaded at.
    pub dll_base: u32,
    _reserved_01c: [AtomicU8; 4],
    /// Size of the loaded image in bytes.
    pub image_size: u32,
    /// Path of the module.
    pub full_dll_name: UnicodeString,
    /// File name of the module.
    pub base_dll_name: UnicodeString,
    /// Loader flags of the module.
    pub flags: KernelU32,
    _reserved_038: [AtomicU8; 0x18],
    pub time_date_stamp: u32,
    _reserved_054: [AtomicU8; 4],
    /// Address of the XEX header of the image.
    pub xex_header_base: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(LdrDataTableEntry, dll_base) == 0x18);
    assert!(offset_of!(LdrDataTableEntry, image_size) == 0x20);
    assert!(offset_of!(LdrDataTableEntry, full_dll_name) == 0x24);
    assert!(offset_of!(LdrDataTableEntry, base_dll_name) == 0x2c);
    assert!(offset_of!(LdrDataTableEntry, flags) == 0x34);
    assert!(offset_of!(LdrDataTableEntry, time_date_stamp) == 0x50);
    assert!(offset_of!(LdrDataTableEntry, xex_header_base) == 0x58);
};

/// `ExLoadedImageName`: the NUL-terminated object path of the running
/// executable. The kernel rewrites it when it starts a title.
#[repr(C)]
pub struct LoadedImageName {
    _opaque: [u8; 0],
    _pinned: core::marker::PhantomPinned,
}

impl LoadedImageName {
    /// Copy the path into `buffer` up to its NUL or the end of `buffer`,
    /// and return the number of bytes copied.
    pub fn copy_to(&self, buffer: &mut [u8]) -> usize {
        let start = core::ptr::from_ref(self).expose_provenance();
        for (index, slot) in buffer.iter_mut().enumerate() {
            // SAFETY: a `LoadedImageName` exists only as the kernel export,
            // a NUL-terminated string, and the bytes up to the NUL are
            // mapped. Atomic loads allow the kernel to rewrite it.
            let byte = unsafe {
                AtomicU8::from_ptr(core::ptr::with_exposed_provenance_mut(start + index))
            }
            .load(Ordering::Relaxed);

            if byte == 0 {
                return index;
            }

            *slot = byte;
        }

        buffer.len()
    }
}
