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

/// The application of the messages the 2.0.6717 dashboard sends when it
/// signs a user out.
const SIGN_OUT_APP: u32 = 0xF7;
const MESSENGER_STATE_MESSAGE: u32 = 0x20_0002;
const PREPARE_SIGN_OUT_MESSAGE: u32 = 0x20_0018;

/// Message 0x200002 of application 0xF7 for `user`: the first of its two
/// outputs. The 2.0.6717 dashboard's sign-out confirmation says the user
/// is also signed out of Messenger when it is 1. The second output is
/// not read; it gets the 16 bytes the dashboard's stack frame leaves it.
pub fn messenger_state(user: UserIndex) -> Result<u32, Hresult> {
    let mut state = 0u32;
    let mut second = [0u32; 4];
    // Pointers are 32-bit on the console.
    let mut block = [
        user.raw(),
        &raw mut state as usize as u32,
        second.as_mut_ptr() as usize as u32,
    ];
    // SAFETY: the block names live locals that outlive the synchronous
    // request.
    let result = unsafe { app_request(SIGN_OUT_APP, MESSENGER_STATE_MESSAGE, &mut block) };
    if result.is_failure() { Err(result) } else { Ok(state) }
}

/// Message 0x200018 of application 0xF7 with `{user, 1, 0}`, which the
/// 2.0.6717 dashboard sends before `XamUserLogon` signs the user out. Its
/// effect is not known.
pub fn prepare_sign_out(user: UserIndex) -> Hresult {
    let mut block = [user.raw(), 1, 0];
    // SAFETY: the block holds no pointers.
    unsafe { app_request(SIGN_OUT_APP, PREPARE_SIGN_OUT_MESSAGE, &mut block) }
}

/// A request without an overlapped block, as the dashboard's wrapper
/// 0x92586528 makes it: `XMsgInProcessCall` in a system process, else a
/// synchronous `XMsgStartIORequest` that fails with the thread's last
/// error.
///
/// # Safety
///
/// Every pointer in `block` must be valid for the accesses the message
/// makes until the call returns.
unsafe fn app_request(app: u32, message: u32, block: &mut [u32]) -> Hresult {
    // Pointers are 32-bit, so a slice length fits in `u32`.
    let size = size_of_val(block) as u32;
    if xbox360_sys::xboxkrnl::KeGetCurrentProcessType() == xbox360_sys::xboxkrnl::ProcessType::SYSTEM {
        // SAFETY: the caller keeps the block valid; the second parameter
        // is the block size, as the dashboard passes it.
        return unsafe { xam::XMsgInProcessCall(app, message, block.as_mut_ptr(), size as usize as *mut u32) };
    }
    // SAFETY: the caller keeps the block valid until the synchronous
    // request returns.
    let result = unsafe { xam::XMsgStartIORequest(app, message, core::ptr::null_mut(), block.as_mut_ptr().cast(), size) };
    if result.is_failure() {
        return result;
    }
    match xbox360_status::last_error() {
        None => Hresult(0),
        Some(error) if (error.code() as i32) > 0 => Hresult(0x8007_0000 | (error.code() & 0xFFFF)),
        Some(error) => Hresult(error.code()),
    }
}
