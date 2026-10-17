use xex2::opt::ImageFlags;

use crate::pe;

// Sections and the file are aligned to the loader page size: 4 KB with
// SMALL_PAGES, else 64 KB.
pub const LARGE_PAGE: u32 = 0x1_0000;

/// Loader page size, which aligns the load address, sections, and file.
pub fn page_size(image_flags: ImageFlags) -> u32 {
    const SMALL_PAGE: u32 = 0x1000;

    if image_flags.contains(ImageFlags::SMALL_PAGES) {
        SMALL_PAGE
    } else {
        LARGE_PAGE
    }
}

pub fn image_end(sections: &[pe::Section]) -> u32 {
    sections
        .iter()
        .map(|section| section.rva + section.virtual_size)
        .max()
        .unwrap_or(0)
}
