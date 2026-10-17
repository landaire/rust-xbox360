//! XNet and Winsock types.

mod winsock;
mod xnet;

pub use self::winsock::AddressFamily;
pub use self::winsock::FD_SETSIZE;
pub use self::winsock::FdSet;
pub use self::winsock::IoctlCommand;
pub use self::winsock::MessageFlags;
pub use self::winsock::NetworkEvents;
pub use self::winsock::Protocol;
pub use self::winsock::SOCKET_ERROR;
pub use self::winsock::ShutdownHow;
pub use self::winsock::SockAddrIn;
pub use self::winsock::Socket;
pub use self::winsock::SocketLevel;
pub use self::winsock::SocketOption;
pub use self::winsock::SocketType;
pub use self::winsock::TimeVal;
pub use self::winsock::WinsockVersion;
pub use self::winsock::WsaData;
pub use self::xnet::EthernetInterceptFlags;
pub use self::xnet::EthernetLinkStatus;
pub use self::xnet::XnAddr;
pub use self::xnet::XnAddrStatus;
pub use self::xnet::XnetConfigFlags;
pub use self::xnet::XnetConfigParams;
pub use self::xnet::XnetOption;
pub use self::xnet::XnetStartupFlags;
pub use self::xnet::XnetStartupParams;

value_enum! {
    /// `XNCALLER_*`: on whose behalf a network call is made.
    #[non_exhaustive]
    pub enum XnCaller: u32 {
        Title = 1,
        SystemApp = 2,
        DebugMonitor = 3,
    }
}
