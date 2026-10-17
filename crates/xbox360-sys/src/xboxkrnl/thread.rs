//! Threads, processes, interrupt levels and kernel notifications.

use core::sync::atomic::AtomicU8;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use super::field::KernelU8;
use super::field::KernelU32;
use super::field::ListEntry;
use super::object::KspinLock;

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
    /// Address of the [`KswitchFrame`](super::KswitchFrame) the thread saved when it last
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

/// `KPROCESS`, up to the thread list.
#[repr(C)]
pub struct Kprocess {
    /// Guards [`Kprocess::thread_list`].
    pub thread_list_lock: KspinLock,
    /// Head of the list of [`Kthread::thread_list_entry`] links.
    pub thread_list: ListEntry,
}

const _: () = assert!(core::mem::offset_of!(Kprocess, thread_list) == 4);

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

/// Registration record of [`ExRegisterThreadNotification`](super::ExRegisterThreadNotification),
/// [`ExRegisterTitleTerminateNotification`](super::ExRegisterTitleTerminateNotification) and
/// [`HalRegisterSMCNotification`](super::HalRegisterSMCNotification): the routine the kernel calls, the
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
    /// 16-byte [`SmcMessage`](super::SmcMessage), and runs at dispatch level.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_mask_occupies_the_top_byte() {
        let flags = ThreadCreationFlags::processors(1 << 2) | ThreadCreationFlags::SUSPENDED;
        assert_eq!(flags.bits(), 0x0400_0001);
        assert_eq!(ThreadCreationFlags::processors(0xff).bits(), 0x3f00_0000);
        assert_eq!(ThreadCreationFlags::from_bits(flags.bits()), Some(flags));
    }

    #[test]
    fn irql_compares_the_low_byte() {
        assert_eq!(Irql::from_raw(0xdead_0002), Irql::DISPATCH);
        assert!(Irql::from_raw(0x0000_0101) < Irql::DISPATCH);
    }
}
