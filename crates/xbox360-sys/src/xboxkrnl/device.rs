//! Devices, drivers, I/O request packets and disk and SCSI requests.

use super::file::IoStatusBlock;
use crate::status::NtStatus;

/// `DISK_GEOMETRY`: what [`IoControlCode::DISK_GET_DRIVE_GEOMETRY`]
/// returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DiskGeometry {
    pub sectors: u32,
    pub bytes_per_sector: u32,
}

/// `PARTITION_INFORMATION`: what [`IoControlCode::DISK_GET_PARTITION_INFO`]
/// returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PartitionInformation {
    /// Byte offset of the partition on the disk.
    pub starting_offset: u64,
    /// Length of the partition in bytes.
    pub partition_length: u64,
}

const _: () = assert!(core::mem::size_of::<PartitionInformation>() == 0x10);

value_enum! {
    /// `IRP_MJ_*`: the major function of an I/O request, which selects the
    /// [`DriverObject::major_function`] routine.
    #[non_exhaustive]
    pub enum IrpMajorFunction: u8 {
        Read = 2,
        Write = 3,
        FlushBuffers = 6,
        DeviceControl = 9,
    }
}

/// An I/O control code of a device control request.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoControlCode(pub u32);

impl IoControlCode {
    /// The 0x14-byte flash geometry; words 0 and 2 multiply to the write
    /// unit of the device.
    pub const FLASH_GEOMETRY: Self = Self(0x3c_0028);
    /// The sector count and the sector size: a [`DiskGeometry`].
    pub const DISK_GET_DRIVE_GEOMETRY: Self = Self(0x7_0000);
    /// The start offset and the length in bytes: a [`PartitionInformation`].
    pub const DISK_GET_PARTITION_INFO: Self = Self(0x7_4004);
    /// A SCSI command described by a [`ScsiPassThroughDirect`] input.
    pub const SCSI_PASS_THROUGH_DIRECT: Self = Self(0x4_d014);
    /// The authentication state of the optical drive.
    pub const ODD_AUTH_STATE: Self = Self(0x2_40cc);
}

/// `SCSI_PASS_THROUGH_DIRECT`: the input of
/// [`IoControlCode::SCSI_PASS_THROUGH_DIRECT`]. The device transfers data
/// directly to or from `data_buffer`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScsiPassThroughDirect {
    /// Size of the structure in bytes.
    pub length: u16,
    _reserved_02: [u8; 5],
    /// Meaning unknown.
    pub unknown_07: u8,
    /// Bytes to transfer.
    pub data_transfer_length: u32,
    _reserved_0c: u32,
    /// Address of the transfer buffer.
    pub data_buffer: u32,
    /// Command descriptor block.
    pub cdb: [u8; 0x10],
}

impl ScsiPassThroughDirect {
    /// A zeroed request.
    pub const fn new() -> Self {
        Self {
            length: 0,
            _reserved_02: [0; 5],
            unknown_07: 0,
            data_transfer_length: 0,
            _reserved_0c: 0,
            data_buffer: 0,
            cdb: [0; 0x10],
        }
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(ScsiPassThroughDirect, unknown_07) == 0x07);
    assert!(offset_of!(ScsiPassThroughDirect, data_transfer_length) == 0x08);
    assert!(offset_of!(ScsiPassThroughDirect, data_buffer) == 0x10);
    assert!(offset_of!(ScsiPassThroughDirect, cdb) == 0x14);
    assert!(core::mem::size_of::<ScsiPassThroughDirect>() == 0x24);
};

/// The standard data of a SCSI INQUIRY command (0x24 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ScsiInquiryData {
    /// Peripheral qualifier and device type.
    pub device_type: u8,
    /// Bit 7: removable medium.
    pub removable: u8,
    pub version: u8,
    /// Response data format in the low four bits.
    pub response_format: u8,
    /// Bytes that follow this one.
    pub additional_length: u8,
    pub flags: [u8; 3],
    /// ASCII vendor identification.
    pub vendor_id: [u8; 8],
    /// ASCII product identification.
    pub product_id: [u8; 0x10],
    /// ASCII product revision level.
    pub product_revision: [u8; 4],
}

impl ScsiInquiryData {
    /// Zeroed data.
    pub const fn new() -> Self {
        Self {
            device_type: 0,
            removable: 0,
            version: 0,
            response_format: 0,
            additional_length: 0,
            flags: [0; 3],
            vendor_id: [0; 8],
            product_id: [0; 0x10],
            product_revision: [0; 4],
        }
    }
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(ScsiInquiryData, vendor_id) == 0x08);
    assert!(offset_of!(ScsiInquiryData, product_id) == 0x10);
    assert!(offset_of!(ScsiInquiryData, product_revision) == 0x20);
    assert!(core::mem::size_of::<ScsiInquiryData>() == 0x24);
};

/// `FILE_DEVICE_*`: the type of a device, as `IoCreateDevice` takes it and
/// [`FsInformationClass::Device`](super::FsInformationClass::Device) returns it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileDeviceType(pub u32);

flags! {
    /// `DO_*` flags of a [`DeviceObject`].
    pub struct DeviceObjectFlags(u32) {
        /// Set by the kernel at creation. The driver clears it once the
        /// device is ready; requests fail while it is set.
        INITIALIZING = 0x10,
    }
}

flags! {
    /// `IoStackLocation::flags`.
    pub struct IoStackFlags(u8) {
        /// The buffer parameter of a read or write is the transfer address,
        /// not an offset into [`Irp::user_buffer`].
        BUFFER_ADDRESS = 0x1,
    }
}

flags! {
    /// `SL_*` control bits of an [`IoStackLocation`].
    pub struct IoStackControl(u8) {
        /// The dispatch routine returned `STATUS_PENDING`.
        PENDING_RETURNED = 0x1,
        /// Run the completion routine when the request is cancelled.
        INVOKE_ON_CANCEL = 0x20,
        /// Run the completion routine when the request succeeds.
        INVOKE_ON_SUCCESS = 0x40,
        /// Run the completion routine when the request fails.
        INVOKE_ON_ERROR = 0x80,
    }
}

/// `IRP`: an I/O request packet. Only the fields this crate names are
/// public; the kernel allocates the packet with its stack locations.
#[repr(C)]
pub struct Irp {
    _header: [u32; 4],
    pub io_status: IoStatusBlock,
    _stack_count: u8,
    /// Index of the current stack location.
    pub current_location: u8,
    _reserved0: [u8; 2],
    pub user_buffer: *mut u8,
    /// Words whose kernel meaning is not known.
    pub unknown_020: [u32; 2],
    _reserved1: [u32; 10],
    pub current_stack_location: *mut IoStackLocation,
}

/// `IO_STACK_LOCATION`: the part of a request for one driver. The
/// locations of a request are contiguous; the next driver's location is
/// the one before the current one.
#[repr(C)]
pub struct IoStackLocation {
    /// An [`IrpMajorFunction`].
    pub major_function: u8,
    pub minor_function: u8,
    pub flags: IoStackFlags,
    pub control: IoStackControl,
    pub parameters: IoStackParameters,
    pub device_object: *mut DeviceObject,
    _file_object: u32,
    /// Address of the console-ABI completion routine.
    pub completion_routine: u32,
    pub context: u32,
}

/// The parameters of an [`IoStackLocation`]; the major function selects
/// the member.
#[repr(C)]
#[derive(Clone, Copy)]
pub union IoStackParameters {
    pub read_write: ReadWriteParameters,
    pub device_control: DeviceControlParameters,
    /// The words of the parameters. A driver may reuse them once it owns
    /// the request.
    pub words: [u32; 4],
}

/// Parameters of [`IrpMajorFunction::Read`] and [`IrpMajorFunction::Write`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ReadWriteParameters {
    pub length: u32,
    /// The transfer address with [`IoStackFlags::BUFFER_ADDRESS`], else an
    /// offset into [`Irp::user_buffer`].
    pub buffer: u32,
    byte_offset: [u32; 2],
}

impl ReadWriteParameters {
    pub const fn byte_offset(&self) -> u64 {
        ((self.byte_offset[0] as u64) << 32) | self.byte_offset[1] as u64
    }
}

/// Parameters of [`IrpMajorFunction::DeviceControl`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceControlParameters {
    pub output_length: u32,
    pub input_buffer: u32,
    pub input_length: u32,
    pub control_code: IoControlCode,
}

/// `DEVICE_OBJECT`. Only the fields this crate names are public; the
/// kernel allocates the object and its extension.
#[repr(C)]
pub struct DeviceObject {
    _header: [u32; 2],
    pub driver_object: *mut DriverObject,
    _reserved0: u32,
    pub current_irp: *mut Irp,
    pub flags: DeviceObjectFlags,
    pub device_extension: *mut core::ffi::c_void,
    _reserved1: [u8; 2],
    /// Stack locations a request to the device needs.
    pub stack_size: u8,
    _reserved2: u8,
    _reserved3: u32,
    /// Mask of the address bits a transfer buffer must have clear.
    pub alignment_requirement: u32,
}

/// A driver routine for a request: `(device, irp) -> NTSTATUS`, entered in
/// the console ABI (see [`bridged!`](crate::bridged)).
pub type DriverDispatch = unsafe extern "C" fn(*mut DeviceObject, *mut Irp) -> NtStatus;

/// The start I/O routine of a driver: `(device, irp)`, entered in the
/// console ABI.
pub type DriverStartIo = unsafe extern "C" fn(*mut DeviceObject, *mut Irp);

/// `DRIVER_OBJECT`: the routines of a driver.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DriverObject {
    pub start_io: Option<DriverStartIo>,
    _reserved: [u32; 2],
    /// The routine of each [`IrpMajorFunction`].
    pub major_function: [Option<DriverDispatch>; 11],
}

impl DriverObject {
    pub const fn new(
        start_io: Option<DriverStartIo>,
        major_function: [Option<DriverDispatch>; 11],
    ) -> Self {
        Self {
            start_io,
            _reserved: [0; 2],
            major_function,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<IoStackParameters>() == 0x10);
    assert!(core::mem::size_of::<ReadWriteParameters>() == 0x10);
    assert!(core::mem::size_of::<DeviceControlParameters>() == 0x10);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    assert!(core::mem::offset_of!(Irp, io_status) == 0x10);
    assert!(core::mem::offset_of!(Irp, current_location) == 0x19);
    assert!(core::mem::offset_of!(Irp, user_buffer) == 0x1c);
    assert!(core::mem::offset_of!(Irp, unknown_020) == 0x20);
    assert!(core::mem::offset_of!(Irp, current_stack_location) == 0x50);
    assert!(core::mem::offset_of!(IoStackLocation, control) == 3);
    assert!(core::mem::offset_of!(IoStackLocation, parameters) == 4);
    assert!(core::mem::offset_of!(IoStackLocation, device_object) == 0x14);
    assert!(core::mem::offset_of!(IoStackLocation, completion_routine) == 0x1c);
    assert!(core::mem::offset_of!(IoStackLocation, context) == 0x20);
    assert!(core::mem::size_of::<IoStackLocation>() == 0x24);
    assert!(core::mem::offset_of!(DeviceObject, driver_object) == 8);
    assert!(core::mem::offset_of!(DeviceObject, current_irp) == 0x10);
    assert!(core::mem::offset_of!(DeviceObject, flags) == 0x14);
    assert!(core::mem::offset_of!(DeviceObject, device_extension) == 0x18);
    assert!(core::mem::offset_of!(DeviceObject, stack_size) == 0x1e);
    assert!(core::mem::offset_of!(DeviceObject, alignment_requirement) == 0x24);
    assert!(core::mem::offset_of!(DriverObject, major_function) == 0xc);
    assert!(core::mem::size_of::<DriverObject>() == 0x38);
};
