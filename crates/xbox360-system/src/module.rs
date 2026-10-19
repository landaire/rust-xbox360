//! Loaded modules, exports by ordinal, and loading and unloading images.

use core::ffi::CStr;
use core::marker::PhantomData;
use core::num::NonZeroU32;
use core::sync::atomic::Ordering;

use xbox360_sys::xboxkrnl;

use xbox360_status::KernelError;
use xbox360_status::NtStatus;

pub use xbox360_sys::Ordinal;
pub use xbox360_sys::xboxkrnl::ImageFlags;
pub use xbox360_sys::xboxkrnl::ImageVersion;

/// `STATUS_DLL_NOT_FOUND`, for a lookup that reports success without a
/// handle.
const DLL_NOT_FOUND: KernelError = match KernelError::from_status(NtStatus(0xc000_0135)) {
    Some(error) => error,
    None => panic!("STATUS_DLL_NOT_FOUND is a failure status"),
};

/// Loader handle of a module that stays loaded for `'a`: its data table
/// entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleHandle<'a> {
    handle: NonZeroU32,
    _module: PhantomData<&'a ()>,
}

impl<'a> ModuleHandle<'a> {
    /// The handle as the loader exports take it.
    pub const fn raw(self) -> u32 {
        self.handle.get()
    }

    /// The loader's data table entry of the module.
    fn entry(self) -> &'a xboxkrnl::LdrDataTableEntry {
        // SAFETY: the handle is the address of the data table entry of a
        // module that stays loaded for `'a`.
        unsafe {
            &*core::ptr::with_exposed_provenance::<xboxkrnl::LdrDataTableEntry>(
                self.handle.get() as usize
            )
        }
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

/// Address of the export `ordinal` of `module`. An address equal to the
/// image base (`dll_base`) of the module counts as absent.
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

    let absent = module.entry().dll_base;
    if address == absent {
        Err(KernelError::EntryPointNotFound)
    } else {
        Ok(ExportAddress(address))
    }
}

/// Handle of the running executable, which stays loaded while the title runs.
///
/// # Panics
///
/// If the loader did not resolve `XexExecutableModuleHandle`, or it holds no
/// handle.
pub fn executable_module() -> ModuleHandle<'static> {
    let Some(handle) = xboxkrnl::XexExecutableModuleHandle.get() else {
        xbox360_status::unresolved_import("XexExecutableModuleHandle");
    };

    let Some(handle) = NonZeroU32::new(handle.load(Ordering::Relaxed)) else {
        panic!("XexExecutableModuleHandle holds no handle");
    };

    ModuleHandle {
        handle,
        _module: PhantomData,
    }
}

/// The bytes of the resource `name` of `module`, which the image holds where
/// the resource header of its XEX places them. `None` when the module has no
/// resource of that name. Names compare byte for byte, so case matters, and a
/// name longer than 8 bytes never matches. A resource that the header places
/// outside the image fails with `KernelError::InvalidParameter`, because the
/// kernel returns the stored address and size unchecked.
pub fn resource<'a>(
    module: ModuleHandle<'a>,
    name: &CStr,
) -> Result<Option<&'a [u8]>, KernelError> {
    let mut address = 0u32;
    let mut size = 0u32;

    // SAFETY: the handle names a module loaded for its lifetime, `name` is
    // NUL-terminated, and `address` and `size` are live local storage.
    let status = unsafe {
        xboxkrnl::XexGetModuleSection(module.raw(), name.as_ptr().cast(), &mut address, &mut size)
    };
    match status.result() {
        Ok(()) => {}
        Err(KernelError::NotFound) => return Ok(None),
        Err(error) => return Err(error),
    }

    // An empty resource may store any address, including null.
    if size == 0 {
        return Ok(Some(&[]));
    }

    let entry = module.entry();
    let image_start = u64::from(entry.dll_base);
    let image_end = image_start + u64::from(entry.image_size);
    let start = u64::from(address);
    if start < image_start || start + u64::from(size) > image_end {
        return Err(KernelError::InvalidParameter);
    }

    // SAFETY: the bytes lie inside the module's image, checked above, which
    // stays mapped while the module is loaded for `'a`, and no image is
    // larger than `isize::MAX` bytes.
    let bytes = unsafe {
        core::slice::from_raw_parts(
            core::ptr::with_exposed_provenance::<u8>(address as usize),
            size as usize,
        )
    };

    Ok(Some(bytes))
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
