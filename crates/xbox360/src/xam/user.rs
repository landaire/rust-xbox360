//! Sign-in state of user slots.
//!
//! Checked only against Xenia: the ordinal (0x210 `XamUserGetSigninState`)
//! and the state values.

use xbox360_sys::xam;

use super::UserIndex;

pub use xbox360_sys::xam::SigninKind;
pub use xbox360_sys::xam::SigninState;

/// Sign-in state of `user`.
pub fn signin_state(user: UserIndex) -> SigninState {
    xam::XamUserGetSigninState(user)
}
