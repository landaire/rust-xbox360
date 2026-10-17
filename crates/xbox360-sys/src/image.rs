//! PE structures of loaded images and XEX header keys.
//!
//! The PE headers of an image are little-endian. Their fields are
//! [`Le16`] and [`Le32`] words, which keep the stored bytes and convert on
//! read.

/// A little-endian 16-bit field.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Le16(u16);

impl Le16 {
    pub const fn get(self) -> u16 {
        u16::from_le(self.0)
    }
}

/// A little-endian 32-bit field.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Le32(u32);

impl Le32 {
    pub const fn get(self) -> u32 {
        u32::from_le(self.0)
    }

    /// The bytes as stored.
    pub const fn bytes(self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }
}

value_enum! {
    /// Index of a PE data directory (`IMAGE_DIRECTORY_ENTRY_*`).
    #[non_exhaustive]
    pub enum ImageDirectory: u32 {
        /// The function table (`.pdata`).
        Exception = 3,
        Debug = 6,
    }
}

flags! {
    /// `IMAGE_FILE_*` characteristics of the PE file header.
    pub struct ImageCharacteristics(u16) {
        DLL = 0x2000,
    }
}

/// `IMAGE_FILE_HEADER`.
#[repr(C)]
pub struct ImageFileHeader {
    _machine: Le16,
    pub number_of_sections: Le16,
    pub time_date_stamp: Le32,
    _symbols: [Le32; 2],
    pub size_of_optional_header: Le16,
    characteristics: Le16,
}

impl ImageFileHeader {
    pub const fn characteristics(&self) -> ImageCharacteristics {
        ImageCharacteristics::from_bits_retain(self.characteristics.get())
    }
}

/// The leading fields of `IMAGE_OPTIONAL_HEADER` up to the checksum.
#[repr(C)]
pub struct ImageOptionalHeader {
    _reserved: [Le32; 14],
    pub size_of_image: Le32,
    _size_of_headers: Le32,
    pub check_sum: Le32,
}

/// `IMAGE_NT_HEADERS`, as `RtlImageNtHeader` returns it. The section
/// table follows the optional header.
#[repr(C)]
pub struct ImageNtHeaders {
    _signature: Le32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader,
}

impl ImageNtHeaders {
    /// The section table.
    ///
    /// # Safety
    ///
    /// `self` must be the NT headers of a mapped image whose section table
    /// is mapped and stays mapped for the returned lifetime. The table must
    /// be aligned for [`ImageSectionHeader`] (4 bytes), which holds when the
    /// headers are and `size_of_optional_header` is a multiple of 4, as the
    /// linker emits them.
    pub unsafe fn sections(&self) -> &[ImageSectionHeader] {
        let table = (&raw const self.optional_header)
            .cast::<u8>()
            .wrapping_add(usize::from(self.file_header.size_of_optional_header.get()));
        // SAFETY: guaranteed by the caller.
        unsafe {
            core::slice::from_raw_parts(
                table.cast(),
                usize::from(self.file_header.number_of_sections.get()),
            )
        }
    }
}

/// `IMAGE_SECTION_HEADER`.
#[repr(C)]
pub struct ImageSectionHeader {
    /// The name, padded with NULs. A name of eight bytes has no NUL.
    pub name: [u8; 8],
    pub virtual_size: Le32,
    /// Offset of the section from the image base.
    pub virtual_address: Le32,
    _file: [Le32; 4],
    _counts: [Le16; 2],
    pub characteristics: Le32,
}

/// `IMAGE_DEBUG_DIRECTORY`: one entry of the debug data directory.
#[repr(C)]
pub struct ImageDebugDirectory {
    _reserved: [Le32; 3],
    debug_type: Le32,
    _size_of_data: Le32,
    /// Offset of the debug data from the image base.
    pub address_of_raw_data: Le32,
    _pointer_to_raw_data: Le32,
}

impl ImageDebugDirectory {
    pub const fn debug_type(&self) -> DebugType {
        DebugType(self.debug_type.get())
    }
}

/// `IMAGE_DEBUG_TYPE_*` of a debug directory entry.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugType(pub u32);

impl DebugType {
    pub const CODEVIEW: Self = Self(2);
}

/// A CodeView `RSDS` record: the PDB signature, age and path that the
/// [`DebugType::CODEVIEW`] entry of the debug directory addresses.
#[repr(C)]
pub struct CodeViewRsds {
    pub signature: [u8; 4],
    pub guid: [u8; 16],
    pub age: Le32,
    /// The start of the NUL-terminated PDB path.
    pub path: [u8; 0],
}

impl CodeViewRsds {
    pub const SIGNATURE: [u8; 4] = *b"RSDS";
}

/// Key of a XEX optional header, as `RtlImageXexHeaderField` takes it.
/// The low byte gives the form of the data: 0 is a value stored in the
/// key entry, 1 one word, 0xff data that starts with its size in bytes,
/// and any other value that many words.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XexHeaderKey(pub u32);

impl XexHeaderKey {
    /// The entry point, stored in the key entry.
    pub const ENTRY_POINT: Self = Self(0x1_0100);
    pub const ENABLED_FOR_CALLCAP: Self = Self(0x1_8102);
    pub const ENABLED_FOR_FASTCAP: Self = Self(0x1_8200);
    /// The linked static libraries: 16-byte entries `{name[8], major,
    /// minor, build, qfe}` after the size word.
    pub const STATIC_LIBRARIES: Self = Self(0x2_00ff);
    pub const EXECUTION_INFO: Self = Self(0x4_0006);
    pub const ADDITIONAL_TITLE_MEMORY: Self = Self(0x4_0801);
}

/// Data of the [`XexHeaderKey::EXECUTION_INFO`] header (Xenia's
/// `xex2_opt_execution_info`). XEX headers are big-endian, so the fields
/// are native words.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XexExecutionInfo {
    pub media_id: u32,
    pub version: u32,
    pub base_version: u32,
    pub title_id: u32,
    pub platform: u8,
    pub executable_type: u8,
    pub disc_number: u8,
    pub disc_count: u8,
    pub savegame_id: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(XexExecutionInfo, title_id) == 0xc);
    assert!(core::mem::offset_of!(XexExecutionInfo, savegame_id) == 0x14);
    assert!(core::mem::size_of::<XexExecutionInfo>() == 0x18);
};

const _: () = {
    assert!(core::mem::size_of::<ImageFileHeader>() == 0x14);
    assert!(core::mem::offset_of!(ImageNtHeaders, file_header) == 4);
    assert!(core::mem::offset_of!(ImageNtHeaders, optional_header) == 0x18);
    assert!(core::mem::offset_of!(ImageFileHeader, number_of_sections) == 2);
    assert!(core::mem::offset_of!(ImageFileHeader, time_date_stamp) == 4);
    assert!(core::mem::offset_of!(ImageFileHeader, size_of_optional_header) == 0x10);
    assert!(core::mem::offset_of!(ImageFileHeader, characteristics) == 0x12);
    assert!(core::mem::offset_of!(ImageOptionalHeader, size_of_image) == 0x38);
    assert!(core::mem::offset_of!(ImageOptionalHeader, check_sum) == 0x40);
    assert!(core::mem::size_of::<ImageSectionHeader>() == 0x28);
    assert!(core::mem::offset_of!(ImageSectionHeader, virtual_size) == 8);
    assert!(core::mem::offset_of!(ImageSectionHeader, virtual_address) == 0xc);
    assert!(core::mem::offset_of!(ImageSectionHeader, characteristics) == 0x24);
    assert!(core::mem::size_of::<ImageDebugDirectory>() == 0x1c);
    assert!(core::mem::offset_of!(ImageDebugDirectory, debug_type) == 0xc);
    assert!(core::mem::offset_of!(ImageDebugDirectory, address_of_raw_data) == 0x14);
    assert!(core::mem::offset_of!(CodeViewRsds, age) == 0x14);
    assert!(core::mem::offset_of!(CodeViewRsds, path) == 0x18);
};
