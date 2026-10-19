//! Physically contiguous memory that the GPU reads or writes.

use core::ptr::NonNull;

use xbox360_sys::xboxkrnl::MemoryRegion;
use xbox360_sys::xboxkrnl::MmAllocatePhysicalMemoryEx;
use xbox360_sys::xboxkrnl::MmFreePhysicalMemory;
use xbox360_sys::xboxkrnl::MmGetPhysicalAddress;
use xbox360_sys::xboxkrnl::PageProtection;

/// GPU (physical) address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuAddress(pub u32);

/// CPU caching of a [`PhysicalBuffer`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caching {
    /// Stores merge in the write-combining buffers; for memory the CPU only
    /// writes in order.
    WriteCombined,
    /// Every access reaches memory; for memory the GPU writes and the CPU
    /// polls.
    Uncached,
}

/// Owned physically contiguous allocation in the title region.
pub struct PhysicalBuffer {
    virtual_address: NonNull<u32>,
    physical: GpuAddress,
    bytes: u32,
}

impl PhysicalBuffer {
    /// Allocate `bytes` (a multiple of 4 KB) aligned to 4 KB, or `None`.
    pub fn new(bytes: u32, caching: Caching) -> Option<Self> {
        let protect = PageProtection::READWRITE
            | match caching {
                Caching::WriteCombined => PageProtection::WRITECOMBINE,
                Caching::Uncached => PageProtection::NOCACHE,
            };
        // The GPU addresses the low 512 MB.
        // SAFETY: the call has no preconditions; 0 reports failure.
        let address = unsafe { MmAllocatePhysicalMemoryEx(MemoryRegion::Title, bytes, protect, 0, 0x1fff_ffff, 0x1000) };
        let virtual_address = NonNull::new(address as usize as *mut u32)?;
        // SAFETY: the address is mapped.
        let physical = GpuAddress(unsafe { MmGetPhysicalAddress(address) });
        Some(Self { virtual_address, physical, bytes })
    }

    pub fn physical(&self) -> GpuAddress {
        self.physical
    }

    /// Virtual address of byte `offset`.
    pub fn virtual_address(&self, offset: u32) -> u32 {
        self.virtual_address.as_ptr() as u32 + offset
    }

    pub fn words(&mut self) -> &mut [u32] {
        // SAFETY: the allocation is `bytes` long, 4 KB aligned and owned by `self`.
        unsafe { core::slice::from_raw_parts_mut(self.virtual_address.as_ptr(), (self.bytes / 4) as usize) }
    }

    /// Volatile read of the word at byte `offset`, for memory the GPU writes.
    pub fn read_word(&self, offset: u32) -> u32 {
        debug_assert!(offset < self.bytes);
        // SAFETY: in bounds and aligned.
        unsafe { self.virtual_address.as_ptr().add((offset / 4) as usize).read_volatile() }
    }
}

impl Drop for PhysicalBuffer {
    fn drop(&mut self) {
        // SAFETY: allocated in the title region by `new`; the owner stops the GPU from using it first.
        unsafe { MmFreePhysicalMemory(MemoryRegion::Title, self.virtual_address.as_ptr() as u32) };
    }
}
