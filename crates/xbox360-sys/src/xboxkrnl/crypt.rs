//! Structures of the kernel cryptography exports.

/// `XeCryptDesKey` key schedule: 16 rounds of two words.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DesState(pub [u32; 32]);

const _: () = assert!(core::mem::size_of::<DesState>() == 0x80);
