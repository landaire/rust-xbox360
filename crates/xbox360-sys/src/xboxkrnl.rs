//! xboxkrnl.exe imports, kernel structures, and the types and constants of
//! their arguments.
//!
//! Kernel structures that Rust code only reads or that the kernel shares,
//! such as [`Kthread`], are views of the fields in use. A field that the
//! kernel or another processor changes is an atomic, and the bytes between
//! the fields are private. A field that the kernel acts on is a
//! [`KernelU32`] or [`KernelU8`], which safe code can only read.

use core::sync::atomic::AtomicU8;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::ImageDirectory;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::ImageNtHeaders;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::image::XexHeaderKey;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::lib_index::XBOXKRNL;
use crate::processor::Msr;
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
    /// Pseudo handle of the `\??` object directory (Xenia's
    /// `ObDosDevices`): the root directory of an object name that does not
    /// start with a backslash.
    pub const DOS_DEVICES: Self = Self(0xffff_fffd);

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

impl From<bool> for Boolean {
    /// `TRUE` is 1.
    fn from(value: bool) -> Self {
        Self(u32::from(value))
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

impl Kevent {
    /// Initialize the event at its current address with plain stores, as
    /// `KeInitializeEvent` does: the type in the first header byte, the
    /// signal state, and an empty wait list that points at itself.
    ///
    /// # Safety
    ///
    /// No kernel call or other thread uses the event during the stores, and
    /// the event does not move while the kernel may use it afterwards.
    #[inline]
    pub unsafe fn initialize(&self, kind: EventType, signaled: bool) {
        let header = self.as_ptr().cast::<[u32; 4]>();
        let list = header.cast::<u32>().wrapping_add(2) as usize as u32;
        // SAFETY: the storage is 0x10 bytes aligned to 4; exclusivity per
        // the contract.
        unsafe { header.write([kind.raw() << 24, signaled as u32, list, list]) };
    }
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
    /// `KDPC` storage (0x1c bytes). Byte 2 holds the hardware thread the
    /// DPC is queued on plus one, 0 while it is not queued:
    /// `KeInsertQueueDpc` sets it and `KeRemoveQueueDpc` clears it. Byte 3
    /// holds the hardware thread that runs the DPC plus one, 0 for the
    /// hardware thread that queues it: `KeInsertQueueDpc` queues on that
    /// processor's PRCB. The kernel has no export that sets byte 3; its own
    /// callers store it directly.
    pub struct Kdpc(size = 0x1c, align = 4);
}

impl Kdpc {
    /// Select the hardware thread that runs the DPC (byte 3); `None` runs
    /// it on the hardware thread that queues it. `KeInitializeDpc` clears
    /// the byte, so set it afterwards.
    ///
    /// # Safety
    ///
    /// The DPC must not be queued, and no other code may access it during
    /// the call.
    pub unsafe fn set_target_processor(&self, processor: Option<crate::processor::HardwareThread>) {
        const TARGET_PROCESSOR_OFFSET: usize = 3;
        let value = match processor {
            Some(processor) => processor.get() + 1,
            None => 0,
        };
        // SAFETY: the byte lies inside the storage, and the caller excludes
        // concurrent access.
        unsafe {
            self.as_ptr()
                .cast::<u8>()
                .add(TARGET_PROCESSOR_OFFSET)
                .write(value)
        };
    }
}

kernel_object! {
    /// `RTL_CRITICAL_SECTION` storage (0x1c bytes): a dispatcher header,
    /// the lock count, the recursion count and the owning thread.
    pub struct RtlCriticalSection(size = 0x1c, align = 4);
}

/// The fields of an initialized [`RtlCriticalSection`] that code other than
/// the critical section routines reads. The threads that use the section
/// change them at any time.
#[repr(C)]
pub struct RtlCriticalSectionFields {
    _header: [AtomicU32; 4],
    /// Bits of the signed count of entries minus one: -1 when the section
    /// is free.
    pub lock_count: KernelU32,
    /// Bits of the signed count of recursive entries of the owner.
    pub recursion_count: KernelU32,
    /// Address of the [`Kthread`] that owns the section, or 0.
    pub owning_thread: KernelU32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(RtlCriticalSectionFields, lock_count) == 0x10);
    assert!(offset_of!(RtlCriticalSectionFields, recursion_count) == 0x14);
    assert!(offset_of!(RtlCriticalSectionFields, owning_thread) == 0x18);
    assert!(core::mem::size_of::<RtlCriticalSectionFields>() == 0x1c);
};

impl RtlCriticalSection {
    /// The field view of the section.
    pub fn fields(&self) -> &RtlCriticalSectionFields {
        // SAFETY: the view has the size and alignment of the storage, whose
        // `UnsafeCell` allows the shared writes the atomic fields admit.
        unsafe { &*self.as_ptr().cast::<RtlCriticalSectionFields>() }
    }
}

kernel_object! {
    /// `KSPIN_LOCK` storage (one word, 0 when free).
    pub struct KspinLock(size = 4, align = 4);
}

macro_rules! kernel_field {
    ($(#[$meta:meta])* $name:ident($atomic:ty, $value:ty)) => {
        $(#[$meta])*
        #[repr(transparent)]
        pub struct $name($atomic);

        impl $name {
            #[inline]
            pub fn load(&self, order: Ordering) -> $value {
                self.0.load(order)
            }

            /// # Safety
            ///
            /// The kernel must accept `value`: what it does with the field
            /// after the store must not access memory that is not valid for
            /// it or break the state of other code.
            #[inline]
            pub unsafe fn store(&self, value: $value, order: Ordering) {
                self.0.store(value, order)
            }

            /// Store `value` and return the previous value.
            ///
            /// # Safety
            ///
            /// As for `store`.
            #[inline]
            pub unsafe fn swap(&self, value: $value, order: Ordering) -> $value {
                self.0.swap(value, order)
            }
        }
    };
}

kernel_field! {
    /// A word of a kernel structure that the kernel acts on, such as an
    /// address it dereferences. Loads are safe, stores are unsafe.
    KernelU32(AtomicU32, u32)
}

kernel_field! {
    /// A byte of a kernel structure that the kernel acts on. Loads are
    /// safe, stores are unsafe.
    KernelU8(AtomicU8, u8)
}

/// `LIST_ENTRY`: a link of a doubly linked list, as addresses of the next
/// and the previous link. The kernel changes the links of the lists it
/// shares under a lock and follows them.
#[repr(C)]
pub struct ListEntry {
    pub flink: KernelU32,
    pub blink: KernelU32,
}

/// `KTHREAD`: a thread object that the kernel allocates, up to the last
/// field in use.
///
/// The kernel frees the object of a thread after the thread ends. `Kthread`
/// is not `Sync`, so a reference to it is not `Send` and stays on the
/// thread that took it, which keeps the object of the calling thread
/// allocated.
///
/// The kernel does not act on [`Kthread::debug_data`] and
/// [`Kthread::last_error`], so they are plain atomics. The other fields
/// are kernel state.
#[repr(C)]
pub struct Kthread {
    _header_type: [AtomicU8; 4],
    /// Signal state of the dispatcher header: nonzero once the thread has
    /// ended.
    pub signal_state: KernelU32,
    _reserved_008: [AtomicU8; 0x50],
    /// Clock ticks of 1 ms that the thread ran. The clock interrupt counts
    /// them. For the idle thread of a processor, the idle time of the
    /// processor.
    pub run_time: KernelU32,
    /// Highest address of the stack.
    pub stack_base: KernelU32,
    /// Lowest address of the stack.
    pub stack_limit: KernelU32,
    /// Address of the [`KswitchFrame`] the thread saved when it last
    /// stopped running.
    pub switch_frame: KernelU32,
    /// Address of the thread local storage block.
    pub tls_base: u32,
    _reserved_06c: [AtomicU8; 4],
    pub priority: KernelU8,
    _reserved_071: AtomicU8,
    /// [`ProcessType`] of the process that owns the thread.
    pub process_type: u8,
    /// [`ProcessType`] that `KeGetCurrentProcessType` reports for the
    /// thread. The kernel changes it while the thread runs on behalf of
    /// another process.
    pub current_process_type: KernelU8,
    _reserved_074: [AtomicU8; 0x10],
    /// Address of the [`Kprocess`] that owns the thread.
    pub process: u32,
    _reserved_088: [AtomicU8; 0x30],
    pub priority_boost: KernelU8,
    pub base_priority: KernelU8,
    _reserved_0ba: AtomicU8,
    pub saturation: KernelU8,
    pub suspend_count: KernelU8,
    _reserved_0bd: [AtomicU8; 2],
    processor: KernelU8,
    _reserved_0c0: [AtomicU8; 9],
    /// Priority that `KeSetBasePriorityThread` adds its increment to.
    pub process_priority: KernelU8,
    /// Highest base priority that `KeSetBasePriorityThread` sets.
    pub priority_limit: KernelU8,
    _reserved_0cb: [AtomicU8; 0x45],
    /// Link in the thread list of the process.
    pub thread_list_entry: ListEntry,
    _reserved_118: [AtomicU8; 0x18],
    /// Creation time as a FILETIME, high word first.
    create_time: [u32; 2],
    _reserved_138: [AtomicU8; 8],
    pub exit_status: KernelU32,
    _reserved_144: [AtomicU8; 8],
    pub id: u32,
    pub start_address: u32,
    _reserved_154: [AtomicU8; 8],
    /// Word for the debug monitor's own use. Stock XBDM keeps the address
    /// of its record of the thread here. The kernel leaves it to the
    /// monitor, so code that takes an address from it must not rely on
    /// other code leaving it unchanged.
    pub debug_data: AtomicU32,
    /// Win32 last error.
    pub last_error: AtomicU32,
    _reserved_164: [AtomicU8; 8],
    pub creation_flags: ThreadCreationFlags,
    _thread_bound: core::marker::PhantomData<*const ()>,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(Kthread, signal_state) == 0x4);
    assert!(offset_of!(Kthread, run_time) == 0x58);
    assert!(offset_of!(Kthread, stack_base) == 0x5c);
    assert!(offset_of!(Kthread, stack_limit) == 0x60);
    assert!(offset_of!(Kthread, switch_frame) == 0x64);
    assert!(offset_of!(Kthread, tls_base) == 0x68);
    assert!(offset_of!(Kthread, priority) == 0x70);
    assert!(offset_of!(Kthread, process_type) == 0x72);
    assert!(offset_of!(Kthread, current_process_type) == 0x73);
    assert!(offset_of!(Kthread, process) == 0x84);
    assert!(offset_of!(Kthread, priority_boost) == 0xb8);
    assert!(offset_of!(Kthread, base_priority) == 0xb9);
    assert!(offset_of!(Kthread, saturation) == 0xbb);
    assert!(offset_of!(Kthread, suspend_count) == 0xbc);
    assert!(offset_of!(Kthread, processor) == 0xbf);
    assert!(offset_of!(Kthread, process_priority) == 0xc9);
    assert!(offset_of!(Kthread, priority_limit) == 0xca);
    assert!(offset_of!(Kthread, thread_list_entry) == 0x110);
    assert!(offset_of!(Kthread, create_time) == 0x130);
    assert!(offset_of!(Kthread, exit_status) == 0x140);
    assert!(offset_of!(Kthread, id) == 0x14c);
    assert!(offset_of!(Kthread, start_address) == 0x150);
    assert!(offset_of!(Kthread, debug_data) == 0x15c);
    assert!(offset_of!(Kthread, last_error) == 0x160);
    assert!(offset_of!(Kthread, creation_flags) == 0x16c);
    assert!(core::mem::size_of::<Kthread>() == 0x170);
};

impl Kthread {
    /// Creation time as a FILETIME.
    pub const fn create_time(&self) -> u64 {
        ((self.create_time[0] as u64) << 32) | self.create_time[1] as u64
    }

    /// Hardware thread the thread runs or last ran on.
    pub fn processor(&self) -> crate::processor::HardwareThread {
        crate::processor::HardwareThread::from_kernel(self.processor.load(Ordering::Relaxed))
    }
}

/// `KPROCESS`, up to the thread list.
#[repr(C)]
pub struct Kprocess {
    /// Guards [`Kprocess::thread_list`].
    pub thread_list_lock: KspinLock,
    /// Head of the list of [`Kthread::thread_list_entry`] links.
    pub thread_list: ListEntry,
}

const _: () = assert!(core::mem::offset_of!(Kprocess, thread_list) == 4);

/// `OBJECT_TYPE`: an object type that the kernel exports, such as
/// [`ExThreadObjectType`]. Object calls take its address.
#[repr(C)]
pub struct ObjectType {
    _opaque: [u8; 0],
    _pinned: core::marker::PhantomPinned,
}

flags! {
    /// `EXCEPTION_*` flags of an [`ExceptionRecord`].
    pub struct ExceptionFlags(u32) {
        /// Execution cannot continue after the exception.
        NONCONTINUABLE = 0x1,
    }
}

/// `EXCEPTION_RECORD`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionRecord {
    pub code: NtStatus,
    pub flags: ExceptionFlags,
    /// Address of the record of an exception raised while this one was
    /// handled, or 0.
    pub next_record: u32,
    /// Address of the instruction that raised the exception.
    pub address: u32,
    /// Number of [`ExceptionRecord::parameters`] in use.
    pub parameter_count: u32,
    pub parameters: [u32; 15],
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(ExceptionRecord, address) == 0xc);
    assert!(offset_of!(ExceptionRecord, parameters) == 0x14);
    assert!(core::mem::size_of::<ExceptionRecord>() == 0x50);
};

flags! {
    /// `CONTEXT_*`: the register groups of a [`Context`] that hold values.
    pub struct ContextFlags(u32) {
        /// MSR, IAR, LR and CTR.
        CONTROL = 0x1,
        /// FPR and FPSCR.
        FLOATING_POINT = 0x2,
        /// GPR, CR and XER.
        INTEGER = 0x4,
        /// VR and VSCR.
        VECTOR = 0x10,
    }
}

impl ContextFlags {
    /// Every register group, as `RtlCaptureContext` requests.
    pub const FULL: Self = Self::CONTROL
        .union(Self::FLOATING_POINT)
        .union(Self::INTEGER)
        .union(Self::VECTOR);
}

/// `CONTEXT`: the registers of a thread. The vector registers need the
/// 16-byte alignment.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub context_flags: ContextFlags,
    pub msr: Msr,
    pub iar: u32,
    pub lr: u32,
    pub ctr: u64,
    pub gpr: [u64; 32],
    pub cr: u32,
    pub xer: u32,
    pub fpscr: u64,
    /// Floating point registers as IEEE bits.
    pub fpr: [u64; 32],
    _reserved_228: [u32; 2],
    pub vscr: [u32; 4],
    pub vr: [[u32; 4]; 128],
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(Context, msr) == 0x4);
    assert!(offset_of!(Context, iar) == 0x8);
    assert!(offset_of!(Context, lr) == 0xc);
    assert!(offset_of!(Context, ctr) == 0x10);
    assert!(offset_of!(Context, gpr) == 0x18);
    assert!(offset_of!(Context, cr) == 0x118);
    assert!(offset_of!(Context, xer) == 0x11c);
    assert!(offset_of!(Context, fpscr) == 0x120);
    assert!(offset_of!(Context, fpr) == 0x128);
    assert!(offset_of!(Context, vscr) == 0x230);
    assert!(offset_of!(Context, vr) == 0x240);
    assert!(core::mem::size_of::<Context>() == 0xa40);
};

impl Context {
    /// A context with every register 0 that requests the groups of
    /// `flags`.
    pub const fn new(flags: ContextFlags) -> Self {
        Self {
            context_flags: flags,
            msr: Msr::empty(),
            iar: 0,
            lr: 0,
            ctr: 0,
            gpr: [0; 32],
            cr: 0,
            xer: 0,
            fpscr: 0,
            fpr: [0; 32],
            _reserved_228: [0; 2],
            vscr: [0; 4],
            vr: [[0; 4]; 128],
        }
    }
}

/// `KTRAP_FRAME`: the registers the kernel saves on an interrupt or
/// exception, up to the last field in use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KtrapFrame {
    _reserved_000: [u32; 0x14],
    pub gpr: [u64; 32],
    _reserved_150: [u32; 0x16],
    pub lr: u32,
    pub iar: u32,
    _msr_high: u32,
    /// Low word of the saved MSR.
    pub msr: Msr,
    pub xer: u32,
    _reserved_1bc: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(KtrapFrame, gpr) == 0x50);
    assert!(offset_of!(KtrapFrame, iar) == 0x1ac);
    assert!(offset_of!(KtrapFrame, lr) == 0x1a8);
    assert!(offset_of!(KtrapFrame, msr) == 0x1b4);
    assert!(offset_of!(KtrapFrame, xer) == 0x1b8);
    assert!(core::mem::size_of::<KtrapFrame>() == 0x1c0);
};

/// `KSWITCHFRAME`: the nonvolatile registers a thread saves on its stack
/// when it stops running, up to the last field in use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KswitchFrame {
    _reserved_000: [u32; 0x14],
    /// r14 to r31.
    pub gpr: [u64; 18],
    pub cr: u32,
    pub iar: u32,
    pub lr: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(KswitchFrame, gpr) == 0x50);
    assert!(offset_of!(KswitchFrame, cr) == 0xe0);
    assert!(offset_of!(KswitchFrame, iar) == 0xe4);
    assert!(offset_of!(KswitchFrame, lr) == 0xe8);
};

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
    ///
    /// The kernel keeps the whole word in [`Kthread::creation_flags`].
    /// `ExCreateThread` does not test bit 0x4, nor bits 0x100, 0x200 and
    /// 0x400, which only code that reads the thread object acts on.
    pub struct ThreadCreationFlags(u32) {
        /// Create the thread suspended.
        SUSPENDED = 0x1,
        /// Create the thread in the system process.
        SYSTEM_PROCESS = 0x2,
        /// Allocate the TLS data block of the process with the thread and
        /// copy the TLS template of the process into it.
        TLS_DATA = 0x8,
        /// Set the priority class of the thread to 13. Takes precedence
        /// over [`ThreadCreationFlags::PRIORITY_CLASS_5`].
        PRIORITY_CLASS_13 = 0x20,
        /// Set the priority class of the thread to 5.
        PRIORITY_CLASS_5 = 0x40,
        /// Store a referenced thread object instead of a handle.
        RETURN_THREAD_OBJECT = 0x80,
        /// The thread is the entry thread of a title. A debug monitor reads
        /// this bit from the thread object.
        TITLE_ENTRY = 0x100,
        /// A debug monitor does not stop the thread when it stops
        /// execution.
        EXEMPT_FROM_STOPS = 0x200,
        /// A debug monitor does not debug the thread.
        NOT_DEBUGGABLE = 0x400,
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
    /// Byte 0 of an [`SmcMessage`]: the command of a request to the system
    /// management controller, or the kind of a message it sends.
    #[non_exhaustive]
    pub enum SmcCommand: u8 {
        /// An I2C transfer.
        I2c = 0x11,
        /// The optical drive tray state. Byte 1 is 0x60 plus the state.
        TrayState = 0x83,
    }
}

/// A 16-byte message to or from the system management controller. Byte 0
/// is the [`SmcCommand`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SmcMessage(pub [u8; 16]);

impl SmcMessage {
    /// A message of `command` with the other bytes zero.
    pub const fn new(command: SmcCommand) -> Self {
        let mut bytes = [0; 16];
        bytes[0] = command.raw();
        Self(bytes)
    }

    /// `None` for a command without a variant.
    pub const fn command(&self) -> Option<SmcCommand> {
        SmcCommand::from_raw(self.0[0])
    }
}

/// Bus address of the device of an [`SmcCommand::I2c`] transfer.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmcI2cDevice(pub u8);

impl SmcI2cDevice {
    /// The EEPROM that holds the debug monitor settings block.
    pub const SETTINGS_EEPROM: Self = Self(0xd4);
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
        /// Return the CD-ROM channel extension and its packet routines in a
        /// [`CdromChannelInformation`].
        CdromChannel = 4,
        /// Return the address of the trace exception hook.
        TraceHook = 6,
        /// Select hypervisor DVD emulation.
        DvdEmulation = 7,
        /// Fill debug-adjusted memory statistics.
        MemoryStatistics = 8,
    }
}

/// Output of [`DebugMonitorService::CdromChannel`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CdromChannelInformation {
    _reserved_00: [u32; 2],
    /// Address of the channel extension of the CD-ROM SATA channel, or 0.
    pub channel: u32,
    /// Routine `(channel, irp)` that queues a request on the channel; 0
    /// when the kernel gives none.
    pub start_packet: u32,
    /// Routine `(channel)` that starts the next queued request; 0 when the
    /// kernel gives none.
    pub start_next_packet: u32,
    _reserved_14: [u32; 3],
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(CdromChannelInformation, channel) == 0x08);
    assert!(offset_of!(CdromChannelInformation, start_next_packet) == 0x10);
    assert!(core::mem::size_of::<CdromChannelInformation>() == 0x20);
};

/// A `KeBugCheck` code. Codes that are not named here are module-specific.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BugCheckCode(pub u32);

impl BugCheckCode {
    pub const KMODE_EXCEPTION_NOT_HANDLED: Self = Self(0x1e);
}

value_enum! {
    /// Reason the loader passes to the entry point of a module.
    #[non_exhaustive]
    pub enum DllEntryReason: u32 {
        /// The module is unloaded.
        ProcessDetach = 0,
        /// The module is loaded.
        ProcessAttach = 1,
    }
}

/// A special purpose register of `KeGetSpecialPurposeRegister` and
/// `KeSetSpecialPurposeRegister`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialPurposeRegister(pub u32);

impl SpecialPurposeRegister {
    /// Data address breakpoint register.
    pub const DABR: Self = Self(0x3f5);
    /// Performance monitor control: bit 1 freezes and bit 2 enables the
    /// counters; bits 10 to 15 select the event bus width.
    pub const MONITOR_CONTROL: Self = Self(0x3f6);
    /// Performance monitor event selection.
    pub const EVENT_SELECT: Self = Self(0x3fa);
}

/// A performance monitor bus register of `KeGetPMWRegister` and
/// `KeSetPMWRegister`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PmwRegister(pub u32);

impl PmwRegister {
    /// Lane sources of the bus units.
    pub const NCU0: Self = Self(0x480);
    pub const NCU1: Self = Self(0x580);
    pub const NCU2: Self = Self(0x680);
    pub const CIU: Self = Self(0x708);
    pub const BIU: Self = Self(0xb50);
    /// L2 event group selection.
    pub const L2_GROUP: Self = Self(0x310);
    pub const L2: Self = Self(0x318);
    /// L2 ECC error counts.
    pub const L2_ECC: Self = Self(0x860);
}

/// A performance monitor register of `KeSetPRVRegister`. Values are in
/// the high word.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrvRegister(pub u32);

impl PrvRegister {
    /// Counter run control.
    pub const RUN: Self = Self(0x1c0);
    /// Interval timer control.
    pub const INTERVAL_CONTROL: Self = Self(0x1c8);
    /// Processor whose events trigger the counters.
    pub const TRIGGER: Self = Self(0x1d8);
    pub const INTERVAL: Self = Self(0x1e0);
    pub const INTERVAL_RELOAD: Self = Self(0x1e8);

    /// The value register of counter `counter`. Panics unless `counter` is
    /// 0 to 15.
    pub const fn counter_value(counter: u8) -> Self {
        assert!(counter < 16, "the performance monitor has 16 counters");
        Self(0x200 + counter as u32 * 8)
    }

    /// The control register of counter `counter`. Panics unless `counter`
    /// is 0 to 15.
    pub const fn counter_control(counter: u8) -> Self {
        assert!(counter < 16, "the performance monitor has 16 counters");
        Self(0x280 + counter as u32 * 8)
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

/// Result of `NtQueryFullAttributesFile` and of `NtQueryInformationFile`
/// with [`FileInformationClass::NetworkOpen`]. Times are FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileNetworkOpenInformation {
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub allocation_size: u64,
    /// File size in bytes.
    pub end_of_file: u64,
    pub attributes: FileAttributes,
    _pad: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileNetworkOpenInformation, end_of_file) == 0x28);
    assert!(offset_of!(FileNetworkOpenInformation, attributes) == 0x30);
    assert!(core::mem::size_of::<FileNetworkOpenInformation>() == 0x38);
};

/// `FILE_DIRECTORY_INFORMATION`: an entry of `NtQueryDirectoryFile`. The
/// name, `file_name_length` bytes, follows at 0x40. Times are FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileDirectoryInformation {
    /// Offset of the next entry, or 0 for the last one.
    pub next_entry_offset: u32,
    pub file_index: u32,
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    /// File size in bytes.
    pub end_of_file: u64,
    pub allocation_size: u64,
    pub attributes: FileAttributes,
    pub file_name_length: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileDirectoryInformation, creation_time) == 0x08);
    assert!(offset_of!(FileDirectoryInformation, end_of_file) == 0x28);
    assert!(offset_of!(FileDirectoryInformation, attributes) == 0x38);
    assert!(offset_of!(FileDirectoryInformation, file_name_length) == 0x3c);
    assert!(core::mem::size_of::<FileDirectoryInformation>() == 0x40);
};

/// `FILE_BASIC_INFORMATION` of [`FileInformationClass::Basic`]. Times are
/// FILETIMEs.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileBasicInformation {
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub attributes: FileAttributes,
    _pad: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileBasicInformation, attributes) == 0x20);
    assert!(core::mem::size_of::<FileBasicInformation>() == 0x28);
};

/// `FILE_DISPOSITION_INFORMATION` of [`FileInformationClass::Disposition`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileDispositionInformation {
    /// Delete the file when its last handle is closed.
    pub delete_file: bool,
}

const _: () = assert!(core::mem::size_of::<FileDispositionInformation>() == 1);

/// `FILE_POSITION_INFORMATION` of [`FileInformationClass::Position`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FilePositionInformation {
    pub current_byte_offset: u64,
}

/// `FILE_END_OF_FILE_INFORMATION` of [`FileInformationClass::EndOfFile`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FileEndOfFileInformation {
    pub end_of_file: u64,
}

/// `FILE_RENAME_INFORMATION` of [`FileInformationClass::Rename`].
#[repr(C)]
pub struct FileRenameInformation {
    pub replace_if_exists: bool,
    /// Directory the name is relative to, or [`Handle::NULL`].
    pub root_directory: Handle,
    pub file_name: AnsiString,
}

#[cfg(target_pointer_width = "32")]
const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(FileRenameInformation, root_directory) == 0x04);
    assert!(offset_of!(FileRenameInformation, file_name) == 0x08);
    assert!(core::mem::size_of::<FileRenameInformation>() == 0x10);
};

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

/// `MEMORY_BASIC_INFORMATION` that [`NtQueryVirtualMemory`] fills for the
/// region containing an address.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoryBasicInformation {
    /// Page of the queried address.
    pub base_address: u32,
    pub allocation_base: u32,
    pub allocation_protect: PageProtection,
    /// Bytes from `base_address` with the same state and protection.
    pub region_size: u32,
    /// [`AllocationType::COMMIT`], [`AllocationType::RESERVE`] or
    /// [`AllocationType::FREE`].
    pub state: AllocationType,
    pub protect: PageProtection,
    /// [`AllocationType::PRIVATE`] for an allocated region.
    pub kind: AllocationType,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(MemoryBasicInformation, region_size) == 0xc);
    assert!(offset_of!(MemoryBasicInformation, state) == 0x10);
    assert!(offset_of!(MemoryBasicInformation, protect) == 0x14);
    assert!(core::mem::size_of::<MemoryBasicInformation>() == 0x1c);
};

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
        /// The automatic sign-in profile setting (configuration setting 3,
        /// 8) is available.
        AUTO_PROFILE = 0x10,
        HARD_DISK = 0x20,
        /// The kernel debugger is enabled at boot.
        KERNEL_DEBUGGER_ENABLED = 0x80,
        /// A kernel debugger is attached.
        KERNEL_DEBUGGER = 0x100,
        /// The console has 1 GB of memory, as some development kits do.
        ONE_GIGABYTE = 0x0001_0000,
        /// The console is a test kit.
        TEST_KIT = 0x0200_0000,
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
    /// The Trinity motherboard, the first Xbox 360 S.
    pub const TRINITY: Self = Self(4);

    pub const fn get(self) -> u8 {
        self.0
    }
}

flags! {
    /// Boot flags of [`HardwareInfo`].
    pub struct HardwareBootFlags(u16) {
        /// The console booted from a disc.
        FROM_DISC = 0x400,
    }
}

/// `XboxHardwareInfo`: console hardware information, up to the boot flags.
#[repr(C)]
pub struct HardwareInfo {
    /// [`HardwareFlags`] word. The debug monitor changes it while the
    /// system runs, and the kernel acts on it.
    pub flags: KernelU32,
    _reserved_004: [AtomicU8; 0xa],
    pub boot_flags: HardwareBootFlags,
}

const _: () = {
    assert!(core::mem::offset_of!(HardwareInfo, boot_flags) == 0xe);
    assert!(core::mem::size_of::<HardwareInfo>() == 0x10);
};

/// A kernel version, as `XboxKrnlVersion` and `XboxKrnlBaseVersion` hold
/// it.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
    pub qfe: u16,
}

const _: () = assert!(core::mem::size_of::<KernelVersion>() == 8);

impl core::fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.qfe
        )
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

/// `UNICODE_STRING` in a kernel structure: a counted UTF-16 string.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnicodeString {
    /// Length in bytes.
    pub length: u16,
    pub maximum_length: u16,
    /// Address of the first code unit.
    pub buffer: u32,
}

/// `LDR_DATA_TABLE_ENTRY`: the loader's record of a loaded module, up to
/// the last field in use. The address of the entry is the module handle.
/// The loader changes the links and the flags under its lock.
#[repr(C)]
pub struct LdrDataTableEntry {
    /// Link in the loaded module list.
    pub in_load_order_links: ListEntry,
    _reserved_008: [AtomicU8; 0x10],
    /// Address the image is loaded at.
    pub dll_base: u32,
    _reserved_01c: [AtomicU8; 4],
    /// Size of the loaded image in bytes.
    pub image_size: u32,
    /// Path of the module.
    pub full_dll_name: UnicodeString,
    /// File name of the module.
    pub base_dll_name: UnicodeString,
    /// Loader flags of the module.
    pub flags: KernelU32,
    _reserved_038: [AtomicU8; 0x18],
    pub time_date_stamp: u32,
    _reserved_054: [AtomicU8; 4],
    /// Address of the XEX header of the image.
    pub xex_header_base: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(LdrDataTableEntry, dll_base) == 0x18);
    assert!(offset_of!(LdrDataTableEntry, image_size) == 0x20);
    assert!(offset_of!(LdrDataTableEntry, full_dll_name) == 0x24);
    assert!(offset_of!(LdrDataTableEntry, base_dll_name) == 0x2c);
    assert!(offset_of!(LdrDataTableEntry, flags) == 0x34);
    assert!(offset_of!(LdrDataTableEntry, time_date_stamp) == 0x50);
    assert!(offset_of!(LdrDataTableEntry, xex_header_base) == 0x58);
};

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

/// `FILE_FS_SIZE_INFORMATION`: what [`FsInformationClass::Size`] returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FsSizeInformation {
    pub total_allocation_units: u64,
    pub available_allocation_units: u64,
    pub sectors_per_allocation_unit: u32,
    pub bytes_per_sector: u32,
}

const _: () = assert!(core::mem::size_of::<FsSizeInformation>() == 0x18);

/// `FILE_FS_DEVICE_INFORMATION`: what [`FsInformationClass::Device`]
/// returns.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsDeviceInformation {
    pub device_type: FileDeviceType,
    pub characteristics: u32,
}

const _: () = assert!(core::mem::size_of::<FsDeviceInformation>() == 8);

/// `FILE_FS_ATTRIBUTE_INFORMATION`: what [`FsInformationClass::Attribute`]
/// returns. The file system name, `file_system_name_length` bytes, follows
/// at 0xc.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FsAttributeInformation {
    /// `FILE_*` file system flags.
    pub attributes: u32,
    pub maximum_component_name_length: u32,
    pub file_system_name_length: u32,
}

const _: () = assert!(core::mem::size_of::<FsAttributeInformation>() == 0xc);

flags! {
    /// `FILE_*` file system flags of [`FsAttributeInformation::attributes`].
    pub struct FileSystemAttributes(u32) {
        CASE_SENSITIVE_SEARCH = 0x1,
        CASE_PRESERVED_NAMES = 0x2,
        UNICODE_ON_DISK = 0x4,
        PERSISTENT_ACLS = 0x8,
        FILE_COMPRESSION = 0x10,
        VOLUME_IS_COMPRESSED = 0x8000,
        READ_ONLY_VOLUME = 0x8_0000,
    }
}

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

#[cfg(target_pointer_width = "32")]
const _: () = assert!(core::mem::size_of::<ObjectAttributes>() == 12);
const _: () = assert!(core::mem::size_of::<IoStatusBlock>() == 8);

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
/// [`FsInformationClass::Device`] returns it.
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

/// Registration record of [`ExRegisterThreadNotification`],
/// [`ExRegisterTitleTerminateNotification`] and
/// [`HalRegisterSMCNotification`]: the routine the kernel calls, the
/// priority that orders the routines, and the link of the kernel's list,
/// which the kernel owns while the record is registered. The kernel reads
/// and writes all 0x10 bytes.
#[repr(C)]
pub struct NotificationRegistration {
    /// Address of the routine, entered in the console ABI. A thread
    /// notification routine takes `(registration, thread, created)`, with
    /// `created` 1 when the thread starts and 0 when it ends. A title
    /// termination routine takes `(registration)`. An SMC notification
    /// routine takes `(registration, message)`, with the address of a
    /// 16-byte [`SmcMessage`], and runs at dispatch level.
    pub routine: KernelU32,
    /// Signed priority. The kernel calls the routines of higher priority
    /// first.
    pub priority: KernelU32,
    pub link: ListEntry,
}

impl NotificationRegistration {
    pub const fn new() -> Self {
        Self {
            routine: KernelU32(AtomicU32::new(0)),
            priority: KernelU32(AtomicU32::new(0)),
            link: ListEntry {
                flink: KernelU32(AtomicU32::new(0)),
                blink: KernelU32(AtomicU32::new(0)),
            },
        }
    }

    /// The record as the registration imports take it.
    pub const fn as_ptr(&self) -> *mut Self {
        core::ptr::from_ref(self).cast_mut()
    }
}

impl Default for NotificationRegistration {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = assert!(core::mem::size_of::<NotificationRegistration>() == 0x10);

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

flags! {
    /// Flags word of [`DebugMonitorData`].
    pub struct DebugMonitorFlags(u32) {
        /// `KeBugCheckEx` sets it before it stops the system.
        BUG_CHECK = 0x4,
        /// DVD commands are recorded at
        /// [`DebugMonitorData::dvd_trace_pointer`]. The kernel does not test
        /// it.
        DVD_TRACE = 0x20,
        /// The debug monitor wrote a crash dump.
        DUMP = 0x40,
    }
}

/// The debug monitor data block. `ExDebugMonitorService` service
/// [`DebugMonitorService::Query`] fills the kernel's fields, the debug
/// monitor fills its own, and the monitor publishes the block through
/// [`KeDebugMonitorData`]. Through the fields that hold the address of a
/// kernel variable, the monitor installs routines that the kernel calls.
///
/// The kernel calls [`DebugMonitorData::control_routine`] through the
/// published block. The other address fields are [`KernelU32`] because
/// code dereferences the addresses they hold, so a store to them must keep
/// them valid.
#[repr(C)]
pub struct DebugMonitorData {
    /// Address of `KiDebugRoutine`, the routine the kernel calls for each
    /// exception.
    pub debug_routine: KernelU32,
    /// Address of `PsLoadedModuleList`, the head of the loaded module list.
    pub loaded_module_list: KernelU32,
    /// Address of `XexLoadedModuleListLock`, the spin lock that guards
    /// both module lists.
    pub loaded_module_list_lock: KernelU32,
    /// [`DebugMonitorFlags`] word, shared with the kernel.
    pub flags: AtomicU32,
    /// Address of a kernel routine variable for the kernel callback
    /// notification, or 0.
    pub callback_routine: KernelU32,
    _reserved_14: AtomicU32,
    /// Routine the kernel calls with a debug monitor control code and its
    /// argument: `(code, argument) -> u32` in the console ABI.
    pub control_routine: KernelU32,
    /// Address of `KiContextSwapNotification`, the routine the kernel calls
    /// on each thread switch.
    pub context_swap_notification: KernelU32,
    /// Address of the title interface block of the debug monitor.
    pub title_interface: AtomicU32,
    /// Address of the second title block of the debug monitor.
    pub title_reserved: AtomicU32,
    _reserved_28: [AtomicU32; 5],
    /// Write position of the DVD command trace.
    pub dvd_trace_pointer: KernelU32,
    _reserved_40: [AtomicU32; 3],
    /// `KeNumberProcessors`.
    pub processor_count: AtomicU32,
    /// Address of `KeSystemProcess`, the [`Kprocess`] of the system.
    pub system_process: KernelU32,
    /// Address of `KeTitleProcess`, the [`Kprocess`] of the title.
    pub title_process: KernelU32,
    /// Address of `KiStackOverflowNotification`, the routine the kernel
    /// calls before the bug check of a stack overflow.
    pub stack_overflow_notification: KernelU32,
    /// Address of `KiProcessorBlock`, the addresses of the processor
    /// control blocks of the hardware threads.
    pub processor_blocks: KernelU32,
    _reserved_60: AtomicU32,
    /// Address of `PsLoadedUsermodeModuleList`, the head of the second
    /// loaded module list.
    pub loaded_usermode_module_list: KernelU32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(DebugMonitorData, flags) == 0xc);
    assert!(offset_of!(DebugMonitorData, callback_routine) == 0x10);
    assert!(offset_of!(DebugMonitorData, control_routine) == 0x18);
    assert!(offset_of!(DebugMonitorData, context_swap_notification) == 0x1c);
    assert!(offset_of!(DebugMonitorData, title_interface) == 0x20);
    assert!(offset_of!(DebugMonitorData, title_reserved) == 0x24);
    assert!(offset_of!(DebugMonitorData, dvd_trace_pointer) == 0x3c);
    assert!(offset_of!(DebugMonitorData, processor_count) == 0x4c);
    assert!(offset_of!(DebugMonitorData, system_process) == 0x50);
    assert!(offset_of!(DebugMonitorData, title_process) == 0x54);
    assert!(offset_of!(DebugMonitorData, stack_overflow_notification) == 0x58);
    assert!(offset_of!(DebugMonitorData, processor_blocks) == 0x5c);
    assert!(offset_of!(DebugMonitorData, loaded_usermode_module_list) == 0x64);
    assert!(core::mem::size_of::<DebugMonitorData>() == 0x68);
};

impl DebugMonitorData {
    /// A zero block, for storage that the query service fills.
    pub const fn new() -> Self {
        Self {
            debug_routine: KernelU32(AtomicU32::new(0)),
            loaded_module_list: KernelU32(AtomicU32::new(0)),
            loaded_module_list_lock: KernelU32(AtomicU32::new(0)),
            flags: AtomicU32::new(0),
            callback_routine: KernelU32(AtomicU32::new(0)),
            _reserved_14: AtomicU32::new(0),
            control_routine: KernelU32(AtomicU32::new(0)),
            context_swap_notification: KernelU32(AtomicU32::new(0)),
            title_interface: AtomicU32::new(0),
            title_reserved: AtomicU32::new(0),
            _reserved_28: [const { AtomicU32::new(0) }; 5],
            dvd_trace_pointer: KernelU32(AtomicU32::new(0)),
            _reserved_40: [const { AtomicU32::new(0) }; 3],
            processor_count: AtomicU32::new(0),
            system_process: KernelU32(AtomicU32::new(0)),
            title_process: KernelU32(AtomicU32::new(0)),
            stack_overflow_notification: KernelU32(AtomicU32::new(0)),
            processor_blocks: KernelU32(AtomicU32::new(0)),
            _reserved_60: AtomicU32::new(0),
            loaded_usermode_module_list: KernelU32(AtomicU32::new(0)),
        }
    }
}

impl Default for DebugMonitorData {
    fn default() -> Self {
        Self::new()
    }
}

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
        ".long 0x78a6000e", // rldimi 6, 5, 32, 0
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
    pub KeSetSpecialPurposeRegister(register: SpecialPurposeRegister, value: u64) -> u32,
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

/// `KeTimeStampBundle`: times that the kernel updates on each clock tick.
#[repr(C)]
pub struct TimeStampBundle {
    _times: [KernelU32; 4],
    /// Milliseconds since boot.
    pub tick_count: KernelU32,
    _reserved_14: KernelU32,
}

const _: () = {
    assert!(core::mem::offset_of!(TimeStampBundle, tick_count) == 0x10);
    assert!(core::mem::size_of::<TimeStampBundle>() == 0x18);
};

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

value_enum! {
    /// The memory region of [`MmAllocatePhysicalMemoryEx`],
    /// [`MmFreePhysicalMemory`] and [`NtAllocateVirtualMemory`]. The kernel
    /// rejects every other value: the allocation returns 0 or
    /// `STATUS_INVALID_PARAMETER`, and the free does nothing.
    pub enum MemoryRegion: u32 {
        /// The title region for a thread of the title process, the system
        /// region otherwise.
        CurrentProcess = 0,
        Title = 1,
        System = 2,
        /// The title region, counted as the extra memory of a development
        /// kit.
        TitleDebug = 3,
        /// The system region, counted as the extra memory of a development
        /// kit.
        SystemDebug = 4,
    }
}

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

/// Page and byte counts of one process type in [`MmStatistics`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MmProcessStatistics {
    pub available_pages: u32,
    pub total_virtual_memory_bytes: u32,
    pub reserved_virtual_memory_bytes: u32,
    pub physical_pages: u32,
    pub pool_pages: u32,
    pub stack_pages: u32,
    pub image_pages: u32,
    pub heap_pages: u32,
    pub virtual_pages: u32,
    pub page_table_pages: u32,
    pub cache_pages: u32,
}

/// `MM_STATISTICS`: the memory statistics that `MmQueryStatistics` fills.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MmStatistics {
    /// Size of the structure in bytes, which the caller sets.
    pub length: u32,
    pub total_physical_pages: u32,
    pub kernel_pages: u32,
    pub title: MmProcessStatistics,
    pub system: MmProcessStatistics,
    pub highest_physical_page: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(MmStatistics, title) == 0xc);
    assert!(offset_of!(MmStatistics, system) == 0x38);
    assert!(offset_of!(MmStatistics, highest_physical_page) == 0x64);
    assert!(core::mem::size_of::<MmStatistics>() == 0x68);
};

impl MmStatistics {
    /// Zero statistics with [`MmStatistics::length`] set, for
    /// `MmQueryStatistics` to fill.
    pub const fn new() -> Self {
        const ZERO: MmProcessStatistics = MmProcessStatistics {
            available_pages: 0,
            total_virtual_memory_bytes: 0,
            reserved_virtual_memory_bytes: 0,
            physical_pages: 0,
            pool_pages: 0,
            stack_pages: 0,
            image_pages: 0,
            heap_pages: 0,
            virtual_pages: 0,
            page_table_pages: 0,
            cache_pages: 0,
        };
        Self {
            length: core::mem::size_of::<Self>() as u32,
            total_physical_pages: 0,
            kernel_pages: 0,
            title: ZERO,
            system: ZERO,
            highest_physical_page: 0,
        }
    }
}

impl Default for MmStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// The block of [`DebugMonitorService::MemoryStatistics`]: [`MmStatistics`]
/// followed by the debug monitor's own counts. Fields named `unknown_*` have
/// no known meaning; consumers use their values as stock does.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugMemoryStatistics {
    /// Size of the structure in bytes, which the caller sets. The service
    /// replaces it with the size it filled.
    pub length: u32,
    pub statistics: MmStatistics,
    pub unknown_06c: u32,
    /// Bits 8 to 19 hold the debug memory size.
    pub debug_memory: u32,
    pub unknown_074: u32,
    pub unknown_078: u32,
    /// Page count of the base memory configuration.
    pub page_threshold: u32,
    pub unknown_080: u32,
    pub unknown_084: u32,
    pub unknown_088: u32,
    pub unknown_08c: u32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(DebugMemoryStatistics, statistics) == 0x4);
    assert!(offset_of!(DebugMemoryStatistics, unknown_06c) == 0x6c);
    assert!(offset_of!(DebugMemoryStatistics, debug_memory) == 0x70);
    assert!(offset_of!(DebugMemoryStatistics, page_threshold) == 0x7c);
    assert!(offset_of!(DebugMemoryStatistics, unknown_08c) == 0x8c);
    assert!(core::mem::size_of::<DebugMemoryStatistics>() == 0x90);
};

impl DebugMemoryStatistics {
    /// Zero statistics with both length fields set, for the service to
    /// fill.
    pub const fn new() -> Self {
        Self {
            length: core::mem::size_of::<Self>() as u32,
            statistics: MmStatistics::new(),
            unknown_06c: 0,
            debug_memory: 0,
            unknown_074: 0,
            unknown_078: 0,
            page_threshold: 0,
            unknown_080: 0,
            unknown_084: 0,
            unknown_088: 0,
            unknown_08c: 0,
        }
    }

    /// Whether the service filled the whole block.
    pub const fn is_complete(&self) -> bool {
        self.length as usize >= core::mem::size_of::<Self>()
    }
}

impl Default for DebugMemoryStatistics {
    fn default() -> Self {
        Self::new()
    }
}

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

/// `ExLoadedImageName`: the NUL-terminated object path of the running
/// executable. The kernel rewrites it when it starts a title.
#[repr(C)]
pub struct LoadedImageName {
    _opaque: [u8; 0],
    _pinned: core::marker::PhantomPinned,
}

impl LoadedImageName {
    /// Copy the path into `buffer` up to its NUL or the end of `buffer`,
    /// and return the number of bytes copied.
    pub fn copy_to(&self, buffer: &mut [u8]) -> usize {
        let start = core::ptr::from_ref(self).expose_provenance();
        for (index, slot) in buffer.iter_mut().enumerate() {
            // SAFETY: a `LoadedImageName` exists only as the kernel export,
            // a NUL-terminated string, and the bytes up to the NUL are
            // mapped. Atomic loads allow the kernel to rewrite it.
            let byte = unsafe {
                AtomicU8::from_ptr(core::ptr::with_exposed_provenance_mut(start + index))
            }
            .load(Ordering::Relaxed);
            if byte == 0 {
                return index;
            }
            *slot = byte;
        }
        buffer.len()
    }
}

crate::import_variable!(
    /// Object path of the running executable.
    pub safe ExLoadedImageName: LoadedImageName,
    lib = XBOXKRNL,
    ordinal = 0x1AF
);

/// The 0x58-byte display information block that
/// [`VdGetCurrentDisplayInformation`] fills, with the fields in use named.
#[repr(C)]
pub struct DisplayInformation {
    pub surface_width: u16,
    _reserved_002: [u8; 2],
    pub colorspace: u8,
    _reserved_005: [u8; 3],
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub scaled_width: u32,
    pub scaled_height: u32,
    _reserved_020: [u32; 0xe],
}

impl DisplayInformation {
    pub const fn new() -> Self {
        Self {
            surface_width: 0,
            _reserved_002: [0; 2],
            colorspace: 0,
            _reserved_005: [0; 3],
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            scaled_width: 0,
            scaled_height: 0,
            _reserved_020: [0; 0xe],
        }
    }

    /// The block as [`VdGetCurrentDisplayInformation`] takes it.
    pub fn as_mut_ptr(&mut self) -> *mut Self {
        core::ptr::from_mut(self)
    }
}

const _: () = {
    assert!(core::mem::offset_of!(DisplayInformation, colorspace) == 4);
    assert!(core::mem::offset_of!(DisplayInformation, left) == 8);
    assert!(core::mem::offset_of!(DisplayInformation, scaled_height) == 0x1c);
    assert!(core::mem::size_of::<DisplayInformation>() == 0x58);
};

crate::import_thunk!(
    /// Fills a display information structure.
    pub VdGetCurrentDisplayInformation(information: *mut DisplayInformation) -> (),
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
    /// Stores the 5-byte console ID and, when `text` is not null, its 12-character text form.
    pub XeKeysGetConsoleID(raw: *mut u8, text: *mut u8) -> NtStatus,
    lib = XBOXKRNL,
    ordinal = 0x246
);

crate::import_thunk!(
    /// True while the power-down deferred procedure call runs.
    pub safe HalIsExecutingPowerDownDpc() -> Boolean8,
    lib = XBOXKRNL,
    ordinal = 0x267
);
