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
