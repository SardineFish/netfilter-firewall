extern crate netlink_sys;
extern crate netlink_packet_core;

use netlink_sys::{Socket,SocketAddr};
use netlink_packet_core as packet;

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

impl packet::NetlinkSerializable<Message> for Message {
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

impl packet::NetlinkDeserializable<Message> for Message {
    fn deserialize(header: &packet::NetlinkHeader, payload: &[u8]) -> Result<Message, Self::Error> {
        return Ok(Message {
            size: 0,
            data: vec![],
        });
    }
    type Error = packet::DecodeError;
}

impl From<Message> for packet::NetlinkPayload<Message> {
    fn from(msg: Message) -> Self {
        packet::NetlinkPayload::InnerMessage(msg)
    }
}
pub fn main() {
    // let addr = SocketAddr::new(5325, 1);
    // let socket = Socket::new(16)
    //     .expect("Failed to create netlink socket on protocol 16");
    // socket
    //     .connect(&addr)
    //     .expect("Failed to connect to remote socket.");

    let msg = Message::new("Hellow kernel!".as_bytes());

    let mut netlink_msg = packet::NetlinkMessage::from(msg);
    netlink_msg.finalize();
    let addr = SocketAddr::new(0, 0);

    let mut socket = Socket::new(NETLINK_PROTOCOL as isize).unwrap();
    let mut buf = vec![0; 8192];
    netlink_msg.serialize(&mut buf);

    socket.send_to(&buf[..netlink_msg.buffer_len()], &addr, 0).unwrap();
}
