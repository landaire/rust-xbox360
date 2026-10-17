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
mod ui;
mod user;
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
pub use self::ui::MessageBoxFlags;
pub use self::ui::MessageBoxIcon;
pub use self::user::SigninKind;
pub use self::user::SigninState;
pub use self::user::UserIndex;
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
    /// Returns the default heap of the process.
    pub safe GetProcessHeap() -> u32,
    lib = XAM,
    ordinal = 0x7D2
);
