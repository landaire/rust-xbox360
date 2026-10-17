//! Raw bindings to the Xbox 360 kernel and XAM for the
//! `powerpc-unknown-xbox360` target: imports, exports, entries for callers
//! that use the console ABI, the interlocked operations the kernel loader
//! accepts, and the types and constants of the import signatures.
//!
//! Each import is a 16-byte thunk in its own `.xthunks.*` section. The first
//! big-endian DWORD is the import record `(type<<24 | lib<<16 | ordinal)`.
//! `xex-link` collects these into the XEX `ImportLibraries` table, and at load
//! time Xenia (and a real console) rewrites the thunk to a syscall trap. The
//! remaining DWORDs (`nop; nop; blr`) are only a fallback if never patched.
//!
//! Console callees home r3-r10 into the caller frame at sp+0x10..sp+0x50.
//! SysV frames do not reserve that area, so callers reach each thunk through
//! a stub that allocates it.
//!
//! Console callers expect r14-r31 preserved in full 64 bits. Code for this
//! target saves only their low words, so code that the kernel, the loader or
//! another module calls (an entry point, an export, a thread start routine,
//! a callback) enters through [`entry_bridge`], which [`bridged!`] and
//! [`export!`] arrange.
//!
//! Types, constants, structure layouts and [`status`] compile on every
//! target, so host code can share them. Imports, the functions of
//! [`processor`], [`mmio`], [`interlocked`] and the entry bridge exist only
//! for `target_os = "xbox360"`. Documentation builds them on another
//! PowerPC target with `--cfg xbox360_docs`.
//!
//! Every type in an import signature is 32 bits wide or narrower and passes
//! like the C integer of its width: enums are `#[repr(u32)]` (or the C
//! width), and newtypes are `#[repr(transparent)]`. Values the kernel
//! returns are newtypes, so an unknown value is never undefined behaviour.
//! The few exports that take a 64-bit value have adapters that join the
//! SysV register pair into one register.
//!
//! Imports are `unsafe extern "C" fn`. The few that take no address or
//! object the kernel uses and have no effect that safe code relies on are
//! declared `safe` and are plain `extern "C" fn`.

#![no_std]
#![allow(non_snake_case, non_upper_case_globals)]
// The crate documentation links console-only items, which other targets
// lack unless built with `--cfg xbox360_docs`.
#![cfg_attr(
    not(any(target_os = "xbox360", xbox360_docs)),
    allow(rustdoc::broken_intra_doc_links)
)]

#[macro_use]
mod flags;
#[macro_use]
mod value_enum;
#[macro_use]
mod import;

mod entry;
mod error;
mod export;
pub mod image;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod interlocked;
pub mod lib_index;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod mmio;
pub mod processor;
pub mod status;
pub mod xam;
pub mod xboxkrnl;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use crate::entry::entry_bridge;
pub use crate::error::UnknownValue;
pub use crate::export::ExportRecord;
pub use crate::import::ImportedVariable;
pub use crate::import::Ordinal;
