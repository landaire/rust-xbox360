//! Emit a minimal Xbox 360 PE image (the XEX base file).
//!
//! Xenia copies the base file verbatim to the load address and then reads the
//! PE headers in place, mapping each section at `image_base + VirtualAddress`
//! without relocating its bytes. So every section's file offset must equal its
//! RVA, and the headers must fit below the first section.

use std::io::Write;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use rootcause::IntoReport;

use crate::error::Report;
use crate::error::XexLinkError;

// Section characteristics.
pub const SCN_CODE: u32 = 0x0000_0020;
pub const SCN_INITIALIZED_DATA: u32 = 0x0000_0040;
pub const SCN_UNINITIALIZED_DATA: u32 = 0x0000_0080;
pub const SCN_MEM_EXECUTE: u32 = 0x2000_0000;
pub const SCN_MEM_READ: u32 = 0x4000_0000;
pub const SCN_MEM_WRITE: u32 = 0x8000_0000;

const DOS_HEADER_LEN: u32 = 0x40;
const OPT_HEADER_LEN: u32 = 0xE0; // IMAGE_SIZEOF_NT_OPTIONAL_HEADER (PE32, 16 dirs)

pub struct Section {
    pub name: [u8; 8],
    pub rva: u32,
    pub virtual_size: u32,
    /// File-backed bytes. Shorter than `virtual_size` for `.bss` (the tail is
    /// zero-filled in the image).
    pub data: Vec<u8>,
    pub characteristics: u32,
}

pub struct PeImage {
    pub image_base: u32,
    pub entry_rva: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub sections: Vec<Section>,
}

impl PeImage {
    pub fn build(&self) -> Result<Vec<u8>, Report> {
        const PE_SIG_LEN: u32 = 4;
        const COFF_HEADER_LEN: u32 = 20;
        const SECTION_HEADER_LEN: u32 = 40;

        if self.sections.is_empty() {
            return Err(XexLinkError::NoSections.into_report());
        }

        let headers_len = DOS_HEADER_LEN
            + PE_SIG_LEN
            + COFF_HEADER_LEN
            + OPT_HEADER_LEN
            + self.sections.len() as u32 * SECTION_HEADER_LEN;
        let size_of_headers = align_up(headers_len, self.file_alignment);

        for section in &self.sections {
            if section.rva < headers_len {
                return Err(XexLinkError::HeaderOverlap {
                    name: name_str(&section.name),
                    rva: section.rva,
                    headers: headers_len,
                }
                .into_report());
            }

            if section.data.len() as u32 > section.virtual_size {
                return Err(XexLinkError::SectionDataOverflow {
                    name: name_str(&section.name),
                    data_len: section.data.len(),
                    virtual_size: section.virtual_size,
                }
                .into_report());
            }
        }

        let image_size = align_up(
            self.sections
                .iter()
                .map(|section| section.rva + section.virtual_size)
                .max()
                .unwrap_or(0),
            self.section_alignment,
        );

        let mut image = vec![0u8; image_size as usize];
        for section in &self.sections {
            let start = section.rva as usize;
            image[start..start + section.data.len()].copy_from_slice(&section.data);
        }

        let header = self.build_headers(image_size, size_of_headers)?;
        image[..header.len()].copy_from_slice(&header);

        Ok(image)
    }

    fn build_headers(&self, image_size: u32, size_of_headers: u32) -> Result<Vec<u8>, Report> {
        const IMAGE_FILE_MACHINE_POWERPCBE: u16 = 0x01F2;
        const IMAGE_SUBSYSTEM_XBOX: u16 = 14;

        let sum_where = |mask: u32| -> u32 {
            self.sections
                .iter()
                .filter(|section| section.characteristics & mask != 0)
                .map(|section| section.virtual_size)
                .sum()
        };
        let min_rva_where = |mask: u32, want: bool| -> u32 {
            self.sections
                .iter()
                .filter(|section| (section.characteristics & mask != 0) == want)
                .map(|section| section.rva)
                .min()
                .unwrap_or(0)
        };

        // The PE container is little-endian even though the payload is
        // big-endian PowerPC code.
        let mut headers: Vec<u8> = Vec::new();
        w16(&mut headers, 0x5A4D); // 'MZ'
        // e_cblp = 0x90 so the first DWORD reads 0x00905A4D, which Xenia's
        // is_valid_executable() requires.
        w16(&mut headers, 0x0090);
        headers.resize(0x3C, 0);
        w32(&mut headers, DOS_HEADER_LEN); // e_lfanew -> PE signature at 0x40

        w32(&mut headers, 0x0000_4550); // "PE\0\0"

        // COFF file header.
        w16(&mut headers, IMAGE_FILE_MACHINE_POWERPCBE);
        w16(&mut headers, self.sections.len() as u16);
        w32(&mut headers, 0); // TimeDateStamp
        w32(&mut headers, 0); // PointerToSymbolTable
        w32(&mut headers, 0); // NumberOfSymbols
        w16(&mut headers, OPT_HEADER_LEN as u16); // SizeOfOptionalHeader
        w16(&mut headers, 0x0102); // EXECUTABLE_IMAGE | 32BIT_MACHINE

        // Optional header (PE32).
        w16(&mut headers, 0x010B); // magic PE32
        headers.write_u8(8).ok(); // MajorLinkerVersion
        headers.write_u8(0).ok(); // MinorLinkerVersion
        w32(&mut headers, sum_where(SCN_CODE)); // SizeOfCode
        w32(&mut headers, sum_where(SCN_INITIALIZED_DATA)); // SizeOfInitializedData
        w32(&mut headers, sum_where(SCN_UNINITIALIZED_DATA)); // SizeOfUninitializedData
        w32(&mut headers, self.entry_rva); // AddressOfEntryPoint
        w32(&mut headers, min_rva_where(SCN_MEM_EXECUTE, true)); // BaseOfCode
        w32(&mut headers, min_rva_where(SCN_MEM_EXECUTE, false)); // BaseOfData
        w32(&mut headers, self.image_base);
        w32(&mut headers, self.section_alignment);
        w32(&mut headers, self.file_alignment);
        w16(&mut headers, 4); // MajorOperatingSystemVersion
        w16(&mut headers, 0);
        w16(&mut headers, 0); // MajorImageVersion
        w16(&mut headers, 0);
        w16(&mut headers, 4); // MajorSubsystemVersion
        w16(&mut headers, 0);
        w32(&mut headers, 0); // Win32VersionValue
        w32(&mut headers, image_size); // SizeOfImage
        w32(&mut headers, size_of_headers); // SizeOfHeaders
        w32(&mut headers, 0); // CheckSum
        w16(&mut headers, IMAGE_SUBSYSTEM_XBOX);
        w16(&mut headers, 0); // DllCharacteristics
        w32(&mut headers, 0x40000); // SizeOfStackReserve
        w32(&mut headers, 0x1000); // SizeOfStackCommit
        w32(&mut headers, 0x100000); // SizeOfHeapReserve
        w32(&mut headers, 0x1000); // SizeOfHeapCommit
        w32(&mut headers, 0); // LoaderFlags
        w32(&mut headers, 16); // NumberOfRvaAndSizes
        for _ in 0..16 {
            w32(&mut headers, 0); // DataDirectory[i].VirtualAddress
            w32(&mut headers, 0); // DataDirectory[i].Size
        }

        // Section table.
        for section in &self.sections {
            headers.write_all(&section.name).ok();
            w32(&mut headers, section.virtual_size);
            w32(&mut headers, section.rva);
            w32(&mut headers, section.data.len() as u32); // SizeOfRawData
            w32(&mut headers, section.rva); // PointerToRawData == RVA
            w32(&mut headers, 0); // PointerToRelocations
            w32(&mut headers, 0); // PointerToLinenumbers
            w16(&mut headers, 0); // NumberOfRelocations
            w16(&mut headers, 0); // NumberOfLinenumbers
            w32(&mut headers, section.characteristics);
        }

        Ok(headers)
    }
}

fn align_up(value: u32, align: u32) -> u32 {
    value.div_ceil(align) * align
}

fn name_str(name: &[u8; 8]) -> String {
    let end = name
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(name.len());

    String::from_utf8_lossy(&name[..end]).into_owned()
}

fn w16(bytes: &mut Vec<u8>, value: u16) {
    bytes.write_u16::<LittleEndian>(value).ok();
}

fn w32(bytes: &mut Vec<u8>, value: u32) {
    bytes.write_u32::<LittleEndian>(value).ok();
}
