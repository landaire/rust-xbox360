use byteorder::BigEndian;
use byteorder::ByteOrder;
use object::Object;
use object::ObjectSection;
use rootcause::IntoReport;
use xenon_types::Version;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::layout::LARGE_PAGE;
use crate::manifest::ExportSettings;
use crate::manifest::Manifest;
use crate::pe;

// Exported functions are declared in this section as 8-byte records holding
// the ordinal and the function address. It is consumed here and not loaded.
pub const EXPORT_SECTION: &str = ".xexports";
const FIRST_EXPORT_ORDINAL: u16 = 1;

/// One exported function.
pub struct Export {
    ordinal: u16,
    va: u32,
}

/// Identity fields of an export table.
#[derive(Clone, Copy)]
struct ExportIdentity {
    module_id: u32,
    version: Version,
    min_version: Version,
}

/// Scan `.xexports` and return every declared export.
pub fn collect_exports(elf: &object::File) -> Result<Vec<Export>> {
    const EXPORT_RECORD_SIZE: usize = 8;

    let Some(sec) = elf.section_by_name(EXPORT_SECTION) else {
        return Ok(Vec::new());
    };

    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;
    if data.len() % EXPORT_RECORD_SIZE != 0 {
        return Err(XexLinkError::BadExportSection { len: data.len() }.into_report());
    }

    let mut exports = Vec::with_capacity(data.len() / EXPORT_RECORD_SIZE);
    for record in data.as_chunks::<EXPORT_RECORD_SIZE>().0 {
        let ordinal = BigEndian::read_u32(&record[0..4]);
        let va = BigEndian::read_u32(&record[4..8]);
        let ordinal = u16::try_from(ordinal)
            .ok()
            .filter(|ordinal| *ordinal >= FIRST_EXPORT_ORDINAL)
            .ok_or_else(|| XexLinkError::BadExportOrdinal { ordinal, va }.into_report())?;
        exports.push(Export { ordinal, va });
    }

    Ok(exports)
}

/// Encode the export table the manifest asks for. Exports without an
/// `[exports]` manifest table are an error.
pub fn manifest_export_table(
    manifest: &Manifest,
    exports: &[Export],
    sections: &[pe::Section],
    base: u32,
) -> Result<Option<Vec<u8>>> {
    let Some(ExportSettings {
        module,
        min_version,
    }) = &manifest.exports
    else {
        if exports.is_empty() {
            return Ok(None);
        }

        return Err(XexLinkError::ExportsWithoutModule {
            count: exports.len(),
        }
        .into_report());
    };

    // The table stores the image base as its upper half only.
    if !base.is_multiple_of(LARGE_PAGE) {
        return Err(XexLinkError::UnalignedExportBase { base }.into_report());
    }

    if u32::from(manifest.version) < u32::from(*min_version) {
        return Err(XexLinkError::ExportVersionBelowMinimum {
            version: manifest.version,
            minimum: *min_version,
        }
        .into_report());
    }

    let identity = ExportIdentity {
        module_id: module.import_id,
        version: manifest.version,
        min_version: *min_version,
    };

    Ok(Some(build_export_table(exports, sections, base, identity)?))
}

/// Encode the loader export table: three magic words, the module id as two
/// halves, the two versions (low 24 bits each, then both top bytes), the
/// image base >> 16, the slot count, the first ordinal, and one image offset
/// per ordinal with 0 for an unused ordinal.
fn build_export_table(
    exports: &[Export],
    sections: &[pe::Section],
    base: u32,
    identity: ExportIdentity,
) -> Result<Vec<u8>> {
    // Leading words of the stock xbdm.xex export table.
    const EXPORT_TABLE_MAGIC: [u32; 3] = [0x4800_0000, 0x0048_5645, 0x4800_0000];

    let slot_count = exports
        .iter()
        .map(|export| export.ordinal)
        .max()
        .map_or(0, |last| last - FIRST_EXPORT_ORDINAL + 1);
    let mut offsets: Vec<Option<u32>> = vec![None; usize::from(slot_count)];
    for export in exports {
        let in_image = sections.iter().any(|section| {
            let start = base + section.rva;
            export.va >= start && export.va < start + section.virtual_size
        });
        if !in_image {
            return Err(XexLinkError::BadExportAddress {
                ordinal: export.ordinal,
                va: export.va,
            }
            .into_report());
        }

        let slot = &mut offsets[usize::from(export.ordinal - FIRST_EXPORT_ORDINAL)];
        if slot.is_some() {
            return Err(XexLinkError::DuplicateExport {
                ordinal: export.ordinal,
            }
            .into_report());
        }

        *slot = Some(export.va - base);
    }

    let version = u32::from(identity.version);
    let min_version = u32::from(identity.min_version);
    let header = [
        EXPORT_TABLE_MAGIC[0],
        EXPORT_TABLE_MAGIC[1],
        EXPORT_TABLE_MAGIC[2],
        identity.module_id >> 16,
        identity.module_id & 0xFFFF,
        version & 0x00FF_FFFF,
        min_version & 0x00FF_FFFF,
        ((version >> 24) << 8) | (min_version >> 24),
        base >> 16,
        u32::from(slot_count),
        u32::from(FIRST_EXPORT_ORDINAL),
    ];
    let mut table = Vec::with_capacity((header.len() + offsets.len()) * 4);
    for word in &header {
        table.extend_from_slice(&word.to_be_bytes());
    }

    // The loader reads offset 0 as an unused ordinal.
    for offset in &offsets {
        table.extend_from_slice(&offset.unwrap_or(0).to_be_bytes());
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::DEFAULT_IMPORT_MIN_VERSION;

    fn text_section() -> pe::Section {
        pe::Section {
            name: *b".text\0\0\0",
            rva: 0x1_0000,
            virtual_size: 0x9_0000,
            data: Vec::new(),
            characteristics: pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
        }
    }

    fn xbdm_identity() -> ExportIdentity {
        ExportIdentity {
            module_id: 0xeceb_8109,
            version: Version {
                major: 2,
                minor: 0,
                build: 21256,
                revision: 0,
            },
            min_version: DEFAULT_IMPORT_MIN_VERSION,
        }
    }

    #[test]
    fn export_table_matches_stock_xbdm_layout() {
        let base = 0x91f0_0000;
        // Stock ordinals 1, 2, 4, 5 (4 and 5 share an address); 3 left unused.
        let exports = [
            Export {
                ordinal: 2,
                va: 0x91f3_30d0,
            },
            Export {
                ordinal: 1,
                va: 0x91f3_30e0,
            },
            Export {
                ordinal: 4,
                va: 0x91f3_3b58,
            },
            Export {
                ordinal: 5,
                va: 0x91f3_3b58,
            },
        ];

        let table = build_export_table(&exports, &[text_section()], base, xbdm_identity()).unwrap();

        let words: Vec<u32> = table
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| u32::from_be_bytes(*word))
            .collect();
        assert_eq!(
            words,
            [
                0x4800_0000,
                0x0048_5645,
                0x4800_0000,
                0x0000_eceb,
                0x0000_8109,
                0x0053_0800,
                0x0007_4500,
                0x0000_2020,
                0x0000_91f0,
                5,
                1,
                0x0003_30e0,
                0x0003_30d0,
                0,
                0x0003_3b58,
                0x0003_3b58,
            ]
        );
    }

    #[test]
    fn export_table_rejects_duplicate_ordinals_and_foreign_addresses() {
        let base = 0x91f0_0000;
        let duplicate = [
            Export {
                ordinal: 7,
                va: 0x91f1_0000,
            },
            Export {
                ordinal: 7,
                va: 0x91f1_0010,
            },
        ];
        let error =
            build_export_table(&duplicate, &[text_section()], base, xbdm_identity()).unwrap_err();
        assert!(matches!(
            error.current_context(),
            XexLinkError::DuplicateExport { ordinal: 7 }
        ));

        let outside = [Export {
            ordinal: 1,
            va: 0x91fa_0000,
        }];
        let error =
            build_export_table(&outside, &[text_section()], base, xbdm_identity()).unwrap_err();
        assert!(matches!(
            error.current_context(),
            XexLinkError::BadExportAddress {
                ordinal: 1,
                va: 0x91fa_0000
            }
        ));
    }
}
