//! The kernel loader's scan for unprotected reservations.
//!
//! On load, the devkit kernel walks every code section word by word and
//! prints "unprotected lwarx/ldarx/stwcx/stdcx instruction" for each
//! reservation outside the interrupt-protected form:
//!
//! - `lwarx`/`ldarx` directly after `mtmsrd r13, 1`, or after the four
//!   words `mtmsrd r13, 1`; `or r8, r8, r8` or `nop`; `or r8, r8, r8` or
//!   `nop`; `or r8, r8, r8`.
//! - `stwcx.`/`stdcx.` directly followed by `mtmsrd rS, 1`.
//!
//! The first word of a section has no predecessor and the last no
//! successor; both count as unprotected.

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

mod error;

pub use self::error::ElfError;

/// `mtmsrd r13, 1`: r13 holds the processor control region address,
/// whose external interrupt enable bit is clear.
const MTMSRD_R13: u32 = 0x7da1_0164;
const OR_R8: u32 = 0x7d08_4378;
const NOP: u32 = 0x6000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reservation {
    Lwarx,
    Ldarx,
    Stwcx,
    Stdcx,
}

impl Reservation {
    fn decode(word: u32) -> Option<Reservation> {
        if word >> 26 != 31 {
            return None;
        }

        let record = word & 1 != 0;
        match (word >> 1) & 0x3ff {
            20 => Some(Reservation::Lwarx),
            84 => Some(Reservation::Ldarx),
            150 if record => Some(Reservation::Stwcx),
            214 if record => Some(Reservation::Stdcx),
            _ => None,
        }
    }
}

impl fmt::Display for Reservation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Reservation::Lwarx => "lwarx",
            Reservation::Ldarx => "ldarx",
            Reservation::Stwcx => "stwcx.",
            Reservation::Stdcx => "stdcx.",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unprotected {
    pub address: u32,
    pub reservation: Reservation,
}

impl fmt::Display for Unprotected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x} {}", self.address, self.reservation)
    }
}

/// Unprotected reservations in the code section loaded at `base`.
pub fn scan(base: u32, words: &[u32]) -> Vec<Unprotected> {
    /// `mtmsrd rS, 1` with the RS field cleared.
    const MTMSRD_ANY: u32 = 0x7c01_0164;
    const RS_FIELD: u32 = 0x1f << 21;

    let protected_load = |index: usize| {
        let before = &words[..index];
        match before {
            [.., previous] if *previous == MTMSRD_R13 => true,
            [.., first, second, third, fourth] => {
                *first == MTMSRD_R13
                    && matches!(*second, OR_R8 | NOP)
                    && matches!(*third, OR_R8 | NOP)
                    && *fourth == OR_R8
            }
            _ => false,
        }
    };

    let protected_store = |index: usize| {
        words
            .get(index + 1)
            .is_some_and(|next| next & !RS_FIELD == MTMSRD_ANY)
    };

    words
        .iter()
        .enumerate()
        .filter_map(|(index, word)| {
            let reservation = Reservation::decode(*word)?;
            let protected = match reservation {
                Reservation::Lwarx | Reservation::Ldarx => protected_load(index),
                Reservation::Stwcx | Reservation::Stdcx => protected_store(index),
            };
            let offset = u32::try_from(index * 4).ok()?;
            (!protected).then_some(Unprotected {
                address: base.wrapping_add(offset),
                reservation,
            })
        })
        .collect()
}

fn be32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn be16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_at(file: &mut File, offset: u64, buffer: &mut [u8]) -> Result<(), ElfError> {
    file.seek(SeekFrom::Start(offset)).map_err(ElfError::Read)?;
    file.read_exact(buffer).map_err(ElfError::Read)
}

/// Unprotected reservations in the executable sections of an ELF image,
/// with each section's size rounded down to whole words as the loader
/// does.
pub fn scan_elf(path: &Path) -> Result<Vec<Unprotected>, ElfError> {
    const SHT_NOBITS: u32 = 8;
    const SHF_EXECINSTR: u32 = 4;

    let mut file = File::open(path).map_err(ElfError::Read)?;
    let mut header = [0u8; 52];
    read_at(&mut file, 0, &mut header)?;
    if header[..4] != *b"\x7fELF" || header[4] != 1 || header[5] != 2 {
        return Err(ElfError::NotElf32BigEndian);
    }

    let section_offset = u64::from(be32(&header, 0x20));
    let entry_size = be16(&header, 0x2e);
    let section_count = be16(&header, 0x30);
    if entry_size < 40 {
        return Err(ElfError::SectionHeaderSize { size: entry_size });
    }

    let mut found = Vec::new();
    let mut entry = vec![0u8; usize::from(entry_size)];

    for index in 0..u64::from(section_count) {
        read_at(
            &mut file,
            section_offset + index * u64::from(entry_size),
            &mut entry,
        )?;
        let kind = be32(&entry, 4);
        let flags = be32(&entry, 8);
        if kind == SHT_NOBITS || flags & SHF_EXECINSTR == 0 {
            continue;
        }

        let address = be32(&entry, 12);
        let offset = u64::from(be32(&entry, 16));
        let size = (be32(&entry, 20) & !3) as usize;
        let mut bytes = vec![0u8; size];
        read_at(&mut file, offset, &mut bytes)?;

        let (chunks, _) = bytes.as_chunks::<4>();
        let words: Vec<u32> = chunks
            .iter()
            .map(|word| u32::from_be_bytes(*word))
            .collect();
        found.extend(scan(address, &words));
    }

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LWARX: u32 = 0x7cc0_2028;
    const LDARX: u32 = 0x7cc0_18a8;
    const STWCX: u32 = 0x7cc0_212d;
    const STDCX: u32 = 0x7cc0_19ad;
    const MTMSRD_R9: u32 = 0x7d21_0164;
    const SUB: u32 = 0x7cc5_3050;
    const LWSYNC: u32 = 0x7c20_04ac;

    fn kinds(found: &[Unprotected]) -> Vec<(u32, Reservation)> {
        found
            .iter()
            .map(|site| (site.address, site.reservation))
            .collect()
    }

    #[test]
    fn accepts_protected_forms() {
        let words = [
            MTMSRD_R13, LWARX, SUB, STWCX, MTMSRD_R9, MTMSRD_R13, NOP, OR_R8, OR_R8, LDARX, STDCX,
            MTMSRD_R9,
        ];
        assert!(scan(0x1000, &words).is_empty());
    }

    #[test]
    fn reports_compiler_reservations() {
        let words = [LWSYNC, LWARX, SUB, STWCX, SUB];
        assert_eq!(
            kinds(&scan(0x1000, &words)),
            [(0x1004, Reservation::Lwarx), (0x100c, Reservation::Stwcx)]
        );
    }

    #[test]
    fn reports_section_edges() {
        assert_eq!(
            kinds(&scan(0x2000, &[LDARX, STDCX])),
            [(0x2000, Reservation::Ldarx), (0x2004, Reservation::Stdcx)]
        );
        // The word before the reservation must be `or r8, r8, r8`.
        let words = [MTMSRD_R13, NOP, NOP, NOP, LWARX];
        assert_eq!(kinds(&scan(0, &words)), [(0x10, Reservation::Lwarx)]);
    }

    #[test]
    fn ignores_stores_without_record_bit() {
        assert!(scan(0, &[STWCX & !1, SUB]).is_empty());
    }
}
