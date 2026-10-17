//! Fields of kernel structures that the kernel acts on, and list links.

use core::sync::atomic::AtomicU8;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

macro_rules! kernel_field {
    ($(#[$meta:meta])* $name:ident($atomic:ty, $value:ty)) => {
        $(#[$meta])*
        #[repr(transparent)]
        pub struct $name(pub(super) $atomic);

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
