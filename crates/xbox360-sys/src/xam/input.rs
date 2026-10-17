flags! {
    /// `XINPUT_FLAG_*`: which devices a controller query covers.
    pub struct InputFlags(u32) {
        GAMEPAD = 0x1,
    }
}

/// `XINPUT_DEVTYPE_*` as XAM reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DeviceType(u8);

/// Device types this crate names.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    Gamepad,
}

impl DeviceType {
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// `None` for a type this crate does not name.
    pub const fn kind(self) -> Option<DeviceKind> {
        match self.0 {
            0x01 => Some(DeviceKind::Gamepad),
            _ => None,
        }
    }
}

/// `XINPUT_DEVSUBTYPE_*` as XAM reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DeviceSubtype(u8);

/// Device subtypes this crate names.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceSubkind {
    Gamepad,
}

impl DeviceSubtype {
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// `None` for a subtype this crate does not name.
    pub const fn kind(self) -> Option<DeviceSubkind> {
        match self.0 {
            0x01 => Some(DeviceSubkind::Gamepad),
            _ => None,
        }
    }
}

flags! {
    /// Pressed buttons of a gamepad.
    pub struct Buttons(u16) {
        DPAD_UP = 0x0001,
        DPAD_DOWN = 0x0002,
        DPAD_LEFT = 0x0004,
        DPAD_RIGHT = 0x0008,
        START = 0x0010,
        BACK = 0x0020,
        LEFT_THUMB = 0x0040,
        RIGHT_THUMB = 0x0080,
        LEFT_SHOULDER = 0x0100,
        RIGHT_SHOULDER = 0x0200,
        GUIDE = 0x0400,
        A = 0x1000,
        B = 0x2000,
        X = 0x4000,
        Y = 0x8000,
    }
}

/// `X_INPUT_GAMEPAD`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Gamepad {
    pub buttons: Buttons,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub thumb_left_x: i16,
    pub thumb_left_y: i16,
    pub thumb_right_x: i16,
    pub thumb_right_y: i16,
}

/// Counter that changes whenever the controller state changes.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct PacketNumber(u32);

impl PacketNumber {
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// `X_INPUT_STATE`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InputState {
    pub packet_number: PacketNumber,
    pub gamepad: Gamepad,
}

/// `X_INPUT_VIBRATION`: motor speeds, 0 to 65535.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vibration {
    pub left_motor_speed: u16,
    pub right_motor_speed: u16,
}

flags! {
    /// `X_INPUT_CAPS` flags.
    pub struct CapabilityFlags(u16) {
        FORCE_FEEDBACK = 0x0001,
        WIRELESS = 0x0002,
        VOICE = 0x0004,
        PLUG_IN_MODULES = 0x0008,
        NO_NAVIGATION = 0x0010,
    }
}

/// `X_INPUT_CAPABILITIES`. The gamepad and vibration fields hold the
/// ranges the device supports.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Capabilities {
    pub device_type: DeviceType,
    pub device_subtype: DeviceSubtype,
    pub flags: CapabilityFlags,
    pub gamepad: Gamepad,
    pub vibration: Vibration,
}

const _: () = assert!(core::mem::size_of::<Gamepad>() == 12);
const _: () = assert!(core::mem::size_of::<InputState>() == 16);
const _: () = assert!(core::mem::size_of::<Vibration>() == 4);
const _: () = assert!(core::mem::size_of::<Capabilities>() == 20);
