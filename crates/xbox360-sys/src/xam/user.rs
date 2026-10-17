value_enum! {
    /// A user slot, or a choice the export makes.
    #[non_exhaustive]
    pub enum UserIndex: u32 {
        User0 = 0,
        User1 = 1,
        User2 = 2,
        User3 = 3,
        /// No user, for operations on behalf of the title itself.
        NoUser = 0xfe,
        /// Any signed-in user or connected controller, as the export decides.
        Any = 0xff,
    }
}

impl UserIndex {
    /// Slot 0 to 3. `None` otherwise.
    pub const fn from_slot(slot: u8) -> Option<Self> {
        match slot {
            0 => Some(Self::User0),
            1 => Some(Self::User1),
            2 => Some(Self::User2),
            3 => Some(Self::User3),
            _ => None,
        }
    }
}

/// Sign-in state as XAM reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SigninState(u32);

/// Sign-in states this crate names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SigninKind {
    NotSignedIn,
    SignedInLocally,
    SignedInToLive,
}

impl SigninState {
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// `None` for a state this crate does not name.
    pub const fn kind(self) -> Option<SigninKind> {
        match self.0 {
            0 => Some(SigninKind::NotSignedIn),
            1 => Some(SigninKind::SignedInLocally),
            2 => Some(SigninKind::SignedInToLive),
            _ => None,
        }
    }
}
