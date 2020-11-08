#![no_std]

mod serialize;
mod deserialize;

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

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
