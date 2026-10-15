//! Link a fully-linked PPC ELF executable into an Xbox 360 XEX.
//!
//! This is the single post-link step of the toolchain: it takes the ELF that
//! rustc/clang + rust-lld produce and emits a devkit-signed XEX that Xenia (and
//! the devkit) load directly. The intermediate PE image is built in memory; no
//! separate ELF-to-EXE or EXE-to-XEX tool is required.

mod error;
mod pe;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::error::io;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use object::Object;
use object::ObjectSection;
use object::SectionFlags;
use object::SectionKind;
use rootcause::IntoReport;
use std::collections::BTreeMap;
use xenon_types::MediaId;
use xenon_types::Sha1Hash;
use xenon_types::TitleId;
use xenon_types::Version;
use xenon_types::VirtualAddress;
use xex2::builder::BoundingPath;
use xex2::builder::EncryptionTarget;
use xex2::builder::Xex2Builder;
use xex2::imports::ImportLibrary;
use xex2::opt::ImageFlags;
use xex2::opt::ModuleFlags;

const DEFAULT_LOAD_ADDRESS: u32 = 0x8200_0000;
// Sections and the file are aligned to the loader page size: 4 KB with
// SMALL_PAGES, else 64 KB.
const SMALL_PAGE: u32 = 0x1000;
const LARGE_PAGE: u32 = 0x1_0000;
const SHF_ALLOC: u64 = 0x2;

// Import thunks live in this section; each is a 16-byte loader record.
const THUNK_SECTION: &str = ".xthunks";
const THUNK_SIZE: usize = 16;
const RECORD_TYPE_THUNK: u32 = 1;
const RECORD_TYPE_THUNK_TARGET: u32 = 2;
// Data imports live in this section; each is a 4-byte slot holding
// `library << 16 | ordinal` that the loader replaces with the export address.
const VARIABLE_SECTION: &str = ".xvars";
const VARIABLE_SIZE: usize = 4;
// Exported functions are declared in this section as 8-byte records holding
// the ordinal and the function address. It is consumed here and not loaded.
const EXPORT_SECTION: &str = ".xexports";
const EXPORT_RECORD_SIZE: usize = 8;
// Leading words of the stock xbdm.xex export table.
const EXPORT_TABLE_MAGIC: [u32; 3] = [0x4800_0000, 0x0048_5645, 0x4800_0000];
const FIRST_EXPORT_ORDINAL: u16 = 1;
// Loader-read data appended after the ELF image: the IAT, then the export
// table in a code section.
const LOADER_SECTION: [u8; 8] = *b".xload\0\0";
const EXPORT_TABLE_SECTION: [u8; 8] = *b".xexp\0\0\0";

/// An importable system module. The loader rejects an import library whose
/// `import_id` differs from the module number in the exporter's export table.
struct ExportingModule {
    name: &'static str,
    import_id: u32,
}

// Import library index -> exporting module. The xbox-sys thunks encode the index.
const LIBRARIES: &[ExportingModule] = &[
    ExportingModule { name: "xboxkrnl.exe", import_id: 0x45dc_17e0 },
    ExportingModule { name: "xam.xex", import_id: 0xfca1_5c76 },
    ExportingModule { name: "xbdm.xex", import_id: 0xeceb_8109 },
];
const DEFAULT_IMPORT_VERSION: Version = Version { major: 2, minor: 0, build: 21173, revision: 0 };
const DEFAULT_IMPORT_MIN_VERSION: Version = Version { major: 2, minor: 0, build: 1861, revision: 0 };

const USAGE: &str = "\
usage: xex-link <input.elf> <output.xex> [options]

options:
  --title-id HEX          title id (default 0x00000000)
  --load-address HEX      image base the ELF was linked at (default 0x82000000)
  --media-id HEX          media id (default 0x00000000)
  --version VER           title version and export table version, dotted a.b.c.d or 0xHEX (default 0.0.0.0)
  --base-version VER      minimum loader version, same format (default 0.0.0.0)
  --import-version VER    imported module version (default 2.0.21173.0)
  --import-min-version VER
                          minimum imported module version (default 2.0.1861.0)
  --image-flags FLAGS     comma-separated ImageFlags or 0xHEX (e.g. SMALL_PAGES)
  --module-flags FLAGS    comma-separated ModuleFlags or 0xHEX (default TITLE)
  --compress              LZX-compress the image (Normal, 64 KB window)
  --encrypt TARGET        encrypt payload with the devkit or retail master key
  --bounding-path PATH    device path the loaded file must resolve to
  --export-module NAME    emit an export table with NAME's module id
                          (xboxkrnl.exe, xam.xex, xbdm.xex)
  --export-min-version VER
                          oldest compatible version recorded in the export
                          table (default 2.0.1861.0)";

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(report) => {
            eprintln!("xex-link: {report:?}\n\n{USAGE}");
            std::process::ExitCode::FAILURE
        }
    }
}

struct Config {
    input: String,
    output: String,
    title_id: TitleId,
    load_address: u32,
    media_id: MediaId,
    version: Version,
    base_version: Version,
    import_version: Version,
    import_min_version: Version,
    image_flags: ImageFlags,
    module_flags: ModuleFlags,
    compress: bool,
    encryption_target: Option<EncryptionTarget>,
    bounding_path: Option<BoundingPath>,
    export_module: Option<&'static ExportingModule>,
    export_min_version: Version,
}

fn parse_args() -> Result<Config> {
    let mut input = None;
    let mut output = None;
    let mut title_id = TitleId(0);
    let mut load_address = DEFAULT_LOAD_ADDRESS;
    let mut media_id = MediaId(0);
    let mut version = Version::from(0u32);
    let mut base_version = Version::from(0u32);
    let mut import_version = DEFAULT_IMPORT_VERSION;
    let mut import_min_version = DEFAULT_IMPORT_MIN_VERSION;
    let mut image_flags = ImageFlags::empty();
    let mut module_flags = ModuleFlags::TITLE;
    let mut compress = false;
    let mut encryption_target = None;
    let mut bounding_path = None;
    let mut export_module = None;
    let mut export_min_version = DEFAULT_IMPORT_MIN_VERSION;

    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut value = |flag: &str| -> Result<String> {
            it.next().ok_or_else(|| XexLinkError::MissingValue(flag.to_string()).into_report())
        };
        match a.as_str() {
            "--title-id" => title_id = TitleId(parse_hex_u32("--title-id", &value("--title-id")?)?),
            "--load-address" => load_address = parse_hex_u32("--load-address", &value("--load-address")?)?,
            "--media-id" => media_id = MediaId(parse_hex_u32("--media-id", &value("--media-id")?)?),
            "--version" => version = parse_version("--version", &value("--version")?)?,
            "--base-version" => base_version = parse_version("--base-version", &value("--base-version")?)?,
            "--import-version" => import_version = parse_version("--import-version", &value("--import-version")?)?,
            "--import-min-version" => {
                import_min_version = parse_version("--import-min-version", &value("--import-min-version")?)?
            }
            "--image-flags" => image_flags = parse_image_flags(&value("--image-flags")?)?,
            "--module-flags" => module_flags = parse_module_flags(&value("--module-flags")?)?,
            "--compress" => compress = true,
            "--encrypt" => encryption_target = Some(parse_encryption_target(&value("--encrypt")?)?),
            "--bounding-path" => {
                bounding_path = Some(
                    BoundingPath::new(value("--bounding-path")?)
                        .map_err(|e| XexLinkError::Xex(format!("{e:?}")).into_report())?,
                )
            }
            "--export-module" => export_module = Some(parse_export_module(&value("--export-module")?)?),
            "--export-min-version" => {
                export_min_version = parse_version("--export-min-version", &value("--export-min-version")?)?
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            _ if a.starts_with('-') => {
                return Err(XexLinkError::UnexpectedArg(a).into_report());
            }
            _ if input.is_none() => input = Some(a),
            _ if output.is_none() => output = Some(a),
            _ => return Err(XexLinkError::UnexpectedArg(a).into_report()),
        }
    }

    let (input, output) = match (input, output) {
        (Some(i), Some(o)) => (i, o),
        _ => return Err(XexLinkError::MissingPaths.into_report()),
    };
    Ok(Config {
        input,
        output,
        title_id,
        load_address,
        media_id,
        version,
        base_version,
        import_version,
        import_min_version,
        image_flags,
        module_flags,
        compress,
        encryption_target,
        bounding_path,
        export_module,
        export_min_version,
    })
}

fn parse_export_module(value: &str) -> Result<&'static ExportingModule> {
    LIBRARIES.iter().find(|module| module.name == value).ok_or_else(|| {
        bad("--export-module", value, "expected a known module (xboxkrnl.exe, xam.xex, xbdm.xex)")
    })
}

fn parse_encryption_target(value: &str) -> Result<EncryptionTarget> {
    match value {
        "devkit" => Ok(EncryptionTarget::Devkit),
        "retail" => Ok(EncryptionTarget::Retail),
        _ => Err(bad("--encrypt", value, "expected devkit or retail")),
    }
}

fn parse_hex_u32(flag: &str, value: &str) -> Result<u32> {
    let digits = value.trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(digits, 16).map_err(|e| {
        XexLinkError::BadValue { flag: flag.to_string(), value: value.to_string(), reason: e.to_string() }.into_report()
    })
}

/// Parse a version as dotted `major.minor.build.revision`, or a packed `0xHEX`.
fn parse_version(flag: &str, value: &str) -> Result<Version> {
    if value.contains('.') {
        let mut parts = [0u16; 4];
        let mut count = 0;
        for (i, field) in value.split('.').enumerate() {
            if i >= 4 {
                return Err(bad(flag, value, "expected at most 4 dotted fields"));
            }
            parts[i] = field.parse::<u16>().map_err(|e| bad(flag, value, &e.to_string()))?;
            count = i + 1;
        }
        if count == 0 {
            return Err(bad(flag, value, "empty version"));
        }
        // Version packs as major:4, minor:4, build:16, revision:8. Reject fields
        // that would overflow their width and silently corrupt neighbors.
        if parts[0] > 0xF {
            return Err(bad(flag, value, "major must be 0..15"));
        }
        if parts[1] > 0xF {
            return Err(bad(flag, value, "minor must be 0..15"));
        }
        if parts[3] > 0xFF {
            return Err(bad(flag, value, "revision must be 0..255"));
        }
        Ok(Version { major: parts[0], minor: parts[1], build: parts[2], revision: parts[3] })
    } else {
        Ok(Version::from(parse_hex_u32(flag, value)?))
    }
}

/// Parse an ImageFlags value: comma-separated flag names, or a packed `0xHEX`.
/// bitflags' parser accepts `A | B | 0x4`; we accept commas so the value needs
/// no shell quoting.
fn parse_image_flags(value: &str) -> Result<ImageFlags> {
    if value.trim().is_empty() {
        return Err(bad("--image-flags", value, "empty flags"));
    }
    let normalized = value.replace(',', " | ");
    bitflags::parser::from_str::<ImageFlags>(&normalized).map_err(|e| bad("--image-flags", value, &e.to_string()))
}

fn parse_module_flags(value: &str) -> Result<ModuleFlags> {
    if value.trim().is_empty() {
        return Err(bad("--module-flags", value, "empty flags"));
    }
    let normalized = value.replace(',', " | ");
    bitflags::parser::from_str::<ModuleFlags>(&normalized).map_err(|e| bad("--module-flags", value, &e.to_string()))
}

fn bad(flag: &str, value: &str, reason: &str) -> crate::error::Report {
    XexLinkError::BadValue { flag: flag.to_string(), value: value.to_string(), reason: reason.to_string() }
        .into_report()
}

fn run() -> Result<()> {
    let cfg = parse_args()?;
    let base = cfg.load_address;
    let page = if cfg.image_flags.contains(ImageFlags::SMALL_PAGES) { SMALL_PAGE } else { LARGE_PAGE };
    if base % page != 0 {
        return Err(XexLinkError::UnalignedLoadAddress { base, align: page }.into_report());
    }

    let data = io(&cfg.input, std::fs::read(&cfg.input))?;
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
    let (imports, loader_data) = prepare_direct_imports(
        &thunks,
        &variables,
        &mut sections,
        base,
        base + loader_rva,
        cfg.import_version,
        cfg.import_min_version,
    )?;
    let export_table = match cfg.export_module {
        Some(module) => {
            // The table stores the image base as its upper half only.
            if base % LARGE_PAGE != 0 {
                return Err(XexLinkError::UnalignedExportBase { base }.into_report());
            }
            if u32::from(cfg.version) < u32::from(cfg.export_min_version) {
                return Err(XexLinkError::ExportVersionBelowMinimum {
                    version: cfg.version,
                    minimum: cfg.export_min_version,
                }
                .into_report());
            }
            let identity = ExportIdentity {
                module_id: module.import_id,
                version: cfg.version,
                min_version: cfg.export_min_version,
            };
            Some(build_export_table(&exports, &sections, base, identity)?)
        }
        None if exports.is_empty() => None,
        None => return Err(XexLinkError::ExportsWithoutModule { count: exports.len() }.into_report()),
    };
    let mut next_rva = loader_rva;
    if !loader_data.is_empty() {
        next_rva = (loader_rva + loader_data.len() as u32).div_ceil(page) * page;
        sections.push(pe::Section {
            name: LOADER_SECTION,
            rva: loader_rva,
            virtual_size: loader_data.len() as u32,
            data: loader_data,
            characteristics: pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
        });
    }
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
        .load_address(VirtualAddress(base))
        .entry_point(VirtualAddress(entry))
        .title_id(cfg.title_id)
        .media_id(cfg.media_id)
        .version(cfg.version)
        .base_version(cfg.base_version)
        .image_flags(cfg.image_flags)
        .module_flags(cfg.module_flags)
        .imports(imports);
    if let Some(table) = export_table {
        builder = builder.export_table(table);
    }
    if let Some(target) = cfg.encryption_target {
        builder = builder.encrypt(target);
    }
    if let Some(path) = cfg.bounding_path {
        builder = builder.bounding_path(path);
    }
    if cfg.compress {
        builder = builder.compress();
    }
    let xex = builder.build().map_err(|e| XexLinkError::Xex(format!("{e:?}")).into_report())?;

    io(&cfg.output, std::fs::write(&cfg.output, &xex))?;
    println!(
        "wrote {} ({} bytes), entry {:#010x}, base {:#010x}, {} section(s), {} import(s), {} export(s){}{}",
        cfg.output,
        xex.len(),
        entry,
        base,
        section_count,
        import_count,
        exports.len(),
        if cfg.compress { ", compressed" } else { "" },
        match cfg.encryption_target {
            Some(EncryptionTarget::Devkit) => ", encrypted with devkit key",
            Some(EncryptionTarget::Retail) => ", encrypted with retail key",
            None => "",
        },
    );
    Ok(())
}

/// One imported function thunk.
struct Thunk {
    va: u32,
    library_index: u8,
    ordinal: u16,
}

/// Scan `.xthunks` and return every import thunk with its VA, library, ordinal.
fn collect_thunks(elf: &object::File) -> Result<Vec<Thunk>> {
    let Some(sec) = elf.section_by_name(THUNK_SECTION) else {
        return Ok(Vec::new());
    };
    let base = sec.address() as u32;
    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;
    let mut out = Vec::new();
    let mut offset = 0usize;
    while offset + 4 <= data.len() {
        let record = BigEndian::read_u32(&data[offset..offset + 4]);
        if record != 0 {
            let va = base + offset as u32;
            if (record >> 24) & 0xFF != RECORD_TYPE_THUNK {
                return Err(XexLinkError::BadThunk { va, record }.into_report());
            }
            out.push(Thunk { va, library_index: ((record >> 16) & 0xFF) as u8, ordinal: (record & 0xFFFF) as u16 });
        }
        offset += THUNK_SIZE;
    }
    Ok(out)
}

/// One imported data export.
struct Variable {
    va: u32,
    library_index: u8,
    ordinal: u16,
}

/// Scan `.xvars` and return every data import slot.
fn collect_variables(elf: &object::File) -> Result<Vec<Variable>> {
    let Some(sec) = elf.section_by_name(VARIABLE_SECTION) else {
        return Ok(Vec::new());
    };
    let base = sec.address() as u32;
    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;
    let mut out = Vec::new();
    for (index, slot) in data.chunks_exact(VARIABLE_SIZE).enumerate() {
        let record = BigEndian::read_u32(slot);
        if record == 0 {
            continue;
        }
        let va = base + (index * VARIABLE_SIZE) as u32;
        if record >> 24 != 0 {
            return Err(XexLinkError::BadVariable { va, record }.into_report());
        }
        out.push(Variable { va, library_index: ((record >> 16) & 0xFF) as u8, ordinal: (record & 0xFFFF) as u16 });
    }
    Ok(out)
}

/// Find the section byte range of a `size`-byte record at `va`.
fn record_slot<'s>(sections: &'s mut [pe::Section], base: u32, va: u32, size: usize) -> Option<&'s mut [u8]> {
    let section = sections.iter_mut().find(|section| {
        let start = base + section.rva;
        va >= start && va < start + section.virtual_size
    })?;
    let offset = (va - (base + section.rva)) as usize;
    section.data.get_mut(offset..offset + size)
}

fn image_end(sections: &[pe::Section]) -> u32 {
    sections.iter().map(|section| section.rva + section.virtual_size).max().unwrap_or(0)
}

/// Build the IAT required by the XEX loader (returned for placement at
/// `iat_va`), pair each slot with its function thunk, and list data import
/// slots in the import table.
fn prepare_direct_imports(
    thunks: &[Thunk],
    variables: &[Variable],
    sections: &mut [pe::Section],
    base: u32,
    iat_va: u32,
    version: Version,
    version_min: Version,
) -> Result<(Vec<ImportLibrary>, Vec<u8>)> {
    if thunks.is_empty() && variables.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let mut iat = Vec::with_capacity(thunks.len() * 4);
    let mut by_library: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
    for library in thunks.iter().map(|t| t.library_index).chain(variables.iter().map(|v| v.library_index)) {
        by_library.entry(library).or_default();
    }
    let dense_library_indices: BTreeMap<u8, u8> =
        by_library.keys().enumerate().map(|(index, library)| (*library, index as u8)).collect();

    for (index, thunk) in thunks.iter().enumerate() {
        let library_index = dense_library_indices[&thunk.library_index];
        let slot = record_slot(sections, base, thunk.va, THUNK_SIZE)
            .ok_or_else(|| XexLinkError::ImportSlotOutsideImage { va: thunk.va }.into_report())?;
        let thunk_record = (RECORD_TYPE_THUNK << 24) | (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut slot[0..4], thunk_record);
        let target_record =
            (RECORD_TYPE_THUNK_TARGET << 24) | (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut slot[4..8], target_record);
        BigEndian::write_u32(&mut slot[8..12], 0x7d69_03a6);
        BigEndian::write_u32(&mut slot[12..16], 0x4e80_0420);

        let iat_record = (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        iat.extend_from_slice(&iat_record.to_be_bytes());
        let slot_va = iat_va + (index * 4) as u32;
        by_library.entry(thunk.library_index).or_default().extend_from_slice(&[slot_va, thunk.va]);
    }

    for variable in variables {
        let library_index = dense_library_indices[&variable.library_index];
        let slot = record_slot(sections, base, variable.va, VARIABLE_SIZE)
            .ok_or_else(|| XexLinkError::ImportSlotOutsideImage { va: variable.va }.into_report())?;
        BigEndian::write_u32(slot, (u32::from(library_index) << 16) | u32::from(variable.ordinal));
        by_library.entry(variable.library_index).or_default().push(variable.va);
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
    Ok((libraries, iat))
}

/// One exported function.
struct Export {
    ordinal: u16,
    va: u32,
}

/// Scan `.xexports` and return every declared export.
fn collect_exports(elf: &object::File) -> Result<Vec<Export>> {
    let Some(sec) = elf.section_by_name(EXPORT_SECTION) else {
        return Ok(Vec::new());
    };
    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;
    if data.len() % EXPORT_RECORD_SIZE != 0 {
        return Err(XexLinkError::BadExportSection { len: data.len() }.into_report());
    }
    let mut out = Vec::with_capacity(data.len() / EXPORT_RECORD_SIZE);
    for record in data.as_chunks::<EXPORT_RECORD_SIZE>().0 {
        let ordinal = BigEndian::read_u32(&record[0..4]);
        let va = BigEndian::read_u32(&record[4..8]);
        let ordinal = u16::try_from(ordinal)
            .ok()
            .filter(|ordinal| *ordinal >= FIRST_EXPORT_ORDINAL)
            .ok_or_else(|| XexLinkError::BadExportOrdinal { ordinal, va }.into_report())?;
        out.push(Export { ordinal, va });
    }
    Ok(out)
}

/// Identity fields of an export table.
#[derive(Clone, Copy)]
struct ExportIdentity {
    module_id: u32,
    version: Version,
    min_version: Version,
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
    let slot_count = exports.iter().map(|export| export.ordinal).max().map_or(0, |last| last - FIRST_EXPORT_ORDINAL + 1);
    let mut offsets: Vec<Option<u32>> = vec![None; usize::from(slot_count)];
    for export in exports {
        let in_image = sections.iter().any(|section| {
            let start = base + section.rva;
            export.va >= start && export.va < start + section.virtual_size
        });
        if !in_image {
            return Err(XexLinkError::BadExportAddress { ordinal: export.ordinal, va: export.va }.into_report());
        }
        let slot = &mut offsets[usize::from(export.ordinal - FIRST_EXPORT_ORDINAL)];
        if slot.is_some() {
            return Err(XexLinkError::DuplicateExport { ordinal: export.ordinal }.into_report());
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

fn collect_sections(elf: &object::File, base: u32) -> Result<Vec<pe::Section>> {
    let mut sections = Vec::new();
    for sec in elf.sections() {
        let allocated = matches!(sec.flags(), SectionFlags::Elf { sh_flags } if sh_flags & SHF_ALLOC != 0);
        if !allocated || sec.name() == Ok(EXPORT_SECTION) {
            continue;
        }
        let addr = sec.address() as u32;
        let size = sec.size() as u32;
        if addr == 0 || size == 0 {
            continue;
        }
        if addr < base {
            return Err(XexLinkError::SectionBelowBase { name: sec.name().unwrap_or("?").to_string(), addr, base }
                .into_report());
        }

        let (characteristics, data) = match sec.kind() {
            SectionKind::Text => (pe::SCN_CODE | pe::SCN_MEM_EXECUTE | pe::SCN_MEM_READ, section_data(&sec)?),
            SectionKind::ReadOnlyData | SectionKind::ReadOnlyString => {
                (pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ, section_data(&sec)?)
            }
            SectionKind::Data => (pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE, section_data(&sec)?),
            SectionKind::UninitializedData => {
                (pe::SCN_UNINITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE, Vec::new())
            }
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

fn section_data<'d>(sec: &impl ObjectSection<'d>) -> Result<Vec<u8>> {
    Ok(sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?.to_vec())
}

fn section_name<'d>(sec: &impl ObjectSection<'d>) -> [u8; 8] {
    let mut name = [0u8; 8];
    if let Ok(raw) = sec.name_bytes() {
        let n = raw.len().min(8);
        name[..n].copy_from_slice(&raw[..n]);
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Thunk { va: base + 0x1_0000, library_index: 0, ordinal: 3 },
            Thunk { va: base + 0x1_0000 + THUNK_SIZE as u32, library_index: 1, ordinal: 0x33 },
        ];

        let (imports, iat) = prepare_direct_imports(
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
        assert_eq!(imports[1].records, [IAT_VA + 4, base + 0x1_0000 + THUNK_SIZE as u32]);
        assert_eq!(&sections[0].data[4..16], &[0x02, 0, 0, 3, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20]);
        assert_eq!(
            &sections[0].data[THUNK_SIZE + 4..THUNK_SIZE * 2],
            &[0x02, 0x01, 0, 0x33, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20]
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
        let thunks = [Thunk { va: base + 0x1_0000, library_index: 1, ordinal: 0x33 }];

        let (imports, iat) = prepare_direct_imports(
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
        assert_eq!(&sections[0].data[4..16], &[0x02, 0, 0, 0x33, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20]);
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
        let thunks = [Thunk { va: base + 0x1_0000, library_index: 1, ordinal: 0x33 }];
        let variables = [
            Variable { va: base + 0x2_0000, library_index: 1, ordinal: 0x10 },
            Variable { va: base + 0x2_0004, library_index: 2, ordinal: 0x156 },
        ];

        let (imports, _) = prepare_direct_imports(
            &thunks,
            &variables,
            &mut sections,
            base,
            IAT_VA,
            DEFAULT_IMPORT_VERSION,
            DEFAULT_IMPORT_MIN_VERSION,
        )
        .unwrap();

        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].name, "xam.xex");
        assert_eq!(imports[0].records, [IAT_VA, base + 0x1_0000, base + 0x2_0000]);
        assert_eq!(imports[1].name, "xbdm.xex");
        assert_eq!(imports[1].records, [base + 0x2_0004]);
        assert_eq!(sections[1].data, [0, 0, 0, 0x10, 0, 1, 0x01, 0x56]);
    }

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
            version: Version { major: 2, minor: 0, build: 21256, revision: 0 },
            min_version: DEFAULT_IMPORT_MIN_VERSION,
        }
    }

    #[test]
    fn export_table_matches_stock_xbdm_layout() {
        let base = 0x91f0_0000;
        // Stock ordinals 1, 2, 4, 5 (4 and 5 share an address); 3 left unused.
        let exports = [
            Export { ordinal: 2, va: 0x91f3_30d0 },
            Export { ordinal: 1, va: 0x91f3_30e0 },
            Export { ordinal: 4, va: 0x91f3_3b58 },
            Export { ordinal: 5, va: 0x91f3_3b58 },
        ];

        let table = build_export_table(&exports, &[text_section()], base, xbdm_identity()).unwrap();

        let words: Vec<u32> = table.as_chunks::<4>().0.iter().map(|word| u32::from_be_bytes(*word)).collect();
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
        let duplicate = [Export { ordinal: 7, va: 0x91f1_0000 }, Export { ordinal: 7, va: 0x91f1_0010 }];
        let error = build_export_table(&duplicate, &[text_section()], base, xbdm_identity()).unwrap_err();
        assert!(matches!(error.current_context(), XexLinkError::DuplicateExport { ordinal: 7 }));

        let outside = [Export { ordinal: 1, va: 0x91fa_0000 }];
        let error = build_export_table(&outside, &[text_section()], base, xbdm_identity()).unwrap_err();
        assert!(matches!(error.current_context(), XexLinkError::BadExportAddress { ordinal: 1, va: 0x91fa_0000 }));
    }
}
