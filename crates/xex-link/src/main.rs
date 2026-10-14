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
const SECTION_ALIGNMENT: u32 = 0x1_0000; // 64 KB page range at the load address
const FILE_ALIGNMENT: u32 = 0x1_0000;
const SHF_ALLOC: u64 = 0x2;

// Import thunks live in this section; each is a 16-byte loader record.
const THUNK_SECTION: &str = ".xthunks";
const THUNK_SIZE: usize = 16;
const RECORD_TYPE_THUNK: u32 = 1;
const RECORD_TYPE_THUNK_TARGET: u32 = 2;

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
  --version VER           title version, dotted a.b.c.d or 0xHEX (default 0.0.0.0)
  --base-version VER      minimum loader version, same format (default 0.0.0.0)
  --import-version VER    imported module version (default 2.0.21173.0)
  --import-min-version VER
                          minimum imported module version (default 2.0.1861.0)
  --image-flags FLAGS     comma-separated ImageFlags or 0xHEX (e.g. SMALL_PAGES)
  --module-flags FLAGS    comma-separated ModuleFlags or 0xHEX (default TITLE)
  --compress              LZX-compress the image (Normal, 64 KB window)
  --encrypt TARGET        encrypt payload with the devkit or retail master key
  --bounding-path PATH    device path the loaded file must resolve to";

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
    if base % SECTION_ALIGNMENT != 0 {
        return Err(XexLinkError::UnalignedLoadAddress { base, align: SECTION_ALIGNMENT }.into_report());
    }

    let data = io(&cfg.input, std::fs::read(&cfg.input))?;
    let elf = object::File::parse(&*data).map_err(|e| XexLinkError::Elf(e).into_report())?;

    let entry = elf.entry() as u32;
    if entry < base {
        return Err(XexLinkError::EntryBelowBase { entry, base }.into_report());
    }

    let mut sections = collect_sections(&elf, base)?;

    let thunks = collect_thunks(&elf)?;
    let imports = prepare_direct_imports(&thunks, &mut sections, base, cfg.import_version, cfg.import_min_version)?;
    let image = pe::PeImage {
        image_base: base,
        entry_rva: entry - base,
        section_alignment: SECTION_ALIGNMENT,
        file_alignment: FILE_ALIGNMENT,
        sections,
    };
    let pe_bytes = image.build()?;

    let import_count: usize = imports.iter().map(|library| library.records.len() / 2).sum();
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
        "wrote {} ({} bytes), entry {:#010x}, base {:#010x}, {} section(s), {} import(s){}{}",
        cfg.output,
        xex.len(),
        entry,
        base,
        section_count,
        import_count,
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

/// Add the IAT slots required by the XEX loader and pair each slot with its
/// function thunk in the import table.
fn prepare_direct_imports(
    thunks: &[Thunk],
    sections: &mut Vec<pe::Section>,
    base: u32,
    version: Version,
    version_min: Version,
) -> Result<Vec<ImportLibrary>> {
    if thunks.is_empty() {
        return Ok(Vec::new());
    }

    let image_end = sections.iter().map(|section| section.rva + section.virtual_size).max().unwrap_or(0);
    let iat_rva = image_end.div_ceil(SECTION_ALIGNMENT) * SECTION_ALIGNMENT;
    let mut iat = Vec::with_capacity(thunks.len() * 4);
    let mut by_library: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
    for thunk in thunks {
        by_library.entry(thunk.library_index).or_default();
    }
    let dense_library_indices: BTreeMap<u8, u8> =
        by_library.keys().enumerate().map(|(index, library)| (*library, index as u8)).collect();

    for (index, thunk) in thunks.iter().enumerate() {
        let section = sections
            .iter_mut()
            .find(|section| {
                let start = base + section.rva;
                thunk.va >= start && thunk.va < start + section.virtual_size
            })
            .ok_or_else(|| XexLinkError::BadThunk { va: thunk.va, record: 0 }.into_report())?;
        let offset = (thunk.va - (base + section.rva)) as usize;
        if offset + THUNK_SIZE > section.data.len() {
            return Err(XexLinkError::BadThunk { va: thunk.va, record: 0 }.into_report());
        }
        let library_index = dense_library_indices[&thunk.library_index];
        let thunk_record = (RECORD_TYPE_THUNK << 24) | (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut section.data[offset..offset + 4], thunk_record);
        let target_record =
            (RECORD_TYPE_THUNK_TARGET << 24) | (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        BigEndian::write_u32(&mut section.data[offset + 4..offset + 8], target_record);
        BigEndian::write_u32(&mut section.data[offset + 8..offset + 12], 0x7d69_03a6);
        BigEndian::write_u32(&mut section.data[offset + 12..offset + 16], 0x4e80_0420);

        let iat_record = (u32::from(library_index) << 16) | u32::from(thunk.ordinal);
        iat.extend_from_slice(&iat_record.to_be_bytes());
        let iat_va = base + iat_rva + (index * 4) as u32;
        by_library.entry(thunk.library_index).or_default().extend_from_slice(&[iat_va, thunk.va]);
    }

    sections.push(pe::Section {
        name: *b".xiat\0\0\0",
        rva: iat_rva,
        virtual_size: iat.len() as u32,
        data: iat,
        characteristics: pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ | pe::SCN_MEM_WRITE,
    });

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
    Ok(libraries)
}

fn collect_sections(elf: &object::File, base: u32) -> Result<Vec<pe::Section>> {
    let mut sections = Vec::new();
    for sec in elf.sections() {
        let allocated = matches!(sec.flags(), SectionFlags::Elf { sh_flags } if sh_flags & SHF_ALLOC != 0);
        if !allocated {
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

        let imports =
            prepare_direct_imports(&thunks, &mut sections, base, DEFAULT_IMPORT_VERSION, DEFAULT_IMPORT_MIN_VERSION)
                .unwrap();

        assert_eq!(sections.len(), 2);
        assert_eq!(sections[1].rva, 0x2_0000);
        assert_eq!(sections[1].data, [0, 0, 0, 3, 0, 1, 0, 0x33]);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].records, [base + 0x2_0000, base + 0x1_0000]);
        assert_eq!(imports[1].records, [base + 0x2_0004, base + 0x1_0000 + THUNK_SIZE as u32]);
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

        let imports =
            prepare_direct_imports(&thunks, &mut sections, base, DEFAULT_IMPORT_VERSION, DEFAULT_IMPORT_MIN_VERSION)
                .unwrap();

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0].name, "xam.xex");
        assert_eq!(sections[1].data, [0, 0, 0, 0x33]);
        assert_eq!(&sections[0].data[4..16], &[0x02, 0, 0, 0x33, 0x7d, 0x69, 0x03, 0xa6, 0x4e, 0x80, 0x04, 0x20]);
        assert_eq!(&sections[0].data[..4], &[0x01, 0, 0, 0x33]);
    }
}
