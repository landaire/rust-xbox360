//! Console information, debug output, the title and module loaders, and a
//! global allocator over the kernel pool.

#![no_std]
#![cfg(any(target_os = "xbox360", xbox360_docs))]

pub mod console;
pub mod debug;
pub mod module;
pub mod pool;
pub mod title;
