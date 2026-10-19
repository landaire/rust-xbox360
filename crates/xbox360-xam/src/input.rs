//! Controller state, capabilities and vibration.
//!
//! Checked only against Xenia: the ordinals (0x190 `XamInputGetCapabilities`,
//! 0x191 `XamInputGetState`, 0x192 `XamInputSetState`), the argument order,
//! the structure layouts, and the use of `XINPUT_FLAG_GAMEPAD` for the flags
//! argument. Xenia ignores the second argument of `XamInputSetState`; this
//! module passes 0.

use xbox360_sys::xam;

use super::UserIndex;
use xbox360_status::Win32Error;
use xbox360_status::Win32ErrorCode;

pub use xbox360_sys::xam::Buttons;
pub use xbox360_sys::xam::Capabilities;
pub use xbox360_sys::xam::CapabilityFlags;
pub use xbox360_sys::xam::DeviceKind;
pub use xbox360_sys::xam::DeviceSubkind;
pub use xbox360_sys::xam::DeviceSubtype;
pub use xbox360_sys::xam::DeviceType;
pub use xbox360_sys::xam::Gamepad;
pub use xbox360_sys::xam::InputFlags;
pub use xbox360_sys::xam::InputState;
pub use xbox360_sys::xam::Keystroke;
pub use xbox360_sys::xam::KeystrokeFlags;
pub use xbox360_sys::xam::KeystrokeQuery;
pub use xbox360_sys::xam::PadKey;
pub use xbox360_sys::xam::PacketNumber;
pub use xbox360_sys::xam::Vibration;

/// State of the controller of `user`. `None` without a controller.
pub fn state(user: UserIndex) -> Result<Option<InputState>, Win32Error> {
    let mut state = InputState::default();

    // SAFETY: `state` is a live local `XINPUT_STATE`.
    let code = unsafe { xam::XamInputGetState(user, InputFlags::GAMEPAD, &mut state) };

    Ok(connected(code)?.then_some(state))
}

/// Capabilities of the controller of `user`. `None` without a controller.
pub fn capabilities(user: UserIndex) -> Result<Option<Capabilities>, Win32Error> {
    let mut capabilities = Capabilities::default();

    // SAFETY: `capabilities` is a live local `XINPUT_CAPABILITIES`.
    let code =
        unsafe { xam::XamInputGetCapabilities(user, InputFlags::GAMEPAD, &mut capabilities) };

    Ok(connected(code)?.then_some(capabilities))
}

/// Set the motor speeds of the controller of `user`.
pub fn set_vibration(user: UserIndex, vibration: Vibration) -> Result<(), Win32Error> {
    // The second argument of `XamInputSetState`, which Xenia ignores.
    const SET_STATE_RESERVED: u32 = 0;

    // SAFETY: `vibration` is a live local `XINPUT_VIBRATION`.
    unsafe { xam::XamInputSetState(user, SET_STATE_RESERVED, &vibration) }.result()
}

/// The next queued key transition of any user and device class, as dash
/// polls it once per frame. `None` when the queue is empty.
pub fn next_keystroke() -> Option<Keystroke> {
    // With ANY_USER the slot argument must hold user 0; XAM writes back the
    // slot of the keystroke, which is also in `Keystroke::user_index`.
    let mut user = 0;
    let mut keystroke = Keystroke::default();
    // SAFETY: both outputs are live locals.
    let code = unsafe {
        xam::XamInputGetKeystrokeEx(&mut user, KeystrokeQuery::ALL_DEVICES | KeystrokeQuery::ANY_USER, &mut keystroke)
    };
    code.result().is_ok().then_some(keystroke)
}

/// Whether a controller answered. `false` for `ERROR_DEVICE_NOT_CONNECTED`.
fn connected(code: Win32ErrorCode) -> Result<bool, Win32Error> {
    match code.result() {
        Ok(()) => Ok(true),
        Err(Win32Error::DEVICE_NOT_CONNECTED) => Ok(false),
        Err(error) => Err(error),
    }
}
