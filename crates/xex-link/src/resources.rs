//! Files embedded in the image as named resources, which the title reads in
//! place through `XexGetModuleSection`.

use std::io::Read;

use rootcause::IntoReport;
use xenon_types::VirtualAddress;
use xex2::opt::ResourceEntry;
use xex2::opt::ResourceName;

use crate::error::Result;
use crate::error::XexLinkError;
use crate::error::io;
use crate::pe;

/// A file to embed, as given by `--resource NAME=PATH`.
pub struct ResourceFile {
    pub name: ResourceName,
    pub path: String,
}

impl ResourceFile {
    pub fn parse(arg: &str) -> Result<Self> {
        let Some((name, path)) = arg.split_once('=') else {
            return Err(XexLinkError::BadResourceArg(arg.to_owned()).into_report());
        };

        let name = ResourceName::new(name)
            .map_err(|_| XexLinkError::BadResourceName(name.to_owned()).into_report())?;

        Ok(Self {
            name,
            path: path.to_owned(),
        })
    }
}

/// The resource section and the header entries that describe its files.
pub struct PlacedResources {
    pub section: pe::Section,
    pub entries: Vec<ResourceEntry>,
}

/// Lay the files out in one read-only section at `rva`, each at a 128-byte
/// boundary as in the dashboard images, and read them into it.
pub fn place_resources(files: &[ResourceFile], base: u32, rva: u32) -> Result<PlacedResources> {
    const SECTION_NAME: [u8; 8] = *b".xres\0\0\0";
    const ALIGN: u32 = 0x80;

    let mut entries = Vec::with_capacity(files.len());
    let mut end = 0u32;
    for file in files {
        let len = io(&file.path, std::fs::metadata(&file.path))?.len();
        let too_large = || XexLinkError::ResourceTooLarge {
            path: file.path.clone(),
            len,
        };
        let size = u32::try_from(len).map_err(|_| too_large())?;
        let offset = end.checked_next_multiple_of(ALIGN).ok_or_else(too_large)?;
        let next_end = offset.checked_add(size).ok_or_else(too_large)?;

        // The whole image, up to the end of this file, must stay addressable.
        let address = base
            .checked_add(rva)
            .and_then(|start| start.checked_add(offset))
            .filter(|address| address.checked_add(size).is_some())
            .ok_or_else(too_large)?;

        entries.push(ResourceEntry {
            name: file.name,
            address: VirtualAddress(address),
            size,
        });
        end = next_end;
    }

    let mut data = vec![0u8; end as usize];
    for (file, entry) in files.iter().zip(&entries) {
        let start = (entry.address.0 - base - rva) as usize;
        let mut reader = io(&file.path, std::fs::File::open(&file.path))?;
        io(
            &file.path,
            reader.read_exact(&mut data[start..start + entry.size as usize]),
        )?;
    }

    let section = pe::Section {
        name: SECTION_NAME,
        rva,
        virtual_size: end,
        data,
        characteristics: pe::SCN_INITIALIZED_DATA | pe::SCN_MEM_READ,
    };

    Ok(PlacedResources { section, entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_start_on_128_byte_boundaries() {
        let dir = std::env::temp_dir().join(format!("xex-link-resources-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let first = dir.join("first");
        let second = dir.join("second");
        std::fs::write(&first, [0xAA; 0x81]).unwrap();
        std::fs::write(&second, [0xBB; 3]).unwrap();
        let files = [
            ResourceFile::parse(&format!("dashmain={}", first.display())).unwrap(),
            ResourceFile::parse(&format!("FFFE07D1={}", second.display())).unwrap(),
        ];

        let placed = place_resources(&files, 0x8200_0000, 0x3_0000).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();

        let layout = placed
            .entries
            .iter()
            .map(|entry| (entry.name.to_string(), entry.address.0, entry.size))
            .collect::<Vec<_>>();
        assert_eq!(
            layout,
            [
                ("dashmain".to_owned(), 0x8203_0000, 0x81),
                ("FFFE07D1".to_owned(), 0x8203_0100, 3),
            ]
        );
        assert_eq!(placed.section.virtual_size, 0x103);
        assert!(placed.section.data[..0x81].iter().all(|byte| *byte == 0xAA));
        assert!(
            placed.section.data[0x81..0x100]
                .iter()
                .all(|byte| *byte == 0)
        );
        assert_eq!(placed.section.data[0x100..], [0xBB; 3]);
    }

    #[test]
    fn resource_argument_needs_a_valid_name() {
        for arg in ["dashmain", "=file", "dashmain1=file"] {
            assert!(ResourceFile::parse(arg).is_err(), "{arg}");
        }
    }
}
