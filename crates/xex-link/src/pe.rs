//! Emit a minimal Xbox 360 PE image (the XEX base file).
//!
//! Xenia copies the base file verbatim to the load address and then reads the
//! PE headers in place, mapping each section at `image_base + VirtualAddress`
//! without relocating its bytes. So every section's file offset must equal its
//! RVA, and the headers must fit below the first section.

use crate::error::Report;
use crate::error::XexLinkError;
use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use rootcause::IntoReport;
use std::io::Write;

pub const IMAGE_FILE_MACHINE_POWERPCBE: u16 = 0x01F2;
pub const IMAGE_SUBSYSTEM_XBOX: u16 = 14;

// Section characteristics.
pub const SCN_CODE: u32 = 0x0000_0020;
pub const SCN_INITIALIZED_DATA: u32 = 0x0000_0040;
pub const SCN_UNINITIALIZED_DATA: u32 = 0x0000_0080;
pub const SCN_MEM_EXECUTE: u32 = 0x2000_0000;
pub const SCN_MEM_READ: u32 = 0x4000_0000;
pub const SCN_MEM_WRITE: u32 = 0x8000_0000;

const DOS_HEADER_LEN: u32 = 0x40;
const PE_SIG_LEN: u32 = 4;
const COFF_HEADER_LEN: u32 = 20;
const OPT_HEADER_LEN: u32 = 0xE0; // IMAGE_SIZEOF_NT_OPTIONAL_HEADER (PE32, 16 dirs)
const SECTION_HEADER_LEN: u32 = 40;

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
        if self.sections.is_empty() {
            return Err(XexLinkError::NoSections.into_report());
        }

        let headers_len = DOS_HEADER_LEN
            + PE_SIG_LEN
            + COFF_HEADER_LEN
            + OPT_HEADER_LEN
            + self.sections.len() as u32 * SECTION_HEADER_LEN;
        let size_of_headers = align_up(headers_len, self.file_alignment);

        for s in &self.sections {
            if s.rva < headers_len {
                return Err(XexLinkError::HeaderOverlap { name: name_str(&s.name), rva: s.rva, headers: headers_len }
                    .into_report());
            }
            if s.data.len() as u32 > s.virtual_size {
                return Err(XexLinkError::SectionDataOverflow {
                    name: name_str(&s.name),
                    data_len: s.data.len(),
                    virtual_size: s.virtual_size,
                }
                .into_report());
            }
        }

        let image_size =
            align_up(self.sections.iter().map(|s| s.rva + s.virtual_size).max().unwrap_or(0), self.section_alignment);

        let mut buf = vec![0u8; image_size as usize];
        for s in &self.sections {
            let start = s.rva as usize;
            buf[start..start + s.data.len()].copy_from_slice(&s.data);
        }

        let header = self.build_headers(image_size, size_of_headers)?;
        buf[..header.len()].copy_from_slice(&header);
        Ok(buf)
    }

    fn build_headers(&self, image_size: u32, size_of_headers: u32) -> Result<Vec<u8>, Report> {
        let sum_where = |mask: u32| -> u32 {
            self.sections.iter().filter(|s| s.characteristics & mask != 0).map(|s| s.virtual_size).sum()
        };
        let min_rva_where = |mask: u32, want: bool| -> u32 {
            self.sections.iter().filter(|s| (s.characteristics & mask != 0) == want).map(|s| s.rva).min().unwrap_or(0)
        };

        let mut h: Vec<u8> = Vec::new();
        // The PE container is little-endian even though the payload is
        // big-endian PowerPC code.
        w16(&mut h, 0x5A4D); // 'MZ'
        // e_cblp = 0x90 so the first DWORD reads 0x00905A4D, which Xenia's
        // is_valid_executable() requires.
        w16(&mut h, 0x0090);
        h.resize(0x3C, 0);
        w32(&mut h, DOS_HEADER_LEN); // e_lfanew -> PE signature at 0x40

        w32(&mut h, 0x0000_4550); // "PE\0\0"

        // COFF file header.
        w16(&mut h, IMAGE_FILE_MACHINE_POWERPCBE);
        w16(&mut h, self.sections.len() as u16);
        w32(&mut h, 0); // TimeDateStamp
        w32(&mut h, 0); // PointerToSymbolTable
        w32(&mut h, 0); // NumberOfSymbols
        w16(&mut h, OPT_HEADER_LEN as u16); // SizeOfOptionalHeader
        w16(&mut h, 0x0102); // EXECUTABLE_IMAGE | 32BIT_MACHINE

        // Optional header (PE32).
        w16(&mut h, 0x010B); // magic PE32
        h.write_u8(8).ok(); // MajorLinkerVersion
        h.write_u8(0).ok(); // MinorLinkerVersion
        w32(&mut h, sum_where(SCN_CODE)); // SizeOfCode
        w32(&mut h, sum_where(SCN_INITIALIZED_DATA)); // SizeOfInitializedData
        w32(&mut h, sum_where(SCN_UNINITIALIZED_DATA)); // SizeOfUninitializedData
        w32(&mut h, self.entry_rva); // AddressOfEntryPoint
        w32(&mut h, min_rva_where(SCN_MEM_EXECUTE, true)); // BaseOfCode
        w32(&mut h, min_rva_where(SCN_MEM_EXECUTE, false)); // BaseOfData
        w32(&mut h, self.image_base);
        w32(&mut h, self.section_alignment);
        w32(&mut h, self.file_alignment);
        w16(&mut h, 4); // MajorOperatingSystemVersion
        w16(&mut h, 0);
        w16(&mut h, 0); // MajorImageVersion
        w16(&mut h, 0);
        w16(&mut h, 4); // MajorSubsystemVersion
        w16(&mut h, 0);
        w32(&mut h, 0); // Win32VersionValue
        w32(&mut h, image_size); // SizeOfImage
        w32(&mut h, size_of_headers); // SizeOfHeaders
        w32(&mut h, 0); // CheckSum
        w16(&mut h, IMAGE_SUBSYSTEM_XBOX);
        w16(&mut h, 0); // DllCharacteristics
        w32(&mut h, 0x40000); // SizeOfStackReserve
        w32(&mut h, 0x1000); // SizeOfStackCommit
        w32(&mut h, 0x100000); // SizeOfHeapReserve
        w32(&mut h, 0x1000); // SizeOfHeapCommit
        w32(&mut h, 0); // LoaderFlags
        w32(&mut h, 16); // NumberOfRvaAndSizes
        for _ in 0..16 {
            w32(&mut h, 0); // DataDirectory[i].VirtualAddress
            w32(&mut h, 0); // DataDirectory[i].Size
        }

        // Section table.
        for s in &self.sections {
            h.write_all(&s.name).ok();
            w32(&mut h, s.virtual_size);
            w32(&mut h, s.rva);
            w32(&mut h, s.data.len() as u32); // SizeOfRawData
            w32(&mut h, s.rva); // PointerToRawData == RVA
            w32(&mut h, 0); // PointerToRelocations
            w32(&mut h, 0); // PointerToLinenumbers
            w16(&mut h, 0); // NumberOfRelocations
            w16(&mut h, 0); // NumberOfLinenumbers
            w32(&mut h, s.characteristics);
        }

        Ok(h)
    }
}

fn align_up(x: u32, a: u32) -> u32 {
    x.div_ceil(a) * a
}

fn name_str(name: &[u8; 8]) -> String {
    let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
    String::from_utf8_lossy(&name[..end]).into_owned()
}

fn w16(v: &mut Vec<u8>, x: u16) {
    v.write_u16::<LittleEndian>(x).ok();
}

fn w32(v: &mut Vec<u8>, x: u32) {
    v.write_u32::<LittleEndian>(x).ok();
}
