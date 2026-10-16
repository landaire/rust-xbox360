//! Global allocator for the Xbox 360 devkit.
//!
//! Backed by the kernel pool (`ExAllocatePoolWithTag`/`ExFreePool`). The pool
//! guarantees at least machine-word alignment, so an over-allocation shim
//! stores the pool address just below the aligned pointer it returns.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::mem::size_of;
use crate::ptr;

use xbox360_sys::xboxkrnl::{self, PoolTag};

const TAG: PoolTag = PoolTag::new(*b"Rstx");

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align().max(size_of::<u32>());
        // Room to align up past the stored pool address. `Layout` bounds the
        // size, so the sum does not overflow.
        let request = layout.size() + align + size_of::<u32>();
        // SAFETY: allocating pool memory has no preconditions.
        let base = unsafe { xboxkrnl::ExAllocatePoolWithTag(request as u32, TAG) };
        if base == 0 {
            return ptr::null_mut();
        }
        let user = (base as usize + size_of::<u32>()).next_multiple_of(align);
        let user = ptr::with_exposed_provenance_mut::<u8>(user);
        // SAFETY: `user - 4` is at or above `base` and inside the block.
        unsafe { user.cast::<u32>().sub(1).write(base) };
        user
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: `alloc` stored the pool address in the word below `ptr`.
        let base = unsafe { ptr.cast::<u32>().sub(1).read() };
        // SAFETY: `base` is the pool address that `alloc` stored before `ptr`.
        unsafe { xboxkrnl::ExFreePool(base) };
    }
}
