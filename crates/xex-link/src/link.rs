use object::Object;
use rootcause::IntoReport;
use xenon_types::VirtualAddress;
use xex2::builder::EncryptionTarget;
use xex2::builder::Xex2Builder;
use xex2::opt::TlsInfo;

use crate::args::Config;
use crate::elf::collect_sections;
use crate::error::Result;
use crate::error::XexLinkError;
use crate::error::io;
use crate::exports::collect_exports;
use crate::exports::manifest_export_table;
use crate::imports::DirectImports;
use crate::imports::collect_thunks;
use crate::imports::collect_variables;
use crate::imports::prepare_direct_imports;
use crate::layout::image_end;
use crate::layout::page_size;
use crate::pe;
use crate::resources::place_resources;

/// Link the ELF at `config.input` into a XEX at `config.output` and print a
/// summary line.
pub fn link(config: Config) -> Result<()> {
    // Loader-read data appended after the ELF image: the IAT, then the export
    // table in a code section, then the resources.
    const LOADER_SECTION: [u8; 8] = *b".xload\0\0";
    const EXPORT_TABLE_SECTION: [u8; 8] = *b".xexp\0\0\0";

    let Config {
        input,
        output,
        manifest,
        resources,
    } = config;
    let base = manifest.load_address.0;
    let page = page_size(manifest.image_flags);

    let data = io(&input, std::fs::read(&input))?;
    let elf = object::File::parse(&*data).map_err(|e| XexLinkError::Elf(e).into_report())?;

    let entry = elf.entry() as u32;
    if entry < base {
        return Err(XexLinkError::EntryBelowBase { entry, base }.into_report());
    }

    let mut sections = collect_sections(&elf, base)?;

    let thunks = collect_thunks(&elf)?;
    let variables = collect_variables(&elf)?;
    let exports = collect_exports(&elf)?;
    let loader_rva = image_end(&sections).div_ceil(page) * page;
    let DirectImports {
        libraries: imports,
        iat: loader_data,
    } = prepare_direct_imports(
        &thunks,
        &variables,
        &mut sections,
        base,
        base + loader_rva,
        manifest.imports.version,
        manifest.imports.min_version,
    )?;
    let export_table = manifest_export_table(&manifest, &exports, &sections, base)?;

    let next_rva = if loader_data.is_empty() {
        loader_rva
    } else {
        let next_rva = (loader_rva + loader_data.len() as u32).div_ceil(page) * page;
        sections.push(pe::Section {
            name: LOADER_SECTION,
            rva: loader_rva,
            virtual_size: loader_data.len() as u32,
            data: loader_data,
            characteristics: pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
        });
        next_rva
    };

    // The hypervisor resolves imports only against an export table in pages it
    // loads into protected memory, which are the code pages; stock xbdm.xex
    // keeps its table in .text.
    let export_table = export_table.map(|table| {
        sections.push(pe::Section {
            name: EXPORT_TABLE_SECTION,
            rva: next_rva,
            virtual_size: table.len() as u32,
            data: table,
            characteristics: pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ,
        });
        VirtualAddress(base + next_rva)
    });

    // Resources start on their own page, so their pages are read-only data.
    let resource_entries = if resources.is_empty() {
        Vec::new()
    } else {
        let rva = image_end(&sections).div_ceil(page) * page;
        let placed = place_resources(&resources, base, rva)?;
        sections.push(placed.section);
        placed.entries
    };

    let image = pe::PeImage {
        image_base: base,
        entry_rva: entry - base,
        section_alignment: page,
        file_alignment: page,
        sections,
    };
    let pe_bytes = image.build()?;

    let import_count = thunks.len() + variables.len();
    let section_count = image.sections.len();
    let mut builder = Xex2Builder::new(pe_bytes)
        .load_address(manifest.load_address)
        .entry_point(VirtualAddress(entry))
        .title_id(manifest.title_id)
        .media_id(manifest.media_id)
        .version(manifest.version)
        .base_version(manifest.base_version)
        .image_flags(manifest.image_flags)
        .module_flags(manifest.module_flags)
        .imports(imports)
        .resources(resource_entries);
    if let Some(table) = export_table {
        builder = builder.export_table(table);
    }
    if let Some(target) = manifest.encryption_target {
        builder = builder.encrypt(target);
    }
    if let Some(path) = manifest.bounding_path {
        builder = builder.bounding_path(path);
    }
    if manifest.compress {
        builder = builder.compress();
    }
    if let Some(slot_count) = manifest.tls_slots {
        builder = builder.tls_info(TlsInfo {
            slot_count,
            raw_data_address: 0,
            data_size: 0,
            raw_data_size: 0,
        });
    }

    let xex = builder
        .build()
        .map_err(|e| XexLinkError::Xex(format!("{e:?}")).into_report())?;

    io(&output, std::fs::write(&output, &xex))?;

    let compressed = if manifest.compress {
        ", compressed"
    } else {
        ""
    };
    let encrypted = match manifest.encryption_target {
        Some(EncryptionTarget::Devkit) => ", encrypted with devkit key",
        Some(EncryptionTarget::Retail) => ", encrypted with retail key",
        None => "",
    };
    println!(
        "wrote {output} ({} bytes), entry {entry:#010x}, base {base:#010x}, {section_count} section(s), {import_count} import(s), {} export(s), {} resource(s){compressed}{encrypted}",
        xex.len(),
        exports.len(),
        resources.len(),
    );

    Ok(())
}
