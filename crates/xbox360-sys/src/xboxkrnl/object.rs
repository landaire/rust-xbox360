//! Kernel objects that the caller allocates, object types, waits and object access.

use core::sync::atomic::AtomicU32;

use super::field::KernelU32;
use super::handle::Handle;
use super::string::AnsiString;

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

impl RtlCriticalSection {
    /// The field view of the section.
    pub fn fields(&self) -> &RtlCriticalSectionFields {
        // SAFETY: the view has the size and alignment of the storage, whose
        // `UnsafeCell` allows the shared writes the atomic fields admit.
        unsafe { &*self.as_ptr().cast::<RtlCriticalSectionFields>() }
    }
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
    /// Address of the [`Kthread`](super::Kthread) that owns the section, or 0.
    pub owning_thread: KernelU32,
}

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(RtlCriticalSectionFields, lock_count) == 0x10);
    assert!(offset_of!(RtlCriticalSectionFields, recursion_count) == 0x14);
    assert!(offset_of!(RtlCriticalSectionFields, owning_thread) == 0x18);
    assert!(core::mem::size_of::<RtlCriticalSectionFields>() == 0x1c);
};

kernel_object! {
    /// `KSPIN_LOCK` storage (one word, 0 when free).
    pub struct KspinLock(size = 4, align = 4);
}

/// `OBJECT_TYPE`: an object type that the kernel exports, such as
/// [`ExThreadObjectType`](super::ExThreadObjectType). Object calls take its address.
#[repr(C)]
pub struct ObjectType {
    _opaque: [u8; 0],
    _pinned: core::marker::PhantomPinned,
}

/// Any dispatcher object a wait takes: a [`Kevent`], [`Ksemaphore`],
/// [`Ktimer`], [`Kthread`](super::Kthread) or another object that starts with a dispatcher
/// header. Cast the address of the object to it.
#[repr(C)]
pub struct DispatcherObject {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
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

/// Object attributes. The console layout has no length or security fields.
#[repr(C)]
pub struct ObjectAttributes {
    pub root_directory: Handle,
    pub object_name: *const AnsiString,
    pub attributes: ObjectAttributeFlags,
}

#[cfg(target_pointer_width = "32")]
const _: () = assert!(core::mem::size_of::<ObjectAttributes>() == 12);
