//! Link a fully-linked PPC ELF executable into an Xbox 360 XEX.
//!
//! The ELF is a throwaway host-side intermediate produced by rustc + rust-lld;
//! the console only ever loads the XEX this tool emits.

mod error;
mod pe;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::error::io;
use object::Object;
use object::ObjectSection;
use object::SectionFlags;
use object::SectionKind;
use rootcause::IntoReport;
use xenon_types::TitleId;
use xenon_types::VirtualAddress;
use xex2::builder::Xex2Builder;

const IMAGE_BASE: u32 = 0x8200_0000;
const SECTION_ALIGNMENT: u32 = 0x1_0000; // 64 KB page range at 0x82000000
const FILE_ALIGNMENT: u32 = 0x1_0000;
const SHF_ALLOC: u64 = 0x2;

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(report) => {
            eprintln!("xex-link: {report:?}");
            std::process::ExitCode::FAILURE
        }
    }
}

struct Args {
    input: String,
    output: String,
    title_id: TitleId,
}

fn parse_args() -> Result<Args> {
    let mut input = None;
    let mut output = None;
    let mut title_id = TitleId(0);
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--title-id" => {
                let v = it.next().ok_or_else(|| XexLinkError::Usage.into_report())?;
                let v = v.trim_start_matches("0x");
                title_id = TitleId(
                    u32::from_str_radix(v, 16).map_err(|_| XexLinkError::Usage.into_report())?,
                );
            }
            _ if input.is_none() => input = Some(a),
            _ if output.is_none() => output = Some(a),
            _ => return Err(XexLinkError::Usage.into_report()),
        }
    }
    Ok(Args {
        input: input.ok_or_else(|| XexLinkError::Usage.into_report())?,
        output: output.ok_or_else(|| XexLinkError::Usage.into_report())?,
        title_id,
    })
}

fn run() -> Result<()> {
    let args = parse_args()?;

    let data = io(&args.input, std::fs::read(&args.input))?;
    let elf = object::File::parse(&*data).map_err(|e| XexLinkError::Elf(e).into_report())?;

    let entry = elf.entry() as u32;
    if entry < IMAGE_BASE {
        return Err(XexLinkError::EntryBelowBase {
            entry,
            base: IMAGE_BASE,
        }
        .into_report());
    }

    let sections = collect_sections(&elf)?;

    let image = pe::PeImage {
        image_base: IMAGE_BASE,
        entry_rva: entry - IMAGE_BASE,
        section_alignment: SECTION_ALIGNMENT,
        file_alignment: FILE_ALIGNMENT,
        sections,
    };
    let pe_bytes = image.build()?;

    let xex = Xex2Builder::new(pe_bytes)
        .load_address(VirtualAddress(IMAGE_BASE))
        .entry_point(VirtualAddress(entry))
        .title_id(args.title_id)
        .build()
        .map_err(|e| XexLinkError::Xex(format!("{e:?}")).into_report())?;

    io(&args.output, std::fs::write(&args.output, &xex))?;
    println!(
        "wrote {} ({} bytes), entry {:#010x}, {} section(s)",
        args.output,
        xex.len(),
        entry,
        image.sections.len(),
    );
    Ok(())
}

fn collect_sections(elf: &object::File) -> Result<Vec<pe::Section>> {
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
            rva: addr - IMAGE_BASE,
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
