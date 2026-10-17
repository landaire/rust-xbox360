//! XAM: controllers, user interface, content packages and sign-in state.
//!
//! The signatures and structure layouts of these modules come from Xenia's
//! implementations of the exports. Each module states what was checked
//! only against Xenia.

pub mod content;
pub mod input;
mod overlapped;
pub mod ui;
pub mod user;
mod wide;

pub use xbox360_sys::xam::UserIndex;

pub use self::wide::WideCStr;
pub use self::wide::encode_utf16;
pub use self::wide::utf16_length;
