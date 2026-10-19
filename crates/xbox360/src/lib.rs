//! Safe interfaces to the Xbox 360 kernel and XAM for code built for the
//! `powerpc-unknown-xbox360` target.
//!
//! Titles that use the standard library use `std` for threads, locks, time,
//! allocation and output. This crate covers what `std` does not provide
//! (kernel events and spin locks, module exports, console information,
//! directory listing, the title loader, controllers, XAM user interface and
//! content), and serves `no_std` code such as system modules and small
//! titles.
//!
//! The crate re-exports topical crates under one name: `xbox360-status`,
//! `xbox360-atomic`, `xbox360-time`, `xbox360-sync`, `xbox360-thread`,
//! `xbox360-system` and `xbox360-xam`. A program that needs one area can
//! depend on its crate alone. The types of the kernel and XAM interfaces
//! come from `xbox360-sys`, re-exported as [`sys`]. [`status`] compiles on
//! every target. Every other module exists only for
//! `target_os = "xbox360"`.

#![no_std]
// The crate documentation links console-only items, which other targets
// lack unless built with `--cfg xbox360_docs`.
#![cfg_attr(
    not(any(target_os = "xbox360", xbox360_docs)),
    allow(rustdoc::broken_intra_doc_links)
)]

pub use xbox360_status as status;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_atomic as atomic;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_sync as sync;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::console;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::debug;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::directory;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::module;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::pool;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::title;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_thread as thread;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_time as time;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_xam as xam;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::dbg_print;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_system::dbg_println;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_thread::thread_entry;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub use xbox360_xam::wide_cstr;

/// The raw bindings this crate is built on.
pub use xbox360_sys as sys;
