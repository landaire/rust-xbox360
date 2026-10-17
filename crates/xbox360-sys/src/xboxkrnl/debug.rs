//! Debug monitor interface.

use core::sync::atomic::AtomicU32;

use super::field::KernelU32;
use super::memory::MmStatistics;

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
/// [`KeDebugMonitorData`](super::KeDebugMonitorData). Through the fields that hold the address of a
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
    /// Address of `KeSystemProcess`, the [`Kprocess`](super::Kprocess) of the system.
    pub system_process: KernelU32,
    /// Address of `KeTitleProcess`, the [`Kprocess`](super::Kprocess) of the title.
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

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(DebugMemoryStatistics, statistics) == 0x4);
    assert!(offset_of!(DebugMemoryStatistics, unknown_06c) == 0x6c);
    assert!(offset_of!(DebugMemoryStatistics, debug_memory) == 0x70);
    assert!(offset_of!(DebugMemoryStatistics, page_threshold) == 0x7c);
    assert!(offset_of!(DebugMemoryStatistics, unknown_08c) == 0x8c);
    assert!(core::mem::size_of::<DebugMemoryStatistics>() == 0x90);
};
