//! xam.xex imports, structures, and the types and constants of their
//! arguments.
//!
//! The controller, content and message box structures and values come from
//! Xenia's implementations of the exports.

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::lib_index::XAM;
pub use crate::status::Win32ErrorCode;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::AccessMask;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::FileAttributes;
use crate::xboxkrnl::Handle;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::ShareAccess;

pub use crate::xboxkrnl::Boolean;

value_enum! {
    /// `XNCALLER_*`: on whose behalf a network call is made.
    #[non_exhaustive]
    pub enum XnCaller: u32 {
        Title = 1,
        SystemApp = 2,
        DebugMonitor = 3,
    }
}

/// A Winsock socket, or [`Socket::INVALID`] from a call that failed.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Socket(u32);

impl Socket {
    /// `INVALID_SOCKET`.
    pub const INVALID: Self = Self(u32::MAX);

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Result of a Winsock call that has no count to return: 0 or this.
pub const SOCKET_ERROR: i32 = -1;

value_enum! {
    /// Address family of a socket.
    #[non_exhaustive]
    pub enum AddressFamily: u32 {
        /// `AF_INET`.
        Inet = 2,
    }
}

value_enum! {
    /// `SOCK_*` socket type.
    #[non_exhaustive]
    pub enum SocketType: u32 {
        Stream = 1,
        Datagram = 2,
    }
}

value_enum! {
    /// `IPPROTO_*` socket protocol.
    #[non_exhaustive]
    pub enum Protocol: u32 {
        Tcp = 6,
        Udp = 17,
    }
}

value_enum! {
    /// `SD_*`: what `NetDll_shutdown` disables.
    pub enum ShutdownHow: i32 {
        Receive = 0,
        Send = 1,
        Both = 2,
    }
}

value_enum! {
    /// `NetDll_ioctlsocket` command.
    #[non_exhaustive]
    pub enum IoctlCommand: u32 {
        /// `FIONBIO`: set non-blocking mode from a nonzero argument.
        Fionbio = 0x8004_667e,
        /// `FIONREAD`: store the number of bytes ready to read.
        Fionread = 0x4004_667f,
    }
}

value_enum! {
    /// Level of a socket option.
    #[non_exhaustive]
    pub enum SocketLevel: i32 {
        /// `SOL_SOCKET`.
        Socket = 0xffff,
        /// `IPPROTO_TCP`.
        Tcp = 6,
    }
}

/// A socket option. The numbers of different levels overlap.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketOption(pub i32);

impl SocketOption {
    pub const SO_REUSEADDR: Self = Self(0x4);
    pub const SO_KEEPALIVE: Self = Self(0x8);
    pub const SO_BROADCAST: Self = Self(0x20);
    pub const SO_LINGER: Self = Self(0x80);
    pub const SO_SNDBUF: Self = Self(0x1001);
    pub const SO_RCVBUF: Self = Self(0x1002);
    pub const SO_SNDTIMEO: Self = Self(0x1005);
    pub const SO_RCVTIMEO: Self = Self(0x1006);
    pub const SO_ERROR: Self = Self(0x1007);
    pub const SO_TYPE: Self = Self(0x1008);
    /// Level [`SocketLevel::Tcp`].
    pub const TCP_NODELAY: Self = Self(0x1);
}

flags! {
    /// `MSG_*` flags of sends and receives.
    pub struct MessageFlags(u32) {
        OOB = 0x1,
        PEEK = 0x2,
        DONTROUTE = 0x4,
    }
}

flags! {
    /// `FD_*` network events of `NetDll_WSAEventSelect`.
    pub struct NetworkEvents(u32) {
        READ = 0x1,
        WRITE = 0x2,
        OOB = 0x4,
        ACCEPT = 0x8,
        CONNECT = 0x10,
        CLOSE = 0x20,
    }
}

/// Sockets per select set.
pub const FD_SETSIZE: usize = 64;

/// An XNet option of `NetDll_XNetGetOpt` and `NetDll_XNetSetOpt`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XnetOption(pub u32);

impl XnetOption {
    /// Promiscuous receive.
    pub const PROMISCUOUS: Self = Self(0x1390);
    /// Link flags of the network interface.
    pub const LINK_FLAGS: Self = Self(0x1395);
    /// Reports the secondary network interface.
    pub const SECONDARY_NIC: Self = Self(0x2f00);
}

flags! {
    /// `XNET_GET_XNADDR_*` result of the XNADDR queries. An empty value is
    /// `XNET_GET_XNADDR_PENDING`.
    pub struct XnAddrStatus(u32) {
        NONE = 0x1,
        ETHERNET = 0x2,
        STATIC = 0x4,
        DHCP = 0x8,
        PPPOE = 0x10,
        GATEWAY = 0x20,
        DNS = 0x40,
        ONLINE = 0x80,
        TROUBLESHOOT = 0x8000,
    }
}

flags! {
    /// `XNET_ETHERNET_LINK_*` status of the network interface.
    pub struct EthernetLinkStatus(u32) {
        ACTIVE = 0x1,
        SPEED_100MBPS = 0x2,
        SPEED_10MBPS = 0x4,
        FULL_DUPLEX = 0x8,
        HALF_DUPLEX = 0x10,
    }
}

/// IPv4 socket address. Port and address are in network byte order.
#[repr(C)]
pub struct SockAddrIn {
    pub family: u16,
    pub port: u16,
    pub address: u32,
    pub zero: [u8; 8],
}

/// Winsock descriptor set.
#[repr(C)]
pub struct FdSet {
    pub count: u32,
    pub sockets: [Socket; FD_SETSIZE],
}

impl FdSet {
    pub const fn new() -> Self {
        Self {
            count: 0,
            sockets: [Socket(0); FD_SETSIZE],
        }
    }
}

impl Default for FdSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Winsock time interval.
#[repr(C)]
pub struct TimeVal {
    pub seconds: i32,
    pub microseconds: i32,
}

impl TimeVal {
    pub const fn from_milliseconds(milliseconds: u32) -> Self {
        Self {
            seconds: (milliseconds / 1000) as i32,
            microseconds: ((milliseconds % 1000) * 1000) as i32,
        }
    }
}

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

/// `XOVERLAPPED`: completion state of an asynchronous XAM call.
#[repr(C)]
#[derive(Debug, Default)]
pub struct Overlapped {
    pub result: Win32ErrorCode,
    pub length: u32,
    pub context: u32,
    /// Event that XAM signals on completion, or [`Handle::NULL`].
    pub event: Handle,
    pub completion_routine: u32,
    pub completion_context: u32,
    pub extended_error: u32,
}

const _: () = assert!(core::mem::size_of::<Overlapped>() == 0x1c);

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

/// A storage device id, as the device selector reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentDevice(u32);

impl ContentDevice {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub const fn id(self) -> u32 {
        self.0
    }
}

value_enum! {
    /// Kind of a content package (`XCONTENTTYPE_*`), with Xenia's values.
    #[non_exhaustive]
    pub enum ContentType: u32 {
        SavedGame = 0x0000_0001,
        MarketplaceContent = 0x0000_0002,
        Publisher = 0x0000_0003,
        Xbox360Title = 0x0000_1000,
        IptvPauseBuffer = 0x0000_2000,
        XnaCommunity = 0x0000_3000,
        InstalledGame = 0x0000_4000,
        XboxTitle = 0x0000_5000,
        SocialTitle = 0x0000_6000,
        GamesOnDemand = 0x0000_7000,
        StoragePack = 0x0000_8000,
        AvatarItem = 0x0000_9000,
        Profile = 0x0001_0000,
        GamerPicture = 0x0002_0000,
        Theme = 0x0003_0000,
        CacheFile = 0x0004_0000,
        StorageDownload = 0x0005_0000,
        XboxSavedGame = 0x0006_0000,
        XboxDownload = 0x0007_0000,
        GameDemo = 0x0008_0000,
        Video = 0x0009_0000,
        GameTitle = 0x000a_0000,
        Installer = 0x000b_0000,
        GameTrailer = 0x000c_0000,
        ArcadeTitle = 0x000d_0000,
        Xna = 0x000e_0000,
        LicenseStore = 0x000f_0000,
        Movie = 0x0010_0000,
        Tv = 0x0020_0000,
        MusicVideo = 0x0030_0000,
        GameVideo = 0x0040_0000,
        PodcastVideo = 0x0050_0000,
        ViralVideo = 0x0060_0000,
        CommunityGame = 0x0200_0000,
    }
}

/// Length of the display name, in UTF-16 units including the NUL.
pub const DISPLAY_NAME_UNITS: usize = 128;
/// Length of the file name in bytes. A name of this length has no NUL.
pub const FILE_NAME_BYTES: usize = 42;

/// `XCONTENT_DATA`: the device, kind and names of a package.
#[repr(C)]
#[derive(Clone)]
pub struct ContentData {
    device: ContentDevice,
    content_type: ContentType,
    display_name: [u16; DISPLAY_NAME_UNITS],
    file_name: [u8; FILE_NAME_BYTES],
    padding: [u8; 2],
}

const _: () = assert!(core::mem::size_of::<ContentData>() == 0x134);

/// Why [`ContentData::new`] rejected a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentDataError {
    /// The display name needs more than 127 UTF-16 units.
    DisplayNameTooLong { units: usize },
    /// The file name is empty or longer than [`FILE_NAME_BYTES`].
    FileNameLength { bytes: usize },
    /// The file name contains a NUL byte.
    FileNameNul,
}

impl core::fmt::Display for ContentDataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DisplayNameTooLong { units } => {
                write!(
                    f,
                    "display name of {units} UTF-16 units exceeds {}",
                    DISPLAY_NAME_UNITS - 1
                )
            }
            Self::FileNameLength { bytes } => write!(
                f,
                "file name of {bytes} bytes is not 1 to {FILE_NAME_BYTES}"
            ),
            Self::FileNameNul => f.write_str("file name contains NUL"),
        }
    }
}

impl core::error::Error for ContentDataError {}

impl ContentData {
    pub fn new(
        device: ContentDevice,
        content_type: ContentType,
        display_name: &str,
        file_name: &[u8],
    ) -> Result<Self, ContentDataError> {
        if file_name.is_empty() || file_name.len() > FILE_NAME_BYTES {
            return Err(ContentDataError::FileNameLength {
                bytes: file_name.len(),
            });
        }
        if file_name.contains(&0) {
            return Err(ContentDataError::FileNameNul);
        }
        let units = display_name.encode_utf16().count();
        if units >= DISPLAY_NAME_UNITS {
            return Err(ContentDataError::DisplayNameTooLong { units });
        }
        let mut data = Self {
            device,
            content_type,
            display_name: [0; DISPLAY_NAME_UNITS],
            file_name: [0; FILE_NAME_BYTES],
            padding: [0; 2],
        };
        for (slot, unit) in data
            .display_name
            .iter_mut()
            .zip(display_name.encode_utf16())
        {
            *slot = unit;
        }
        data.file_name[..file_name.len()].copy_from_slice(file_name);
        Ok(data)
    }

    pub const fn device(&self) -> ContentDevice {
        self.device
    }

    pub const fn content_type(&self) -> ContentType {
        self.content_type
    }
}

value_enum! {
    /// Win32 creation disposition, as `XamContentCreate` and `CreateFileA`
    /// take it: what to do when the target exists or does not.
    pub enum CreationDisposition: u32 {
        /// Create; fail if it exists.
        CreateNew = 1,
        /// Create; replace an existing one.
        CreateAlways = 2,
        /// Open; fail if it does not exist.
        OpenExisting = 3,
        /// Open, or create if it does not exist.
        OpenAlways = 4,
        /// Replace; fail if it does not exist.
        TruncateExisting = 5,
    }
}

/// `XamContentCreate` flags. The low four bits are the
/// [`CreationDisposition`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContentCreateFlags(u32);

impl ContentCreateFlags {
    pub const fn new(disposition: CreationDisposition) -> Self {
        Self(disposition as u32)
    }

    /// `bits` including bits without a name, for values passed on unchanged.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// What `XamContentCreate` did, as XAM reports it.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Disposition(u32);

/// Dispositions this crate names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispositionKind {
    Created,
    Opened,
}

impl Disposition {
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// `None` for a value this crate does not name.
    pub const fn kind(self) -> Option<DispositionKind> {
        match self.0 {
            1 => Some(DispositionKind::Created),
            2 => Some(DispositionKind::Opened),
            _ => None,
        }
    }
}

value_enum! {
    /// Icon of a message box.
    pub enum MessageBoxIcon: u32 {
        None = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
    }
}

/// `XamShowMessageBoxUI` flags. The low four bits are the
/// [`MessageBoxIcon`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageBoxFlags(u32);

impl MessageBoxFlags {
    pub const fn new(icon: MessageBoxIcon) -> Self {
        Self(icon as u32)
    }

    /// `bits` including bits without a name, for values passed on unchanged.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// `WaitForSingleObject` timeout that waits without limit.
pub const INFINITE: u32 = u32::MAX;

/// Result of `WaitForSingleObject`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaitResult(pub u32);

impl WaitResult {
    pub const OBJECT_0: Self = Self(0);
    pub const ABANDONED_0: Self = Self(0x80);
    pub const TIMEOUT: Self = Self(0x102);
    pub const FAILED: Self = Self(u32::MAX);
}

/// `CreateFileA` result of a failed call.
pub const INVALID_HANDLE_VALUE: Handle = Handle::from_raw(u32::MAX);

flags! {
    /// `DUPLICATE_*` options of `DuplicateHandle`.
    pub struct DuplicateOptions(u32) {
        CLOSE_SOURCE = 0x1,
        SAME_ACCESS = 0x2,
    }
}

/// `SYSTEMTIME`, as `GetLocalTime` stores it.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SystemTime {
    pub year: u16,
    pub month: u16,
    pub day_of_week: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub milliseconds: u16,
}

crate::import_thunk!(
    /// Initializes Winsock. `data` receives a WSADATA.
    pub NetDll_WSAStartup(caller: XnCaller, version: u32, data: *mut u8) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x001
);

crate::import_thunk!(
    /// Creates a socket. Returns the socket or [`Socket::INVALID`].
    pub NetDll_socket(
        caller: XnCaller,
        af: AddressFamily,
        ty: SocketType,
        protocol: Protocol,
    ) -> Socket,
    lib = XAM,
    ordinal = 0x003
);

crate::import_thunk!(
    /// Closes a socket.
    pub NetDll_closesocket(caller: XnCaller, socket: Socket) -> i32,
    lib = XAM,
    ordinal = 0x004
);

crate::import_thunk!(
    /// Disables sends, receives or both on a socket.
    pub NetDll_shutdown(caller: XnCaller, socket: Socket, how: ShutdownHow) -> i32,
    lib = XAM,
    ordinal = 0x005
);

crate::import_thunk!(
    /// Controls a socket mode.
    pub NetDll_ioctlsocket(
        caller: XnCaller,
        socket: Socket,
        command: IoctlCommand,
        argument: *mut u32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x006
);

crate::import_thunk!(
    /// Sets a socket option.
    pub NetDll_setsockopt(
        caller: XnCaller,
        socket: Socket,
        level: SocketLevel,
        name: SocketOption,
        value: *const u8,
        length: i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x007
);

crate::import_thunk!(
    /// Reads a socket option.
    pub NetDll_getsockopt(
        caller: XnCaller,
        socket: Socket,
        level: SocketLevel,
        name: SocketOption,
        value: *mut u8,
        length: *mut i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x008
);

crate::import_thunk!(
    /// Stores the local address of a socket.
    pub NetDll_getsockname(
        caller: XnCaller,
        socket: Socket,
        name: *mut u8,
        length: *mut i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x009
);

crate::import_thunk!(
    /// Stores the remote address of a connected socket.
    pub NetDll_getpeername(
        caller: XnCaller,
        socket: Socket,
        name: *mut u8,
        length: *mut i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x00A
);

crate::import_thunk!(
    /// Binds a socket to a local address.
    pub NetDll_bind(caller: XnCaller, socket: Socket, name: *const u8, length: i32) -> i32,
    lib = XAM,
    ordinal = 0x00B
);

crate::import_thunk!(
    /// Connects a socket to a remote address.
    pub NetDll_connect(caller: XnCaller, socket: Socket, name: *const u8, length: i32) -> i32,
    lib = XAM,
    ordinal = 0x00C
);

crate::import_thunk!(
    /// Places a socket in the listening state.
    pub NetDll_listen(caller: XnCaller, socket: Socket, backlog: i32) -> i32,
    lib = XAM,
    ordinal = 0x00D
);

crate::import_thunk!(
    /// Accepts a connection. Returns the new socket or [`Socket::INVALID`].
    pub NetDll_accept(caller: XnCaller, socket: Socket, name: *mut u8, length: *mut i32) -> Socket,
    lib = XAM,
    ordinal = 0x00E
);

crate::import_thunk!(
    /// Waits until sockets are readable, writable or in error. Returns the number of ready sockets.
    pub NetDll_select(
        caller: XnCaller,
        count: i32,
        read: *mut FdSet,
        write: *mut FdSet,
        except: *mut FdSet,
        timeout: *const TimeVal,
    ) -> i32,
    lib = XAM,
    ordinal = 0x00F
);

crate::import_thunk!(
    /// Receives from a connected socket. Returns the byte count or `SOCKET_ERROR`.
    pub NetDll_recv(
        caller: XnCaller,
        socket: Socket,
        buffer: *mut u8,
        length: i32,
        flags: MessageFlags,
    ) -> i32,
    lib = XAM,
    ordinal = 0x012
);

crate::import_thunk!(
    /// Receives a datagram and its source address.
    pub NetDll_recvfrom(
        caller: XnCaller,
        socket: Socket,
        buffer: *mut u8,
        length: i32,
        flags: MessageFlags,
        from: *mut u8,
        from_length: *mut i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x014
);

crate::import_thunk!(
    /// Sends on a connected socket. Returns the byte count or `SOCKET_ERROR`.
    pub NetDll_send(
        caller: XnCaller,
        socket: Socket,
        buffer: *const u8,
        length: i32,
        flags: MessageFlags,
    ) -> i32,
    lib = XAM,
    ordinal = 0x016
);

crate::import_thunk!(
    /// Sends a datagram to an address.
    pub NetDll_sendto(
        caller: XnCaller,
        socket: Socket,
        buffer: *const u8,
        length: i32,
        flags: MessageFlags,
        to: *const u8,
        to_length: i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x018
);

crate::import_thunk!(
    /// Returns the last Winsock error of the current thread.
    pub safe NetDll_WSAGetLastError(caller: XnCaller) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x01B
);

crate::import_thunk!(
    /// Associates network events of a socket with an event object. A null `event` and no events end the association.
    pub NetDll_WSAEventSelect(
        caller: XnCaller,
        socket: Socket,
        event: Handle,
        events: NetworkEvents,
    ) -> i32,
    lib = XAM,
    ordinal = 0x023
);

crate::import_thunk!(
    /// Initializes the network stack. `params` may be null.
    pub NetDll_XNetStartup(caller: XnCaller, params: *const u8) -> i32,
    lib = XAM,
    ordinal = 0x033
);

crate::import_thunk!(
    /// Fills a buffer with random bytes.
    pub NetDll_XNetRandom(caller: XnCaller, buffer: *mut u8, length: u32) -> i32,
    lib = XAM,
    ordinal = 0x035
);

crate::import_thunk!(
    /// Stores the title XNADDR.
    pub NetDll_XNetGetTitleXnAddr(caller: XnCaller, address: *mut u8) -> XnAddrStatus,
    lib = XAM,
    ordinal = 0x049
);

crate::import_thunk!(
    /// Stores the debug XNADDR.
    pub NetDll_XNetGetDebugXnAddr(caller: XnCaller, address: *mut u8) -> XnAddrStatus,
    lib = XAM,
    ordinal = 0x04A
);

crate::import_thunk!(
    /// Returns the link status of the network interface.
    pub safe NetDll_XNetGetEthernetLinkStatus(caller: XnCaller) -> EthernetLinkStatus,
    lib = XAM,
    ordinal = 0x04B
);

crate::import_thunk!(
    /// Reads a network stack option.
    pub NetDll_XNetGetOpt(
        caller: XnCaller,
        option: XnetOption,
        value: *mut u8,
        length: *mut i32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x04E
);

crate::import_thunk!(
    /// Writes a network stack option.
    pub NetDll_XNetSetOpt(
        caller: XnCaller,
        option: XnetOption,
        value: *const u8,
        length: u32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x04F
);

crate::import_thunk!(
    /// Applies a network configuration.
    pub NetDll_XnpConfig(caller: XnCaller, config: *const u8, flags: u32) -> i32,
    lib = XAM,
    ordinal = 0x068
);

crate::import_thunk!(
    /// Stores the network configuration status.
    pub NetDll_XnpGetConfigStatus(caller: XnCaller, status: *mut u8) -> i32,
    lib = XAM,
    ordinal = 0x069
);

crate::import_thunk!(
    /// Installs or removes Ethernet frame transmit and receive callbacks.
    pub NetDll_XnpEthernetInterceptSetCallbacks(
        caller: XnCaller,
        transmit: usize,
        receive: usize,
        context: usize,
        flags: u32,
    ) -> i32,
    lib = XAM,
    ordinal = 0x06D
);

crate::import_thunk!(
    /// Transmits a raw Ethernet frame.
    pub NetDll_XnpEthernetInterceptXmit(caller: XnCaller, frame: *mut u8, length: u32) -> u32,
    lib = XAM,
    ordinal = 0x06E
);

crate::import_thunk!(
    /// Delivers a raw Ethernet frame to the network stack.
    pub NetDll_XnpEthernetInterceptRecv(caller: XnCaller, frame: *mut u8, length: u32) -> u32,
    lib = XAM,
    ordinal = 0x06F
);

crate::import_thunk!(
    /// Stores the list of open sockets. `count` holds the capacity on entry and the count on return.
    pub NetDll_XnpGetActiveSocketList(caller: XnCaller, buffer: *mut u8, count: *mut u32) -> u32,
    lib = XAM,
    ordinal = 0x07A
);

crate::import_thunk!(
    /// Informs the network stack that the system time changed.
    pub NetDll_XnpNoteSystemTime(caller: XnCaller) -> u32,
    lib = XAM,
    ordinal = 0x07B
);

crate::import_thunk!(
    /// Initializes the HTTP client.
    pub NetDll_XHttpStartup(caller: XnCaller, reserved: u32, parameters: u32) -> u32,
    lib = XAM,
    ordinal = 0x0C9
);

crate::import_thunk!(
    /// Shuts down the HTTP client.
    pub NetDll_XHttpShutdown(caller: XnCaller) -> u32,
    lib = XAM,
    ordinal = 0x0CA
);

crate::import_thunk!(
    /// Opens an HTTP session. Returns the session handle or 0.
    pub NetDll_XHttpOpen(
        caller: XnCaller,
        agent: *const u8,
        access: u32,
        proxy: u32,
        bypass: u32,
        flags: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0CB
);

crate::import_thunk!(
    /// Closes an HTTP handle.
    pub NetDll_XHttpCloseHandle(caller: XnCaller, handle: u32) -> u32,
    lib = XAM,
    ordinal = 0x0CC
);

crate::import_thunk!(
    /// Opens an HTTP connection to a server. Returns the connection handle or 0.
    pub NetDll_XHttpConnect(
        caller: XnCaller,
        session: u32,
        server: *const u8,
        port: u32,
        flags: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0CD
);

crate::import_thunk!(
    /// Installs a status callback on an HTTP handle.
    pub NetDll_XHttpSetStatusCallback(
        caller: XnCaller,
        handle: u32,
        callback: u32,
        flags: u32,
        reserved: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0CE
);

crate::import_thunk!(
    /// Opens an HTTP request. Returns the request handle or 0.
    pub NetDll_XHttpOpenRequest(
        caller: XnCaller,
        connection: u32,
        verb: *const u8,
        path: *const u8,
        version: u32,
        referrer: u32,
        accept: u32,
        flags: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0CF
);

crate::import_thunk!(
    /// Sends an HTTP request.
    pub NetDll_XHttpSendRequest(
        caller: XnCaller,
        request: u32,
        headers: u32,
        headers_length: u32,
        optional: u32,
        optional_length: u32,
        total_length: u32,
        context: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0D1
);

crate::import_thunk!(
    /// Waits for the response to an HTTP request.
    pub NetDll_XHttpReceiveResponse(caller: XnCaller, request: u32, reserved: u32) -> u32,
    lib = XAM,
    ordinal = 0x0D2
);

crate::import_thunk!(
    /// Reads response header information.
    pub NetDll_XHttpQueryHeaders(
        caller: XnCaller,
        request: u32,
        level: u32,
        name: u32,
        buffer: *mut u32,
        length: *mut u32,
        index: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0D3
);

crate::import_thunk!(
    /// Reads response body data. The byte count is stored at `read`.
    pub NetDll_XHttpReadData(
        caller: XnCaller,
        request: u32,
        buffer: *mut u8,
        length: u32,
        read: *mut u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0D4
);

crate::import_thunk!(
    /// Sets an option on an HTTP handle.
    pub NetDll_XHttpSetOption(
        caller: XnCaller,
        handle: u32,
        option: u32,
        value: *mut u32,
        length: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x0D7
);

crate::import_thunk!(
    /// Processes pending HTTP work for a session.
    pub NetDll_XHttpDoWork(caller: XnCaller, session: u32, timeout: u32) -> u32,
    lib = XAM,
    ordinal = 0x0D8
);

crate::import_thunk!(
    /// Reads the capabilities of the controller of a user slot.
    /// Returns a Win32 error code: `ERROR_DEVICE_NOT_CONNECTED` without a controller.
    pub XamInputGetCapabilities(
        user_index: UserIndex,
        flags: InputFlags,
        capabilities: *mut Capabilities,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x190
);

crate::import_thunk!(
    /// Reads the state of the controller of a user slot.
    /// Returns a Win32 error code: `ERROR_DEVICE_NOT_CONNECTED` without a controller.
    pub XamInputGetState(
        user_index: UserIndex,
        flags: InputFlags,
        state: *mut InputState,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x191
);

crate::import_thunk!(
    /// Sets the motor speeds of the controller of a user slot. Xenia ignores `flags`.
    /// Returns a Win32 error code.
    pub XamInputSetState(
        user_index: UserIndex,
        flags: u32,
        vibration: *const Vibration,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x192
);

crate::import_thunk!(
    /// Applies the configured inactivity timeout.
    pub XamSetInactivityTimeFromConfig(apply: u32) -> u32,
    lib = XAM,
    ordinal = 0x1A2
);

crate::import_thunk!(
    /// Ends the running title and launches the executable at `path` (null or a NUL-terminated path) with `flags`.
    /// A null `path` returns to the dashboard.
    /// Call it only from a thread of the title process: XAM then ends the title and the calling thread with it.
    pub XamLoaderLaunchTitle(path: *const u8, flags: u32) -> !,
    lib = XAM,
    ordinal = 0x1A4
);

crate::import_thunk!(
    /// Ends the running title.
    /// Call it only from a thread of the title process: XAM then ends the title and the calling thread with it.
    pub XamLoaderTerminateTitle() -> !,
    lib = XAM,
    ordinal = 0x1A9
);

crate::import_thunk!(
    /// Returns the optical drive tray state.
    pub safe XamLoaderGetDvdTrayState() -> u32,
    lib = XAM,
    ordinal = 0x1AA
);

crate::import_thunk!(
    /// Starts an I/O request to a XAM application. Completes through `overlapped` when not null.
    pub XMsgStartIORequest(
        app: u32,
        message: u32,
        overlapped: *mut Overlapped,
        buffer: *mut u8,
        buffer_length: u32,
    ) -> u32,
    lib = XAM,
    ordinal = 0x1F7
);

crate::import_thunk!(
    /// Returns the sign-in state of a user slot.
    pub safe XamUserGetSigninState(user_index: UserIndex) -> SigninState,
    lib = XAM,
    ordinal = 0x210
);

crate::import_thunk!(
    /// Creates or opens a content package and maps it to the drive `root_name`.
    /// The disposition taken is stored at `disposition_out` when not null.
    /// Returns a Win32 error code, or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamContentCreate(
        user_index: UserIndex,
        root_name: *const u8,
        content_data: *const ContentData,
        flags: ContentCreateFlags,
        disposition_out: *mut Disposition,
        license_mask_out: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x258
);

crate::import_thunk!(
    /// Closes the content drive `root_name`. Returns a Win32 error code, or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamContentClose(root_name: *const u8, overlapped: *mut Overlapped) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x25A
);

crate::import_thunk!(
    /// Deletes a content package.
    /// Returns a Win32 error code, or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamContentDelete(
        user_index: UserIndex,
        content_data: *const ContentData,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x25B
);

crate::import_thunk!(
    /// Returns the system software version.
    pub safe XamGetSystemVersion() -> u32,
    lib = XAM,
    ordinal = 0x282
);

import_adapter!(
    /// Creates a notification listener for the notification areas in `mask`. Returns the listener handle.
    /// The export takes the 64-bit `mask` in one register.
    pub XamNotifyCreateListenerInternal(mask: u64, is_system: u32, max_version: u32) -> Handle,
    lib = XAM,
    ordinal = 0x291,
    // mask in r3:r4, is_system in r5, max_version in r6.
    prologue = [
        ".long 0x7864000e", // rldimi 4, 3, 32, 0
        "mr 3, 4",
        "mr 4, 5",
        "mr 5, 6",
    ]
);

crate::import_thunk_stack!(
    /// Shows a message box with up to three buttons. `title`, `text` and each entry of `buttons` are NUL-terminated UTF-16 strings.
    /// Completes through `overlapped` and stores the result at `result_out`: an 8-byte `MESSAGEBOX_RESULT` whose first word is the chosen button index.
    /// Returns `ERROR_IO_PENDING` when the box is shown, or another Win32 error code.
    pub XamShowMessageBoxUI(
        user_index: UserIndex,
        title: *const u16,
        text: *const u16,
        button_count: u32,
        buttons: *const u32,
        focused_button: u32,
        flags: MessageBoxFlags,
        result_out: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2CA
);

crate::import_thunk!(
    /// Binds an automation controller to a user slot.
    pub XAutomationpBindController(user: UserIndex) -> u32,
    lib = XAM,
    ordinal = 0x3D5
);

crate::import_thunk!(
    /// Removes the automation controller from a user slot.
    pub XAutomationpUnbindController(user: UserIndex) -> u32,
    lib = XAM,
    ordinal = 0x3D6
);

crate::import_thunk!(
    /// Sends a Guide button press from the automation controller of a user slot.
    pub XAutomationpInputXenonButton(user: UserIndex) -> u32,
    lib = XAM,
    ordinal = 0x3D7
);

crate::import_thunk!(
    /// Allocates from a heap. Returns the address or 0.
    pub RtlAllocateHeap(heap: u32, flags: u32, size: u32) -> u32,
    lib = XAM,
    ordinal = 0x3FC
);

crate::import_thunk!(
    /// Frees a heap block.
    pub RtlFreeHeap(heap: u32, flags: u32, block: u32) -> u32,
    lib = XAM,
    ordinal = 0x3FF
);

crate::import_thunk!(
    /// Returns the ID of the current thread.
    pub safe GetCurrentThreadId() -> u32,
    lib = XAM,
    ordinal = 0x410
);

crate::import_thunk!(
    /// Waits on a handle for `milliseconds`, or without limit for [`INFINITE`].
    pub WaitForSingleObject(handle: Handle, milliseconds: u32) -> WaitResult,
    lib = XAM,
    ordinal = 0x425
);

crate::import_thunk!(
    /// Sets an event to not signaled.
    pub ResetEvent(event: Handle) -> Boolean,
    lib = XAM,
    ordinal = 0x42B
);

crate::import_thunk!(
    /// Stores the performance counter.
    pub QueryPerformanceCounter(counter: *mut u64) -> Boolean,
    lib = XAM,
    ordinal = 0x42F
);

crate::import_thunk!(
    /// Stores the performance counter frequency.
    pub QueryPerformanceFrequency(frequency: *mut u64) -> Boolean,
    lib = XAM,
    ordinal = 0x430
);

crate::import_thunk!(
    /// Decrements the suspend count of a thread. Returns the previous count.
    pub ResumeThread(thread: Handle) -> u32,
    lib = XAM,
    ordinal = 0x43D
);

crate::import_thunk!(
    /// Stores the local time.
    pub GetLocalTime(time: *mut SystemTime) -> (),
    lib = XAM,
    ordinal = 0x444
);

crate::import_thunk!(
    /// Creates an event. Returns the handle or [`Handle::NULL`].
    pub CreateEventA(
        attributes: u32,
        manual_reset: bool,
        initial_state: bool,
        name: *const u8,
    ) -> Handle,
    lib = XAM,
    ordinal = 0x446
);

crate::import_thunk!(
    /// Creates or opens a file. `name` is the address of a NUL-terminated path. Returns the handle or [`INVALID_HANDLE_VALUE`].
    pub CreateFileA(
        name: u32,
        access: AccessMask,
        share: ShareAccess,
        security: u32,
        disposition: CreationDisposition,
        flags: FileAttributes,
        template: Handle,
    ) -> Handle,
    lib = XAM,
    ordinal = 0x447
);

crate::import_thunk!(
    /// Duplicates a handle.
    pub DuplicateHandle(
        source_process: Handle,
        source: Handle,
        target_process: Handle,
        target: *mut Handle,
        access: AccessMask,
        inherit: bool,
        options: DuplicateOptions,
    ) -> Boolean,
    lib = XAM,
    ordinal = 0x45B
);

crate::import_thunk!(
    /// Returns the default heap of the process.
    pub safe GetProcessHeap() -> u32,
    lib = XAM,
    ordinal = 0x7D2
);
