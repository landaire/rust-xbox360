/// An XNet option of `NetDll_XNetGetOpt` and `NetDll_XNetSetOpt`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XnetOption(pub u32);

impl XnetOption {
    /// A `u64` that changes with network traffic (the 2.0.6717 dashboard
    /// shows activity when it changes).
    pub const ACTIVITY_COUNTER: Self = Self(0x4);
    /// Promiscuous receive.
    pub const PROMISCUOUS: Self = Self(0x1390);
    /// Link flags of the network interface.
    pub const LINK_FLAGS: Self = Self(0x1395);
    /// Reports the secondary network interface.
    pub const SECONDARY_NIC: Self = Self(0x2f00);
    /// A `u32` state of the USB wireless adapter; bit 0 is set while it is
    /// attached.
    pub const WIRELESS_ADAPTER_STATE: Self = Self(0x2af9);
    /// An `i32`: received signal strength of the wireless adapter in dBm.
    pub const WIRELESS_RSSI: Self = Self(0x0d01_0206);
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

/// `XNADDR`, as the XNADDR queries store it. Addresses and the port are in
/// network byte order.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct XnAddr {
    /// IP address; zero without a static or DHCP address.
    pub address: u32,
    /// Online IP address; zero when not online.
    pub online_address: u32,
    pub online_port: u16,
    pub ethernet_address: [u8; 6],
    pub online_id: [u8; 20],
}

const _: () = assert!(core::mem::size_of::<XnAddr>() == 0x24);

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

flags! {
    /// `XNET_STARTUP_*` flags of [`XnetStartupParams`].
    pub struct XnetStartupFlags(u8) {
        /// Accept insecure connections, as development kits allow.
        BYPASS_SECURITY = 0x01,
    }
}

/// `XNetStartupParams` of `NetDll_XNetStartup`, with Xenia's field order.
/// A zero field selects the default of the network stack.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XnetStartupParams {
    /// The size of the block, 13.
    pub size: u8,
    pub flags: XnetStartupFlags,
    pub max_datagram_sockets: u8,
    pub max_stream_sockets: u8,
    pub default_receive_buffer_kib: u8,
    pub default_send_buffer_kib: u8,
    pub max_key_registrations: u8,
    pub max_security_associations: u8,
    pub qos_data_limit_div4: u8,
    pub qos_probe_timeout_seconds: u8,
    pub qos_probe_retries: u8,
    pub qos_server_max_simultaneous_responses: u8,
    pub qos_pair_wait_time_seconds: u8,
}

impl XnetStartupParams {
    /// The block with its size set and every other field zero.
    pub const fn new() -> Self {
        Self {
            size: core::mem::size_of::<Self>() as u8,
            flags: XnetStartupFlags::empty(),
            max_datagram_sockets: 0,
            max_stream_sockets: 0,
            default_receive_buffer_kib: 0,
            default_send_buffer_kib: 0,
            max_key_registrations: 0,
            max_security_associations: 0,
            qos_data_limit_div4: 0,
            qos_probe_timeout_seconds: 0,
            qos_probe_retries: 0,
            qos_server_max_simultaneous_responses: 0,
            qos_pair_wait_time_seconds: 0,
        }
    }
}

impl Default for XnetStartupParams {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::size_of::<XnetStartupParams>() == 13);
    assert!(core::mem::offset_of!(XnetStartupParams, default_receive_buffer_kib) == 4);
};

flags! {
    /// Flags of an [`XnetConfigParams`] record. The meaning of each bit is
    /// not known; a debug monitor sets 0xc with a static address, subnet
    /// and gateway.
    pub struct XnetConfigFlags(u16) {}
}

/// `XNetConfigParams`, the record that `NetDll_XnpConfig` applies. Fields
/// without a known meaning are reserved; a caller zeroes them.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XnetConfigParams {
    pub reserved0: [u8; 0x4c],
    pub flags: XnetConfigFlags,
    pub reserved1: [u8; 6],
    /// Static IP address, network byte order.
    pub address: u32,
    /// Subnet mask, network byte order.
    pub subnet: u32,
    /// Default gateway, network byte order.
    pub gateway: u32,
    pub reserved2: [u8; 8],
    /// DHCP host name, NUL-terminated.
    pub host_name: [u8; 0x184],
}

impl XnetConfigParams {
    /// A record with every field zero.
    pub const fn new() -> Self {
        Self {
            reserved0: [0; 0x4c],
            flags: XnetConfigFlags::empty(),
            reserved1: [0; 6],
            address: 0,
            subnet: 0,
            gateway: 0,
            reserved2: [0; 8],
            host_name: [0; 0x184],
        }
    }
}

impl Default for XnetConfigParams {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(core::mem::offset_of!(XnetConfigParams, flags) == 0x4c);
    assert!(core::mem::offset_of!(XnetConfigParams, address) == 0x54);
    assert!(core::mem::offset_of!(XnetConfigParams, subnet) == 0x58);
    assert!(core::mem::offset_of!(XnetConfigParams, gateway) == 0x5c);
    assert!(core::mem::offset_of!(XnetConfigParams, host_name) == 0x68);
    assert!(core::mem::size_of::<XnetConfigParams>() == 0x1ec);
};

flags! {
    /// Flags of `NetDll_XnpEthernetInterceptSetCallbacks`. The meaning of
    /// each bit is not known; a debug monitor's network simulator sets 0x2
    /// and a frame capture sets none.
    pub struct EthernetInterceptFlags(u32) {}
}
