/// A NUL-terminated UTF-16 string.
#[repr(transparent)]
pub struct WideCStr([u16]);

impl WideCStr {
    /// `units` with exactly one NUL, at the end. `None` otherwise.
    pub const fn from_units_with_nul(units: &[u16]) -> Option<&Self> {
        let [text @ .., 0] = units else {
            return None;
        };

        let mut index = 0;

        while index < text.len() {
            if text[index] == 0 {
                return None;
            }
            index += 1;
        }

        // SAFETY: `WideCStr` is a transparent wrapper of `[u16]`.
        Some(unsafe { &*(units as *const [u16] as *const Self) })
    }

    pub const fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    /// The units without the NUL.
    pub const fn units(&self) -> &[u16] {
        let [text @ .., _] = &self.0 else {
            unreachable!()
        };

        text
    }
}

/// Number of UTF-16 units of `text`.
#[doc(hidden)]
pub const fn utf16_length(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut length = 0;
    let mut index = 0;

    while index < bytes.len() {
        let lead = bytes[index];
        let (width, units) = match lead {
            0x00..=0x7f => (1, 1),
            0xc0..=0xdf => (2, 1),
            0xe0..=0xef => (3, 1),
            _ => (4, 2),
        };
        index += width;
        length += units;
    }

    length
}

/// `text` as UTF-16 followed by a NUL. `N` is `utf16_length(text) + 1`.
#[doc(hidden)]
pub const fn encode_utf16<const N: usize>(text: &str) -> [u16; N] {
    let bytes = text.as_bytes();
    let mut units = [0u16; N];
    let mut index = 0;
    let mut written = 0;

    while index < bytes.len() {
        let lead = bytes[index] as u32;
        let (scalar, width) = match lead {
            0x00..=0x7f => (lead, 1),
            0xc0..=0xdf => (((lead & 0x1f) << 6) | continuation(bytes, index + 1), 2),
            0xe0..=0xef => (
                ((lead & 0x0f) << 12)
                    | (continuation(bytes, index + 1) << 6)
                    | continuation(bytes, index + 2),
                3,
            ),
            _ => (
                ((lead & 0x07) << 18)
                    | (continuation(bytes, index + 1) << 12)
                    | (continuation(bytes, index + 2) << 6)
                    | continuation(bytes, index + 3),
                4,
            ),
        };

        if scalar == 0 {
            panic!("a wide string cannot contain NUL");
        }

        if scalar >= 0x1_0000 {
            let offset = scalar - 0x1_0000;
            units[written] = (0xd800 | (offset >> 10)) as u16;
            units[written + 1] = (0xdc00 | (offset & 0x3ff)) as u16;
            written += 2;
        } else {
            units[written] = scalar as u16;
            written += 1;
        }

        index += width;
    }

    if written + 1 != N {
        panic!("the array length must be the UTF-16 length plus one");
    }

    units
}

/// The payload bits of the UTF-8 continuation byte at `index`.
const fn continuation(bytes: &[u8], index: usize) -> u32 {
    (bytes[index] & 0x3f) as u32
}

/// A `&'static WideCStr` from a string literal, encoded at compile time.
///
/// ```ignore
/// let title: &xbox360::xam::WideCStr = xbox360::wide_cstr!("Saving");
/// ```
#[macro_export]
macro_rules! wide_cstr {
    ($text:expr) => {{
        const TEXT: &str = $text;
        const UNITS: [u16; $crate::xam::utf16_length(TEXT) + 1] = $crate::xam::encode_utf16(TEXT);
        const STRING: &$crate::xam::WideCStr =
            match $crate::xam::WideCStr::from_units_with_nul(&UNITS) {
                Some(string) => string,
                None => panic!("a wide string cannot contain NUL"),
            };
        STRING
    }};
}
