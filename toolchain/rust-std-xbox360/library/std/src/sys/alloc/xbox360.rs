//! Global allocator for the Xbox 360 devkit.
//!
//! Backed by the kernel pool (`ExAllocatePoolWithTag`/`ExFreePool`). The pool
//! guarantees at least machine-word alignment, so an over-allocation shim
//! stashes the real base pointer just below the aligned pointer we return.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::mem::size_of;
use crate::sys::pal::abi;

const TAG: u32 = 0x5273_7478; // "Rtx"

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align().max(size_of::<usize>());
        // Room to align up past a stashed base pointer.
        let request = layout.size() + align + size_of::<usize>();
        let base = unsafe { abi::ExAllocatePoolWithTag(request, TAG) };
        if base.is_null() {
            return crate::ptr::null_mut();
        }
        let base_addr = base as usize;
        let user = (base_addr + size_of::<usize>() + align - 1) & !(align - 1);
        unsafe { ((user - size_of::<usize>()) as *mut usize).write(base_addr) };
        user as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let base = unsafe { ((ptr as usize - size_of::<usize>()) as *const usize).read() };
        unsafe { abi::ExFreePool(base as *mut u8) };
    }
}
