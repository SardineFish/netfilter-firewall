use crate::kernel_bindings::bindings;
use crate::kernel_bindings::memory;


pub struct NetLinkAddr {
    pub pid: u32,
    pub group: u32,
}

impl Default for NetLinkAddr {
    fn default() -> Self {
        NetLinkAddr {
            group: 0,
            pid: 0,
        }
    }
}

pub type NetLinkHeader = bindings::nlmsghdr;

pub struct NetLinkMessge {
    pub addr: NetLinkAddr,
    pub header: NetLinkHeader,
    pub data: memory::RawData,
}

impl NetLinkMessge {
    pub fn new(data_size: usize) -> NetLinkMessge {
        NetLinkMessge {
            header: Default::default(),
            addr: Default::default(),
            data: memory::RawData::default(),
        }
    }
}