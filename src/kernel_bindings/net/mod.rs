mod constants;

use crate::kernel_bindings::bindings;
pub use constants::ip_protocol;
pub use constants::protocol_family;
pub use constants::icmp_code;
use core::mem::size_of;

extern "C" {
    #[no_mangle]
    pub static mut init_net: bindings::net;
}

pub struct SocketBuffer {
    ptr: *const u8,
}

pub struct NetPacket<'a, T> {
    pub header: T,
    pub payload: &'a [u8],
}

pub enum Port<T> {
    Some(T, T),
    None
}

pub type IpProtocol = u8;
pub type UdpHeader = bindings::udphdr;
pub type TcpHeader = bindings::tcphdr;
pub type IPv4Header = bindings::iphdr;
pub type IcmpHeader = bindings::icmphdr;
pub type IPv4Packet<'a> = NetPacket<'a, IPv4Header>;
pub type TcpPacket<'a> = NetPacket<'a, TcpHeader>;
pub type UdpPacket<'a> = NetPacket<'a, UdpHeader>;

pub trait PacketSize {
    fn packet_size(&self) -> usize;
    fn header_size(&self) -> usize;
    fn payload_size(&self) -> usize {
        self.packet_size() - self.header_size()
    }
}

impl PacketSize for IPv4Header {
    fn packet_size(&self) -> usize {
        self.tot_len.to_be() as usize
    }
    fn header_size(&self) -> usize {
        self._bitfield_1.get(0, 4) as usize * 4
    }
}

impl<'a, T> NetPacket<'a, T>
where
    T: PacketSize + Copy,
{
    pub fn from_buf(buffer: &'a [u8]) -> Self {
        unsafe {
            let header_ptr = buffer.as_ptr() as *const T;
            let header_size = (*header_ptr).header_size();
            use crate::println;
            // println!("{} {}", header_size, buffer.len());
            NetPacket {
                header: *header_ptr,
                payload: &buffer[header_size..],
            }
        }
    }
}

impl IPv4Header {
    pub fn from_skbuff<'buff>(sk_buff: &'buff bindings::sk_buff) -> Option<&'buff IPv4Header> {
        unsafe {
            let header_ptr = bindings::ip_hdr_wrapped(sk_buff as *const bindings::sk_buff);
            header_ptr.as_ref()
        }
    }
}

impl TcpHeader {
    pub fn from_skbuff<'buff>(sk_buff: &'buff bindings::sk_buff) -> Option<&'buff TcpHeader> {
        unsafe {
            let header_ptr = bindings::tcp_hdr_wrapped(sk_buff as *const bindings::sk_buff);
            header_ptr.as_ref()
        }
    }
}

impl IcmpHeader {
    pub fn from_skbuff<'buff>(sk_buff: &'buff bindings::sk_buff) -> Option<&'buff IcmpHeader> {
        unsafe {
            let header_ptr = bindings::icmp_hdr_wrapped(sk_buff as *const bindings::sk_buff);
            header_ptr.as_ref()
        }
    }
}

impl UdpHeader {
    pub fn from_skbuff<'buff>(sk_buff: &'buff bindings::sk_buff) -> Option<&'buff UdpHeader> {
        unsafe {
            let header_ptr = bindings::udp_hdr_wrapped(sk_buff as *const bindings::sk_buff);
            header_ptr.as_ref()
        }
    }
}

impl<'a> NetPacket<'a, TcpHeader> {
    pub fn from_lower(ip_packet: &IPv4Packet<'a>) -> Option<TcpPacket<'a>> {
        unsafe {
            match ip_packet.header.protocol {
                ip_protocol::TCP => {
                    let tcp_header = ip_packet.payload.as_ptr() as *const TcpHeader;
                    let data_offset = (*tcp_header)._bitfield_1.get(4, 4) as usize * 4;
                        
                    Some(TcpPacket::<'a> {
                        header: *tcp_header,
                        payload: &ip_packet.payload[data_offset..],
                    })
                }
                _ => None,
            }
        }
    }
}

impl<'a> NetPacket<'a, UdpHeader> {
    pub fn from_lower(ip_packet:&IPv4Packet<'a>) -> Option<Self> {
        unsafe {
            match ip_packet.header.protocol {
                ip_protocol::UDP => {
                    let udp_header = (ip_packet.payload.as_ptr() as usize + size_of::<IPv4Header>()) as *const UdpHeader;
                    Some(NetPacket::<'a, UdpHeader> {
                        header: *udp_header,
                        payload: &ip_packet.payload[size_of::<UdpHeader>()..],
                    })
                }
                _ => None,
            }
        }
    }
}




// impl From<&TcpHeader> for Port<u16> {
//     fn from(&header: TcpHeader) -> Self {
//         Port::Some(header.source, header.dest)
//     }
// }

// impl From<&UdpHeader> for Port<u16> {
//     fn from(&header: UdpHeader) -> Self {
//         Port::Some(header.source, header.dest)
//     }
// }