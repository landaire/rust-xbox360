//! Loaded modules, exports by ordinal, and loading and unloading images.

use core::ffi::CStr;
use core::marker::PhantomData;
use core::num::NonZeroU16;
use core::num::NonZeroU32;

use xbox360_sys::xboxkrnl;

use crate::status::KernelError;
use crate::status::NtStatus;

/// Loader handle of a module that stays loaded for `'a`: its data table
/// entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleHandle<'a> {
    handle: NonZeroU32,
    _module: PhantomData<&'a ()>,
}

impl ModuleHandle<'_> {
    /// The handle as the loader exports take it.
    pub const fn raw(self) -> u32 {
        self.handle.get()
    }
}

/// An export ordinal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ordinal(NonZeroU16);

impl Ordinal {
    /// `None` for 0, which no export has.
    pub const fn new(ordinal: u16) -> Option<Self> {
        match NonZeroU16::new(ordinal) {
            Some(ordinal) => Some(Self(ordinal)),
            None => None,
        }
    }

    pub const fn get(self) -> u16 {
        self.0.get()
    }
}

/// Address of an export: a function entry or a data item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExportAddress(u32);

impl ExportAddress {
    pub const fn raw(self) -> u32 {
        self.0
    }

    pub fn as_ptr<T>(self) -> *const T {
        self.0 as usize as *const T
    }
}

/// A module that the system loads before any title and never unloads.
///
/// The kernel has further system modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SystemModule {
    /// `xboxkrnl.exe`
    Kernel,
    /// `xam.xex`
    Xam,
}

impl SystemModule {
    pub const fn name(self) -> &'static CStr {
        match self {
            Self::Kernel => c"xboxkrnl.exe",
            Self::Xam => c"xam.xex",
        }
    }

    pub fn handle(self) -> Result<ModuleHandle<'static>, KernelError> {
        // SAFETY: system modules are never unloaded.
        unsafe { module_handle(self.name()) }
    }
}

/// `STATUS_DLL_NOT_FOUND`, for a lookup that reports success without a
/// handle.
const DLL_NOT_FOUND: KernelError = match KernelError::from_status(NtStatus(0xc000_0135)) {
    Some(error) => error,
    None => panic!("STATUS_DLL_NOT_FOUND is a failure status"),
};

/// Handle of the loaded module named `name`.
///
/// # Safety
///
/// The module must stay loaded for as long as the handle is used. Use
/// [`SystemModule::handle`] for modules that are never unloaded, and
/// [`LoadedImage::handle`] for modules this title loads.
pub unsafe fn module_handle(name: &CStr) -> Result<ModuleHandle<'static>, KernelError> {
    let mut handle = 0u32;
    // SAFETY: `name` is NUL-terminated and `handle` is live local storage.
    unsafe { xboxkrnl::XexGetModuleHandle(name.as_ptr().cast(), &mut handle) }.result()?;
    // Xenia reports a missing module with a Win32 code, which reads as a
    // success status, and a zero handle.
    let handle = NonZeroU32::new(handle).ok_or(DLL_NOT_FOUND)?;
    Ok(ModuleHandle {
        handle,
        _module: PhantomData,
    })
}

/// Byte offset in the loader data table entry of the word that stock XBDM
/// compares a resolved address with (`dll_base` in Xenia's layout).
const LDR_ENTRY_ABSENT_ADDRESS: usize = 0x18;

/// Address of the export `ordinal` of `module`. As stock XBDM does, an
/// address equal to the word at handle +0x18 counts as absent.
pub fn resolve_export(
    module: ModuleHandle<'_>,
    ordinal: Ordinal,
) -> Result<ExportAddress, KernelError> {
    let mut address = 0u32;
    // SAFETY: the handle names a module loaded for its lifetime and
    // `address` is live local storage.
    unsafe {
        xboxkrnl::XexGetProcedureAddress(module.raw(), u32::from(ordinal.get()), &mut address)
    }
    .result()?;
    // SAFETY: the handle points at the loader's data table entry of a
    // module that is loaded for the handle's lifetime.
    let absent = unsafe {
        core::ptr::read_volatile((module.raw() as usize + LDR_ENTRY_ABSENT_ADDRESS) as *const u32)
    };
    if address == absent {
        Err(KernelError::EntryPointNotFound)
    } else {
        Ok(ExportAddress(address))
    }
}

pub use xbox360_sys::xboxkrnl::ImageFlags;
pub use xbox360_sys::xboxkrnl::ImageVersion;

/// A module that [`load_image`] loaded. Unloads it on drop.
#[derive(Debug)]
pub struct LoadedImage(NonZeroU32);

impl LoadedImage {
    pub fn handle(&self) -> ModuleHandle<'_> {
        ModuleHandle {
            handle: self.0,
            _module: PhantomData,
        }
    }

    /// Unload the module and report the result.
    pub fn unload(self) -> Result<(), KernelError> {
        let handle = self.0;
        core::mem::forget(self);
        // SAFETY: the handle holds one load reference, which `forget` keeps
        // Drop from releasing again.
        unsafe { xboxkrnl::XexUnloadImage(handle.get()) }.result()
    }
}

impl Drop for LoadedImage {
    fn drop(&mut self) {
        // SAFETY: the handle holds one load reference, released only here.
        unsafe { xboxkrnl::XexUnloadImage(self.0.get()) };
    }
}

/// Load the image at `path`, or add a reference to it if it is loaded.
/// `minimum_version` rejects older images; `None` accepts any version.
///
/// # Safety
///
/// Loading an image runs its entry point and startup code, such as TLS
/// callbacks and static initializers, which may do anything. The caller
/// must trust the image.
pub unsafe fn load_image(
    path: &CStr,
    flags: ImageFlags,
    minimum_version: Option<ImageVersion>,
) -> Result<LoadedImage, KernelError> {
    let version = minimum_version.unwrap_or(ImageVersion::ANY);
    let mut handle = 0u32;
    // SAFETY: `path` is NUL-terminated, `handle` is live local storage, and
    // the caller trusts the image's startup code.
    unsafe { xboxkrnl::XexLoadImage(path.as_ptr().cast(), flags, version, &mut handle) }
        .result()?;
    let handle = NonZeroU32::new(handle).ok_or(DLL_NOT_FOUND)?;
    Ok(LoadedImage(handle))
}
