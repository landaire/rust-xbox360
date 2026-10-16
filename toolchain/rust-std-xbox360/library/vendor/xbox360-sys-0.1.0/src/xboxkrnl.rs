//! xboxkrnl.exe imports, kernel structures, and the types and constants of
//! their arguments.

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::lib_index::XBOXKRNL;
pub use crate::status::NtStatus;
pub use crate::status::Win32ErrorCode;

/// A kernel object handle as the kernel passes it. [`Handle::NULL`] is the
/// kernel's own "no handle": optional handle arguments take it, and calls
/// that fail may store it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Handle(u32);

impl Handle {
    pub const NULL: Self = Self(0);
    /// Pseudo handle of the current process.
    pub const CURRENT_PROCESS: Self = Self(0xffff_ffff);
    /// Pseudo handle of the current thread.
    pub const CURRENT_THREAD: Self = Self(0xffff_fffe);

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}

/// A `BOOL` result: nonzero is true.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boolean(pub u32);

impl Boolean {
    pub const fn get(self) -> bool {
        self.0 != 0
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> bool {
        value.get()
    }
}

/// A `BOOLEAN` result. The export returns one byte in a full register, so
/// only the low byte is the value: nonzero is true.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boolean8(pub u32);

impl Boolean8 {
    pub const fn get(self) -> bool {
        self.0 as u8 != 0
    }
}

impl From<Boolean8> for bool {
    fn from(value: Boolean8) -> bool {
        value.get()
    }
}

/// Define zeroed storage of `$size` bytes for a kernel object that the
/// caller allocates and a kernel call initializes.
macro_rules! kernel_object {
    ($(#[$meta:meta])* pub struct $name:ident(size = $size:literal, align = $align:literal);) => {
        $(#[$meta])*
        ///
        /// Rust code reaches the storage only through [`Self::as_ptr`], and
        /// the kernel serializes its own accesses. An initialized object
        /// holds addresses that point into itself, so it must not move.
        #[repr(C, align($align))]
        pub struct $name(core::cell::UnsafeCell<[u8; $size]>);

        // SAFETY: the storage is reached only through raw pointers, and the
        // kernel serializes its accesses to the object.
        unsafe impl Sync for $name {}

        impl $name {
            /// Zeroed storage for a kernel call to initialize.
            pub const fn new() -> Self {
                Self(core::cell::UnsafeCell::new([0; $size]))
            }

            /// The address that kernel calls take.
            pub const fn as_ptr(&self) -> *mut Self {
                self.0.get().cast()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(core::stringify!($name)).finish_non_exhaustive()
            }
        }

        const _: () = assert!(core::mem::size_of::<$name>() == $size);
    };
}

kernel_object! {
    /// `KEVENT` storage (0x10 bytes): a dispatcher header.
    pub struct Kevent(size = 0x10, align = 4);
}

kernel_object! {
    /// `KSEMAPHORE` storage (0x14 bytes): a dispatcher header and the limit.
    pub struct Ksemaphore(size = 0x14, align = 4);
}

kernel_object! {
    /// `KTIMER` storage (0x28 bytes): a dispatcher header, the 64-bit due
    /// time, the timer list entry, the DPC and the period.
    pub struct Ktimer(size = 0x28, align = 8);
}

kernel_object! {
    /// `KDPC` storage (0x1c bytes).
    pub struct Kdpc(size = 0x1c, align = 4);
}

kernel_object! {
    /// `RTL_CRITICAL_SECTION` storage (0x1c bytes): a dispatcher header,
    /// the lock count, the recursion count and the owning thread.
    pub struct RtlCriticalSection(size = 0x1c, align = 4);
}

kernel_object! {
    /// `KSPIN_LOCK` storage (one word, 0 when free).
    pub struct KspinLock(size = 4, align = 4);
}

/// `KTHREAD`: a thread object that the kernel allocates. Rust code handles
/// only its address.
#[repr(C)]
pub struct Kthread {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// Any dispatcher object a wait takes: a [`Kevent`], [`Ksemaphore`],
/// [`Ktimer`], [`Kthread`] or another object that starts with a dispatcher
/// header. Cast the address of the object to it.
#[repr(C)]
pub struct DispatcherObject {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

/// An interrupt request level (`KIRQL`). The kernel passes it in a full
/// register; only the low byte is the level, and comparisons use only it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Irql(u32);

impl PartialEq for Irql {
    fn eq(&self, other: &Self) -> bool {
        self.level() == other.level()
    }
}

impl Eq for Irql {}

impl PartialOrd for Irql {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Irql {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

impl core::hash::Hash for Irql {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.level().hash(state);
    }
}

impl Irql {
    pub const PASSIVE: Self = Self(0);
    pub const APC: Self = Self(1);
    pub const DISPATCH: Self = Self(2);

    pub const fn new(level: u8) -> Self {
        Self(level as u32)
    }

    /// The level in the low byte of a register value.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn level(self) -> u8 {
        self.0 as u8
    }
}

value_enum! {
    /// `KPROCESSOR_MODE`: the mode a wait is made in.
    pub enum ProcessorMode: u32 {
        Kernel = 0,
        User = 1,
    }
}

value_enum! {
    /// `KWAIT_REASON`: why a thread waits. Xenia ignores it. 3 is the value
    /// stock XBDM passes for waits on behalf of a request; the original
    /// Xbox kernel numbers it `PoolAllocation` and has `UserRequest` at 6.
    #[non_exhaustive]
    pub enum WaitReason: u32 {
        Executive = 0,
        UserRequest = 3,
    }
}

value_enum! {
    /// `WAIT_TYPE` of a wait on several objects.
    pub enum WaitType: u32 {
        /// Wait until every object is signaled.
        All = 0,
        /// Wait until any object is signaled. The status is
        /// `STATUS_WAIT_0` plus the index of the object.
        Any = 1,
    }
}

value_enum! {
    /// `EVENT_TYPE`, which is also the dispatcher object type of an event.
    pub enum EventType: u32 {
        /// Stays signaled until reset and releases every waiter.
        Notification = 0,
        /// Releases one waiter and resets itself.
        Synchronization = 1,
    }
}

value_enum! {
    /// `TIMER_TYPE`.
    pub enum TimerType: u32 {
        /// Stays signaled until set again and releases every waiter.
        Notification = 0,
        /// Releases one waiter and resets itself.
        Synchronization = 1,
    }
}

/// `KeSetEvent` priority increment for the waiters an event releases.
pub const EVENT_INCREMENT: u32 = 1;

/// Process type of a thread, as `KeGetCurrentProcessType` reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessType(pub u32);

impl ProcessType {
    pub const IDLE: Self = Self(0);
    /// The title process (Xenia's `X_PROCTYPE_USER`).
    pub const TITLE: Self = Self(1);
    pub const SYSTEM: Self = Self(2);
}

value_enum! {
    /// Pool of an `ExAllocatePoolTypeWithTag` block. Xenia ignores the
    /// argument. Stock XBDM passes the process type of the thread that owns
    /// the block, and 0 for blocks of no process.
    #[non_exhaustive]
    pub enum PoolType: u32 {
        Default = 0,
        Title = 1,
        System = 2,
    }
}

/// Four-character tag that identifies the owner of pool blocks.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PoolTag(u32);

impl PoolTag {
    /// The tag that reads as `characters` in memory.
    pub const fn new(characters: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(characters))
    }

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

flags! {
    /// `ExCreateThread` creation flags. Bits 24 to 29 select the hardware
    /// threads the new thread may run on; see [`ThreadCreationFlags::processors`].
    pub struct ThreadCreationFlags(u32) {
        /// Create the thread suspended.
        SUSPENDED = 0x1,
        /// Create the thread in the system process.
        SYSTEM_PROCESS = 0x2,
        /// Store a referenced thread object instead of a handle.
        RETURN_THREAD_OBJECT = 0x80,
        PROCESSOR0 = 0x0100_0000,
        PROCESSOR1 = 0x0200_0000,
        PROCESSOR2 = 0x0400_0000,
        PROCESSOR3 = 0x0800_0000,
        PROCESSOR4 = 0x1000_0000,
        PROCESSOR5 = 0x2000_0000,
    }
}

impl ThreadCreationFlags {
    /// The `PROCESSOR<n>` flags of the hardware threads in `mask` (bit n is
    /// hardware thread n, 0 to 5). Higher bits of `mask` are ignored.
    pub const fn processors(mask: u8) -> Self {
        Self(((mask & 0x3f) as u32) << 24)
    }
}

/// A thread local storage index, or [`TlsIndex::OUT_OF_INDEXES`] from
/// `KeTlsAlloc`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlsIndex(pub u32);

impl TlsIndex {
    pub const OUT_OF_INDEXES: Self = Self(0xffff_ffff);
}

value_enum! {
    /// `HalReturnToFirmware` routine.
    #[non_exhaustive]
    pub enum FirmwareRequest: u32 {
        /// `HalRebootRoutine`, the routine Xenia asserts on.
        Reboot = 1,
        /// Reset through the system management controller.
        SmcReset = 4,
        /// Restart the system software. Stock XBDM uses it for a reboot.
        RebootQuiesce = 6,
    }
}

value_enum! {
    /// `ExDebugMonitorService` service. The kernel exports it for the debug
    /// monitor; the names describe what stock XBDM uses each one for.
    #[non_exhaustive]
    pub enum DebugMonitorService: u32 {
        /// Register the debug monitor data block.
        Register = 0,
        /// Query the debug monitor state.
        Query = 1,
        /// Turn the capture of kernel calls on or off.
        KernelCallCapture = 2,
        /// Return the CD-ROM channel extension and its packet routines.
        CdromChannel = 4,
        /// Return the address of the trace exception hook.
        TraceHook = 6,
        /// Select hypervisor DVD emulation.
        DvdEmulation = 7,
        /// Fill debug-adjusted memory statistics.
        MemoryStatistics = 8,
    }
}

flags! {
    /// `ACCESS_MASK` of a file or object open.
    pub struct AccessMask(u32) {
        /// `FILE_READ_DATA`, or `FILE_LIST_DIRECTORY` for a directory.
        READ_DATA = 0x1,
        /// `FILE_WRITE_DATA`, or `FILE_ADD_FILE` for a directory.
        WRITE_DATA = 0x2,
        /// `FILE_APPEND_DATA`, or `FILE_ADD_SUBDIRECTORY` for a directory.
        APPEND_DATA = 0x4,
        READ_EA = 0x8,
        WRITE_EA = 0x10,
        /// `FILE_EXECUTE`, or `FILE_TRAVERSE` for a directory.
        EXECUTE = 0x20,
        DELETE_CHILD = 0x40,
        READ_ATTRIBUTES = 0x80,
        WRITE_ATTRIBUTES = 0x100,
        DELETE = 0x1_0000,
        READ_CONTROL = 0x2_0000,
        WRITE_DAC = 0x4_0000,
        WRITE_OWNER = 0x8_0000,
        SYNCHRONIZE = 0x10_0000,
        MAXIMUM_ALLOWED = 0x200_0000,
        GENERIC_ALL = 0x1000_0000,
        GENERIC_EXECUTE = 0x2000_0000,
        GENERIC_WRITE = 0x4000_0000,
        GENERIC_READ = 0x8000_0000,
    }
}

impl AccessMask {
    /// `FILE_LIST_DIRECTORY`.
    pub const LIST_DIRECTORY: Self = Self::READ_DATA;
    /// `FILE_TRAVERSE`.
    pub const TRAVERSE: Self = Self::EXECUTE;
}

flags! {
    /// `FILE_SHARE_*`: what other opens of a file may do.
    pub struct ShareAccess(u32) {
        READ = 0x1,
        WRITE = 0x2,
        DELETE = 0x4,
    }
}

flags! {
    /// `FILE_ATTRIBUTE_*`.
    pub struct FileAttributes(u32) {
        READONLY = 0x1,
        HIDDEN = 0x2,
        SYSTEM = 0x4,
        DIRECTORY = 0x10,
        ARCHIVE = 0x20,
        DEVICE = 0x40,
        NORMAL = 0x80,
        TEMPORARY = 0x100,
        COMPRESSED = 0x800,
        ENCRYPTED = 0x4000,
    }
}

value_enum! {
    /// `NtCreateFile` create disposition: what to do when the file exists
    /// or does not.
    pub enum CreateDisposition: u32 {
        /// Replace the file if it exists, create it otherwise.
        Supersede = 0,
        /// Open the file; fail if it does not exist.
        Open = 1,
        /// Create the file; fail if it exists.
        Create = 2,
        /// Open the file, or create it if it does not exist.
        OpenIf = 3,
        /// Open and truncate the file; fail if it does not exist.
        Overwrite = 4,
        /// Open and truncate the file, or create it if it does not exist.
        OverwriteIf = 5,
    }
}

flags! {
    /// `FILE_*` create options.
    pub struct CreateOptions(u32) {
        DIRECTORY_FILE = 0x1,
        WRITE_THROUGH = 0x2,
        SEQUENTIAL_ONLY = 0x4,
        NO_INTERMEDIATE_BUFFERING = 0x8,
        SYNCHRONOUS_IO_ALERT = 0x10,
        SYNCHRONOUS_IO_NONALERT = 0x20,
        NON_DIRECTORY_FILE = 0x40,
        RANDOM_ACCESS = 0x800,
        DELETE_ON_CLOSE = 0x1000,
        OPEN_BY_FILE_ID = 0x2000,
        OPEN_FOR_BACKUP_INTENT = 0x4000,
    }
}

value_enum! {
    /// `FILE_INFORMATION_CLASS` of `NtQueryInformationFile` and
    /// `NtSetInformationFile`, with Xenia's values.
    #[non_exhaustive]
    pub enum FileInformationClass: u32 {
        Directory = 1,
        FullDirectory = 2,
        BothDirectory = 3,
        Basic = 4,
        Standard = 5,
        Internal = 6,
        Ea = 7,
        Access = 8,
        Name = 9,
        Rename = 10,
        Link = 11,
        Names = 12,
        Disposition = 13,
        Position = 14,
        FullEa = 15,
        Mode = 16,
        Alignment = 17,
        All = 18,
        Allocation = 19,
        EndOfFile = 20,
        AlternateName = 21,
        Stream = 22,
        MountPartition = 23,
        MountPartitions = 24,
        PipeRemote = 25,
        Sector = 26,
        XctdCompression = 27,
        Compression = 28,
        ObjectId = 29,
        Completion = 30,
        MoveCluster = 31,
        IoPriority = 32,
        ReparsePoint = 33,
        NetworkOpen = 34,
        AttributeTag = 35,
        Tracking = 36,
    }
}

value_enum! {
    /// `FS_INFORMATION_CLASS` of `NtQueryVolumeInformationFile`, with
    /// Xenia's values.
    #[non_exhaustive]
    pub enum FsInformationClass: u32 {
        Volume = 1,
        Size = 3,
        Device = 4,
        Attribute = 5,
    }
}

flags! {
    /// `MEM_*` allocation and free types of virtual memory calls.
    pub struct AllocationType(u32) {
        COMMIT = 0x1000,
        RESERVE = 0x2000,
        DECOMMIT = 0x4000,
        RELEASE = 0x8000,
        FREE = 0x1_0000,
        PRIVATE = 0x2_0000,
        RESET = 0x8_0000,
        TOP_DOWN = 0x10_0000,
        NOZERO = 0x80_0000,
        /// 64 KB pages.
        LARGE_PAGES = 0x2000_0000,
        HEAP = 0x4000_0000,
        /// 16 MB pages.
        PAGES_16MB = 0x8000_0000,
    }
}

flags! {
    /// `PAGE_*` page protection. Physical memory calls take the page size
    /// bits of [`AllocationType`] in the same word.
    pub struct PageProtection(u32) {
        NOACCESS = 0x1,
        READONLY = 0x2,
        READWRITE = 0x4,
        WRITECOPY = 0x8,
        EXECUTE = 0x10,
        EXECUTE_READ = 0x20,
        EXECUTE_READWRITE = 0x40,
        EXECUTE_WRITECOPY = 0x80,
        GUARD = 0x100,
        NOCACHE = 0x200,
        WRITECOMBINE = 0x400,
        /// 64 KB pages.
        LARGE_PAGES = 0x2000_0000,
        /// 16 MB pages.
        PAGES_16MB = 0x8000_0000,
    }
}

flags! {
    /// `ObjectAttributes::attributes` and object lookup flags (`OBJ_*`).
    pub struct ObjectAttributeFlags(u32) {
        INHERIT = 0x2,
        PERMANENT = 0x10,
        EXCLUSIVE = 0x20,
        CASE_INSENSITIVE = 0x40,
        OPENIF = 0x80,
        OPENLINK = 0x100,
        KERNEL_HANDLE = 0x200,
    }
}

flags! {
    /// XEX module flags of `XexLoadImage`, with the names of Xenia's
    /// `xex2_module_flags`.
    pub struct ImageFlags(u32) {
        TITLE = 0x1,
        EXPORTS_TO_TITLE = 0x2,
        SYSTEM_DEBUGGER = 0x4,
        DLL = 0x8,
        MODULE_PATCH = 0x10,
        PATCH_FULL = 0x20,
        PATCH_DELTA = 0x40,
        USER_MODE = 0x80,
    }
}

/// A packed XEX version: 4, 4, 16 and 8 bits of major, minor, build and
/// qfe from the top. As a minimum version, [`ImageVersion::ANY`] accepts
/// every image.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageVersion(u32);

impl ImageVersion {
    pub const ANY: Self = Self(0);

    /// `None` if `major` or `minor` exceeds 15.
    pub const fn new(major: u8, minor: u8, build: u16, qfe: u8) -> Option<Self> {
        if major > 0xf || minor > 0xf {
            return None;
        }
        Some(Self(
            ((major as u32) << 28) | ((minor as u32) << 24) | ((build as u32) << 8) | qfe as u32,
        ))
    }

    pub const fn from_packed(packed: u32) -> Self {
        Self(packed)
    }

    pub const fn packed(self) -> u32 {
        self.0
    }
}

flags! {
    /// The flags word of `XboxHardwareInfo`. The top four bits hold the
    /// [`HardwareGeneration`].
    pub struct HardwareFlags(u32) {
        HARD_DISK = 0x20,
        KERNEL_DEBUGGER = 0x100,
        /// The console has 1 GB of memory, as some development kits do.
        ONE_GIGABYTE = 0x0001_0000,
        NOT_DEBUGGABLE = 0x0200_0000,
    }
}

impl HardwareFlags {
    /// The hardware generation in the top four bits.
    pub const fn generation(self) -> HardwareGeneration {
        HardwareGeneration((self.0 >> 28) as u8)
    }
}

/// Hardware generation number from [`HardwareFlags`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HardwareGeneration(u8);

impl HardwareGeneration {
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Counted ANSI string.
#[repr(C)]
pub struct AnsiString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *const u8,
}

impl AnsiString {
    /// String that refers to `bytes`. `None` if `bytes` is longer than
    /// `u16::MAX`.
    pub const fn new(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > u16::MAX as usize {
            return None;
        }
        Some(Self {
            length: bytes.len() as u16,
            maximum_length: bytes.len() as u16,
            buffer: bytes.as_ptr(),
        })
    }
}

/// Object attributes. The console layout has no length or security fields.
#[repr(C)]
pub struct ObjectAttributes {
    pub root_directory: Handle,
    pub object_name: *const AnsiString,
    pub attributes: ObjectAttributeFlags,
}

/// I/O status block.
#[repr(C)]
pub struct IoStatusBlock {
    pub status: NtStatus,
    pub information: u32,
}

impl Default for IoStatusBlock {
    fn default() -> Self {
        Self {
            status: NtStatus::SUCCESS,
            information: 0,
        }
    }
}

#[cfg(target_pointer_width = "32")]
const _: () = assert!(core::mem::size_of::<ObjectAttributes>() == 12);
const _: () = assert!(core::mem::size_of::<IoStatusBlock>() == 8);

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
    /// Registers or removes a thread creation and deletion callback.
    /// `registration` is `{routine, priority, 0, 0}` and must stay valid while registered.
    pub ExRegisterThreadNotification(registration: *mut u32, register: bool) -> (),
    lib = XBOXKRNL,
    ordinal = 0x014
);

crate::import_thunk!(
    /// Registers or removes a title termination callback.
    /// `registration` is `{routine, priority, 0, 0}` and must stay valid while registered.
    pub ExRegisterTitleTerminateNotification(registration: *mut u32, register: bool) -> (),
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
    /// Object type descriptor of thread objects.
    pub ExThreadObjectType,
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
    /// Opens or closes the optical drive tray.
    pub HalOpenCloseODDTray(value: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x024
);

crate::import_thunk!(
    /// Registers or removes a system management controller event callback.
    /// `registration` is `{routine, priority, 0, 0}` and must stay valid while registered.
    pub HalRegisterSMCNotification(registration: *mut u32, register: bool) -> u32,
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
    pub HalSendSMCMessage(message: *const u8, reply: *mut u8) -> u32,
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
    /// Allocates an I/O request packet with `stack_size` stack locations. Returns its address or 0.
    pub IoAllocateIrp(stack_size: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x02F
);

crate::import_thunk!(
    /// Completes an I/O request packet.
    pub IoCompleteRequest(irp: u32, priority_boost: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x035
);

crate::import_thunk!(
    /// Creates a device object for a driver. The device object address is stored at `device`.
    pub IoCreateDevice(
        driver: u32,
        extension_size: u32,
        name: *const AnsiString,
        device_type: u32,
        options: u32,
        device: *mut u32,
    ) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x037
);

crate::import_variable!(
    /// Object type descriptor of device objects.
    pub IoDeviceObjectType,
    lib = XBOXKRNL,
    ordinal = 0x03A
);

crate::import_thunk!(
    /// Dismounts the file system volume on a device object.
    pub IoDismountVolume(device: u32) -> NtStatus,
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
    pub IoInvalidDeviceRequest(device: u32, irp: u32) -> NtStatus,
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
    pub IoStartNextPacket(device: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x048
);

crate::import_thunk!(
    /// Starts or queues an I/O request packet on a device.
    pub IoStartPacket(device: u32, irp: u32, key: *const u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x04A
);

crate::import_thunk!(
    /// Sends a device I/O control request to a device object and waits for it.
    pub IoSynchronousDeviceIoControlRequest(
        code: u32,
        device: u32,
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
    /// Sends a read or write request to a device object and waits for it. `major` is the IRP major function.
    pub IoSynchronousFsdRequest(
        major: u32,
        device: u32,
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
    pub safe KeBugCheck(code: u32) -> !,
    lib = XBOXKRNL,
    ordinal = 0x052
);

crate::import_thunk!(
    /// Halts the system with a bug check code and four parameters.
    pub safe KeBugCheckEx(
        code: u32,
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
    /// Pointer to the debug monitor data block that the kernel reads.
    pub KeDebugMonitorData,
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
    pub KeGetPMWRegister(register: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x067
);

crate::import_thunk!(
    /// Reads a special purpose register. Only the low word of the result is returned.
    pub KeGetSpecialPurposeRegister(register: u32) -> u32,
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
    pub KeSetPMWRegister(register: u32, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x09F,
    // register in r3, value in r5:r6.
    prologue = [
        ".long 0x78a6000e", // rldimi 6, 5, 32, 0
        "mr 4, 6",
    ]
);

import_adapter!(
    /// Writes the processor version register field. The export takes the 64-bit `value` in one register.
    pub KeSetPRVRegister(register: u32, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A1,
    // register in r3, value in r5:r6.
    prologue = [
        ".long 0x78a6000e", // rldimi 6, 5, 32, 0
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
    pub KeSetSpecialPurposeRegister(register: u32, value: u64) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x0A5,
    // register in r3, value in r5:r6.
    prologue = [
        ".long 0x78a6000e", // rldimi 6, 5, 32, 0
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
        ".long 0x78a6000e", // rldimi 6, 5, 32, 0
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
    /// Kernel time stamp bundle. The dword at +0x10 is the tick count in milliseconds.
    pub KeTimeStampBundle,
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
        kind: u32,
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
    /// Frees physical memory.
    pub MmFreePhysicalMemory(kind: u32, address: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x0BD
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
    /// Fills a memory statistics structure. The first dword is the structure length.
    pub MmQueryStatistics(statistics: *mut u32) -> NtStatus,
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
        options: u32,
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
        control_code: u32,
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
    pub RtlImageXexHeaderField(header: u32, key: u32) -> u32,
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
    /// Console hardware information. The first dword is the flags word.
    pub XboxHardwareInfo,
    lib = XBOXKRNL,
    ordinal = 0x156
);

crate::import_variable!(
    /// Version of the base kernel: four u16 (major, minor, build, qfe).
    pub XboxKrnlBaseVersion,
    lib = XBOXKRNL,
    ordinal = 0x157
);

crate::import_variable!(
    /// Version of the running kernel: four u16 (major, minor, build, qfe).
    pub XboxKrnlVersion,
    lib = XBOXKRNL,
    ordinal = 0x158
);

crate::import_thunk!(
    /// Fills a buffer with random bytes.
    pub XeCryptRandom(buffer: *mut u8, length: u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x18A
);

crate::import_variable!(
    /// Pointer to the module handle of the running executable.
    pub XexExecutableModuleHandle,
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
        directory: u32,
        size: *mut u32,
    ) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1A8
);

crate::import_thunk!(
    /// Returns the address of the PE NT headers of an image.
    pub RtlImageNtHeader(base: u32) -> u32,
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
    /// NUL-terminated object path of the running executable.
    pub ExLoadedImageName,
    lib = XBOXKRNL,
    ordinal = 0x1AF
);

crate::import_thunk!(
    /// Fills a display information structure.
    pub VdGetCurrentDisplayInformation(information: *mut u32) -> (),
    lib = XBOXKRNL,
    ordinal = 0x1BA
);

crate::import_thunk!(
    /// Reads a video encoder register.
    pub VdReadDVERegisterUlong(register: u32) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x1CB
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
    /// Fills a network interface statistics structure.
    pub NicGetStats(device_flags: u32, stats: *mut u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x209
);

crate::import_thunk!(
    /// Writes a boot loader image to flash. A nonzero `flags` value on return indicates failure.
    pub XeKeysSaveBootLoader(flags: *mut u32, size: *mut u32, buffer: u32) -> u32,
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
    /// Stores the 5-byte console ID and, when `text` is not null, its 12-character text form.
    pub XeKeysGetConsoleID(raw: *mut u8, text: *mut u8) -> u32,
    lib = XBOXKRNL,
    ordinal = 0x246
);

crate::import_thunk!(
    /// True while the power-down deferred procedure call runs.
    pub safe HalIsExecutingPowerDownDpc() -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x267
);
