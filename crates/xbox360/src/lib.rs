//! Safe interfaces to the Xbox 360 kernel and XAM for code built for the
//! `powerpc-unknown-xbox360` target.
//!
//! Titles that use the standard library use `std` for threads, locks, time,
//! allocation and output. This crate covers what `std` does not provide
//! (kernel events and spin locks, module exports, console information, the
//! title loader, controllers, XAM user interface and content), and serves
//! `no_std` code such as system modules and small titles.
//!
//! The types of the kernel and XAM interfaces come from `xbox360-sys`,
//! which this crate re-exports as [`sys`] and next to the wrappers that use
//! them. [`status`] compiles on every target. Every other module exists
//! only for `target_os = "xbox360"`.

#![no_std]
// The crate documentation links console-only items, which other targets
// lack unless built with `--cfg xbox360_docs`.
#![cfg_attr(
    not(any(target_os = "xbox360", xbox360_docs)),
    allow(rustdoc::broken_intra_doc_links)
)]

#[cfg(all(feature = "alloc", any(target_os = "xbox360", xbox360_docs)))]
extern crate alloc;

pub mod status;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod atomic;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod console;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod debug;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
mod error;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod module;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod pool;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod sync;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod thread;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod time;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod title;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
pub mod xam;

/// The raw bindings this crate is built on.
pub use xbox360_sys as sys;
