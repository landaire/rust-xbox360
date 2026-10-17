//! Pool, virtual and physical memory.

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

/// `MEMORY_BASIC_INFORMATION` that [`NtQueryVirtualMemory`](super::NtQueryVirtualMemory) fills for the
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

value_enum! {
    /// The memory region of [`MmAllocatePhysicalMemoryEx`](super::MmAllocatePhysicalMemoryEx),
    /// [`MmFreePhysicalMemory`](super::MmFreePhysicalMemory) and [`NtAllocateVirtualMemory`](super::NtAllocateVirtualMemory). The kernel
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

const _: () = {
    use core::mem::offset_of;
    assert!(offset_of!(MmStatistics, title) == 0xc);
    assert!(offset_of!(MmStatistics, system) == 0x38);
    assert!(offset_of!(MmStatistics, highest_physical_page) == 0x64);
    assert!(core::mem::size_of::<MmStatistics>() == 0x68);
};
