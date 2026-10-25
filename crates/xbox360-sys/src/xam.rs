//! xam.xex imports, structures, and the types and constants of their
//! arguments.
//!
//! The controller, content and message box structures and values come from
//! Xenia's implementations of the exports.

mod content;
mod input;
mod io;
mod media;
mod net;
mod notify;
pub mod ordinal;
mod play_timer;
mod profile;
mod ui;
mod user;
mod video;
mod win32;

#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::lib_index::XAM;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::status::Hresult;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::AccessMask;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::FileAttributes;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::Handle;
#[cfg(any(target_os = "xbox360", xbox360_docs))]
use crate::xboxkrnl::ShareAccess;

pub use self::content::ContentCreateFlags;
pub use self::content::ContentData;
pub use self::content::ContentDataError;
pub use self::content::ContentDevice;
pub use self::content::ContentType;
pub use self::content::DISPLAY_NAME_UNITS;
pub use self::content::Disposition;
pub use self::content::DispositionKind;
pub use self::content::FILE_NAME_BYTES;
pub use self::input::Buttons;
pub use self::input::Capabilities;
pub use self::input::CapabilityFlags;
pub use self::input::DeviceKind;
pub use self::input::DeviceSubkind;
pub use self::input::DeviceSubtype;
pub use self::input::DeviceType;
pub use self::input::Gamepad;
pub use self::input::InputFlags;
pub use self::input::InputState;
pub use self::input::Keystroke;
pub use self::input::KeystrokeFlags;
pub use self::input::KeystrokeQuery;
pub use self::input::PadKey;
pub use self::input::PacketNumber;
pub use self::input::Vibration;
pub use self::io::Overlapped;
pub use self::media::MediaVerificationFailedBlocks;
pub use self::net::AddressFamily;
pub use self::net::EthernetInterceptFlags;
pub use self::net::EthernetLinkStatus;
pub use self::net::FD_SETSIZE;
pub use self::net::FdSet;
pub use self::net::IoctlCommand;
pub use self::net::MessageFlags;
pub use self::net::NetworkEvents;
pub use self::net::Protocol;
pub use self::net::SOCKET_ERROR;
pub use self::net::ShutdownHow;
pub use self::net::SockAddrIn;
pub use self::net::Socket;
pub use self::net::SocketLevel;
pub use self::net::SocketOption;
pub use self::net::SocketType;
pub use self::net::TimeVal;
pub use self::net::WinsockVersion;
pub use self::net::WsaData;
pub use self::net::XnAddr;
pub use self::net::XnAddrStatus;
pub use self::net::XnCaller;
pub use self::net::XnetConfigFlags;
pub use self::net::XnetConfigParams;
pub use self::net::XnetOption;
pub use self::net::XnetStartupFlags;
pub use self::net::XnetStartupParams;
pub use self::notify::NotificationAreas;
pub use self::play_timer::PlayTimerData;
pub use self::profile::ProfileEnumResult;
pub use self::profile::UserData;
pub use self::ui::DeviceSelectorFlags;
pub use self::ui::KeyboardFlags;
pub use self::ui::MessageBoxFlags;
pub use self::ui::MessageBoxIcon;
pub use self::ui::MessageBoxResult;
pub use self::ui::SigninFlags;
pub use self::user::LogonFlags;
pub use self::user::SigninKind;
pub use self::user::SigninState;
pub use self::user::UserIndex;
pub use self::video::VideoMode;
pub use self::win32::CreationDisposition;
pub use self::win32::DuplicateOptions;
pub use self::win32::HeapFlags;
pub use self::win32::INFINITE;
pub use self::win32::INVALID_HANDLE_VALUE;
pub use self::win32::SystemTime;
pub use self::win32::WaitResult;
pub use crate::status::Win32ErrorCode;
pub use crate::xboxkrnl::Boolean;

crate::import_thunk!(
    /// Initializes Winsock.
    pub NetDll_WSAStartup(caller: XnCaller, version: WinsockVersion, data: *mut WsaData) -> Win32ErrorCode,
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
    pub NetDll_XNetStartup(caller: XnCaller, params: *const XnetStartupParams) -> i32,
    lib = XAM,
    ordinal = 0x033
);

crate::import_thunk!(
    /// [`NetDll_XNetStartup`] with a third argument. The 2.0.6717 dashboard
    /// calls it in place of `NetDll_XNetStartup` when `XamGetSystemVersion`
    /// is 0x200A3200 or more, with `version` 0x2017F400, and treats the
    /// result as that of `NetDll_XNetStartup`.
    pub NetDll_XNetStartupVersion(caller: XnCaller, params: *const XnetStartupParams, version: u32) -> i32,
    lib = XAM,
    ordinal = 0x050
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
    /// Reads the saved network configuration, a 0x1EC-byte record, into `params`.
    pub NetDll_XnpLoadConfigParams(caller: XnCaller, params: *mut u8) -> Hresult,
    lib = XAM,
    ordinal = 0x065
);

crate::import_thunk!(
    /// Saves the 0x1EC-byte network configuration record `params`. Returns `E_PENDING` while
    /// the save has not finished; the 2.0.6717 dashboard calls it again until it does.
    pub NetDll_XnpSaveConfigParams(caller: XnCaller, params: *const u8) -> Hresult,
    lib = XAM,
    ordinal = 0x066
);

crate::import_thunk!(
    /// Applies a network configuration.
    pub NetDll_XnpConfig(caller: XnCaller, config: *const u8, flags: u32) -> i32,
    lib = XAM,
    ordinal = 0x068
);

/// [`NetDll_XnpConfig`] with a typed record. The meaning of `flags` is not
/// known; a debug monitor passes 0.
///
/// # Safety
///
/// The caller context and `flags` must meet the requirements of the export.
#[cfg(any(target_os = "xbox360", xbox360_docs))]
#[inline(always)]
pub unsafe fn xnp_config(caller: XnCaller, config: &XnetConfigParams, flags: u32) -> i32 {
    // SAFETY: `config` is a whole record the export reads during the call;
    // the rest per the contract.
    unsafe { NetDll_XnpConfig(caller, (config as *const XnetConfigParams).cast(), flags) }
}

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
    /// Starts the UPnP client.
    pub NetDll_UpnpStartup(caller: XnCaller) -> i32,
    lib = XAM,
    ordinal = 0x0FB
);

crate::import_thunk!(
    /// Stops the UPnP client.
    pub NetDll_UpnpCleanup(caller: XnCaller) -> i32,
    lib = XAM,
    ordinal = 0x0FC
);

crate::import_thunk!(
    /// Stores the Xbox LIVE logon status and extended status.
    pub XNetLogonGetExtendedStatus(status: *mut u32, extended: *mut u32) -> u32,
    lib = XAM,
    ordinal = 0x13B
);

crate::import_thunk!(
    /// Discards the cached Xbox LIVE tickets. The 2.0.6717 dashboard passes 0; the meaning of
    /// the argument is unknown.
    pub XNetLogonClearTicketCaches(unknown: u32) -> u32,
    lib = XAM,
    ordinal = 0x13C
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
    /// Takes the next queued key transition. `user_index` selects the user
    /// slot, or with [`KeystrokeQuery::ANY_USER`] receives the slot of the
    /// keystroke. Returns a Win32 error code; nonzero when no keystroke is queued.
    pub XamInputGetKeystrokeEx(
        user_index: *mut u32,
        flags: KeystrokeQuery,
        keystroke: *mut Keystroke,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x198
);

crate::import_thunk!(
    /// Applies the configured inactivity timeout.
    pub XamSetInactivityTimeFromConfig(apply: u32) -> u32,
    lib = XAM,
    ordinal = 0x1A2
);

crate::import_thunk!(
    /// Writes the media type of the disc in the console drive to `media` and its title id to
    /// `title`. The 2.0.6717 dashboard reads media 1 as an Xbox 360 game and -1 as unknown.
    pub XamLoaderGetMediaInfo(media: *mut u32, title: *mut u32) -> (),
    lib = XAM,
    ordinal = 0x1A3
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
    /// Launches the executable at `path` with `flags`, as [`XamLoaderLaunchTitle`] does. The
    /// purpose of `second` and `third` (NUL-terminated strings or null) is not known; the 2.0.6717
    /// dashboard passes null for all three strings to launch the disc.
    pub XamLoaderLaunchTitleEx(path: *const u8, second: *const u8, third: *const u8, flags: u32) -> (),
    lib = XAM,
    ordinal = 0x1A5
);

crate::import_thunk!(
    /// Copies the launch data the previous title set to `buffer`, at most `size` bytes.
    /// Returns a Win32 error code; nonzero when no launch data is set.
    pub XamLoaderGetLaunchData(buffer: *mut u8, size: u32) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x1A8
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
    /// Returns 1 when `title` is a system title id.
    pub safe XamIsSystemTitleId(title: u32) -> Boolean,
    lib = XAM,
    ordinal = 0x1BB
);

crate::import_thunk!(
    /// Returns the title id of the title that ran before the current one, or 0.
    pub safe XamLoaderGetPriorTitleId() -> u32,
    lib = XAM,
    ordinal = 0x1BE
);

crate::import_thunk!(
    /// Writes the NUL-terminated path of the Windows Connect Now configuration file found on a
    /// storage device to `path`, at most `capacity` bytes. Returns `ERROR_FILE_NOT_FOUND` when
    /// no device holds one, or another Win32 error code.
    pub XamGetWCNConfigFile(path: *mut u8, capacity: u32) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x1C6
);

crate::import_thunk!(
    /// Mutes (nonzero) or unmutes the sounds of the system UI.
    pub XamMuteSound(mute: u32) -> (),
    lib = XAM,
    ordinal = 0x1CA
);

crate::import_thunk!(
    /// Sets the dashboard context XAM reports to the system UI. The 2.0.6717 dashboard passes 3
    /// when initial setup starts and 0 when it completes.
    pub XamSetDashContext(context: u32) -> (),
    lib = XAM,
    ordinal = 0x1CC
);

crate::import_thunk!(
    /// Calls a XAM application in the calling process. The meaning of the parameters depends
    /// on `message`.
    pub XMsgInProcessCall(app: u32, message: u32, parameter1: *mut u32, parameter2: *mut u32) -> Hresult,
    lib = XAM,
    ordinal = 0x1F4
);

crate::import_thunk!(
    /// Calls a XAM application synchronously. The dashboard reports a language change with app 0xFE,
    /// message 0x8000000D and no buffer.
    pub XMsgSystemProcessCall(app: u32, message: u32, buffer: *mut u8, buffer_length: u32) -> Hresult,
    lib = XAM,
    ordinal = 0x1F6
);

crate::import_thunk!(
    /// Starts an I/O request to a XAM application. Completes through `overlapped` when not null.
    ///
    /// With `overlapped`, the result is the Win32 code `ERROR_IO_PENDING`
    /// (997, [`crate::status::Win32Error::IO_PENDING`]) and not an HRESULT,
    /// so it reads as a success with facility 0. The outcome of the request
    /// is in `overlapped`.
    pub XMsgStartIORequest(
        app: u32,
        message: u32,
        overlapped: *mut Overlapped,
        buffer: *mut u8,
        buffer_length: u32,
    ) -> Hresult,
    lib = XAM,
    ordinal = 0x1F7
);

crate::import_thunk!(
    /// Cancels the request of `overlapped`. With `wait`, returns after the request completed.
    /// The 2.0.6717 dashboard always waits and does not use the result.
    pub XMsgCancelIORequest(overlapped: *mut Overlapped, wait: bool) -> u32,
    lib = XAM,
    ordinal = 0x1F8
);

crate::import_thunk!(
    /// Starts an I/O request to a XAM application with an extra parameter block. The 2.0.6717
    /// dashboard queries a video license with app 0xFA, message 0x7005D, a 16-byte request and
    /// `extended` pointing to the words 0x20080002 and 0. Without `overlapped` the call completes
    /// before it returns; the dashboard then also takes a nonzero thread error as failure.
    pub XMsgStartIORequestEx(
        app: u32,
        message: u32,
        overlapped: *mut Overlapped,
        buffer: *mut u8,
        buffer_length: u32,
        extended: *mut u32,
    ) -> Hresult,
    lib = XAM,
    ordinal = 0x1FC
);

crate::import_thunk!(
    /// Returns the Win32 result of the request of `overlapped`, or `ERROR_IO_INCOMPLETE` while
    /// it runs and `wait` is false. Stores the result length at `length` when not null.
    pub XamGetOverlappedResult(overlapped: *mut Overlapped, length: *mut u32, wait: bool) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x1FB
);

crate::import_thunk!(
    /// Stores the XUID of a user slot at `xuid`. `flags` selects the XUID kinds accepted
    /// (the 2.0.6717 dashboard passes 1, 3, 4 and 7). Fails when the slot has no such XUID.
    pub XamUserGetXUID(user_index: UserIndex, flags: u32, xuid: *mut u64) -> Hresult,
    lib = XAM,
    ordinal = 0x20A
);

crate::import_thunk!(
    /// Signs in the users of the four XUIDs at `xuids`, one per user slot; zero XUIDs sign
    /// those slots out. XAM copies the XUIDs during the call. Returns `ERROR_IO_PENDING` (a
    /// success value as an HRESULT) with an `overlapped`.
    pub XamUserLogon(xuids: *const u64, flags: LogonFlags, overlapped: *mut Overlapped) -> Hresult,
    lib = XAM,
    ordinal = 0x20B
);

crate::import_thunk!(
    /// Writes the gamertag of a signed-in user slot to `buffer` as NUL-terminated UTF-16,
    /// at most `length` units. The 2.0.6717 dashboard passes 16 units.
    pub XamUserGetGamerTag(user_index: UserIndex, buffer: *mut u16, length: u32) -> Hresult,
    lib = XAM,
    ordinal = 0x20C
);

crate::import_thunk!(
    /// Returns the sign-in state of a user slot.
    pub safe XamUserGetSigninState(user_index: UserIndex) -> SigninState,
    lib = XAM,
    ordinal = 0x210
);

crate::import_thunk!(
    /// Processes the queued Xbox LIVE logons. Returns `ERROR_IO_PENDING` (a success value as an
    /// HRESULT) with an `overlapped`.
    pub XamUserFlushLogonQueue(overlapped: *mut Overlapped) -> Hresult,
    lib = XAM,
    ordinal = 0x220
);

crate::import_thunk!(
    /// Returns nonzero when the profile of a user slot has an Xbox LIVE account.
    pub safe XamUserIsOnlineEnabled(user_index: UserIndex) -> Boolean,
    lib = XAM,
    ordinal = 0x221
);

crate::import_thunk!(
    /// Opens an enumeration of the profiles on `device` (0 for all devices) and stores its handle.
    /// Close the handle with `NtClose`. Returns a Win32 error code.
    pub XamProfileCreateEnumerator(device: u32, handle: *mut Handle) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x231
);

crate::import_thunk!(
    /// Starts the creation of an offline profile on the device `device_id` points to and stores
    /// the creation handle. `account` is a zeroed 0x17C-byte `XAMACCOUNTINFO` with the
    /// big-endian UTF-16 gamertag (16 units) at +8; `block` is a zeroed 0x8F0-byte block. The
    /// 2.0.6717 dashboard passes 1 for `flags` and 0 for `reserved0`, `reserved1` and
    /// `reserved2`. XAM uses the buffers until the creation completes.
    pub XamProfileCreate(
        flags: u32,
        device_id: *const u32,
        reserved0: u32,
        account: *mut [u8; 0x17C],
        block: *mut [u8; 0x8F0],
        reserved1: u32,
        reserved2: u32,
        handle: *mut Handle,
    ) -> Hresult,
    lib = XAM,
    ordinal = 0x230
);

crate::import_thunk!(
    /// Stores the next profile of an enumeration at `result`. Returns a Win32 error code;
    /// nonzero once no profile remains.
    pub XamProfileEnumerate(
        handle: Handle,
        flags: u32,
        result: *mut ProfileEnumResult,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x232
);

crate::import_thunk!(
    /// Stores the XUID of the profile a `XamProfileCreate` handle creates. Returns
    /// `E_PENDING` while the creation runs.
    pub XamProfileGetCreationStatus(handle: Handle, xuid: *mut u64) -> Hresult,
    lib = XAM,
    ordinal = 0x234
);

import_adapter!(
    /// Mounts the dashboard settings of the profile `xuid` as the drive `mount_name`
    /// (NUL-terminated ASCII, such as `DASHUSER`). The 2.0.6717 dashboard passes 0 and null
    /// for `flags` and `content_data`. Returns a Win32 error code.
    /// The export takes the 64-bit `xuid` in one register.
    pub XamProfileOpen(xuid: u64, mount_name: *const u8, flags: u32, content_data: *mut u8) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x237,
    // xuid in r3:r4, mount_name in r5, flags in r6, content_data in r7.
    prologue = [
        "rldimi 4, 3, 32, 0",
        "mr 3, 4",
        "mr 4, 5",
        "mr 5, 6",
        "mr 6, 7",
    ]
);

crate::import_thunk!(
    /// Unmounts a drive that [`XamProfileOpen`] mounted. Returns a Win32 error code.
    pub XamProfileClose(mount_name: *const u8) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x238
);

crate::import_thunk!(
    /// Unloads the XAM applications stacked above the dashboard. The 2.0.6717 dashboard passes
    /// 0 for `flags`; the result is not used.
    pub XamAppUnloadStack(flags: u32) -> u32,
    lib = XAM,
    ordinal = 0x246
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
    /// Creates an enumerator of the storage devices accepting
    /// `content_types`, returning `items_per_call` 0x50-byte device records
    /// per [`XamEnumerate`]. Returns a Win32 error code.
    pub XamContentCreateDeviceEnumerator(
        content_types: u32,
        flags: u32,
        items_per_call: u32,
        buffer_size_out: *mut u32,
        enumerator_out: *mut Handle,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x25D
);

crate::import_thunk!(
    /// Fills the 0x50-byte device record of `device_id`. Returns a Win32
    /// error code.
    pub XamContentGetDeviceData(device_id: u32, device_data: *mut u8) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x25E
);

crate::import_thunk!(
    /// Commits the writes to the content drive `root_name`. The 2.0.6717 dashboard flushes
    /// `DASHUSER` after it writes `DashPreferences`. Returns a Win32 error code.
    pub XamContentFlush(root_name: *const u8, overlapped: *mut Overlapped) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x267
);

crate::import_thunk!(
    /// Reads the header of the package of the 0x200-byte aggregate record `content` into
    /// `buffer`, 0x93D6 bytes in the 2.0.6717 dashboard. Returns a Win32 error code.
    pub XamContentGetMetaDataInternal(content: *const u8, buffer: *mut u8, overlapped: *mut Overlapped) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x26E
);

crate::import_thunk!(
    /// Deletes the package of the 0x200-byte aggregate record `content`. Returns a Win32 error
    /// code, or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamContentDeleteInternal(content: *const u8, overlapped: *mut Overlapped) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x270
);

import_adapter_stack!(
    /// Opens the package of the 0x200-byte aggregate record `content` and maps it to the drive
    /// `root_name`, as [`XamContentCreate`]. The 2.0.6717 dashboard opens existing packages
    /// (`flags` 3) and passes 0 for `reserved` and `size`. Returns a Win32 error code.
    /// The export takes `size` in one register.
    pub XamContentCreateInternal(
        root_name: *const u8,
        content: *const u8,
        flags: u32,
        disposition_out: *mut u32,
        license_mask_out: *mut u32,
        reserved: u32,
        size: u64,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x271,
    // root_name..reserved in r3-r8; size in r9:r10; overlapped at 0x78(1).
    prologue = [
        "rldimi 10, 9, 32, 0",
        "mr 9, 10",
        "lwz 10, 0x78(1)",
    ]
);

crate::import_thunk!(
    /// Launches the NUL-terminated executable path `image` inside the package of the 0x200-byte
    /// aggregate record `content`. Returns 0 on success; the system then ends the caller.
    pub XamContentLaunchImageInternal(content: *const u8, image: *const u8) -> u32,
    lib = XAM,
    ordinal = 0x273
);

crate::import_thunk!(
    /// Writes the path of the package of the 0x200-byte aggregate record `content` to `path` as a
    /// NUL-terminated string of at most `capacity` bytes. The 2.0.6717 dashboard passes 0x104
    /// and 0 for the reserved arguments. Returns a Win32 error code.
    pub XamContentResolveInternal(
        content: *const u8,
        path: *mut u8,
        capacity: u32,
        reserved1: u32,
        reserved2: u32,
        reserved3: u32,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x276
);

crate::import_thunk!(
    /// Writes the 0x24-byte attribute block of the package of the 0x200-byte aggregate record
    /// `content` to `attributes`; +0x4 holds the creation FILETIME. Returns a Win32 error code,
    /// or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamContentGetAttributesInternal(
        content: *const u8,
        attributes: *mut u8,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x277
);

crate::import_thunk!(
    /// Returns the system software version.
    pub safe XamGetSystemVersion() -> u32,
    lib = XAM,
    ordinal = 0x282
);

crate::import_thunk!(
    /// Takes the next queued notification of `listener` and stores its id and parameter.
    /// A nonzero `match_id` takes only notifications with that id.
    /// Returns nonzero when a notification was taken.
    pub XNotifyGetNext(listener: Handle, match_id: u32, id: *mut u32, parameter: *mut u32) -> Boolean,
    lib = XAM,
    ordinal = 0x28B
);

import_adapter!(
    /// Creates a notification listener for the notification areas in `mask`. Returns the listener handle.
    /// The export takes the 64-bit `mask` in one register.
    pub XamNotifyCreateListenerInternal(mask: u64, is_system: u32, max_version: u32) -> Handle,
    lib = XAM,
    ordinal = 0x291,
    // mask in r3:r4, is_system in r5, max_version in r6.
    prologue = [
        "rldimi 4, 3, 32, 0",
        "mr 3, 4",
        "mr 4, 5",
        "mr 5, 6",
    ]
);

crate::import_thunk!(
    /// Opens the sign-in UI with `panes` user panes. Returns a Win32 error code; the UI does
    /// not report when it closes.
    pub XamShowSigninUI(panes: u32, flags: SigninFlags) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2BC
);

crate::import_thunk!(
    /// [`XamShowSigninUI`] for the controller `user_index`. Returns a Win32 error code; the UI
    /// does not report when it closes.
    pub XamShowSigninUIp(user_index: UserIndex, panes: u32, flags: SigninFlags) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2BE
);

crate::import_thunk!(
    /// Shows the virtual keyboard. `default_text`, `title` and `description` are NUL-terminated
    /// UTF-16 strings. Completes through `overlapped` and writes the NUL-terminated text to
    /// `buffer`, at most `length` units. Returns `ERROR_IO_PENDING` when the keyboard is shown,
    /// or another Win32 error code.
    pub XamShowKeyboardUI(
        user_index: UserIndex,
        flags: KeyboardFlags,
        default_text: *const u16,
        title: *const u16,
        description: *const u16,
        buffer: *mut u16,
        length: u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2C1
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

import_adapter!(
    /// Shows the storage device selector for content of type `content_type` (0 for any) that
    /// needs `bytes_requested` free bytes (0 for any). `device` holds the device selected
    /// first and receives the chosen device id. Completes through `overlapped`. Returns
    /// `ERROR_IO_PENDING` when the selector is shown, or another Win32 error code.
    /// The export takes the 64-bit `bytes_requested` in one register.
    pub XamShowDeviceSelectorUI(
        user_index: UserIndex,
        content_type: u32,
        flags: DeviceSelectorFlags,
        bytes_requested: u64,
        device: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2CB,
    // user_index in r3, content_type in r4, flags in r5, bytes_requested in r7:r8, device in
    // r9, overlapped in r10.
    prologue = [
        "rldimi 8, 7, 32, 0",
        "mr 6, 8",
        "mr 7, 9",
        "mr 8, 10",
    ]
);

crate::import_thunk!(
    /// Opens the theme picker of the system UI for `user_index`. The dashboard passes 0 for the other
    /// two arguments; their meaning is unknown.
    pub XamShowPersonalizationUI(user_index: u32, unknown1: u32, unknown2: u32) -> u32,
    lib = XAM,
    ordinal = 0x2E1
);

crate::import_thunk!(
    /// Returns nonzero when the running system UI was opened by a title.
    pub safe XamIsSysUiInvokedByTitle() -> u32,
    lib = XAM,
    ordinal = 0x2EB
);

crate::import_thunk!(
    /// Returns nonzero while a system UI is shown.
    pub safe XamIsUIActive() -> Boolean,
    lib = XAM,
    ordinal = 0x2EC
);

crate::import_thunk!(
    /// Writes the gamer picture of `title` with the key words `key_high` and `key_low` to the
    /// profile signed in on `user_index`. The 2.0.6717 dashboard passes 0 for `reserved`.
    /// Returns a Win32 error code, `ERROR_IO_PENDING` with an `overlapped`.
    pub XamWriteGamerTile(
        user_index: UserIndex,
        title: u32,
        key_high: u32,
        key_low: u32,
        reserved: u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2F0
);

crate::import_thunk!(
    /// Writes the locator of the system resource `name` to `locator`, at most `capacity` UTF-16 units: `section://<xam.xex module handle>,shrdres#<name>`, or a `memory://` locator for eight legacy names. In system software 2.0.21256.0 it asserts (`twi 31`) unless the running title is one of two system titles, which halts other titles under the debug monitor.
    pub XamBuildLegacySystemResourceLocator(name: *const u16, locator: *mut u16, capacity: u32) -> Hresult,
    lib = XAM,
    ordinal = 0x31C
);

crate::import_thunk!(
    /// Returns the audio flags (XConfig user setting 0xB).
    pub safe XGetAudioFlags() -> u32,
    lib = XAM,
    ordinal = 0x3CA
);

crate::import_thunk!(
    /// Returns the AV pack value.
    pub safe XGetAVPack() -> u32,
    lib = XAM,
    ordinal = 0x3CB
);

crate::import_thunk!(
    /// Returns the game region of the console (XConfig).
    pub safe XGetGameRegion() -> u32,
    lib = XAM,
    ordinal = 0x3CC
);

crate::import_thunk!(
    /// Stores the current video mode.
    pub XGetVideoMode(mode: *mut VideoMode) -> (),
    lib = XAM,
    ordinal = 0x3D1
);

crate::import_thunk!(
    /// Returns the console language.
    pub safe XamGetLanguage() -> u32,
    lib = XAM,
    ordinal = 0x3D2
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
    pub RtlAllocateHeap(heap: u32, flags: HeapFlags, size: u32) -> u32,
    lib = XAM,
    ordinal = 0x3FC
);

crate::import_thunk!(
    /// Frees a heap block.
    pub RtlFreeHeap(heap: u32, flags: HeapFlags, block: u32) -> u32,
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
    /// Removes the IPTV service registration.
    pub XamIptvUninstall() -> Hresult,
    lib = XAM,
    ordinal = 0x517
);

crate::import_thunk!(
    /// Stores the DVR storage in megabytes: the space given to recordings, the space they
    /// use, and the largest allocation the service accepts.
    pub XamGetDvrStorage(allocated: *mut u32, used: *mut u32, maximum: *mut u32) -> Hresult,
    lib = XAM,
    ordinal = 0x518
);

crate::import_thunk!(
    /// Sets the space given to recordings, in megabytes. Completes through `overlapped`;
    /// the 2.0.6717 dashboard treats a negative result as a failure.
    pub XamSetDvrStorage(allocated: u32, overlapped: *mut Overlapped) -> Hresult,
    lib = XAM,
    ordinal = 0x519
);

crate::import_thunk!(
    /// Stores a pointer to the NUL-terminated UTF-16 name of the IPTV service at `name`.
    pub XamIptvGetServiceName(name: *mut *const u16) -> Hresult,
    lib = XAM,
    ordinal = 0x51A
);

crate::import_thunk!(
    /// Returns the default heap of the process.
    pub safe GetProcessHeap() -> u32,
    lib = XAM,
    ordinal = 0x7D2
);

crate::import_thunk!(
    /// Copies the background download queue of `user` to `items`, 0x250-byte records with the
    /// package path at +0x138. `count` holds the capacity in records on entry and the number
    /// copied on return. The 2.0.6717 dashboard passes a capacity of 6 and reads only `count`.
    pub XamBackgroundDownloadGetItems(user_index: UserIndex, items: *mut u8, count: *mut u32) -> u32,
    lib = XAM,
    ordinal = 0x9C7
);

crate::import_thunk!(
    /// Copies the play timer record to `data`. The 2.0.6717 dashboard does not read the result.
    pub XamPlayTimerGetData(data: *mut PlayTimerData) -> (),
    lib = XAM,
    ordinal = 0xA5A
);

crate::import_thunk!(
    /// Replaces the play timer record with `data`. The 2.0.6717 dashboard passes 1 for `save`
    /// and does not read the result.
    pub XamPlayTimerSetData(data: *const PlayTimerData, save: u32) -> (),
    lib = XAM,
    ordinal = 0xA5B
);

crate::import_thunk!(
    /// Resumes the play timer. The 2.0.6717 dashboard passes 1.
    pub safe XamPlayTimerResume(resume: u32) -> (),
    lib = XAM,
    ordinal = 0xA5C
);

crate::import_thunk!(
    /// Suspends the play timer.
    pub safe XamPlayTimerSuspend() -> (),
    lib = XAM,
    ordinal = 0xA5D
);

crate::import_thunk!(
    /// Whether the play timer is counting.
    pub safe XamPlayTimerIsRunning() -> Boolean,
    lib = XAM,
    ordinal = 0xA5E
);

crate::import_thunk!(
    /// Whether a play timer period is set.
    pub safe XamPlayTimerIsEnabled() -> Boolean,
    lib = XAM,
    ordinal = 0xA5F
);

crate::import_thunk!(
    /// Writes the next reset of `period` to `data.next_reset`.
    pub XamPlayTimerGetNextResetDate(period: u32, data: *mut PlayTimerData) -> (),
    lib = XAM,
    ordinal = 0xA60
);

crate::import_thunk!(
    /// Posts the play timer notification.
    pub safe XamPlayTimerForceNotification() -> (),
    lib = XAM,
    ordinal = 0xA64
);

crate::import_thunk_u64!(
    /// The security violations the system has detected, as a 64-bit value.
    pub safe XamGetSecurityViolationsDetected() -> u64,
    lib = XAM,
    ordinal = 0xAF3
);

crate::import_thunk!(
    /// Gives the PNG of the default gamer picture: stores its address at `image` and its
    /// length at `size`. The 2.0.6717 dashboard does not read the result.
    pub XamGetDefaultSystemImage(image: *mut *const u8, size: *mut u32) -> u32,
    lib = XAM,
    ordinal = 0x1C5
);

crate::import_thunk!(
    /// Returns nonzero when `title` is an original Xbox title.
    pub safe XamIsXbox1TitleId(title: u32) -> Boolean,
    lib = XAM,
    ordinal = 0x1BF
);

crate::import_thunk!(
    /// Returns the title id of the running title, 0 for none.
    pub safe XamGetCurrentTitleId() -> u32,
    lib = XAM,
    ordinal = 0x1CF
);

import_adapter!(
    /// Writes the time of the local `FILETIME` `time` for `user` as NUL-terminated UTF-16, at most
    /// `length` units. The 2.0.6717 dashboard does not read the result.
    /// The export takes `time` in one register.
    pub XamFormatTimeString(user: u32, time: u64, buffer: *mut u16, length: u32) -> (),
    lib = XAM,
    ordinal = 0x1E7,
    // user in r3, time in r5:r6, buffer in r7, length in r8.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
        "mr 5, 7",
        "mr 6, 8",
    ]
);

import_adapter!(
    /// Writes the date of the local `FILETIME` `time` for `user` as NUL-terminated UTF-16, at most
    /// `length` units. The 2.0.6717 dashboard does not read the result.
    /// The export takes `time` in one register.
    pub XamFormatDateString(user: u32, time: u64, buffer: *mut u16, length: u32) -> (),
    lib = XAM,
    ordinal = 0x1E8,
    // user in r3, time in r5:r6, buffer in r7, length in r8.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 4, 6",
        "mr 5, 7",
        "mr 6, 8",
    ]
);

crate::import_thunk_stack!(
    /// Reads the profile settings `ids` (`count` entries) of `title` for `user`, or of the
    /// `xuid_count` profiles at `xuids`. Without `buffer`, stores the size needed at `size`.
    /// Returns a Win32 error code, or `ERROR_IO_PENDING` with an `overlapped`.
    pub XamUserReadProfileSettings(
        title: u32,
        user: UserIndex,
        xuid_count: u32,
        xuids: *const u64,
        count: u32,
        ids: *const u32,
        size: *mut u32,
        buffer: *mut u8,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x219
);

crate::import_thunk!(
    /// Returns the Xbox LIVE membership type of `user` in the low byte.
    pub safe XamUserGetMembershipType(user: UserIndex) -> u32,
    lib = XAM,
    ordinal = 0x21B
);

crate::import_thunk!(
    /// Reads the next records of an enumeration into `buffer`. Stores the record count at
    /// `items` when not null. Returns a Win32 error code: `ERROR_NO_MORE_FILES` after the
    /// last record, or `ERROR_IO_PENDING` with an `overlapped`, which then holds the count
    /// in its length.
    pub XamEnumerate(
        handle: Handle,
        flags: u32,
        buffer: *mut u8,
        size: u32,
        items: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x250
);

crate::import_thunk!(
    /// Opens an enumeration of the packages of `content_type` and `title` on `device` (0: every
    /// device). The 2.0.6717 dashboard passes 0 as the first argument; its Memory area passes a
    /// device id, other callers 0. Returns a Win32 error code.
    pub XamContentAggregateCreateEnumerator(
        reserved1: u32,
        device: u32,
        content_type: u32,
        title: u32,
        handle: *mut Handle,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x279
);

crate::import_thunk!(
    /// Writes the cached name of `title` as NUL-terminated UTF-16 to `buffer`, at most
    /// `*length` units. Returns a Win32 error code.
    pub XamGetCachedTitleName(title: u32, buffer: *mut u16, length: *mut u32) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2B6
);

import_adapter!(
    /// Opens an enumeration of the achievements of `title` for `user` or `xuid` (0 for the
    /// user). Stores the buffer size for `count` records at `size`. Returns a Win32 error code.
    /// The export takes `xuid` in one register.
    pub XamUserCreateAchievementEnumerator(
        title: u32,
        user: UserIndex,
        xuid: u64,
        details: u32,
        first: u32,
        count: u32,
        size: *mut u32,
        handle: *mut Handle,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2EE,
    // title, user in r3, r4; xuid in r5:r6; details..size in r7-r10; handle at 8(1).
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 5, 6",
        "mr 6, 7",
        "mr 7, 8",
        "mr 8, 9",
        "mr 9, 10",
        "lwz 10, 8(1)",
    ]
);

import_adapter!(
    /// Reads a tile PNG into `buffer`. Without `buffer`, stores the size needed at `size`.
    /// Returns a Win32 error code, or `ERROR_IO_PENDING` with an `overlapped`.
    /// The export takes `tile` in one register.
    pub XamReadTile(
        tile_type: u32,
        title: u32,
        tile: u64,
        user: UserIndex,
        buffer: *mut u8,
        size: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2EF,
    // tile_type, title in r3, r4; tile in r5:r6; the rest in r7-r10.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 5, 6",
        "mr 6, 7",
        "mr 7, 8",
        "mr 8, 9",
        "mr 9, 10",
    ]
);

import_adapter!(
    /// Opens an enumeration of the titles `user` or `xuid` (0 for the user) has played.
    /// Stores the buffer size for `count` records at `size`. Returns a Win32 error code.
    /// The export takes `xuid` in one register.
    pub XamUserCreateTitlesPlayedEnumerator(
        title: u32,
        user: UserIndex,
        xuid: u64,
        first: u32,
        count: u32,
        size: *mut u32,
        handle: *mut Handle,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2F3,
    // title, user in r3, r4; xuid in r5:r6; the rest in r7-r10.
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 5, 6",
        "mr 6, 7",
        "mr 7, 8",
        "mr 8, 9",
        "mr 9, 10",
    ]
);

import_adapter_stack!(
    /// Reads a tile PNG into `buffer` of `*size` bytes and stores the length at `size`.
    /// `key` is the picture key setting for tile type 0xC; the 2.0.6717 dashboard passes
    /// `has_key` 0 with it. Returns a Win32 error code, or `ERROR_IO_PENDING` with an
    /// `overlapped`. The export takes `tile` in one register.
    pub XamReadTileEx(
        tile_type: u32,
        title: u32,
        tile: u64,
        user: UserIndex,
        has_key: u32,
        key: *const UserData,
        buffer: *mut u8,
        size: *mut u32,
        overlapped: *mut Overlapped,
    ) -> Win32ErrorCode,
    lib = XAM,
    ordinal = 0x2FC,
    // tile_type, title in r3, r4; tile in r5:r6; user..buffer in r7-r10; size and
    // overlapped at 0x78(1) and 0x7c(1).
    prologue = [
        "rldimi 6, 5, 32, 0",
        "mr 5, 6",
        "mr 6, 7",
        "mr 7, 8",
        "mr 8, 9",
        "mr 9, 10",
        "lwz 10, 0x78(1)",
        "li 12, 0",
        "lwz 11, 0x7c(1)",
        "stw 12, 0x50(1)",
        "stw 11, 0x54(1)",
    ]
);
