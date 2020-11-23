use netlink_sys;
use netlink_packet_core as nl_packet;
use packet::{deserialize, packets};
use std::net;

pub const NETLINK_PROTOCOL: i32 = 17;

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

pub struct Socket {
    socket: netlink_sys::Socket
}

impl Socket {
    pub fn new(protocol: isize) -> Self {
        let socket = netlink_sys::Socket::new(protocol).unwrap();
        Socket {
            socket: socket
        }
    }
    pub fn send<T: packet::Serialize>(&self, portid: u32, groupid: u32, payload: T) {

        let mut serialized_payload = vec![0; 1024];
        let size = packet::serialize(&payload, &mut serialized_payload);
        let mut msg = nl_packet::NetlinkMessage::from(Message::new(&serialized_payload[..size]));
        msg.finalize();

        let mut buf = vec![0; 1024];
        msg.serialize(&mut buf);

        let addr = netlink_sys::SocketAddr::new(portid, groupid);

        self.socket.send_to(&buf[..msg.buffer_len()], &addr, 0).unwrap();
    }
    pub fn recv<T: packet::Deserialize<T>>(&self) -> T {
        let mut buf = vec![0; 65536];

        let (size, addr) = self.socket.recv_from(&mut buf, 0).unwrap();
        {
            let mut netlink_buf = nl_packet::NetlinkBuffer::new(&buf[..size]);
        }

        let parsed = nl_packet::NetlinkMessage::<Message>::deserialize(&buf[..size]).unwrap();
        if let nl_packet::NetlinkPayload::InnerMessage(msg) = parsed.payload {
            let packet = deserialize::<T>(&msg.data).unwrap();
            return packet;
        }
        panic!("Receive invalid packet.");
    }
}