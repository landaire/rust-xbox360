//! XAM message boxes.
//!
//! Checked only against Xenia: the ordinal (0x2CA `XamShowMessageBoxUI`),
//! the nine arguments, the icon values, and completion through
//! `XOVERLAPPED`. Xenia in headless mode picks the focused button at once.
//! Xenia writes one word of the result. The 8-byte result buffer follows
//! the XDK's `MESSAGEBOX_RESULT` and is not checked on hardware.

use xbox360_sys::xam;
use xbox360_sys::xam::MessageBoxFlags;

use super::OverlappedCall;
use super::UserIndex;
use super::WideCStr;
use crate::status::KernelError;
use crate::status::Win32Error;

/// Most buttons a message box shows.
pub const MAX_BUTTONS: usize = 3;

pub use xbox360_sys::xam::MessageBoxIcon;

/// A message box for [`show_message_box`].
#[derive(Clone, Copy)]
pub struct MessageBox<'a> {
    pub user: UserIndex,
    pub title: &'a WideCStr,
    pub text: &'a WideCStr,
    /// One to [`MAX_BUTTONS`] button labels.
    pub buttons: &'a [&'a WideCStr],
    /// Index into `buttons` of the button that has the focus first.
    pub focused_button: usize,
    pub icon: MessageBoxIcon,
}

/// Why [`show_message_box`] returned no button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MessageBoxError {
    /// No buttons, or more than [`MAX_BUTTONS`].
    ButtonCount { count: usize },
    /// `focused_button` is not an index into `buttons`.
    FocusedButton { index: usize, count: usize },
    /// Creating the completion event failed.
    Kernel(KernelError),
    /// XAM failed or the user closed the box (`ERROR_CANCELLED`).
    Xam(Win32Error),
    /// XAM reported a button the box does not have.
    ChosenButton { index: u32, count: usize },
}

impl core::fmt::Display for MessageBoxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ButtonCount { count } => write!(
                f,
                "a message box takes 1 to {MAX_BUTTONS} buttons, not {count}"
            ),
            Self::FocusedButton { index, count } => {
                write!(f, "focused button {index} is not one of {count} buttons")
            }
            Self::Kernel(error) => error.fmt(f),
            Self::Xam(error) => error.fmt(f),
            Self::ChosenButton { index, count } => write!(f, "XAM chose button {index} of {count}"),
        }
    }
}

impl core::error::Error for MessageBoxError {}

/// Show a message box and wait until the user chooses a button. Returns
/// the index of the button.
///
/// # Panics
///
/// If waiting for the box fails, which leaves XAM with pointers into this
/// call's frame.
pub fn show_message_box(message_box: &MessageBox<'_>) -> Result<usize, MessageBoxError> {
    let count = message_box.buttons.len();
    if count == 0 || count > MAX_BUTTONS {
        return Err(MessageBoxError::ButtonCount { count });
    }
    if message_box.focused_button >= count {
        return Err(MessageBoxError::FocusedButton {
            index: message_box.focused_button,
            count,
        });
    }
    let mut buttons = [0u32; MAX_BUTTONS];
    for (slot, label) in buttons.iter_mut().zip(message_box.buttons) {
        *slot = label.as_ptr() as usize as u32;
    }
    let overlapped = OverlappedCall::new().map_err(MessageBoxError::Kernel)?;
    // MESSAGEBOX_RESULT: the button index, or a four-halfword passcode.
    let mut result = [0u32; 2];
    // SAFETY: the strings, `buttons`, `result` and the overlapped block
    // outlive the call, and `complete` waits for XAM to finish with them
    // before they are released.
    let status = unsafe {
        xam::XamShowMessageBoxUI(
            message_box.user,
            message_box.title.as_ptr(),
            message_box.text.as_ptr(),
            count as u32,
            buttons.as_ptr(),
            message_box.focused_button as u32,
            MessageBoxFlags::new(message_box.icon),
            result.as_mut_ptr(),
            overlapped.as_ptr(),
        )
    };
    overlapped.complete(status).map_err(MessageBoxError::Xam)?;
    let chosen = result[0];
    match usize::try_from(chosen) {
        Ok(index) if index < count => Ok(index),
        _ => Err(MessageBoxError::ChosenButton {
            index: chosen,
            count,
        }),
    }
}
