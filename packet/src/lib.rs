// #![no_std]
#![cfg_attr(feature = "no_std", no_std)]
// #![feature(lang_items)]

pub mod serialize;
pub mod deserialize;
pub mod packets;

#[cfg(test)]
mod test;

pub use serialize::{ serialize, Serialize };
pub use deserialize::{ Deserialize, deserialize };

// extern crate serde;

// use serde::{ Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct NetlinkHeader {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

pub struct NetlinkMessage<T> {
    pub header: NetlinkHeader,
    pub payload: T,
}
impl<T> NetlinkMessage<T> where T : Serialize {
    pub fn pack(&self, buffer: &mut [u8]) -> usize {
        serialize(&self.payload, buffer)
    } 
}
impl<T> NetlinkMessage<T> where T : Deserialize<T> {
    pub fn from(header: NetlinkHeader, payload_buffer: &[u8]) -> Self {
        NetlinkMessage {
            header: header,
            payload: deserialize::<T>(payload_buffer).unwrap()
        }
    }
}