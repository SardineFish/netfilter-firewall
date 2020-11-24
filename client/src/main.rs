extern crate netlink_sys;
extern crate netlink_packet_core;

mod protocol;

use netlink_sys::{Socket,SocketAddr};
use netlink_packet_core as nl_packet;
use packet::{deserialize, packets};
use std::net;

const NETLINK_PROTOCOL: i32 = 17;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Message {
    pub size: usize,
    pub data: Vec<u8>,
}

impl Message {
    pub fn new(data: &[u8]) -> Self {
        Message {
            size: data.len(),
            data: data.to_vec(),
        }
    }
}

impl nl_packet::NetlinkSerializable<Message> for Message {
    fn buffer_len(&self) -> usize {
        self.size
    }
    fn message_type(&self) -> u16 {
        NETLINK_PROTOCOL as u16
    }
    fn serialize(&self, buffer: &mut [u8]) {
        buffer.clone_from_slice(&self.data);
    }
}

impl nl_packet::NetlinkDeserializable<Message> for Message {
    type Error = nl_packet::DecodeError;
    fn deserialize(header: &nl_packet::NetlinkHeader, payload: &[u8]) -> Result<Self, Self::Error> {
        Ok(Message {
            data: payload.to_vec(),
            size: payload.len(),
        })
    }
}

impl From<Message> for nl_packet::NetlinkPayload<Message> {
    fn from(msg: Message) -> Self {
        nl_packet::NetlinkPayload::InnerMessage(msg)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PacketMessage<T : deserialize::Deserialize<T> + packet::Serialize> {
    pub packet: T
}

impl<T> nl_packet::NetlinkSerializable<PacketMessage<T>> for PacketMessage<T> where T : deserialize::Deserialize<T> + packet::Serialize  {
    fn buffer_len(&self) -> usize {
        8192
    }
    fn message_type(&self) -> u16 {
        NETLINK_PROTOCOL as u16
    }
    fn serialize(&self, buffer: &mut [u8]) {
        packet::serialize(&self.packet, buffer);
    }
}

impl<T> nl_packet::NetlinkDeserializable<PacketMessage<T>> for PacketMessage<T> where T : deserialize::Deserialize<T> + packet::Serialize {
    type Error = nl_packet::DecodeError;
    fn deserialize(header: &nl_packet::NetlinkHeader, payload: &[u8]) -> Result<PacketMessage<T>, Self::Error> {
        match deserialize::<T>(payload) {
            Ok(packet) => Ok(PacketMessage {
                packet: packet
            }),
            Err(err) => Err(nl_packet::DecodeError::from("Failed"))
        }
    }
}

pub fn main() {

    let args:Vec<String> = std::env::args().collect();

    println!("{:?}", args);
    return;

    

    let mut buffer: [u8; 1024] = [0; 1024];
    let rules = packets::FilterRule {
        source_ip: 0,
        source_mask: 0,
        dest_ip: net::Ipv4Addr::new(127, 0, 0, 1).into(),
        dest_mask: net::Ipv4Addr::new(255, 255, 255, 255).into(),
        source_port: 0,
        dest_port: 0,
        protocol: 0
    };
    let size = packet::serialize(&rules, &mut buffer);
    let buffer = &buffer[..size];

    let msg = Message::new(buffer);

    let mut netlink_msg = nl_packet::NetlinkMessage::from(msg);
    netlink_msg.finalize();
    let addr = SocketAddr::new(0, 0);

    let mut socket = Socket::new(NETLINK_PROTOCOL as isize).unwrap();
    let mut buf = vec![0; 65536];
    netlink_msg.serialize(&mut buf);

    socket.send_to(&buf[..netlink_msg.buffer_len()], &addr, 0).unwrap();

    

    
    loop {
        let (size, addr) = socket.recv_from(&mut buf, 0).unwrap();
        {
            let mut netlink_buf = nl_packet::NetlinkBuffer::new(&buf[..size]);
        }

        let parsed = nl_packet::NetlinkMessage::<Message>::deserialize(&buf[..size]).unwrap();
        if let nl_packet::NetlinkPayload::InnerMessage(msg) = parsed.payload {
            let packet = deserialize::<packets::CapturedPacket>(&msg.data).unwrap();
            if packet.protocol == protocol::ip_protocol::TCP {
                let src = net::Ipv4Addr::from(packet.source_ip.to_be()).to_string();
                let dst = net::Ipv4Addr::from(packet.dest_ip.to_be()).to_string();
                println!("TCP {}:{} -> {}:{}", src, packet.source_port.to_be(), dst, packet.dest_port.to_be());
            }
            else if packet.protocol == protocol::ip_protocol::UDP {
                // println!("UDP {}:{} -> {}:{}", packet.source_ip, packet.source_port, packet.dest_ip.to_be(), packet.dest_port.to_be());
            }



            // println!("Receive {}", std::str::from_utf8(&msg.data).unwrap());
        }

    }
    
}
