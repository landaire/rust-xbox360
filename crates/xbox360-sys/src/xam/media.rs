/// The record that `XamMediaVerificationFailedBlocks` ([`ordinal`](super::ordinal)) fills.
/// The caller sets [`MediaVerificationFailedBlocks::size`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MediaVerificationFailedBlocks {
    /// Size of the record in bytes.
    pub size: u32,
    pub failed_reads: u32,
    pub failed_hashes: u32,
    pub blocks_checked: u32,
    pub total_blocks: u32,
    /// Nonzero once the verification is complete.
    pub completed: u32,
}

impl MediaVerificationFailedBlocks {
    /// A zero record with its size set.
    pub const fn new() -> Self {
        Self {
            size: core::mem::size_of::<Self>() as u32,
            failed_reads: 0,
            failed_hashes: 0,
            blocks_checked: 0,
            total_blocks: 0,
            completed: 0,
        }
    }
}

impl Default for MediaVerificationFailedBlocks {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = assert!(core::mem::size_of::<MediaVerificationFailedBlocks>() == 0x18);
