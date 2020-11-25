#[allow(dead_code)]
pub mod ip_protocol {
    pub const IP: u8 = 0;
    pub const ICMP: u8 = 1;
    pub const IGMP: u8 = 2;
    pub const IPIP: u8 = 4;
    pub const TCP: u8 = 6;
    pub const EGP: u8 = 8;
    pub const PUP: u8 = 12;
    pub const UDP: u8 = 17;
    pub const IDP: u8 = 22;
    pub const TP: u8 = 29;
    pub const DCCP: u8 = 33;
    pub const IPV6: u8 = 41;
    pub const RSVP: u8 = 46;
    pub const GRE: u8 = 47;
    pub const ESP: u8 = 50;
    pub const AH: u8 = 51;
    pub const MTP: u8 = 92;
    pub const BEETPH: u8 = 94;
    pub const ENCAP: u8 = 98;
    pub const PIM: u8 = 103;
    pub const COMP: u8 = 108;
    pub const SCTP: u8 = 132;
    pub const UDPLITE: u8 = 136;
    pub const MPLS: u8 = 137;
    pub const RAW: u8 = 255;
}
pub type IpProtocol = u8;

#[allow(dead_code)]
pub mod protocol_family {
    pub const UNSPEC: u8 = 0;
    pub const LOCAL: u8 = 1;
    pub const UNIX: u8 = LOCAL;
    pub const FILE: u8 = LOCAL;
    pub const INET: u8 = 2;
    pub const AX25: u8 = 3;
    pub const IPX: u8 = 4;
    pub const APPLETALK: u8 = 5;
    pub const NETROM: u8 = 6;
    pub const BRIDGE: u8 = 7;
    pub const ATMPVC: u8 = 8;
    pub const X25: u8 = 9;
    pub const INET6: u8 = 10;
    pub const ROSE: u8 = 11;
    pub const DECnet: u8 = 12;
    pub const NETBEUI: u8 = 13;
    pub const SECURITY: u8 = 14;
    pub const KEY: u8 = 15;
    pub const NETLINK: u8 = 1;
    pub const ROUTE: u8 = NETLINK;
    pub const PACKET: u8 = 17;
    pub const ASH: u8 = 18;
    pub const ECONET: u8 = 19;
    pub const ATMSVC: u8 = 20;
    pub const RDS: u8 = 21;
    pub const SNA: u8 = 22;
    pub const IRDA: u8 = 23;
    pub const PPPOX: u8 = 24;
    pub const WANPIPE: u8 = 25;
    pub const LLC: u8 = 26;
    pub const IB: u8 = 27;
    pub const MPLS: u8 = 28;
    pub const CAN: u8 = 29;
    pub const TIPC: u8 = 30;
    pub const BLUETOOTH: u8 = 31;
    pub const IUCV: u8 = 32;
    pub const RXRPC: u8 = 33;
    pub const ISDN: u8 = 34;
    pub const PHONET: u8 = 35;
    pub const IEEE802154: u8 = 36;
    pub const CAIF: u8 = 37;
    pub const ALG: u8 = 38;
    pub const NFC: u8 = 39;
    pub const VSOCK: u8 = 40;
    pub const KCM: u8 = 41;
    pub const QIPCRTR: u8 = 42;
    pub const SMC: u8 = 43;
    pub const XDP: u8 = 44;
    pub const MAX: u8 = 45;
}

pub mod icmp_code {
    pub const ICMP_ECHOREPLY: u8 = 0;
    pub const ICMP_DEST_UNREACH: u8 = 3;
    pub const ICMP_SOURCE_QUENCH: u8 = 4;
    pub const ICMP_REDIRECT: u8 = 5;
    pub const ICMP_ECHO: u8 = 8;
    pub const ICMP_TIME_EXCEEDED: u8 = 11;
    pub const ICMP_PARAMETERPROB: u8 = 12;
    pub const ICMP_TIMESTAMP: u8 = 13;
    pub const ICMP_TIMESTAMPREPLY: u8 = 14;
    pub const ICMP_INFO_REQUEST: u8 = 15;
    pub const ICMP_INFO_REPLY: u8 = 16;
    pub const ICMP_ADDRESS: u8 = 17;
    pub const ICMP_ADDRESSREPLY: u8 = 18;
}
