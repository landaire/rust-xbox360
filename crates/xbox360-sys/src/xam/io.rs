use crate::status::Win32ErrorCode;
use crate::xboxkrnl::Handle;

/// `XOVERLAPPED`: completion state of an asynchronous XAM call.
#[repr(C)]
#[derive(Debug, Default)]
pub struct Overlapped {
    pub result: Win32ErrorCode,
    pub length: u32,
    pub context: u32,
    /// Event that XAM signals on completion, or [`Handle::NULL`].
    pub event: Handle,
    pub completion_routine: u32,
    pub completion_context: u32,
    pub extended_error: u32,
}

const _: () = assert!(core::mem::size_of::<Overlapped>() == 0x1c);
