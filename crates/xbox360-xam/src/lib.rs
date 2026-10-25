//! XAM: controllers, user interface, content packages, users and profiles,
//! notifications, system settings and system resource locators.
//!
//! The signatures and structure layouts of these modules come from Xenia's
//! implementations of the exports. Each module states what was checked
//! only against Xenia.

#![no_std]
#![cfg(any(target_os = "xbox360", xbox360_docs))]

pub mod content;
pub mod input;
pub mod media;
pub mod media_library;
pub mod music;
pub mod notify;
pub mod net;
mod overlapped;
pub mod profile;
pub mod resource;
pub mod system;
pub mod title;
pub mod ui;
pub mod user;
mod wide;

pub use xbox360_sys::xam::UserIndex;

pub use self::overlapped::AsyncOperation;
pub use self::overlapped::AsyncState;

pub use self::wide::WideCStr;
pub use self::wide::encode_utf16;
pub use self::wide::utf16_length;
