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
use xex2::builder::Xex2Builder;
use xex2::imports::ImportLibrary;
use xex2::opt::ImageFlags;
use xex2::opt::ModuleFlags;

const DEFAULT_LOAD_ADDRESS: u32 = 0x8200_0000;
const SECTION_ALIGNMENT: u32 = 0x1_0000; // 64 KB page range at the load address
const FILE_ALIGNMENT: u32 = 0x1_0000;
const SHF_ALLOC: u64 = 0x2;

// Import thunks live in this section; each is a 16-byte record whose first
// big-endian DWORD encodes (type<<24 | library_index<<16 | ordinal).
const THUNK_SECTION: &str = ".xthunks";
const THUNK_SIZE: usize = 16;
const RECORD_TYPE_THUNK: u32 = 1;

// Import library index -> module name. The xbox-sys thunks encode the index.
const LIBRARY_NAMES: &[&str] = &["xboxkrnl.exe", "xam.xex", "xbdm.xex"];

const USAGE: &str = "\
usage: xex-link <input.elf> <output.xex> [options]

options:
  --title-id HEX          title id (default 0x00000000)
  --load-address HEX      image base the ELF was linked at (default 0x82000000)
  --media-id HEX          media id (default 0x00000000)
  --version VER           title version, dotted a.b.c.d or 0xHEX (default 0.0.0.0)
  --base-version VER      minimum loader version, same format (default 0.0.0.0)
  --image-flags FLAGS     comma-separated ImageFlags or 0xHEX (e.g. SMALL_PAGES)
  --module-flags FLAGS    comma-separated ModuleFlags or 0xHEX (default TITLE)
  --compress              LZX-compress the image (Normal, 64 KB window)";

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
    image_flags: ImageFlags,
    module_flags: ModuleFlags,
    compress: bool,
}

fn parse_args() -> Result<Config> {
    let mut input = None;
    let mut output = None;
    let mut title_id = TitleId(0);
    let mut load_address = DEFAULT_LOAD_ADDRESS;
    let mut media_id = MediaId(0);
    let mut version = Version::from(0u32);
    let mut base_version = Version::from(0u32);
    let mut image_flags = ImageFlags::empty();
    let mut module_flags = ModuleFlags::TITLE;
    let mut compress = false;

    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        let mut value = |flag: &str| -> Result<String> {
            it.next()
                .ok_or_else(|| XexLinkError::MissingValue(flag.to_string()).into_report())
        };
        match a.as_str() {
            "--title-id" => title_id = TitleId(parse_hex_u32("--title-id", &value("--title-id")?)?),
            "--load-address" => {
                load_address = parse_hex_u32("--load-address", &value("--load-address")?)?
            }
            "--media-id" => media_id = MediaId(parse_hex_u32("--media-id", &value("--media-id")?)?),
            "--version" => version = parse_version("--version", &value("--version")?)?,
            "--base-version" => {
                base_version = parse_version("--base-version", &value("--base-version")?)?
            }
            "--image-flags" => image_flags = parse_image_flags(&value("--image-flags")?)?,
            "--module-flags" => module_flags = parse_module_flags(&value("--module-flags")?)?,
            "--compress" => compress = true,
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
        image_flags,
        module_flags,
        compress,
    })
}

fn parse_hex_u32(flag: &str, value: &str) -> Result<u32> {
    let digits = value.trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(digits, 16).map_err(|e| {
        XexLinkError::BadValue {
            flag: flag.to_string(),
            value: value.to_string(),
            reason: e.to_string(),
        }
        .into_report()
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
            parts[i] = field
                .parse::<u16>()
                .map_err(|e| bad(flag, value, &e.to_string()))?;
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
        Ok(Version {
            major: parts[0],
            minor: parts[1],
            build: parts[2],
            revision: parts[3],
        })
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
    bitflags::parser::from_str::<ImageFlags>(&normalized)
        .map_err(|e| bad("--image-flags", value, &e.to_string()))
}

fn parse_module_flags(value: &str) -> Result<ModuleFlags> {
    if value.trim().is_empty() {
        return Err(bad("--module-flags", value, "empty flags"));
    }
    let normalized = value.replace(',', " | ");
    bitflags::parser::from_str::<ModuleFlags>(&normalized)
        .map_err(|e| bad("--module-flags", value, &e.to_string()))
}

fn bad(flag: &str, value: &str, reason: &str) -> crate::error::Report {
    XexLinkError::BadValue {
        flag: flag.to_string(),
        value: value.to_string(),
        reason: reason.to_string(),
    }
    .into_report()
}

fn run() -> Result<()> {
    let cfg = parse_args()?;
    let base = cfg.load_address;
    if base % SECTION_ALIGNMENT != 0 {
        return Err(XexLinkError::UnalignedLoadAddress {
            base,
            align: SECTION_ALIGNMENT,
        }
        .into_report());
    }

    let data = io(&cfg.input, std::fs::read(&cfg.input))?;
    let elf = object::File::parse(&*data).map_err(|e| XexLinkError::Elf(e).into_report())?;

    let entry = elf.entry() as u32;
    if entry < base {
        return Err(XexLinkError::EntryBelowBase { entry, base }.into_report());
    }

    let sections = collect_sections(&elf, base)?;

    let imports = collect_imports(&elf)?;
    let image = pe::PeImage {
        image_base: base,
        entry_rva: entry - base,
        section_alignment: SECTION_ALIGNMENT,
        file_alignment: FILE_ALIGNMENT,
        sections,
    };
    let pe_bytes = image.build()?;

    let import_count: usize = imports.iter().map(|l| l.records.len()).sum();
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
    if cfg.compress {
        builder = builder.compress();
    }
    let xex = builder
        .build()
        .map_err(|e| XexLinkError::Xex(format!("{e:?}")).into_report())?;

    io(&cfg.output, std::fs::write(&cfg.output, &xex))?;
    println!(
        "wrote {} ({} bytes), entry {:#010x}, base {:#010x}, {} section(s), {} import(s){}",
        cfg.output,
        xex.len(),
        entry,
        base,
        section_count,
        import_count,
        if cfg.compress { ", compressed" } else { "" },
    );
    Ok(())
}

/// Scan the `.xthunks` section for import records and group them into
/// libraries. Each 16-byte thunk's first big-endian DWORD encodes the record.
fn collect_imports(elf: &object::File) -> Result<Vec<ImportLibrary>> {
    let Some(sec) = elf.section_by_name(THUNK_SECTION) else {
        return Ok(Vec::new());
    };
    let base = sec.address() as u32;
    let data = sec.data().map_err(|e| XexLinkError::Elf(e).into_report())?;

    let mut by_lib: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
    let mut offset = 0usize;
    while offset + 4 <= data.len() {
        let record = BigEndian::read_u32(&data[offset..offset + 4]);
        if record != 0 {
            let va = base + offset as u32;
            let record_type = (record >> 24) & 0xFF;
            if record_type != RECORD_TYPE_THUNK {
                return Err(XexLinkError::BadThunk { va, record }.into_report());
            }
            let library_index = ((record >> 16) & 0xFF) as u8;
            by_lib.entry(library_index).or_default().push(va);
        }
        offset += THUNK_SIZE;
    }

    let mut libraries = Vec::with_capacity(by_lib.len());
    for (library_index, records) in by_lib {
        let name = LIBRARY_NAMES
            .get(library_index as usize)
            .ok_or_else(|| XexLinkError::UnknownLibrary(library_index).into_report())?;
        libraries.push(ImportLibrary {
            name: (*name).to_string(),
            digest: Sha1Hash::ZERO,
            import_id: 0,
            version: Version::from(0u32),
            version_min: Version::from(0u32),
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
            SectionKind::ReadOnlyData | SectionKind::ReadOnlyString => {
                (pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ, section_data(&sec)?)
            }
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

fn section_data<'d>(sec: &impl ObjectSection<'d>) -> Result<Vec<u8>> {
    Ok(sec
        .data()
        .map_err(|e| XexLinkError::Elf(e).into_report())?
        .to_vec())
}

fn section_name<'d>(sec: &impl ObjectSection<'d>) -> [u8; 8] {
    let mut name = [0u8; 8];
    if let Ok(raw) = sec.name_bytes() {
        let n = raw.len().min(8);
        name[..n].copy_from_slice(&raw[..n]);
    }
    name
}
