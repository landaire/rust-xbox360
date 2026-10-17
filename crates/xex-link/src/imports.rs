use std::collections::BTreeMap;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use object::Object;
use rootcause::IntoReport;
use xenon_types::Sha1Hash;
use xenon_types::Version;
use xex2::imports::ImportLibrary;

use crate::elf::record_section;
use crate::error::Result;
use crate::error::XexLinkError;
use crate::pe;

// Each import thunk is a 16-byte loader record.
const THUNK_SIZE: usize = 16;
const RECORD_TYPE_THUNK: u32 = 1;

// Each data import is a 4-byte slot holding `library << 16 | ordinal` that the
// loader replaces with the export address.
const VARIABLE_SIZE: usize = 4;

// Import library index -> exporting module. The xbox360-sys thunks encode the index.
pub const LIBRARIES: &[ExportingModule] = &[
    ExportingModule {
        name: "xboxkrnl.exe",
        import_id: 0x45dc_17e0,
    },
    ExportingModule {
        name: "xam.xex",
        import_id: 0xfca1_5c76,
    },
    ExportingModule {
        name: "xbdm.xex",
        import_id: 0xeceb_8109,
    },
];

/// An importable system module. The loader rejects an import library whose
/// `import_id` differs from the module number in the exporter's export table.
#[derive(Debug, PartialEq)]
pub struct ExportingModule {
    pub name: &'static str,
    pub import_id: u32,
}

/// One imported function thunk.
pub struct Thunk {
    va: u32,
    library_index: u8,
    ordinal: u16,
}

/// One imported data export.
pub struct Variable {
    va: u32,
    library_index: u8,
    ordinal: u16,
}

/// Import table and the IAT bytes the loader reads.
pub struct DirectImports {
    pub libraries: Vec<ImportLibrary>,
    pub iat: Vec<u8>,
}

/// Scan `.xthunks` and every `.xthunks.*` section and return every import
/// thunk with its VA, library, ordinal.
pub fn collect_thunks(elf: &object::File) -> Result<Vec<Thunk>> {
    const THUNK_SECTION: &str = ".xthunks";

    let mut thunks = Vec::new();
    for sec in elf.sections() {
        let Some((base, data)) = record_section(&sec, THUNK_SECTION, THUNK_SIZE)? else {
            continue;
        };

        for (index, slot) in data.as_chunks::<THUNK_SIZE>().0.iter().enumerate() {
            let record = BigEndian::read_u32(&slot[0..4]);
            if record == 0 {
                continue;
            }

            let va = base + (index * THUNK_SIZE) as u32;
            if (record >> 24) & 0xFF != RECORD_TYPE_THUNK {
                return Err(XexLinkError::BadThunk { va, record }.into_report());
            }

            thunks.push(Thunk {
                va,
                library_index: ((record >> 16) & 0xFF) as u8,
                ordinal: (record & 0xFFFF) as u16,
            });
        }
    }

    thunks.sort_by_key(|thunk| thunk.va);

    Ok(thunks)
}

/// Scan `.xvars` and every `.xvars.*` section and return every data import
/// slot.
pub fn collect_variables(elf: &object::File) -> Result<Vec<Variable>> {
    const VARIABLE_SECTION: &str = ".xvars";

    let mut variables = Vec::new();
    for sec in elf.sections() {
        let Some((base, data)) = record_section(&sec, VARIABLE_SECTION, VARIABLE_SIZE)? else {
            continue;
        };

        for (index, slot) in data.as_chunks::<VARIABLE_SIZE>().0.iter().enumerate() {
            let record = BigEndian::read_u32(slot);
            if record == 0 {
                continue;
            }

            let va = base + (index * VARIABLE_SIZE) as u32;
            if record >> 24 != 0 {
                return Err(XexLinkError::BadVariable { va, record }.into_report());
            }

            variables.push(Variable {
                va,
                library_index: ((record >> 16) & 0xFF) as u8,
                ordinal: (record & 0xFFFF) as u16,
            });
        }
    }

    variables.sort_by_key(|variable| variable.va);

    Ok(variables)
}

/// Build the IAT required by the XEX loader (returned for placement at
/// `iat_va`), pair each slot with its function thunk, and list data import
/// slots in the import table.
pub fn prepare_direct_imports(
    thunks: &[Thunk],
    variables: &[Variable],
    sections: &mut [pe::Section],
    base: u32,
    iat_va: u32,
    version: Version,
    version_min: Version,
) -> Result<DirectImports> {
    const RECORD_TYPE_THUNK_TARGET: u32 = 2;

    if thunks.is_empty() && variables.is_empty() {
        return Ok(DirectImports {
            libraries: Vec::new(),
            iat: Vec::new(),
        });
    }

    let mut by_library: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
    for library in thunks
        .iter()
        .map(|thunk| thunk.library_index)
        .chain(variables.iter().map(|variable| variable.library_index))
    {
        by_library.entry(library).or_default();
    }

    let dense_library_indices: BTreeMap<u8, u8> = by_library
        .keys()
        .enumerate()
        .map(|(index, library)| (*library, index as u8))
        .collect();

    let mut iat = Vec::with_capacity(thunks.len() * 4);
    for (index, thunk) in thunks.iter().enumerate() {
        let library_index = dense_library_indices[&thunk.library_index];
        let slot = record_slot(sections, base, thunk.va, THUNK_SIZE)
            .ok_or_else(|| XexLinkError::ImportSlotOutsideImage { va: thunk.va }.into_report())?;
        let thunk_record =
            (RECORD_TYPE_THUNK << 24) | (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut slot[0..4], thunk_record);
        let target_record = (RECORD_TYPE_THUNK_TARGET << 24)
            | (u32::from(library_index) << 16)
            | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut slot[4..8], target_record);
        BigEndian::write_u32(&mut slot[8..12], 0x7d69_03a6);
        BigEndian::write_u32(&mut slot[12..16], 0x4e80_0420);

        let iat_record = (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        iat.extend_from_slice(&iat_record.to_be_bytes());
        let slot_va = iat_va + (index * 4) as u32;
        by_library
            .entry(thunk.library_index)
            .or_default()
            .extend_from_slice(&[slot_va, thunk.va]);
    }

    for variable in variables {
        let library_index = dense_library_indices[&variable.library_index];
        let slot = record_slot(sections, base, variable.va, VARIABLE_SIZE).ok_or_else(|| {
            XexLinkError::ImportSlotOutsideImage { va: variable.va }.into_report()
        })?;
        BigEndian::write_u32(
            slot,
            (u32::from(library_index) << 16) | u32::from(variable.ordinal),
        );
        by_library
            .entry(variable.library_index)
            .or_default()
            .push(variable.va);
    }

    let mut libraries = Vec::with_capacity(by_library.len());
    for (library_index, records) in by_library {
        let module = LIBRARIES
            .get(library_index as usize)
            .ok_or_else(|| XexLinkError::UnknownLibrary(library_index).into_report())?;
        libraries.push(ImportLibrary {
            name: module.name.to_string(),
            digest: Sha1Hash::ZERO,
            import_id: module.import_id,
            version,
            version_min,
            records,
        });
    }

    Ok(DirectImports { libraries, iat })
}

/// Find the section byte range of a `size`-byte record at `va`.
fn record_slot(sections: &mut [pe::Section], base: u32, va: u32, size: usize) -> Option<&mut [u8]> {
    let section = sections.iter_mut().find(|section| {
        let start = base + section.rva;
        va >= start && va < start + section.virtual_size
    })?;

    let offset = (va - (base + section.rva)) as usize;

    section.data.get_mut(offset..offset + size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::DEFAULT_IMPORT_MIN_VERSION;
    use crate::manifest::DEFAULT_IMPORT_VERSION;

    const IAT_VA: u32 = 0x8203_0000;

    #[test]
    fn direct_imports_pair_iat_slots_with_loader_thunks() {
        let base = 0x8200_0000;
        let mut data = vec![0u8; THUNK_SIZE * 2];
        BigEndian::write_u32(&mut data[0..4], 0x0100_0003);
        BigEndian::write_u32(&mut data[THUNK_SIZE..THUNK_SIZE + 4], 0x0101_0033);
        let mut sections = vec![pe::Section {
            name: *b".xthunk\0",
            rva: 0x1_0000,
            virtual_size: data.len() as u32,
            data,
            characteristics: pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
        }];
        let thunks = vec![
            Thunk {
                va: base + 0x1_0000,
                library_index: 0,
                ordinal: 3,
            },
            Thunk {
                va: base + 0x1_0000 + THUNK_SIZE as u32,
                library_index: 1,
                ordinal: 0x33,
            },
        ];

        let DirectImports {
            libraries: imports,
            iat,
        } = prepare_direct_imports(
            &thunks,
            &[],
            &mut sections,
            base,
            IAT_VA,
            DEFAULT_IMPORT_VERSION,
            DEFAULT_IMPORT_MIN_VERSION,
        )
        .unwrap();

        assert_eq!(iat, [0, 0, 0, 3, 0, 1, 0, 0x33]);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].records, [IAT_VA, base + 0x1_0000]);
        assert_eq!(
            imports[1].records,
            [IAT_VA + 4, base + 0x1_0000 + THUNK_SIZE as u32]
        );
        assert_eq!(
            &sections[0].data[4..16],
            &[
                0x02, 0, 0, 3, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20
            ]
        );
        assert_eq!(
            &sections[0].data[THUNK_SIZE + 4..THUNK_SIZE * 2],
            &[
                0x02, 0x01, 0, 0x33, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20
            ]
        );
    }

    #[test]
    fn sparse_import_libraries_use_dense_descriptor_indices() {
        let base = 0x8200_0000;
        let mut sections = vec![pe::Section {
            name: *b".xthunk\0",
            rva: 0x1_0000,
            virtual_size: THUNK_SIZE as u32,
            data: vec![0u8; THUNK_SIZE],
            characteristics: pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
        }];
        let thunks = [Thunk {
            va: base + 0x1_0000,
            library_index: 1,
            ordinal: 0x33,
        }];

        let DirectImports {
            libraries: imports,
            iat,
        } = prepare_direct_imports(
            &thunks,
            &[],
            &mut sections,
            base,
            IAT_VA,
            DEFAULT_IMPORT_VERSION,
            DEFAULT_IMPORT_MIN_VERSION,
        )
        .unwrap();

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].name, "xam.xex");
        assert_eq!(iat, [0, 0, 0, 0x33]);
        assert_eq!(
            &sections[0].data[4..16],
            &[
                0x02, 0, 0, 0x33, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20
            ]
        );
        assert_eq!(&sections[0].data[..4], &[0x01, 0, 0, 0x33]);
    }

    #[test]
    fn data_imports_list_their_slots_with_dense_library_indices() {
        let base = 0x8200_0000;
        let mut thunk_data = vec![0u8; THUNK_SIZE];
        BigEndian::write_u32(&mut thunk_data[0..4], 0x0101_0033);
        let mut sections = vec![
            pe::Section {
                name: *b".xthunk\0",
                rva: 0x1_0000,
                virtual_size: THUNK_SIZE as u32,
                data: thunk_data,
                characteristics: pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
            },
            pe::Section {
                name: *b".xvars\0\0",
                rva: 0x2_0000,
                virtual_size: 8,
                data: vec![0, 1, 0, 0x10, 0, 2, 0x01, 0x56],
                characteristics: pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
            },
        ];
        let thunks = [Thunk {
            va: base + 0x1_0000,
            library_index: 1,
            ordinal: 0x33,
        }];
        let variables = [
            Variable {
                va: base + 0x2_0000,
                library_index: 1,
                ordinal: 0x10,
            },
            Variable {
                va: base + 0x2_0004,
                library_index: 2,
                ordinal: 0x156,
            },
        ];

        let imports = prepare_direct_imports(
            &thunks,
            &variables,
            &mut sections,
            base,
            IAT_VA,
            DEFAULT_IMPORT_VERSION,
            DEFAULT_IMPORT_MIN_VERSION,
        )
        .unwrap()
        .libraries;

        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].name, "xam.xex");
        assert_eq!(
            imports[0].records,
            [IAT_VA, base + 0x1_0000, base + 0x2_0000]
        );
        assert_eq!(imports[1].name, "xbdm.xex");
        assert_eq!(imports[1].records, [base + 0x2_0004]);
        assert_eq!(sections[1].data, [0, 0, 0, 0x10, 0, 1, 0x01, 0x56]);
    }
}
