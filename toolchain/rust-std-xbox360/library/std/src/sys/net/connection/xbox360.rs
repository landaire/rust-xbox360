//! Networking for the Xbox 360 devkit over the kernel's winsock (`NetDll_*`).
//!
//! IPv4 only. XNet security is bypassed at startup (devkit feature) so title
//! sockets can reach ordinary hosts. The system's sockets are marked insecure one
//! by one instead (`Socket::mark_insecure`). Sockets are blocking by default;
//! `set_nonblocking` flips `FIONBIO`.

use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, ToSocketAddrs};
use crate::sync::atomic::{AtomicU32, Ordering};
use crate::time::Duration;

use xbox360_sys::xam::{
    self, AddressFamily, IoctlCommand, MessageFlags, Protocol, SOCKET_ERROR, ShutdownHow,
    SockAddrIn, SocketType, XnCaller,
};

// std built for a system module (`--cfg xbox360_system`) opens its sockets as
// the system, so they do not belong to the running title.
#[cfg(xbox360_system)]
const CALLER: XnCaller = XnCaller::SystemApp;
#[cfg(not(xbox360_system))]
const CALLER: XnCaller = XnCaller::Title;
const SOCKADDR_LENGTH: i32 = size_of::<SockAddrIn>() as i32;

fn last_error() -> io::Error {
    io::Error::from_raw_os_error(xam::NetDll_WSAGetLastError(CALLER).0 as i32)
}

/// Bring up XNet (bypassing security) and winsock exactly once.
fn ensure_net_init() {
    static STATE: AtomicU32 = AtomicU32::new(0); // 0=unstarted, 1=in-progress/done
    if STATE.swap(1, Ordering::AcqRel) == 0 {
        // XNetStartupParams: cfgSizeOfStruct=0x0D, cfgFlags=XNET_STARTUP_BYPASS_SECURITY(0x01).
        let mut xnsp = [0u8; 16];
        xnsp[0] = 0x0D;
        xnsp[1] = 0x01;
        // SAFETY: `xnsp` and `wsadata` are live locals of the sizes the calls expect.
        unsafe {
            xam::NetDll_XNetStartup(CALLER, xnsp.as_ptr());
            let mut wsadata = [0u8; 512];
            xam::NetDll_WSAStartup(CALLER, 0x0202, wsadata.as_mut_ptr());
        }
    }
}

fn empty_sockaddr() -> SockAddrIn {
    SockAddrIn { family: 0, port: 0, address: 0, zero: [0; 8] }
}

fn to_sockaddr(addr: &SocketAddr) -> io::Result<SockAddrIn> {
    match addr {
        SocketAddr::V4(v4) => Ok(SockAddrIn {
            family: AddressFamily::Inet.raw() as u16,
            // Network byte order is the native big-endian order.
            port: v4.port(),
            address: u32::from_be_bytes(v4.ip().octets()),
            zero: [0; 8],
        }),
        SocketAddr::V6(_) => Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "IPv6 is not supported on xbox360",
        )),
    }
}

fn from_sockaddr(sa: &SockAddrIn) -> SocketAddr {
    let ip = Ipv4Addr::from(sa.address.to_be_bytes());
    SocketAddr::V4(crate::net::SocketAddrV4::new(ip, sa.port))
}

fn sockaddr_ptr(sa: &SockAddrIn) -> *const u8 {
    (sa as *const SockAddrIn).cast()
}

fn sockaddr_mut_ptr(sa: &mut SockAddrIn) -> *mut u8 {
    (sa as *mut SockAddrIn).cast()
}

struct Socket(xam::Socket);

impl Socket {
    fn new(ty: SocketType, proto: Protocol) -> io::Result<Socket> {
        ensure_net_init();
        // SAFETY: creating a socket has no pointer arguments.
        let s = unsafe { xam::NetDll_socket(CALLER, AddressFamily::Inet, ty, proto) };
        if s == xam::Socket::INVALID {
            return Err(last_error());
        }
        let sock = Socket(s);
        #[cfg(xbox360_system)]
        sock.mark_insecure()?;
        Ok(sock)
    }

    // XNET_STARTUP_BYPASS_SECURITY does not cover the system's sockets: on a
    // development kit their datagrams were accepted and never sent. SOL_SOCKET
    // option 0x5801 lets one exchange plain traffic with any host. A connection
    // that a marked listener accepts worked without it.
    #[cfg(xbox360_system)]
    fn mark_insecure(&self) -> io::Result<()> {
        const INSECURE: xam::SocketOption = xam::SocketOption(0x5801);
        let enabled: u32 = 1;
        // SAFETY: `self.0` is an open socket and `enabled` is a live local of the
        // length passed.
        let r = unsafe {
            xam::NetDll_setsockopt(
                CALLER,
                self.0,
                xam::SocketLevel::Socket,
                INSECURE,
                (&raw const enabled).cast(),
                size_of::<u32>() as i32,
            )
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    fn set_flag(&self, option: xam::SocketOption, on: bool) -> io::Result<()> {
        let value: u32 = on as u32;
        // SAFETY: `self.0` is an open socket and `value` is a live local of the
        // length passed.
        let r = unsafe {
            xam::NetDll_setsockopt(
                CALLER,
                self.0,
                xam::SocketLevel::Socket,
                option,
                (&raw const value).cast(),
                size_of::<u32>() as i32,
            )
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    fn set_nonblocking(&self, nb: bool) -> io::Result<()> {
        let mut v: u32 = nb as u32;
        // SAFETY: `self.0` is an open socket and `v` is a live local.
        let r = unsafe { xam::NetDll_ioctlsocket(CALLER, self.0, IoctlCommand::Fionbio, &mut v) };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    fn socket_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = empty_sockaddr();
        let mut len = SOCKADDR_LENGTH;
        // SAFETY: `self.0` is an open socket; `sa` and `len` are live locals and `len`
        // is the size of `sa`.
        let r = unsafe {
            xam::NetDll_getsockname(CALLER, self.0, sockaddr_mut_ptr(&mut sa), &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    fn connect(&self, addr: &SocketAddr) -> io::Result<()> {
        let sa = to_sockaddr(addr)?;
        // SAFETY: `self.0` is an open socket and `sa` is a live `SockAddrIn`.
        let r = unsafe { xam::NetDll_connect(CALLER, self.0, sockaddr_ptr(&sa), SOCKADDR_LENGTH) };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        let sa = to_sockaddr(addr)?;
        // SAFETY: `self.0` is an open socket and `sa` is a live `SockAddrIn`.
        let r = unsafe { xam::NetDll_bind(CALLER, self.0, sockaddr_ptr(&sa), SOCKADDR_LENGTH) };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    fn recv(&self, buf: &mut [u8], flags: MessageFlags) -> io::Result<usize> {
        // SAFETY: `self.0` is an open socket and `buf` is writable for `buf.len()` bytes.
        cvt(unsafe { xam::NetDll_recv(CALLER, self.0, buf.as_mut_ptr(), buf.len() as i32, flags) })
    }

    fn send(&self, buf: &[u8]) -> io::Result<usize> {
        // SAFETY: `self.0` is an open socket and `buf` is readable for `buf.len()` bytes.
        cvt(unsafe {
            xam::NetDll_send(CALLER, self.0, buf.as_ptr(), buf.len() as i32, MessageFlags::empty())
        })
    }

    fn recv_from(&self, buf: &mut [u8], flags: MessageFlags) -> io::Result<(usize, SocketAddr)> {
        let mut sa = empty_sockaddr();
        let mut len = SOCKADDR_LENGTH;
        // SAFETY: `self.0` is an open socket; `buf` is writable for `buf.len()` bytes and
        // `sa`, `len` are live locals with `len` the size of `sa`.
        let r = unsafe {
            xam::NetDll_recvfrom(
                CALLER,
                self.0,
                buf.as_mut_ptr(),
                buf.len() as i32,
                flags,
                sockaddr_mut_ptr(&mut sa),
                &mut len,
            )
        };
        Ok((cvt(r)?, from_sockaddr(&sa)))
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        // SAFETY: `self.0` is owned by this `Socket` and is closed only here.
        unsafe { xam::NetDll_closesocket(CALLER, self.0) };
    }
}

fn cvt(r: i32) -> io::Result<usize> {
    if r == SOCKET_ERROR { Err(last_error()) } else { Ok(r as usize) }
}

pub struct TcpStream(Socket);

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        let mut last = None;
        for a in addr.to_socket_addrs()? {
            match Self::connect_addr(&a) {
                Ok(s) => return Ok(s),
                Err(e) => last = Some(e),
            }
        }
        Err(last.unwrap_or_else(|| io::const_error!(io::ErrorKind::InvalidInput, "no addresses")))
    }

    fn connect_addr(addr: &SocketAddr) -> io::Result<TcpStream> {
        let sock = Socket::new(SocketType::Stream, Protocol::Tcp)?;
        sock.connect(addr)?;
        Ok(TcpStream(sock))
    }

    pub fn connect_timeout(addr: &SocketAddr, _t: Duration) -> io::Result<TcpStream> {
        Self::connect_addr(addr)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf, MessageFlags::empty())
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_, u8>) -> io::Result<()> {
        let n = {
            let dst = unsafe { cursor.as_mut() };
            let slice =
                unsafe { core::slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, dst.len()) };
            self.read(slice)?
        };
        unsafe { cursor.advance(n) };
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        match bufs.iter_mut().find(|b| !b.is_empty()) {
            Some(b) => self.read(b),
            None => Ok(0),
        }
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        match bufs.iter().find(|b| !b.is_empty()) {
            Some(b) => self.write(b),
            None => Ok(0),
        }
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf, MessageFlags::PEEK)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = empty_sockaddr();
        let mut len = SOCKADDR_LENGTH;
        // SAFETY: `self.0.0` is an open socket; `sa` and `len` are live locals and `len`
        // is the size of `sa`.
        let r = unsafe {
            xam::NetDll_getpeername(CALLER, self.0.0, sockaddr_mut_ptr(&mut sa), &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.socket_addr()
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        let how = match how {
            Shutdown::Read => ShutdownHow::Receive,
            Shutdown::Write => ShutdownHow::Send,
            Shutdown::Both => ShutdownHow::Both,
        };
        // SAFETY: `self.0.0` is an open socket.
        let r = unsafe { xam::NetDll_shutdown(CALLER, self.0.0, how) };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        Err(unsupported())
    }
    pub fn set_read_timeout(&self, _t: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn set_write_timeout(&self, _t: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_linger(&self, _l: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn linger(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_nodelay(&self, _v: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn nodelay(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_keepalive(&self, _v: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn keepalive(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_ttl(&self, _t: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, nb: bool) -> io::Result<()> {
        self.0.set_nonblocking(nb)
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TcpStream(socket={})", self.0.0.raw())
    }
}

pub struct TcpListener(Socket);

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        let sock = Socket::new(SocketType::Stream, Protocol::Tcp)?;
        sock.bind(&a)?;
        // SAFETY: `sock.0` is an open, bound socket.
        if unsafe { xam::NetDll_listen(CALLER, sock.0, 128) } == SOCKET_ERROR {
            return Err(last_error());
        }
        Ok(TcpListener(sock))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.socket_addr()
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut sa = empty_sockaddr();
        let mut len = SOCKADDR_LENGTH;
        // SAFETY: `self.0.0` is a listening socket; `sa` and `len` are live locals and
        // `len` is the size of `sa`.
        let s = unsafe {
            xam::NetDll_accept(CALLER, self.0.0, sockaddr_mut_ptr(&mut sa), &mut len)
        };
        if s == xam::Socket::INVALID {
            return Err(last_error());
        }
        Ok((TcpStream(Socket(s)), from_sockaddr(&sa)))
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        Err(unsupported())
    }
    pub fn set_ttl(&self, _t: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn set_only_v6(&self, _v: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn only_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, nb: bool) -> io::Result<()> {
        self.0.set_nonblocking(nb)
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TcpListener(socket={})", self.0.0.raw())
    }
}

pub struct UdpSocket(Socket);

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        let sock = Socket::new(SocketType::Datagram, Protocol::Udp)?;
        sock.bind(&a)?;
        Ok(UdpSocket(sock))
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Err(unsupported())
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0.socket_addr()
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0.recv_from(buf, MessageFlags::empty())
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0.recv_from(buf, MessageFlags::PEEK)
    }

    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        let sa = to_sockaddr(addr)?;
        // SAFETY: `self.0.0` is an open socket, `buf` is readable for `buf.len()` bytes
        // and `sa` is a live `SockAddrIn`.
        cvt(unsafe {
            xam::NetDll_sendto(
                CALLER,
                self.0.0,
                buf.as_ptr(),
                buf.len() as i32,
                MessageFlags::empty(),
                sockaddr_ptr(&sa),
                SOCKADDR_LENGTH,
            )
        })
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf, MessageFlags::empty())
    }
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv(buf, MessageFlags::PEEK)
    }
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf)
    }
    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        self.0.connect(&a)
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        Err(unsupported())
    }
    pub fn set_read_timeout(&self, _t: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn set_write_timeout(&self, _t: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_broadcast(&self, v: bool) -> io::Result<()> {
        // SO_BROADCAST. Without it a socket of the system process receives no
        // datagram sent to a broadcast address.
        self.0.set_flag(xam::SocketOption(0x0020), v)
    }
    pub fn broadcast(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_loop_v4(&self, _v: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_ttl_v4(&self, _v: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        Ok(1)
    }
    pub fn set_multicast_loop_v6(&self, _v: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn join_multicast_v4(&self, _m: &Ipv4Addr, _i: &Ipv4Addr) -> io::Result<()> {
        Err(unsupported())
    }
    pub fn join_multicast_v6(&self, _m: &Ipv6Addr, _i: u32) -> io::Result<()> {
        Err(unsupported())
    }
    pub fn leave_multicast_v4(&self, _m: &Ipv4Addr, _i: &Ipv4Addr) -> io::Result<()> {
        Err(unsupported())
    }
    pub fn leave_multicast_v6(&self, _m: &Ipv6Addr, _i: u32) -> io::Result<()> {
        Err(unsupported())
    }
    pub fn set_ttl(&self, _t: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, nb: bool) -> io::Result<()> {
        self.0.set_nonblocking(nb)
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UdpSocket(socket={})", self.0.0.raw())
    }
}

/// Numeric-only host resolution (no DNS): parses "a.b.c.d".
pub struct LookupHost {
    addr: Option<SocketAddr>,
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.addr.take()
    }
}

pub fn lookup_host(host: &str, port: u16) -> io::Result<LookupHost> {
    use crate::str::FromStr;
    let ip = Ipv4Addr::from_str(host.trim())
        .map_err(|_| io::const_error!(io::ErrorKind::Unsupported, "DNS is not supported on xbox360"))?;
    Ok(LookupHost { addr: Some(SocketAddr::V4(crate::net::SocketAddrV4::new(ip, port))) })
}

fn unsupported() -> io::Error {
    crate::sys::unsupported_err()
}
