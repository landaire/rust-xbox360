//! Networking for the Xbox 360 devkit over the kernel's winsock (`NetDll_*`).
//!
//! IPv4 only. XNet security is bypassed at startup (devkit feature) so sockets
//! can reach ordinary hosts. Sockets are blocking by default; `set_nonblocking`
//! flips `FIONBIO`.

use crate::fmt;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, ToSocketAddrs};
use crate::sync::atomic::{AtomicU32, Ordering};
use crate::sys::pal::abi;
use crate::time::Duration;

const CALLER: u32 = 1; // XNCALLER_TITLE
const AF_INET: i32 = 2;
const SOCK_STREAM: i32 = 1;
const SOCK_DGRAM: i32 = 2;
const IPPROTO_TCP: i32 = 6;
const IPPROTO_UDP: i32 = 17;
const FIONBIO: u32 = 0x8004_667E;
const INVALID_SOCKET: u32 = 0xFFFF_FFFF;
const SOCKET_ERROR: i32 = -1;

fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { abi::NetDll_WSAGetLastError(CALLER) })
}

/// Bring up XNet (bypassing security) and winsock exactly once.
fn ensure_net_init() {
    static STATE: AtomicU32 = AtomicU32::new(0); // 0=unstarted, 1=in-progress/done
    if STATE.swap(1, Ordering::AcqRel) == 0 {
        // XNetStartupParams: cfgSizeOfStruct=0x0D, cfgFlags=XNET_STARTUP_BYPASS_SECURITY(0x01).
        let mut xnsp = [0u8; 16];
        xnsp[0] = 0x0D;
        xnsp[1] = 0x01;
        unsafe { abi::NetDll_XNetStartup(CALLER, xnsp.as_ptr()) };
        let mut wsadata = [0u8; 512];
        unsafe { abi::NetDll_WSAStartup(CALLER, 0x0202, wsadata.as_mut_ptr()) };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SockAddrIn {
    family: u16,
    port: u16, // network byte order == native big-endian
    addr: u32, // network byte order == native big-endian
    zero: [u8; 8],
}

fn to_sockaddr(addr: &SocketAddr) -> io::Result<SockAddrIn> {
    match addr {
        SocketAddr::V4(v4) => Ok(SockAddrIn {
            family: AF_INET as u16,
            port: v4.port(),
            addr: u32::from_be_bytes(v4.ip().octets()),
            zero: [0; 8],
        }),
        SocketAddr::V6(_) => Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "IPv6 is not supported on xbox360",
        )),
    }
}

fn from_sockaddr(sa: &SockAddrIn) -> SocketAddr {
    let ip = Ipv4Addr::from(sa.addr.to_be_bytes());
    SocketAddr::V4(crate::net::SocketAddrV4::new(ip, sa.port))
}

struct Socket(u32);

impl Socket {
    fn new(ty: i32, proto: i32) -> io::Result<Socket> {
        ensure_net_init();
        let s = unsafe { abi::NetDll_socket(CALLER, AF_INET, ty, proto) };
        if s == INVALID_SOCKET { Err(last_error()) } else { Ok(Socket(s)) }
    }

    fn set_nonblocking(&self, nb: bool) -> io::Result<()> {
        let mut v: u32 = nb as u32;
        let r = unsafe { abi::NetDll_ioctlsocket(CALLER, self.0, FIONBIO, &mut v) };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { abi::NetDll_closesocket(CALLER, self.0) };
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
        let sa = to_sockaddr(addr)?;
        let sock = Socket::new(SOCK_STREAM, IPPROTO_TCP)?;
        let r = unsafe {
            abi::NetDll_connect(CALLER, sock.0, &sa as *const SockAddrIn as *const u8, 16)
        };
        if r == SOCKET_ERROR {
            return Err(last_error());
        }
        Ok(TcpStream(sock))
    }

    pub fn connect_timeout(addr: &SocketAddr, _t: Duration) -> io::Result<TcpStream> {
        Self::connect_addr(addr)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(unsafe {
            abi::NetDll_recv(CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 0)
        })
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
        cvt(unsafe {
            abi::NetDll_send(CALLER, self.0.0, buf.as_ptr(), buf.len() as i32, 0)
        })
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
        cvt(unsafe {
            abi::NetDll_recv(CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 2 /*MSG_PEEK*/)
        })
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_getpeername(CALLER, self.0.0, &mut sa as *mut SockAddrIn as *mut u8, &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_getsockname(CALLER, self.0.0, &mut sa as *mut SockAddrIn as *mut u8, &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        let how = match how {
            Shutdown::Read => 0,
            Shutdown::Write => 1,
            Shutdown::Both => 2,
        };
        let r = unsafe { abi::NetDll_shutdown(CALLER, self.0.0, how) };
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
        write!(f, "TcpStream(socket={})", self.0.0)
    }
}

pub struct TcpListener(Socket);

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        let sa = to_sockaddr(&a)?;
        let sock = Socket::new(SOCK_STREAM, IPPROTO_TCP)?;
        let r = unsafe {
            abi::NetDll_bind(CALLER, sock.0, &sa as *const SockAddrIn as *const u8, 16)
        };
        if r == SOCKET_ERROR {
            return Err(last_error());
        }
        if unsafe { abi::NetDll_listen(CALLER, sock.0, 128) } == SOCKET_ERROR {
            return Err(last_error());
        }
        Ok(TcpListener(sock))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_getsockname(CALLER, self.0.0, &mut sa as *mut SockAddrIn as *mut u8, &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let s = unsafe {
            abi::NetDll_accept(CALLER, self.0.0, &mut sa as *mut SockAddrIn as *mut u8, &mut len)
        };
        if s == INVALID_SOCKET {
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
        write!(f, "TcpListener(socket={})", self.0.0)
    }
}

pub struct UdpSocket(Socket);

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        let sa = to_sockaddr(&a)?;
        let sock = Socket::new(SOCK_DGRAM, IPPROTO_UDP)?;
        let r = unsafe {
            abi::NetDll_bind(CALLER, sock.0, &sa as *const SockAddrIn as *const u8, 16)
        };
        if r == SOCKET_ERROR {
            return Err(last_error());
        }
        Ok(UdpSocket(sock))
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Err(unsupported())
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_getsockname(CALLER, self.0.0, &mut sa as *mut SockAddrIn as *mut u8, &mut len)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(from_sockaddr(&sa)) }
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_recvfrom(
                CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 0,
                &mut sa as *mut SockAddrIn as *mut u8, &mut len,
            )
        };
        Ok((cvt(r)?, from_sockaddr(&sa)))
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let mut sa = SockAddrIn { family: 0, port: 0, addr: 0, zero: [0; 8] };
        let mut len: i32 = 16;
        let r = unsafe {
            abi::NetDll_recvfrom(
                CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 2,
                &mut sa as *mut SockAddrIn as *mut u8, &mut len,
            )
        };
        Ok((cvt(r)?, from_sockaddr(&sa)))
    }

    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        let sa = to_sockaddr(addr)?;
        cvt(unsafe {
            abi::NetDll_sendto(
                CALLER, self.0.0, buf.as_ptr(), buf.len() as i32, 0,
                &sa as *const SockAddrIn as *const u8, 16,
            )
        })
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(unsafe { abi::NetDll_recv(CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 0) })
    }
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(unsafe { abi::NetDll_recv(CALLER, self.0.0, buf.as_mut_ptr(), buf.len() as i32, 2) })
    }
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        cvt(unsafe { abi::NetDll_send(CALLER, self.0.0, buf.as_ptr(), buf.len() as i32, 0) })
    }
    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let a = addr.to_socket_addrs()?.next().ok_or_else(|| {
            io::const_error!(io::ErrorKind::InvalidInput, "no addresses")
        })?;
        let sa = to_sockaddr(&a)?;
        let r = unsafe {
            abi::NetDll_connect(CALLER, self.0.0, &sa as *const SockAddrIn as *const u8, 16)
        };
        if r == SOCKET_ERROR { Err(last_error()) } else { Ok(()) }
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
    pub fn set_broadcast(&self, _v: bool) -> io::Result<()> {
        Ok(())
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
        write!(f, "UdpSocket(socket={})", self.0.0)
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
