mod constants;

use crate::kernel_bindings::bindings;
pub use constants::ip_protocol;
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

pub type UdpHeader = bindings::udphdr;
pub type TcpHeader = bindings::tcphdr;
pub type IPv4Header = bindings::iphdr;
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
        self.tot_len as usize
    }
    fn header_size(&self) -> usize {
        core::mem::size_of::<IPv4Header>()
    }
}

impl<'a, T> NetPacket<'a, T>
where
    T: PacketSize + Copy,
{
    pub fn from(header_ptr: *mut T) -> Self {
        unsafe {
            let payload_ptr = (header_ptr as usize + (*header_ptr).header_size()) as *mut u8;
            use crate::println;
            println!("payload size {}", (*header_ptr).payload_size());
            NetPacket {
                header: *header_ptr,
                payload: core::slice::from_raw_parts_mut(payload_ptr, 1),
            }
        }
    }
}

impl<'a> NetPacket<'a, TcpHeader> {
    pub fn from(ip_packet: &IPv4Packet<'a>) -> Option<TcpPacket<'a>> {
        unsafe {
            match ip_packet.header.protocol {
                ip_protocol::TCP => {
                    let tcp_header = (ip_packet.payload.as_ptr() as usize + size_of::<IPv4Header>())
                        as *const TcpHeader;
                    let tcp_payload_size = ip_packet.header.tot_len as usize
                        - size_of::<IPv4Header>()
                        - size_of::<TcpHeader>();
                    Some(TcpPacket::<'a> {
                        header: *tcp_header,
                        payload: &ip_packet.payload[size_of::<TcpHeader>()..],
                    })
                }
                _ => None,
            }
        }
    }
}

impl<'a> NetPacket<'a, UdpHeader> {
    pub fn from(ip_packet:&IPv4Packet<'a>) -> Option<Self> {
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
