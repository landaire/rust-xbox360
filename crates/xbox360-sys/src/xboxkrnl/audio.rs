//! Audio decoder and render-driver structures.

/// One hardware XMA decoder context. Contexts are allocated by
/// [`XMACreateContext`](super::XMACreateContext).
#[repr(C)]
pub struct XmaContextData {
    _private: [u8; 0x40],
}

/// Loop bit offsets and subframe limits for an XMA stream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct XmaLoopData {
    pub start_bit: u32,
    pub end_bit: u32,
    pub count: u8,
    pub subframe_end: u8,
    pub subframe_skip: u8,
    _reserved: u8,
}

/// Initial hardware XMA decoder state. Buffer addresses are 32-bit virtual
/// addresses of physically contiguous memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct XmaContextInit {
    pub input_buffer_0: u32,
    pub input_buffer_0_packets: u32,
    pub input_buffer_1: u32,
    pub input_buffer_1_packets: u32,
    pub input_read_bit: u32,
    pub output_buffer: u32,
    pub output_buffer_blocks: u32,
    pub work_buffer: u32,
    pub subframe_decode_count: u32,
    pub stereo: u32,
    pub sample_rate: u32,
    pub loop_data: XmaLoopData,
}

const _: () = {
    assert!(core::mem::size_of::<XmaContextData>() == 0x40);
    assert!(core::mem::size_of::<XmaLoopData>() == 0x0c);
    assert!(core::mem::size_of::<XmaContextInit>() == 0x38);
};
