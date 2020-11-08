// #![no_std]
#![cfg_attr(feature = "no_std", no_std)]
// #![feature(lang_items)]

pub mod serialize;
pub mod deserialize;

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

#[cfg(feature = "no_std")]
mod panic_handler {

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

}

// #[cfg(feature = "default")]
// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}
// #[lang = "eh_personality"]
// fn eh_personality() {}