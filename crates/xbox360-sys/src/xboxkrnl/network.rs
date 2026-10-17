//! Network adapter statistics.

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use super::NicGetStats;

/// `NIC_STATS`, the adapter counters that `NicGetStats` stores.
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NicStatistics {
    pub transmitted_bytes: u64,
    pub received_unicast_bytes: u64,
    pub received_unicast1_bytes: u64,
    pub received_unicast2_bytes: u64,
    pub received_multicast_bytes: u64,
    pub received_broadcast_bytes: u64,
    pub transmitted_frames: u32,
    pub received_unicast_frames: u32,
    pub received_unicast1_frames: u32,
    pub received_unicast2_frames: u32,
    pub received_multicast_frames: u32,
    pub received_broadcast_frames: u32,
    pub transmit_underrun_errors: u32,
    pub transmit_other_errors: u32,
    pub receive_frame_errors: u32,
    pub receive_crc_errors: u32,
    pub receive_other_errors: u32,
    pub reserved: u32,
}

const _: () = {
    assert!(core::mem::offset_of!(NicStatistics, transmitted_frames) == 0x30);
    assert!(core::mem::offset_of!(NicStatistics, receive_other_errors) == 0x58);
    assert!(core::mem::size_of::<NicStatistics>() == 0x60);
    assert!(core::mem::align_of::<NicStatistics>() == 8);
};

/// [`NicGetStats`] with a typed record.
///
/// # Safety
///
/// `device_flags` must meet the requirements of the export.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub unsafe fn nic_get_stats(device_flags: u32, stats: &mut NicStatistics) -> u32 {
    // SAFETY: `stats` is a whole writable record; the rest per the contract.
    unsafe { NicGetStats(device_flags, (stats as *mut NicStatistics).cast()) }
}
