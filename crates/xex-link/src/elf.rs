use object::Object;
use object::ObjectSection;
use object::SectionFlags;
use object::SectionKind;
use rootcause::IntoReport;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::exports::EXPORT_SECTION;
use crate::pe;

/// Convert every loaded ELF section, except the export declarations, into a PE
/// section relative to `base`.
pub fn collect_sections(elf: &object::File, base: u32) -> Result<Vec<pe::Section>> {
    let mut sections = Vec::new();
    for sec in elf.sections() {
        if !is_allocated(&sec) || sec.name() == Ok(EXPORT_SECTION) {
            continue;
        }

        let addr = sec.address() as u32;
        let size = sec.size() as u32;
        if addr == 0 || size == 0 {
            continue;
        }

        if addr < base {
            return Err(XexLinkError::SectionBelowBase {
                name: sec.name().unwrap_or("?").to_string(),
                addr,
                base,
            }
            .into_report());
        }

        let (characteristics, data) = match sec.kind() {
            SectionKind::Text => (
                pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
                section_data(&sec)?,
            ),
            SectionKind::ReadOnlyData | SectionKind::ReadOnlyString => (
                pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ,
                section_data(&sec)?,
            ),
            SectionKind::Data => (
                pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
                section_data(&sec)?,
            ),
            SectionKind::UninitializedData => (
                pe::SCN_UNINITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
                Vec::new(),
            ),
            _ => continue,
        };

        sections.push(pe::Section {
            name: section_name(&sec),
            rva: addr - base,
            virtual_size: size,
            data,
            characteristics,
        });
    }

    if sections.is_empty() {
        return Err(XexLinkError::NoSections.into_report());
    }

    Ok(sections)
}

/// The address and contents of `sec` when it is an allocated `section` or
/// `section.*` record section. Sections that are not loaded, such as those
/// of a relocatable object, are skipped.
pub fn record_section<'data>(
    sec: &impl ObjectSection<'data>,
    section: &str,
    record_size: usize,
) -> Result<Option<(u32, &'data [u8])>> {
    let Ok(name) = sec.name() else {
        return Ok(None);
    };

    if !is_record_section(name, section) || !is_allocated(sec) {
        return Ok(None);
    }

    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;
    if data.len() % record_size != 0 {
        return Err(XexLinkError::PartialRecord {
            name: name.to_string(),
            size: data.len(),
            record_size,
        }
        .into_report());
    }

    Ok(Some((sec.address() as u32, data)))
}

fn is_allocated<'data>(sec: &impl ObjectSection<'data>) -> bool {
    const SHF_ALLOC: u64 = 0x2;

    matches!(sec.flags(), SectionFlags::Elf { sh_flags } if sh_flags & SHF_ALLOC != 0)
        && sec.address() != 0
}

/// Whether `name` is `section` or one of its per-record sections
/// (`section.*`). Linker scripts that do not merge the per-record sections
/// leave them as separate output sections.
fn is_record_section(name: &str, section: &str) -> bool {
    name.strip_prefix(section)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}

fn section_data<'data>(sec: &impl ObjectSection<'data>) -> Result<Vec<u8>> {
    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;

    Ok(data.to_vec())
}

fn section_name<'data>(sec: &impl ObjectSection<'data>) -> [u8; 8] {
    let mut name = [0u8; 8];
    if let Ok(raw) = sec.name_bytes() {
        let len = raw.len().min(8);
        name[..len].copy_from_slice(&raw[..len]);
    }

    name
}
