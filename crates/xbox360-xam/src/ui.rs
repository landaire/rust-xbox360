//! XAM message boxes, the virtual keyboard, the device selector and the
//! sign-in UI.
//!
//! The blocking [`show_message_box`] is checked only against Xenia:
//! the ordinal (0x2CA `XamShowMessageBoxUI`),
//! the nine arguments, the icon values, and completion through
//! `XOVERLAPPED`. Xenia in headless mode picks the focused button at once.
//! Xenia writes one word of the result. The 8-byte result buffer follows
//! the XDK's `MESSAGEBOX_RESULT` and is not checked on hardware.
//!
//! The asynchronous calls ([`start_message_box`], [`start_keyboard`],
//! [`start_device_selector`]) are checked against the call sites of the
//! 2.0.6717 dashboard: the ordinals (0x2C1 `XamShowKeyboardUI`, 0x2CB
//! `XamShowDeviceSelectorUI`, 0x2BC `XamShowSigninUI`, 0x2EC
//! `XamIsUIActive`), the argument order and the passcode flags. Not run on
//! hardware.

use core::pin::Pin;

use xbox360_status::Win32Error;
use xbox360_status::Win32ErrorCode;
use xbox360_sys::xam;
use xbox360_sys::xam::MessageBoxFlags;

use super::AsyncOperation;
use super::UserIndex;
use super::WideCStr;
use super::overlapped::OverlappedCall;

mod error;

pub use xbox360_sys::xam::DeviceSelectorFlags;
pub use xbox360_sys::xam::KeyboardFlags;
pub use xbox360_sys::xam::MessageBoxIcon;
pub use xbox360_sys::xam::MessageBoxResult;
pub use xbox360_sys::xam::SigninFlags;

pub use self::error::MessageBoxError;

/// Most buttons a message box shows.
pub const MAX_BUTTONS: usize = 3;

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

/// `XamIsUIActive`: a system UI is shown.
pub fn is_ui_active() -> bool {
    xam::XamIsUIActive().get()
}

/// Starts `XamShowMessageBoxUI` on `operation`, which is reset first. A
/// failure to show the box is returned; otherwise the result is read from
/// `operation`, and `result` holds the button or the passcode.
///
/// # Safety
///
/// `title`, `text`, the `buttons` array and the strings it points to, and
/// `result` stay valid and unmoved until `operation` completes. With
/// [`MessageBoxFlags::PASSCODE_VERIFY`], `result` holds the passcode to
/// compare.
pub unsafe fn start_message_box(
    operation: Pin<&AsyncOperation>,
    user: UserIndex,
    title: &WideCStr,
    text: &WideCStr,
    buttons: &[*const u16],
    focused_button: u32,
    flags: MessageBoxFlags,
    result: *mut MessageBoxResult,
) -> Result<(), Win32Error> {
    operation.reset();
    // Pointers are 32-bit, so the count fits in `u32`.
    let count = buttons.len() as u32;

    // SAFETY: the caller keeps the strings, the button array and `result`
    // valid until the pinned block completes.
    let code = unsafe {
        xam::XamShowMessageBoxUI(
            user,
            title.as_ptr(),
            text.as_ptr(),
            count,
            buttons.as_ptr().cast(),
            focused_button,
            flags,
            result.cast(),
            operation.as_ptr(),
        )
    };

    started(code)
}

/// Starts `XamShowKeyboardUI` on `operation`, which is reset first. The
/// text is written to `buffer` as NUL-terminated UTF-16.
///
/// # Safety
///
/// `default_text`, `title`, `description` and `buffer` stay valid and
/// unmoved until `operation` completes, and nothing reads or writes
/// `buffer` until then.
pub unsafe fn start_keyboard(
    operation: Pin<&AsyncOperation>,
    user: UserIndex,
    flags: KeyboardFlags,
    default_text: &WideCStr,
    title: &WideCStr,
    description: &WideCStr,
    buffer: *mut [u16],
) -> Result<(), Win32Error> {
    operation.reset();
    // Pointers are 32-bit, so the length fits in `u32`.
    let length = buffer.len() as u32;

    // SAFETY: the caller keeps the strings and `buffer` valid until the
    // pinned block completes.
    let code = unsafe {
        xam::XamShowKeyboardUI(
            user,
            flags,
            default_text.as_ptr(),
            title.as_ptr(),
            description.as_ptr(),
            buffer.cast(),
            length,
            operation.as_ptr(),
        )
    };

    started(code)
}

/// Starts `XamShowDeviceSelectorUI` on `operation`, which is reset first.
/// `device` holds the device selected first (0 for none) and receives the
/// chosen one.
///
/// # Safety
///
/// `device` stays valid and unmoved until `operation` completes.
pub unsafe fn start_device_selector(
    operation: Pin<&AsyncOperation>,
    user: UserIndex,
    content_type: u32,
    flags: DeviceSelectorFlags,
    bytes_requested: u64,
    device: *mut u32,
) -> Result<(), Win32Error> {
    operation.reset();

    // SAFETY: the caller keeps `device` valid until the pinned block
    // completes.
    let code = unsafe {
        xam::XamShowDeviceSelectorUI(user, content_type, flags, bytes_requested, device, operation.as_ptr())
    };

    started(code)
}

/// `XamShowSigninUI`. The UI does not report when it closes.
pub fn show_signin(panes: u32, flags: SigninFlags) -> Result<(), Win32Error> {
    // SAFETY: no pointers are passed.
    unsafe { xam::XamShowSigninUI(panes, flags) }.result()
}

/// `XamShowSigninUIp`: the sign-in UI for one controller. The UI does not
/// report when it closes.
pub fn show_signin_for(user: UserIndex, panes: u32, flags: SigninFlags) -> Result<(), Win32Error> {
    // SAFETY: no pointers are passed.
    unsafe { xam::XamShowSigninUIp(user, panes, flags) }.result()
}

/// A call shown (`ERROR_IO_PENDING`) or completed at once is started.
fn started(code: Win32ErrorCode) -> Result<(), Win32Error> {
    match code.result() {
        Err(Win32Error::IO_PENDING) | Ok(()) => Ok(()),
        Err(error) => Err(error),
    }
}
