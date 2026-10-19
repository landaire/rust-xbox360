//! Sign-in state, XUIDs and gamertags of user slots.
//!
//! Checked only against Xenia: the ordinal (0x210 `XamUserGetSigninState`)
//! and the state values. Checked against the call sites of the 2.0.6717
//! dashboard and Xenia, not run on hardware: 0x20A `XamUserGetXUID`, 0x20C
//! `XamUserGetGamerTag` and 0x221 `XamUserIsOnlineEnabled`.

use xbox360_status::Hresult;
use xbox360_sys::xam;

use super::UserIndex;

pub use xbox360_sys::xam::SigninKind;
pub use xbox360_sys::xam::SigninState;

/// Units of a gamertag buffer, terminator included, as the dashboard
/// passes it.
pub const GAMERTAG_CAPACITY: usize = 16;

/// Sign-in state of `user`.
pub fn signin_state(user: UserIndex) -> SigninState {
    xam::XamUserGetSigninState(user)
}

/// XUID of `user` among the kinds selected by `flags`.
pub fn xuid(user: UserIndex, flags: u32) -> Result<u64, Hresult> {
    let mut xuid = 0;
    // SAFETY: `xuid` is a live local.
    let result = unsafe { xam::XamUserGetXUID(user, flags, &mut xuid) };
    if result.is_failure() { Err(result) } else { Ok(xuid) }
}

/// Whether the profile of `user` has an Xbox LIVE account.
pub fn is_online_enabled(user: UserIndex) -> bool {
    xam::XamUserIsOnlineEnabled(user).get()
}

/// `XamUserGetMembershipType` of `user`, low byte.
pub fn membership_type(user: UserIndex) -> u8 {
    const TYPE_MASK: u32 = 0xFF;
    (xam::XamUserGetMembershipType(user) & TYPE_MASK) as u8
}

/// A gamertag as XAM wrote it.
pub struct Gamertag {
    units: [u16; GAMERTAG_CAPACITY],
}

impl Gamertag {
    /// The gamertag up to its terminator.
    pub fn units(&self) -> &[u16] {
        let length = self.units.iter().position(|&unit| unit == 0).unwrap_or(self.units.len());
        &self.units[..length]
    }
}

/// Gamertag of a signed-in `user`.
pub fn gamertag(user: UserIndex) -> Result<Gamertag, Hresult> {
    let mut units = [0u16; GAMERTAG_CAPACITY];
    // SAFETY: XAM writes at most `GAMERTAG_CAPACITY` units to `units`.
    let result = unsafe { xam::XamUserGetGamerTag(user, units.as_mut_ptr(), GAMERTAG_CAPACITY as u32) };
    if result.is_failure() { Err(result) } else { Ok(Gamertag { units }) }
}
