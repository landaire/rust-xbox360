//! Xbox 360 PAL.
//!
//! The core PAL surface (init/cleanup/abort) mirrors `unsupported`. Kernel
//! and XAM calls go through `xbox360-sys`, whose import stubs give each
//! callee the console calling convention.

mod common;
pub use common::*;

pub mod futex;
