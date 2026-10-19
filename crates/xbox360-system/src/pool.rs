//! A global allocator over the kernel pool.
//!
//! Titles that use the standard library already allocate through std's
//! system allocator. `no_std` code that uses `alloc` installs
//! [`PoolAllocator`]:
//!
//! ```ignore
//! #[global_allocator]
//! static ALLOCATOR: xbox360::pool::PoolAllocator =
//!     xbox360::pool::PoolAllocator::new(xbox360::pool::PoolTag::new(*b"Titl"));
//! ```

use core::alloc::GlobalAlloc;
use core::alloc::Layout;

use xbox360_sys::xboxkrnl;

pub use xbox360_sys::xboxkrnl::PoolTag;

/// Alignment of every pool block. Larger alignments are served by
/// allocating more and aligning inside the block.
pub const POOL_ALIGNMENT: usize = 8;

/// `GlobalAlloc` over `ExAllocatePoolWithTag` and `ExFreePool`.
#[derive(Debug, Clone, Copy)]
pub struct PoolAllocator {
    tag: PoolTag,
}

impl PoolAllocator {
    pub const fn new(tag: PoolTag) -> Self {
        Self { tag }
    }

    pub const fn tag(&self) -> PoolTag {
        self.tag
    }
}

// SAFETY: pool blocks are unique until freed. A block for an alignment
// above POOL_ALIGNMENT has its pool address in the word below the address
// returned, and dealloc receives the same layout to find it.
unsafe impl GlobalAlloc for PoolAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= POOL_ALIGNMENT {
            return allocate(layout.size(), self.tag);
        }

        let Some(request) = layout.size().checked_add(layout.align()) else {
            return core::ptr::null_mut();
        };

        let base = allocate(request, self.tag);
        if base.is_null() {
            return base;
        }

        // `base` is a multiple of POOL_ALIGNMENT and the alignment is
        // larger, so the aligned address is at least POOL_ALIGNMENT bytes
        // past `base` and the block holds `size` bytes after it.
        let aligned = (base as usize + layout.align()) & !(layout.align() - 1);
        let offset = aligned - base as usize;

        // SAFETY: `aligned` lies within the block, at least one word past
        // its start.
        unsafe {
            let user = base.add(offset);
            user.cast::<usize>().sub(1).write(base as usize);
            user
        }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let base = if layout.align() <= POOL_ALIGNMENT {
            pointer as usize
        } else {
            // SAFETY: alloc stored the pool address in the word below.
            unsafe { pointer.cast::<usize>().sub(1).read() }
        };

        // SAFETY: `base` is the pool address that alloc obtained for this
        // block, and GlobalAlloc callers free each block once.
        unsafe { xboxkrnl::ExFreePool(base as u32) };
    }
}

/// Allocate `size` bytes of pool. Null on failure.
fn allocate(size: usize, tag: PoolTag) -> *mut u8 {
    // SAFETY: the call takes no pointers; failure returns 0, which is null.
    // usize is 32 bits on this target.
    unsafe { xboxkrnl::ExAllocatePoolWithTag(size as u32, tag) as usize as *mut u8 }
}
