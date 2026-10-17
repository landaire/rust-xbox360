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

/// A Winsock version (`MAKEWORD(major, minor)`) of `NetDll_WSAStartup`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinsockVersion(u16);

impl WinsockVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self(((minor as u16) << 8) | major as u16)
    }
}

/// `WSADATA`, as `NetDll_WSAStartup` stores it.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WsaData {
    pub version: WinsockVersion,
    pub high_version: WinsockVersion,
    pub description: [u8; 257],
    pub system_status: [u8; 129],
    pub max_sockets: u16,
    pub max_udp_datagram: u16,
    pub vendor_info: u32,
}

impl WsaData {
    /// Zeroed storage for `NetDll_WSAStartup` to fill.
    pub const fn new() -> Self {
        Self {
            version: WinsockVersion(0),
            high_version: WinsockVersion(0),
            description: [0; 257],
            system_status: [0; 129],
            max_sockets: 0,
            max_udp_datagram: 0,
            vendor_info: 0,
        }
    }
}

impl Default for WsaData {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::offset_of!(WsaData, max_sockets) == 0x186);
    assert!(core::mem::size_of::<WsaData>() == 0x190);
};

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
