//! Media device enumeration: the XMsg request the 2.0.6717 dashboard sends
//! to XAM application 0xFA with message 0x7002B (0x921A12C8).
//!
//! Checked only against the dashboard call sites (0x921A92B0 count query,
//! 0x921AA138 fill); the request has no other public description. Not run
//! on hardware.

use core::ptr;

use xbox360_status::Hresult;
use xbox360_sys::xam;

/// Size of one device descriptor in the request buffer.
pub const DEVICE_DESCRIPTOR_SIZE: usize = 0xB4;

/// Request parameters, five big-endian words as the dashboard builds them.
#[repr(C)]
struct EnumerateParams {
    /// Always 0 at the dashboard call sites; its meaning is not known.
    reserved: u32,
    /// 1: only report the device count.
    count_only: u32,
    buffer: u32,
    /// Descriptors that fit in `buffer`.
    capacity: u32,
    /// Address of the count XAM writes.
    count: u32,
}

/// Number of media devices.
pub fn device_count() -> Result<u32, Hresult> {
    let mut count = 0u32;
    // Pointers are 32-bit on the console.
    let params = EnumerateParams {
        reserved: 0,
        count_only: 1,
        buffer: 0,
        capacity: 0,
        count: ptr::addr_of_mut!(count) as u32,
    };
    request(params)?;

    Ok(count)
}

/// Fills `buffer` with descriptors of [`DEVICE_DESCRIPTOR_SIZE`] bytes and
/// returns how many XAM wrote.
pub fn devices(buffer: &mut [u8]) -> Result<u32, Hresult> {
    let mut count = 0u32;
    let capacity = (buffer.len() / DEVICE_DESCRIPTOR_SIZE) as u32;
    let params = EnumerateParams {
        reserved: 0,
        count_only: 0,
        buffer: buffer.as_mut_ptr() as u32,
        capacity,
        count: ptr::addr_of_mut!(count) as u32,
    };
    request(params)?;

    Ok(count.min(capacity))
}

/// Synchronous request; the dashboard passes no overlapped.
fn request(mut params: EnumerateParams) -> Result<(), Hresult> {
    /// XAM application that serves the request.
    const MEDIA_APP: u32 = 0xFA;
    /// Enumerate media devices.
    const ENUMERATE_DEVICES: u32 = 0x7_002B;

    let size = core::mem::size_of::<EnumerateParams>() as u32;
    // SAFETY: `params` and the buffers it names live until the synchronous
    // request returns; XAM writes at most `capacity` descriptors and the
    // count.
    let result = unsafe {
        xam::XMsgStartIORequest(
            MEDIA_APP,
            ENUMERATE_DEVICES,
            ptr::null_mut(),
            ptr::addr_of_mut!(params).cast(),
            size,
        )
    };
    if result.is_failure() { Err(result) } else { Ok(()) }
}
