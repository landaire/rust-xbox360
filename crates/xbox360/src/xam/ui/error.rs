use super::MAX_BUTTONS;
use crate::status::KernelError;
use crate::status::Win32Error;

/// Why [`show_message_box`](super::show_message_box) returned no button.
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
