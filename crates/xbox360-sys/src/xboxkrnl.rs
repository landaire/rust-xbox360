//! xboxkrnl.exe imports, kernel structures, and the types and constants of
//! their arguments.
//!
//! Kernel structures that Rust code only reads or that the kernel shares,
//! such as [`Kthread`], are views of the fields in use. A field that the
//! kernel or another processor changes is an atomic, and the bytes between
//! the fields are private. A field that the kernel acts on is a
//! [`KernelU32`] or [`KernelU8`], which safe code can only read.

mod audio;
mod boolean;
mod crypt;
mod debug;
mod device;
mod exception;
mod field;
mod file;
mod handle;
mod hardware;
mod loader;
mod memory;
mod network;
mod object;
mod string;
mod system;
mod thread;
mod video;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::ImageDirectory;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::ImageNtHeaders;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::XexHeaderKey;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::lib_index::XBOXKRNL;

pub use self::audio::XmaContextData;
pub use self::audio::XmaContextInit;
pub use self::audio::XmaLoopData;
pub use self::boolean::Boolean;
pub use self::boolean::Boolean8;
pub use self::crypt::DesState;
pub use self::debug::CdromChannelInformation;
pub use self::debug::DebugMemoryStatistics;
pub use self::debug::DebugMonitorData;
pub use self::debug::DebugMonitorFlags;
pub use self::debug::DebugMonitorService;
pub use self::device::DeviceControlParameters;
pub use self::device::DeviceObject;
pub use self::device::DeviceObjectFlags;
pub use self::device::DiskGeometry;
pub use self::device::DriverDispatch;
pub use self::device::DriverObject;
pub use self::device::DriverStartIo;
pub use self::device::FileDeviceType;
pub use self::device::IoControlCode;
pub use self::device::IoStackControl;
pub use self::device::IoStackFlags;
pub use self::device::IoStackLocation;
pub use self::device::IoStackParameters;
pub use self::device::Irp;
pub use self::device::IrpMajorFunction;
pub use self::device::PartitionInformation;
pub use self::device::ReadWriteParameters;
pub use self::device::ScsiInquiryData;
pub use self::device::ScsiPassThroughDirect;
pub use self::exception::BugCheckCode;
pub use self::exception::Context;
pub use self::exception::ContextFlags;
pub use self::exception::ExceptionFlags;
pub use self::exception::ExceptionRecord;
pub use self::exception::KswitchFrame;
pub use self::exception::KtrapFrame;
pub use self::field::KernelU8;
pub use self::field::KernelU32;
pub use self::field::ListEntry;
pub use self::file::CreateDisposition;
pub use self::file::CreateOptions;
pub use self::file::FileAttributes;
pub use self::file::FileBasicInformation;
pub use self::file::FileDirectoryInformation;
pub use self::file::FileDispositionInformation;
pub use self::file::FileEndOfFileInformation;
pub use self::file::FileInformationClass;
pub use self::file::FileNetworkOpenInformation;
pub use self::file::FilePositionInformation;
pub use self::file::FileRenameInformation;
pub use self::file::FileSystemAttributes;
pub use self::file::FsAttributeInformation;
pub use self::file::FsDeviceInformation;
pub use self::file::FsInformationClass;
pub use self::file::FsSizeInformation;
pub use self::file::IoStatusBlock;
pub use self::file::ShareAccess;
pub use self::handle::Handle;
pub use self::hardware::FirmwareRequest;
pub use self::hardware::HardwareBootFlags;
pub use self::hardware::HardwareFlags;
pub use self::hardware::HardwareGeneration;
pub use self::hardware::HardwareInfo;
pub use self::hardware::PmwRegister;
pub use self::hardware::PrvRegister;
pub use self::hardware::SmcCommand;
pub use self::hardware::SmcI2cDevice;
pub use self::hardware::SmcMessage;
pub use self::hardware::SpecialPurposeRegister;
pub use self::loader::DllEntryReason;
pub use self::loader::ImageFlags;
pub use self::loader::ImageVersion;
pub use self::loader::LdrDataTableEntry;
pub use self::loader::LoadedImageName;
pub use self::memory::AllocationType;
pub use self::memory::MemoryBasicInformation;
pub use self::memory::MemoryRegion;
pub use self::memory::MmProcessStatistics;
pub use self::memory::MmStatistics;
pub use self::memory::PageProtection;
pub use self::memory::PoolTag;
pub use self::memory::PoolType;
pub use self::network::NicStatistics;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use self::network::nic_get_stats;
pub use self::object::AccessMask;
pub use self::object::DispatcherObject;
pub use self::object::EVENT_INCREMENT;
pub use self::object::EventType;
pub use self::object::Kdpc;
pub use self::object::Kevent;
pub use self::object::Ksemaphore;
pub use self::object::KspinLock;
pub use self::object::Ktimer;
pub use self::object::ObjectAttributeFlags;
pub use self::object::ObjectAttributes;
pub use self::object::ObjectType;
pub use self::object::ProcessorMode;
pub use self::object::RtlCriticalSection;
pub use self::object::RtlCriticalSectionFields;
pub use self::object::TimerType;
pub use self::object::WaitReason;
pub use self::object::WaitType;
pub use self::string::AnsiString;
pub use self::string::UnicodeString;
pub use self::system::KernelVersion;
pub use self::system::TimeStampBundle;
pub use self::thread::Irql;
pub use self::thread::Kprocess;
pub use self::thread::Kthread;
pub use self::thread::NotificationRegistration;
pub use self::thread::ProcessType;
pub use self::thread::ThreadCreationFlags;
pub use self::thread::TlsIndex;
pub use self::video::DisplayInformation;
pub use self::video::DisplayDiscoveryData;
pub use self::video::DisplayModeOverrideParameters;
pub use self::video::SystemCommandBuffer;
pub use self::video::VideoModeInfo;
pub use crate::status::NtStatus;
pub use crate::status::Win32ErrorCode;

crate::import_thunk!(
    /// Raises a breakpoint exception to the attached debugger.
    pub safe DbgBreakPoint() -> (),
    lib = XBOXKRNL,
    ordinal = 0x001
);

crate::import_thunk!(
    /// Writes a formatted string to the debug output. Variadic: only the format is declared.
    /// The stub forwards r3-r10 unchanged; a caller that passes arguments declares its own typed import with `xbox360_sys::import_thunk!`.
    pub DbgPrint(format: *const u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x003
);

crate::import_thunk!(
    /// Writes a crash dump for an exception record and context.
    pub DumpWriteDump(record: u32, context: u32, flags: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x006
);

crate::import_thunk!(
    /// Allocates `size` bytes of pool memory with a four-character tag. Returns the address or 0.
    pub ExAllocatePoolWithTag(size: u32, tag: PoolTag) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x00A
);

crate::import_thunk!(
    /// Allocates pool memory of type `pool_type` with a four-character tag. Returns the address or 0.
    pub ExAllocatePoolTypeWithTag(size: u32, tag: PoolTag, pool_type: PoolType) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x00B
);

crate::import_variable!(
    /// Console game region. The dashboard 2.0.6717 compares the first 16
    /// bits with 0x0101 to select Japanese video mode names.
    pub safe ExConsoleGameRegion: u32,
    lib = XBOXKRNL,
    ordinal = 0x00C
);

crate::import_thunk!(
    /// Creates a thread. With `xapi_thread_startup` = 0 the thread enters `start_address` with r3 = `start_context` in the console ABI (use a [`bridged!`](crate::bridged) entry).
    /// Returning from the start routine ends the thread with the returned value as its exit code, as `ExTerminateThread` does.
    /// Empty `creation_flags` start it at once. `handle_out` receives the thread handle, or the referenced thread object with [`ThreadCreationFlags::RETURN_THREAD_OBJECT`].
    pub ExCreateThread(
        handle_out: *mut u32,
        stack_size: u32,
        thread_id_out: *mut u32,
        xapi_thread_startup: u32,
        start_address: u32,
        start_context: u32,
        creation_flags: ThreadCreationFlags,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x00D
);

crate::import_thunk!(
    /// Frees pool memory.
    pub ExFreePool(address: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x00F
);

crate::import_thunk!(
    /// Reads a configuration setting into `buffer`. The setting size is stored at `required_size` when not null.
    /// The 16-bit arguments pass zero-extended in full registers.
    pub ExGetXConfigSetting(
        category: u16,
        setting: u16,
        buffer: *mut u8,
        buffer_size: u16,
        required_size: *mut u16,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x010
);

crate::import_thunk!(
    /// Registers a thread creation and deletion callback when `register`
    /// is true and removes it otherwise. `registration` must stay valid
    /// while registered.
    pub ExRegisterThreadNotification(registration: *mut NotificationRegistration, register: bool) -> (),
    lib = XBOXKRNL,
    ordinal = 0x014
);

crate::import_thunk!(
    /// Registers a title termination callback when `register` is true and
    /// removes it otherwise. `registration` must stay valid while
    /// registered.
    pub ExRegisterTitleTerminateNotification(
        registration: *mut NotificationRegistration,
        register: bool,
    ) -> (),
    lib = XBOXKRNL,
    ordinal = 0x015
);

crate::import_thunk!(
    /// Writes a configuration setting from `buffer`. The 16-bit arguments pass zero-extended in full registers.
    pub ExSetXConfigSetting(category: u16, setting: u16, buffer: *const u8, buffer_size: u16) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x018
);

crate::import_thunk!(
    /// Ends the current thread with `exit_code` without unwinding its stack.
    pub ExTerminateThread(exit_code: u32) -> !,
    lib = XBOXKRNL,
    ordinal = 0x019
);

crate::import_variable!(
    /// Object type of thread objects.
    pub safe ExThreadObjectType: ObjectType,
    lib = XBOXKRNL,
    ordinal = 0x01B
);

crate::import_thunk!(
    /// Returns the element count of a file system cache.
    pub FscGetCacheElementCount(cache: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x020
);

crate::import_thunk!(
    /// Sets the element count of a file system cache.
    pub FscSetCacheElementCount(cache: u32, count: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x021
);

crate::import_thunk!(
    /// Returns the AV pack value the hardware reports.
    pub safe HalGetCurrentAVPack() -> u32,
    lib = XBOXKRNL,
    ordinal = 0x022
);

crate::import_thunk!(
    /// Opens or closes the optical drive tray.
    pub HalOpenCloseODDTray(value: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x024
);

crate::import_thunk!(
    /// Registers a system management controller event callback when
    /// `register` is true and removes it otherwise. `registration` must stay
    /// valid while registered.
    pub HalRegisterSMCNotification(registration: *mut NotificationRegistration, register: bool) -> (),
    lib = XBOXKRNL,
    ordinal = 0x027
);

crate::import_thunk!(
    /// Restarts or shuts down the console. Stock XBDM code continues after the call, so it is not declared to diverge.
    pub HalReturnToFirmware(routine: FirmwareRequest) -> (),
    lib = XBOXKRNL,
    ordinal = 0x028
);

crate::import_thunk!(
    /// Sends a 16-byte message to the system management controller and receives the 16-byte reply.
    pub HalSendSMCMessage(message: *const SmcMessage, reply: *mut SmcMessage) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x029
);

crate::import_thunk!(
    /// Removes all entries from a singly linked list. Returns the first entry.
    pub InterlockedFlushSList(header: *mut u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x02B
);

crate::import_thunk!(
    /// Removes the first entry from a singly linked list. Returns the entry or 0.
    pub InterlockedPopEntrySList(header: *mut u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x02C
);

crate::import_thunk!(
    /// Acquires the device object lock. Returns the IRQL that `IoReleaseDeviceObjectLock` restores.
    pub IoAcquireDeviceObjectLock() -> Irql,
    lib = XBOXKRNL,
    ordinal = 0x02E
);

crate::import_thunk!(
    /// Allocates an I/O request packet with `stack_size` stack locations. Returns it or null.
    pub IoAllocateIrp(stack_size: u32) -> *mut Irp,
    lib = XBOXKRNL,
    ordinal = 0x02F
);

crate::import_thunk!(
    /// Completes an I/O request packet.
    pub IoCompleteRequest(irp: *mut Irp, priority_boost: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x035
);

crate::import_thunk!(
    /// Creates a device object for a driver. The device object address is stored at `device`.
    pub IoCreateDevice(
        driver: *mut DriverObject,
        extension_size: u32,
        name: *const AnsiString,
        device_type: FileDeviceType,
        options: u32,
        device: *mut *mut DeviceObject,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x037
);

crate::import_variable!(
    /// Object type of device objects.
    pub safe IoDeviceObjectType: ObjectType,
    lib = XBOXKRNL,
    ordinal = 0x03A
);

crate::import_thunk!(
    /// Dismounts the file system volume on a device object.
    pub IoDismountVolume(device: *mut DeviceObject) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x03B
);

crate::import_thunk!(
    /// Dismounts the file system volume that an open file belongs to.
    pub IoDismountVolumeByFileHandle(handle: Handle) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x03C
);

crate::import_thunk!(
    /// Completes an I/O request packet with an invalid device request status.
    pub IoInvalidDeviceRequest(device: *mut DeviceObject, irp: *mut Irp) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x041
);

crate::import_thunk!(
    /// Releases the device object lock and restores `irql`.
    pub IoReleaseDeviceObjectLock(irql: Irql) -> (),
    lib = XBOXKRNL,
    ordinal = 0x044
);

crate::import_thunk!(
    /// Starts the next queued I/O request packet of a device.
    pub IoStartNextPacket(device: *mut DeviceObject) -> (),
    lib = XBOXKRNL,
    ordinal = 0x048
);

crate::import_thunk!(
    /// Starts or queues an I/O request packet on a device.
    pub IoStartPacket(device: *mut DeviceObject, irp: *mut Irp, key: *const u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x04A
);

crate::import_thunk!(
    /// Sends a device I/O control request to a device object and waits for it.
    pub IoSynchronousDeviceIoControlRequest(
        code: IoControlCode,
        device: *mut DeviceObject,
        input: *const u8,
        input_length: u32,
        output: *mut u8,
        output_length: u32,
        returned: *mut u32,
        internal: bool,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x04B
);

crate::import_thunk!(
    /// Sends a read or write request to a device object and waits for it.
    pub IoSynchronousFsdRequest(
        major: IrpMajorFunction,
        device: *mut DeviceObject,
        buffer: *mut u8,
        length: u32,
        offset: *const u64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x04C
);

crate::import_thunk!(
    /// Acquires a spin lock. The caller is already at DISPATCH_LEVEL.
    pub KeAcquireSpinLockAtRaisedIrql(lock: *mut KspinLock) -> (),
    lib = XBOXKRNL,
    ordinal = 0x04D
);

crate::import_thunk!(
    /// Halts the system with a bug check code.
    pub safe KeBugCheck(code: BugCheckCode) -> !,
    lib = XBOXKRNL,
    ordinal = 0x052
);

crate::import_thunk!(
    /// Halts the system with a bug check code and four parameters.
    pub safe KeBugCheckEx(
        code: BugCheckCode,
        parameter1: u32,
        parameter2: u32,
        parameter3: u32,
        parameter4: u32,
    ) -> !,
    lib = XBOXKRNL,
    ordinal = 0x053
);

crate::import_thunk!(
    /// Cancels a timer. True when the timer was set.
    pub KeCancelTimer(timer: *mut Ktimer) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x054
);

crate::import_thunk!(
    /// Builds a thread context from kernel trap frames.
    pub KeContextFromKframes(frame: u32, context: *mut u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x056
);

crate::import_variable!(
    /// Address of the [`DebugMonitorData`] block that the kernel reads, or
    /// 0. The debug monitor stores it. While a block is published, the
    /// kernel calls the routines it names, so the block must stay valid
    /// and its routine fields must hold routines of the expected form.
    pub safe KeDebugMonitorData: KernelU32,
    lib = XBOXKRNL,
    ordinal = 0x059
);

crate::import_thunk!(
    /// Suspends the current thread for `interval` (100 ns units; negative is relative).
    pub KeDelayExecutionThread(
        mode: ProcessorMode,
        alertable: bool,
        interval: *const i64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x05A
);

crate::import_thunk!(
    /// Enables the processor performance monitor.
    pub KeEnablePPUPerformanceMonitor() -> u32,
    lib = XBOXKRNL,
    ordinal = 0x05E
);

crate::import_thunk!(
    /// Disables normal kernel APC delivery to the current thread.
    pub KeEnterCriticalRegion() -> (),
    lib = XBOXKRNL,
    ordinal = 0x05F
);

crate::import_thunk!(
    /// Flushes the data cache for an address range.
    pub KeFlushCacheRange(address: u32, length: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x061
);

crate::import_thunk!(
    /// Returns the process type of the current thread.
    pub safe KeGetCurrentProcessType() -> ProcessType,
    lib = XBOXKRNL,
    ordinal = 0x066
);

crate::import_thunk!(
    /// Reads a performance monitor register.
    pub KeGetPMWRegister(register: PmwRegister) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x067
);

crate::import_thunk!(
    /// Reads a special purpose register. Only the low word of the result is returned.
    pub KeGetSpecialPurposeRegister(register: SpecialPurposeRegister) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x06A
);

crate::import_thunk!(
    /// Initializes a deferred procedure call object.
    pub KeInitializeDpc(dpc: *mut Kdpc, routine: usize, context: usize) -> (),
    lib = XBOXKRNL,
    ordinal = 0x06F
);

crate::import_thunk!(
    /// Initializes an event object.
    pub KeInitializeEvent(event: *mut Kevent, kind: EventType, initial_state: bool) -> (),
    lib = XBOXKRNL,
    ordinal = 0x070
);

crate::import_thunk!(
    /// Initializes a semaphore object with a count and limit.
    pub KeInitializeSemaphore(semaphore: *mut Ksemaphore, count: u32, limit: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x074
);

crate::import_thunk!(
    /// Initializes a timer object for processes of `process_type`. Stock XBDM passes [`ProcessType::SYSTEM`] for its capture timer.
    pub KeInitializeTimerEx(timer: *mut Ktimer, kind: TimerType, process_type: ProcessType) -> (),
    lib = XBOXKRNL,
    ordinal = 0x075
);

crate::import_thunk!(
    /// Queues a deferred procedure call. True when it was queued.
    pub KeInsertQueueDpc(dpc: *mut Kdpc, argument1: u32, argument2: u32) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x07B
);

crate::import_thunk!(
    /// Runs `routine(context)` on every processor at IPI level.
    pub KeIpiGenericCall(routine: u32, context: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x07C
);

crate::import_thunk!(
    /// Enables normal kernel APC delivery to the current thread.
    pub KeLeaveCriticalRegion() -> (),
    lib = XBOXKRNL,
    ordinal = 0x07D
);

crate::import_thunk!(
    /// Returns the timebase frequency. The export returns 64 bits in r3; the value fits in 32 bits and the stub returns the low word.
    pub safe KeQueryPerformanceFrequency() -> u32,
    lib = XBOXKRNL,
    ordinal = 0x083
);

crate::import_thunk!(
    /// Stores the system time as a FILETIME.
    pub KeQuerySystemTime(time: *mut u64) -> (),
    lib = XBOXKRNL,
    ordinal = 0x084
);

crate::import_thunk!(
    /// Raises the IRQL to DISPATCH_LEVEL. Returns the previous IRQL.
    pub KeRaiseIrqlToDpcLevel() -> Irql,
    lib = XBOXKRNL,
    ordinal = 0x085
);

crate::import_thunk!(
    /// Releases a semaphore object. Returns the previous count.
    pub KeReleaseSemaphore(semaphore: *mut Ksemaphore, increment: u32, adjustment: u32, wait: bool) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x088
);

crate::import_thunk!(
    /// Releases a spin lock without changing the IRQL.
    pub KeReleaseSpinLockFromRaisedIrql(lock: *mut KspinLock) -> (),
    lib = XBOXKRNL,
    ordinal = 0x089
);

crate::import_thunk!(
    /// Removes a deferred procedure call from the queue. True when it was queued.
    pub KeRemoveQueueDpc(dpc: *mut Kdpc) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x08E
);

crate::import_thunk!(
    /// Sets an event object to not signaled. Returns the previous state.
    pub KeResetEvent(event: *mut Kevent) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x08F
);

crate::import_thunk!(
    /// Decrements the suspend count of a thread object. Returns the previous count.
    pub KeResumeThread(thread: *mut Kthread) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x092
);

crate::import_thunk!(
    /// Sets the processor affinity of a thread. The previous affinity is stored at `previous`.
    pub KeSetAffinityThread(thread: *mut Kthread, affinity: u32, previous: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x097
);

crate::import_thunk!(
    /// Sets the base priority increment of a thread object.
    pub KeSetBasePriorityThread(thread: *mut Kthread, increment: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x099
);

crate::import_thunk!(
    /// Sets an event object to signaled. Returns the previous state. `increment` is usually [`EVENT_INCREMENT`].
    pub KeSetEvent(event: *mut Kevent, increment: u32, wait: bool) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x09D
);

import_adapter!(
    /// Writes a performance monitor register. The export takes the 64-bit `value` in one register.
    pub KeSetPMWRegister(register: PmwRegister, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x09F,
    // register in r3, value in r5:r6.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
    ]
);

import_adapter!(
    /// Writes a performance monitor register. The export takes the 64-bit `value` in one register.
    pub KeSetPRVRegister(register: PrvRegister, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A1,
    // register in r3, value in r5:r6.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
    ]
);

crate::import_thunk!(
    /// Sets the priority of a thread object. Returns the previous priority.
    pub KeSetPriorityThread(thread: *mut Kthread, priority: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A3
);

import_adapter!(
    /// Writes a special purpose register. The export takes the 64-bit `value` in one register.
    pub KeSetSpecialPurposeRegister(register: SpecialPurposeRegister, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A5,
    // register in r3, value in r5:r6.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
    ]
);

import_adapter!(
    /// Sets a timer to expire at `due_time` (100 ns units; negative is relative to now), then every `period` milliseconds when `period` is not 0.
    /// `dpc` is queued at expiry when not null. True when the timer was already set.
    pub KeSetTimerEx(timer: *mut Ktimer, due_time: i64, period: u32, dpc: *mut Kdpc) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x0A7,
    // timer in r3, due_time in r5:r6, period in r7, dpc in r8.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
        "mr 5, 7",
        "mr 6, 8",
    ]
);

crate::import_thunk!(
    /// Busy-waits for `microseconds`.
    pub safe KeStallExecutionProcessor(microseconds: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0A8
);

crate::import_thunk!(
    /// Increments the suspend count of a thread object. Returns the previous count.
    pub KeSuspendThread(thread: *mut Kthread) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A9
);

crate::import_thunk!(
    /// Invalidates the instruction cache for an address range.
    pub KeSweepIcacheRange(address: *mut u8, length: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0AB
);

crate::import_variable!(
    /// Kernel time stamp bundle.
    pub safe KeTimeStampBundle: TimeStampBundle,
    lib = XBOXKRNL,
    ordinal = 0x0AD
);

crate::import_thunk!(
    /// Tries to acquire a spin lock at DISPATCH_LEVEL. True when acquired.
    pub KeTryToAcquireSpinLockAtRaisedIrql(lock: *mut KspinLock) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x0AE
);

crate::import_thunk!(
    /// Waits on several dispatcher objects. `wait_blocks` may be null for up to four objects.
    pub KeWaitForMultipleObjects(
        count: u32,
        objects: *const *mut DispatcherObject,
        wait_type: WaitType,
        reason: WaitReason,
        mode: ProcessorMode,
        alertable: bool,
        timeout: *const i64,
        wait_blocks: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0AF
);

crate::import_thunk!(
    /// Waits on a dispatcher object. A null `timeout` waits without limit.
    pub KeWaitForSingleObject(
        object: *mut DispatcherObject,
        reason: WaitReason,
        mode: ProcessorMode,
        alertable: bool,
        timeout: *const i64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0B0
);

crate::import_thunk!(
    /// Raises the IRQL to DISPATCH_LEVEL and acquires a spin lock. Returns the previous IRQL.
    pub KfAcquireSpinLock(lock: *mut KspinLock) -> Irql,
    lib = XBOXKRNL,
    ordinal = 0x0B1
);

crate::import_thunk!(
    /// Raises the IRQL. Returns the previous IRQL.
    pub KfRaiseIrql(irql: Irql) -> Irql,
    lib = XBOXKRNL,
    ordinal = 0x0B2
);

crate::import_thunk!(
    /// Lowers the IRQL.
    pub KfLowerIrql(irql: Irql) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0B3
);

crate::import_thunk!(
    /// Releases a spin lock and restores `irql`.
    pub KfReleaseSpinLock(lock: *mut KspinLock, irql: Irql) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0B4
);

crate::import_thunk!(
    /// Allocates physical memory. Returns the address or 0.
    pub MmAllocatePhysicalMemory(kind: u32, size: u32, protect: PageProtection) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0B9
);

crate::import_thunk!(
    /// Allocates physical memory within an address range and alignment. Returns the address or 0.
    pub MmAllocatePhysicalMemoryEx(
        region: MemoryRegion,
        size: u32,
        protect: PageProtection,
        lowest: u32,
        highest: u32,
        alignment: u32,
    ) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0BA
);

crate::import_thunk!(
    /// Frees physical memory that [`MmAllocatePhysicalMemoryEx`] allocated
    /// in `region`.
    pub MmFreePhysicalMemory(region: MemoryRegion, address: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0BD
);

crate::import_thunk!(
    /// Translates a mapped virtual address to its physical address.
    pub MmGetPhysicalAddress(address: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0BE
);

crate::import_thunk!(
    /// True when an address is mapped.
    pub safe MmIsAddressValid(address: u32) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x0BF
);

crate::import_thunk!(
    /// Returns the page protection of an address.
    pub safe MmQueryAddressProtect(address: u32) -> PageProtection,
    lib = XBOXKRNL,
    ordinal = 0x0C4
);

crate::import_thunk!(
    /// Returns the size of the allocation that contains an address.
    pub MmQueryAllocationSize(address: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0C5
);

crate::import_thunk!(
    /// Fills `statistics`, whose length field the caller sets.
    pub MmQueryStatistics(statistics: *mut MmStatistics) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0C6
);

crate::import_thunk!(
    /// Reserves or commits virtual memory. Base and size are updated in place.
    pub NtAllocateVirtualMemory(
        base: *mut u32,
        size: *mut u32,
        allocation_type: AllocationType,
        protect: PageProtection,
        region: MemoryRegion,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0CC
);

crate::import_thunk!(
    /// Sets an event to not signaled.
    pub NtClearEvent(handle: Handle) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0CE
);

crate::import_thunk!(
    /// Closes a handle.
    pub NtClose(handle: Handle) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0CF
);

crate::import_thunk!(
    /// Creates an object directory.
    pub NtCreateDirectoryObject(
        handle: *mut Handle,
        attributes: *const ObjectAttributes,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D0
);

crate::import_thunk!(
    /// Creates an event.
    pub NtCreateEvent(
        handle: *mut Handle,
        attributes: *const ObjectAttributes,
        kind: EventType,
        initial_state: bool,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D1
);

crate::import_thunk_stack!(
    /// Creates or opens a file.
    pub NtCreateFile(
        handle: *mut Handle,
        access: AccessMask,
        attributes: *const ObjectAttributes,
        io_status: *mut IoStatusBlock,
        allocation_size: *const u64,
        file_attributes: FileAttributes,
        share: ShareAccess,
        disposition: CreateDisposition,
        options: CreateOptions,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D2
);

crate::import_thunk!(
    /// Creates a semaphore with a count and limit.
    pub NtCreateSemaphore(
        handle: *mut Handle,
        attributes: *const ObjectAttributes,
        count: u32,
        limit: u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D5
);

crate::import_thunk!(
    /// Deletes a file.
    pub NtDeleteFile(object_attributes: *const ObjectAttributes) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D8
);

crate::import_thunk_stack!(
    /// Sends a device I/O control request through a file handle.
    pub NtDeviceIoControlFile(
        handle: Handle,
        event: Handle,
        apc_routine: u32,
        apc_context: u32,
        io_status: *mut IoStatusBlock,
        control_code: IoControlCode,
        input: *const u8,
        input_length: u32,
        output: *mut u8,
        output_length: u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0D9
);

crate::import_thunk!(
    /// Writes buffered data of a file to the device.
    pub NtFlushBuffersFile(handle: Handle, io_status: *mut IoStatusBlock) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0DB
);

crate::import_thunk!(
    /// Decommits or releases virtual memory. Base and size are updated in place.
    pub NtFreeVirtualMemory(
        base: *mut u32,
        size: *mut u32,
        free_type: AllocationType,
        options: u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0DC
);

crate::import_thunk!(
    /// Opens an object directory.
    pub NtOpenDirectoryObject(handle: *mut Handle, attributes: *const ObjectAttributes) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0DE
);

crate::import_thunk!(
    /// Opens a file.
    pub NtOpenFile(
        handle: *mut Handle,
        access: AccessMask,
        attributes: *const ObjectAttributes,
        io_status: *mut IoStatusBlock,
        share: ShareAccess,
        options: CreateOptions,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0DF
);

crate::import_thunk_stack!(
    /// Reads directory entries. `file_name` is an optional wildcard pattern.
    pub NtQueryDirectoryFile(
        handle: Handle,
        event: Handle,
        apc_routine: u32,
        apc_context: u32,
        io_status: *mut IoStatusBlock,
        information: *mut u8,
        length: u32,
        file_name: *const AnsiString,
        restart: bool,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0E4
);

crate::import_thunk!(
    /// Reads one entry of an object directory. `context` holds the enumeration position.
    pub NtQueryDirectoryObject(
        handle: Handle,
        buffer: *mut u8,
        length: u32,
        restart: bool,
        context: *mut u32,
        return_length: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0E5
);

crate::import_thunk!(
    /// Reads the network open information of a file by name.
    pub NtQueryFullAttributesFile(
        attributes: *const ObjectAttributes,
        information: *mut u8,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0E7
);

crate::import_thunk!(
    /// Reads information of class `class` about an open file.
    pub NtQueryInformationFile(
        handle: Handle,
        io_status: *mut IoStatusBlock,
        information: *mut u8,
        length: u32,
        class: FileInformationClass,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0E8
);

crate::import_thunk!(
    /// Fills a memory basic information structure for an address.
    pub NtQueryVirtualMemory(address: u32, information: *mut u32, process_type: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0EE
);

crate::import_thunk!(
    /// Reads information of class `class` about a volume.
    pub NtQueryVolumeInformationFile(
        handle: Handle,
        io_status: *mut IoStatusBlock,
        information: *mut u8,
        length: u32,
        class: FsInformationClass,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0EF
);

crate::import_thunk!(
    /// Reads from a file. A null `byte_offset` reads at the current position. A null `event` signals the file handle.
    pub NtReadFile(
        handle: Handle,
        event: Handle,
        apc_routine: u32,
        apc_context: u32,
        io_status: *mut IoStatusBlock,
        buffer: *mut u8,
        length: u32,
        byte_offset: *const u64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F0
);

crate::import_thunk!(
    /// Reads from a file into a list of page-sized segments.
    pub NtReadFileScatter(
        handle: Handle,
        event: Handle,
        apc: u32,
        context: u32,
        io_status: *mut IoStatusBlock,
        segments: *const u32,
        length: u32,
        offset: *const u64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F1
);

crate::import_thunk!(
    /// Releases a semaphore. The previous count is stored at `previous` when not null.
    pub NtReleaseSemaphore(handle: Handle, count: u32, previous: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F3
);

crate::import_thunk!(
    /// Sets an event to signaled. The previous state is stored at `previous_state` when not null.
    pub NtSetEvent(handle: Handle, previous_state: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F6
);

crate::import_thunk!(
    /// Writes information of class `class` about an open file.
    pub NtSetInformationFile(
        handle: Handle,
        io_status: *mut IoStatusBlock,
        information: *const u8,
        length: u32,
        class: FileInformationClass,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F7
);

crate::import_thunk!(
    /// Sets the system time as a FILETIME. The previous time is stored at `previous_time` when not null.
    pub NtSetSystemTime(new_time: *const u64, previous_time: *mut u64) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0F9
);

crate::import_thunk!(
    /// Waits on a handle. A null `timeout` waits without limit.
    pub NtWaitForSingleObjectEx(
        handle: Handle,
        mode: ProcessorMode,
        alertable: bool,
        timeout: *const i64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0FD
);

crate::import_thunk!(
    /// Waits on several handles.
    pub NtWaitForMultipleObjectsEx(
        count: u32,
        handles: *const Handle,
        wait_type: WaitType,
        mode: ProcessorMode,
        alertable: bool,
        timeout: *const i64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0FE
);

crate::import_thunk!(
    /// Writes to a file. A null `byte_offset` writes at the current position. A null `event` signals the file handle.
    pub NtWriteFile(
        handle: Handle,
        event: Handle,
        apc_routine: u32,
        apc_context: u32,
        io_status: *mut IoStatusBlock,
        buffer: *const u8,
        length: u32,
        byte_offset: *const u64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x0FF
);

crate::import_thunk!(
    /// Writes to a file from a list of page-sized segments.
    pub NtWriteFileGather(
        handle: Handle,
        event: Handle,
        apc: u32,
        context: u32,
        io_status: *mut IoStatusBlock,
        segments: *const u32,
        length: u32,
        offset: *const u64,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x100
);

crate::import_thunk!(
    /// Yields the remainder of the time slice of the current thread.
    pub safe NtYieldExecution() -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x101
);

crate::import_thunk!(
    /// Allocates an object of type `object_type` with a body of `body_size` bytes.
    pub ObCreateObject(
        object_type: u32,
        attributes: *const ObjectAttributes,
        body_size: u32,
        object: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x102
);

crate::import_thunk!(
    /// Creates a symbolic link object from `link` to `target`.
    pub ObCreateSymbolicLink(link: *const AnsiString, target: *const AnsiString) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x103
);

crate::import_thunk!(
    /// Deletes the symbolic link object `link`.
    pub ObDeleteSymbolicLink(link: *const AnsiString) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x104
);

crate::import_thunk!(
    /// Releases a reference to an object.
    pub ObDereferenceObject(object: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x105
);

crate::import_thunk!(
    /// Inserts an object into the handle table and object directory. The handle is stored at `handle`.
    pub ObInsertObject(
        object: u32,
        attributes: *const ObjectAttributes,
        pointer_bias: u32,
        handle: *mut Handle,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x108
);

crate::import_thunk!(
    /// Finds a thread object by thread ID and references it.
    pub ObLookupAnyThreadByThreadId(thread_id: u32, thread: *mut *mut Kthread) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x10A
);

crate::import_thunk!(
    /// Removes the permanent flag of a named object.
    pub ObMakeTemporaryObject(object: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x10C
);

crate::import_thunk!(
    /// Opens a named object and stores a handle.
    pub ObOpenObjectByName(
        attributes: *const ObjectAttributes,
        object_type: u32,
        parse_context: u32,
        handle: *mut Handle,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x10D
);

crate::import_thunk!(
    /// References the object of a handle. `object_type` 0 accepts any type.
    pub ObReferenceObjectByHandle(handle: Handle, object_type: u32, object: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x110
);

crate::import_thunk!(
    /// References a named object.
    pub ObReferenceObjectByName(
        name: *const AnsiString,
        attributes: ObjectAttributeFlags,
        object_type: u32,
        parse_context: u32,
        object: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x111
);

crate::import_thunk!(
    /// Reads the target of a symbolic link object into `target`.
    pub ObTranslateSymbolicLink(
        attributes: *const ObjectAttributes,
        target: *mut AnsiString,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x113
);

crate::import_thunk!(
    /// Reports a failed assertion to the debugger.
    pub RtlAssert(message: *const u8, file: *const u8, line: u32, extra: *const u8) -> (),
    lib = XBOXKRNL,
    ordinal = 0x118
);

crate::import_thunk!(
    /// Stores the context of the current thread.
    pub RtlCaptureContext(context: *mut u8) -> (),
    lib = XBOXKRNL,
    ordinal = 0x119
);

crate::import_thunk!(
    /// Enters a critical section.
    pub RtlEnterCriticalSection(section: *mut RtlCriticalSection) -> (),
    lib = XBOXKRNL,
    ordinal = 0x125
);

crate::import_thunk!(
    /// Stores the low and high stack limits of the current thread.
    pub RtlGetStackLimits(low: *mut u32, high: *mut u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x12A
);

crate::import_thunk!(
    /// Returns the value or address of an optional header field of a XEX header.
    pub RtlImageXexHeaderField(header: u32, key: XexHeaderKey) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x12B
);

crate::import_thunk!(
    /// Initializes a critical section.
    pub RtlInitializeCriticalSection(section: *mut RtlCriticalSection) -> (),
    lib = XBOXKRNL,
    ordinal = 0x12E
);

crate::import_thunk!(
    /// Initializes a critical section with a spin count.
    pub RtlInitializeCriticalSectionAndSpinCount(section: *mut RtlCriticalSection, spin_count: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x12F
);

crate::import_thunk!(
    /// Leaves a critical section.
    pub RtlLeaveCriticalSection(section: *mut RtlCriticalSection) -> (),
    lib = XBOXKRNL,
    ordinal = 0x130
);

crate::import_thunk!(
    /// Converts an NTSTATUS value to a Win32 error code.
    pub safe RtlNtStatusToDosError(status: NtStatus) -> Win32ErrorCode,
    lib = XBOXKRNL,
    ordinal = 0x135
);

crate::import_thunk!(
    /// Tries to enter a critical section. True when entered.
    pub RtlTryEnterCriticalSection(section: *mut RtlCriticalSection) -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x141
);

crate::import_thunk!(
    /// Formats into `buffer` from a va_list. `arguments` is a console va_list: 8-byte slots, 32-bit values in the low word.
    pub _vsnprintf(buffer: *mut u8, count: u32, format: u32, arguments: u32) -> i32,
    lib = XBOXKRNL,
    ordinal = 0x14D
);

crate::import_thunk!(
    /// Allocates a thread local storage index. Returns [`TlsIndex::OUT_OF_INDEXES`] when none is free.
    pub safe KeTlsAlloc() -> TlsIndex,
    lib = XBOXKRNL,
    ordinal = 0x152
);

crate::import_thunk!(
    /// Frees a thread local storage index. True on success.
    pub KeTlsFree(index: TlsIndex) -> Boolean,
    lib = XBOXKRNL,
    ordinal = 0x153
);

crate::import_thunk!(
    /// Returns the value of a thread local storage index for the current thread.
    pub safe KeTlsGetValue(index: TlsIndex) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x154
);

crate::import_thunk!(
    /// Sets the value of a thread local storage index for the current thread. True on success.
    pub KeTlsSetValue(index: TlsIndex, value: u32) -> Boolean,
    lib = XBOXKRNL,
    ordinal = 0x155
);

crate::import_variable!(
    /// Console hardware information.
    pub safe XboxHardwareInfo: HardwareInfo,
    lib = XBOXKRNL,
    ordinal = 0x156
);

crate::import_variable!(
    /// Version of the base kernel.
    pub safe XboxKrnlBaseVersion: KernelVersion,
    lib = XBOXKRNL,
    ordinal = 0x157
);

crate::import_variable!(
    /// Version of the running kernel.
    pub safe XboxKrnlVersion: KernelVersion,
    lib = XBOXKRNL,
    ordinal = 0x158
);

crate::import_thunk!(
    /// Sets the parity bit of each byte of a DES key of `size` bytes.
    pub XeCryptDesParity(input: *const u8, size: u32, output: *mut u8) -> (),
    lib = XBOXKRNL,
    ordinal = 0x174
);

crate::import_thunk!(
    /// Expands an 8-byte DES key into `state`.
    pub XeCryptDesKey(state: *mut DesState, key: *const u8) -> (),
    lib = XBOXKRNL,
    ordinal = 0x175
);

crate::import_thunk!(
    /// Encrypts (`encrypt` nonzero) or decrypts one 8-byte block in ECB mode.
    pub XeCryptDesEcb(state: *const DesState, input: *const u8, output: *mut u8, encrypt: Boolean) -> (),
    lib = XBOXKRNL,
    ordinal = 0x176
);

crate::import_thunk_stack!(
    /// HMAC-SHA1 of the concatenation of up to three buffers under `key`; the first `digest_size` bytes of
    /// the digest go to `digest`. The dashboard passes null buffers of size 0 for the unused ones.
    pub XeCryptHmacSha(
        key: *const u8,
        key_size: u32,
        data1: *const u8,
        size1: u32,
        data2: *const u8,
        size2: u32,
        data3: *const u8,
        size3: u32,
        digest: *mut u8,
        digest_size: u32,
    ) -> (),
    lib = XBOXKRNL,
    ordinal = 0x182
);

crate::import_thunk!(
    /// MD5 of the concatenation of up to three buffers; the first `digest_size` bytes of the
    /// digest go to `digest`. The dashboard passes null buffers of size 0 for the unused ones.
    pub XeCryptMd5(
        data1: *const u8,
        size1: u32,
        data2: *const u8,
        size2: u32,
        data3: *const u8,
        size3: u32,
        digest: *mut u8,
        digest_size: u32,
    ) -> (),
    lib = XBOXKRNL,
    ordinal = 0x187
);

crate::import_thunk!(
    /// Fills a buffer with random bytes.
    pub XeCryptRandom(buffer: *mut u8, length: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x18A
);

crate::import_variable!(
    /// Module handle of the running executable: the address of its
    /// [`LdrDataTableEntry`].
    pub safe XexExecutableModuleHandle: KernelU32,
    lib = XBOXKRNL,
    ordinal = 0x193
);

crate::import_thunk!(
    /// Finds a loaded module by name and stores its handle.
    pub XexGetModuleHandle(name: *const u8, handle: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x195
);

crate::import_thunk!(
    /// Finds the resource `name` of a module and stores its address and size. `name` must equal the stored name, which is at most 8 bytes, byte for byte, so case is significant. `0xFFFFFFFF` as `module` names the caller's module. Fails with `STATUS_NOT_FOUND` when the module has no such resource.
    pub XexGetModuleSection(
        module: u32,
        name: *const u8,
        address: *mut u32,
        size: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x196
);

crate::import_thunk!(
    /// Finds an export of a module by ordinal and stores its address. `ordinal` may instead be the address of a NUL-terminated export name.
    pub XexGetProcedureAddress(module: u32, ordinal: u32, address: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x197
);

crate::import_thunk!(
    /// Loads a XEX image and stores its module handle. `version` is the oldest version accepted.
    pub XexLoadImage(
        name: *const u8,
        flags: ImageFlags,
        version: ImageVersion,
        handle: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x199
);

crate::import_thunk!(
    /// Finds the loader entry of the module that contains an address. Returns the image base.
    pub XexPcToFileHeader(address: u32, entry: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x19C
);

crate::import_thunk!(
    /// Unloads a XEX image.
    pub XexUnloadImage(handle: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x1A1
);

crate::import_thunk!(
    /// Returns the address of a PE data directory of an image. Its size is stored at `size`.
    pub RtlImageDirectoryEntryToData(
        base: u32,
        mapped: bool,
        directory: ImageDirectory,
        size: *mut u32,
    ) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1A8
);

crate::import_thunk!(
    /// Returns the PE NT headers of an image, or null.
    pub RtlImageNtHeader(base: u32) -> *const ImageNtHeaders,
    lib = XBOXKRNL,
    ordinal = 0x1A9
);

crate::import_thunk!(
    /// Calls a debug monitor service of the kernel.
    pub ExDebugMonitorService(service: DebugMonitorService, a: u32, b: u32, c: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x1AA
);

crate::import_thunk!(
    /// Returns the address when the debugger may read it, else 0.
    pub MmDbgReadCheck(address: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1AB
);

crate::import_thunk!(
    /// Releases a mapping that `MmDbgWriteCheck` created.
    pub MmDbgReleaseAddress(address: u32, context: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1AC
);

crate::import_thunk!(
    /// Maps `address` writable for the debugger. Returns the mapped address, else 0.
    pub MmDbgWriteCheck(address: u32, context: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1AD
);

crate::import_variable!(
    /// Object path of the running executable.
    pub safe ExLoadedImageName: LoadedImageName,
    lib = XBOXKRNL,
    ordinal = 0x1AF
);

crate::import_thunk!(
    /// Makes the command processor write its ring read pointer, in dwords, to `physical` (dword
    /// aligned). `block_log2` is the read pointer block size field of the ring control register.
    pub VdEnableRingBufferRPtrWriteBack(physical: u32, block_log2: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1B6
);

crate::import_thunk!(
    /// Fills a display information structure.
    pub VdGetCurrentDisplayInformation(information: *mut DisplayInformation) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1BA
);

crate::import_thunk!(
    /// Copies the next of the kernel's three system command buffer descriptors to `buffer` and stores in
    /// `identifier` the value the GPU writes to the identifier address once that buffer has run.
    pub VdGetSystemCommandBuffer(buffer: *mut SystemCommandBuffer, identifier: *mut u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1BD
);

crate::import_thunk!(
    /// Gives the GPU command processor to the calling title: loads the CP microcode (the kernel's own
    /// when both pointers are null), stores `shutdown_callback`/`context` for [`VdShutdownEngines`] and
    /// resets the engines. `unused` is not read. Returns 1. Traps when the engines are already owned.
    pub VdInitializeEngines(
        unused: u32,
        shutdown_callback: u32,
        context: u32,
        prefetch_microcode: *const u32,
        engine_microcode: *const u32,
    ) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1C2
);

crate::import_thunk!(
    /// Points the command processor ring at `physical` with 2^(`size_log2` + 1) dwords and resets its
    /// read and write pointers.
    pub VdInitializeRingBuffer(physical: u32, size_log2: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1C3
);

crate::import_thunk!(
    /// Reads a video encoder register.
    pub VdReadDVERegisterUlong(register: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1CB
);

crate::import_thunk!(
    /// Programs the configured video mode. `flags` selects display-off,
    /// forced-output, color-space, and pixel-format options.
    pub VdSetDisplayMode(flags: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1D3
);

import_adapter!(
    /// Overrides the display mode; nonzero on success. `scan` is 1 for
    /// interlaced and 2 for progressive [INFERENCE]; `parameters` may be
    /// null. All zero arguments remove the override.
    pub VdSetDisplayModeOverride(
        width: u32,
        height: u32,
        refresh_rate: f64,
        scan: u32,
        parameters: *const DisplayModeOverrideParameters,
    ) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1D4,
    // Both ABIs pass the double in f1. The console ABI also reserves r5 for
    // it, so `scan` and `parameters` move from r5:r6 to r6:r7.
    prologue = [
        "mr 7, 6",
        "mr 6, 5",
    ]
);

crate::import_thunk!(
    /// Sets the routine the graphics interrupt handler calls with (0, `context`), or none when 0.
    pub VdSetGraphicsInterruptCallback(callback: u32, context: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1D5
);

crate::import_thunk!(
    /// Sets the virtual address of the dword where the GPU writes the identifier of each finished
    /// system command buffer, or none when 0.
    pub VdSetSystemCommandBufferGpuIdentifierAddress(address: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1D9
);

crate::import_thunk!(
    /// Waits for the GPU to idle, returns the command processor to the kernel's own ring and calls the
    /// shutdown callback of [`VdInitializeEngines`].
    pub VdShutdownEngines() -> (),
    lib = XBOXKRNL,
    ordinal = 0x1DC
);

crate::import_thunk!(
    /// Writes a video encoder register.
    pub VdWriteDVERegisterUlong(register: u32, value: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1E0
);

crate::import_thunk!(
    /// Turns the controller radio on or off.
    pub XInputdSetRFPowerMode(enable: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1EF
);

crate::import_thunk!(
    /// Registers an audio render client. The client handle is stored at `handle`.
    pub XAudioRegisterRenderDriverClient(client: *const u32, handle: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1F3
);

crate::import_thunk!(
    /// Removes an audio render client.
    pub XAudioUnregisterRenderDriverClient(handle: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1F4
);

crate::import_thunk!(
    /// Submits a frame of samples for an audio render client.
    pub XAudioSubmitRenderDriverFrame(client: u32, samples: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1F5
);

crate::import_thunk!(
    /// Returns the audio render driver thread object.
    pub XAudioGetRenderDriverThread() -> *mut Kthread,
    lib = XBOXKRNL,
    ordinal = 0x1FE
);

crate::import_thunk!(
    /// Sets the speaker configuration (the XConfig audio flags value).
    pub XAudioSetSpeakerConfig(config: u32, reserved: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x200
);

crate::import_thunk!(
    /// Fills a network interface statistics structure.
    pub NicGetStats(device_flags: u32, stats: *mut u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x209
);

crate::import_thunk!(
    /// State of the USB camera driver. The dashboard reads 2 as ready.
    pub safe XUsbcamGetState() -> u32,
    lib = XBOXKRNL,
    ordinal = 0x21D
);
crate::import_thunk!(
    /// Creates the USB camera driver context in `workspace` and stores its
    /// camera handle. Returns zero on success.
    pub XUsbcamCreate(workspace: *mut u8, workspace_size: u32, camera: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x222
);

crate::import_thunk!(
    /// Destroys a USB camera driver context.
    pub XUsbcamDestroy(camera: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x223
);

crate::import_thunk!(
    /// Allocates one hardware XMA decoder context.
    pub XMACreateContext(context: *mut *mut XmaContextData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x224
);

crate::import_thunk!(
    /// Initializes an allocated XMA decoder context.
    pub XMAInitializeContext(context: *mut XmaContextData, init: *const XmaContextInit) -> (),
    lib = XBOXKRNL,
    ordinal = 0x225
);

crate::import_thunk!(
    /// Releases a hardware XMA decoder context.
    pub XMAReleaseContext(context: *mut XmaContextData) -> (),
    lib = XBOXKRNL,
    ordinal = 0x226
);

crate::import_thunk!(
    /// Enables a hardware XMA decoder context.
    pub XMAEnableContext(context: *mut XmaContextData) -> (),
    lib = XBOXKRNL,
    ordinal = 0x227
);

crate::import_thunk!(
    /// Disables a hardware XMA decoder context. `wait` waits for in-flight
    /// decoder work.
    pub XMADisableContext(context: *mut XmaContextData, wait: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x228
);

crate::import_thunk!(
    /// Returns the decoder's output ring write offset in 256-byte blocks.
    pub XMAGetOutputBufferWriteOffset(context: *mut XmaContextData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x229
);

crate::import_thunk!(
    /// Sets the output ring read offset in 256-byte blocks.
    pub XMASetOutputBufferReadOffset(context: *mut XmaContextData, offset: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x22A
);

crate::import_thunk!(
    /// Returns the output ring read offset in 256-byte blocks.
    pub XMAGetOutputBufferReadOffset(context: *mut XmaContextData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x22B
);

crate::import_thunk!(
    /// Marks the output ring of an XMA decoder context valid.
    pub XMASetOutputBufferValid(context: *mut XmaContextData) -> (),
    lib = XBOXKRNL,
    ordinal = 0x22C
);

crate::import_thunk!(
    /// Returns whether the output ring is valid.
    pub XMAIsOutputBufferValid(context: *mut XmaContextData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x22D
);

crate::import_thunk!(
    /// Marks input buffer zero of an XMA decoder context valid.
    pub XMASetInputBuffer0Valid(context: *mut XmaContextData) -> (),
    lib = XBOXKRNL,
    ordinal = 0x22E
);

crate::import_thunk!(
    /// Returns whether input buffer zero is valid.
    pub XMAIsInputBuffer0Valid(context: *mut XmaContextData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x22F
);

crate::import_thunk!(
    /// Waits until the decoder is not working on `context`.
    pub XMABlockWhileInUse(context: *mut XmaContextData) -> (),
    lib = XBOXKRNL,
    ordinal = 0x235
);

crate::import_thunk!(
    /// Writes one USB camera configuration value.
    pub XUsbcamSetConfig(
        camera: u32,
        request: u32,
        selector: u32,
        value: *const u8,
        length: u32,
        callback: usize,
        context: usize,
    ) -> i32,
    lib = XBOXKRNL,
    ordinal = 0x21C
);
crate::import_thunk!(
    /// Writes a boot loader image to flash. A nonzero `flags` value on return also indicates failure.
    pub XeKeysSaveBootLoader(flags: *mut u32, size: *mut u32, buffer: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x23D
);

crate::import_thunk!(
    /// Stores the security key status flags.
    pub XeKeysGetStatus(status: *mut u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x23F
);

crate::import_thunk!(
    /// Copies the key `key` into `buffer`; `size` holds the buffer size on entry and the key size on return.
    /// Key 0x14 is the 12-digit console serial number.
    pub XeKeysGetKey(key: u16, buffer: *mut u8, size: *mut u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x244
);

crate::import_thunk!(
    /// Stores the 5-byte console ID and, when `text` is not null, its 12-character text form.
    pub XeKeysGetConsoleID(raw: *mut u8, text: *mut u8) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x246
);

crate::import_thunk_stack!(
    /// Writes a 64-dword front buffer flip into `commands`: when `system_buffer` holds a buffer, fetch
    /// constant 15 from the 6-dword `fetch_constant` (base address virtual) and an indirect buffer call
    /// of it, then an event write of `identifier` to `identifier_address` (virtual), padded with a NOP.
    /// The five outputs are written only when `system_buffer` carries optional state.
    pub VdSwap(
        commands: *mut u32,
        fetch_constant: *const u32,
        identifier_address: u32,
        system_buffer: *const SystemCommandBuffer,
        identifier: u32,
        out0: *mut u32,
        out1: *mut u32,
        out2: *mut u32,
        out3: *mut u32,
        out4: *mut u32,
    ) -> (),
    lib = XBOXKRNL,
    ordinal = 0x25B
);

crate::import_thunk!(
    /// True while the power-down deferred procedure call runs.
    pub safe HalIsExecutingPowerDownDpc() -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x267
);

crate::import_thunk!(
    /// A step of EDRAM retraining. Returns 0 when nothing is due:
    /// `VdHSIOTrainCount` (export 0x26B) is nonzero and `flags` has bit 1
    /// clear. In state 0 it sets `*state` to 1, returns 1 and leaves
    /// `*commands` alone, so a call with a null `*commands` asks whether
    /// retraining is due. D3D then calls it with `*commands` at free ring
    /// space until it returns 0, and takes the advanced `*commands` as the
    /// end of what was written. Returns 0 without effect when `*state` is
    /// above 7, `capacity` (dwords) below 0x928 or `work_bytes` below 0x5C.
    /// A null `state` traps.
    pub VdRetrainEDRAM(flags: u32, commands: *mut *mut u32, capacity: u32, state: *mut u32, work: *mut u8, work_bytes: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x269
);

crate::import_thunk!(
    /// When `VdHSIOTrainCount` is 0: sends message 7 to the system
    /// management controller and stores byte 6 of the reply as the current
    /// EDRAM temperature of the retraining state. Returns 0.
    pub VdRetrainEDRAMWorker(reserved: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x26A
);

crate::import_thunk!(
    /// Calls `callback` with each display mode the attached display
    /// supports; the callback returns nonzero to continue.
    pub VdEnumerateVideoModes(callback: u32, context: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x270
);

crate::import_thunk!(
    /// Fills `data` with the display discovery result; nonzero on success.
    pub VdGetDisplayDiscoveryData(data: *mut DisplayDiscoveryData) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x27E
);

crate::import_thunk!(
    /// Replaces a 4-byte configuration setting with `(old & and_mask) | or_mask`.
    /// The 16-bit arguments pass zero-extended in full registers.
    pub ExReadModifyWriteXConfigSettingUlong(category: u16, setting: u16, and_mask: u32, or_mask: u32) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x2BC
);
