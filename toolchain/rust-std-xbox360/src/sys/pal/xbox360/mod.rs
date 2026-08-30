//! Xbox 360 devkit PAL.
//!
//! The core PAL surface (init/cleanup/abort) mirrors `unsupported`; the kernel
//! and networking ABI lives in [`abi`], where each import is a naked thunk in
//! the `.xthunks` section that `xex-link` rewrites into an IAT jump stub.

mod common;
pub use common::*;

pub mod abi;
