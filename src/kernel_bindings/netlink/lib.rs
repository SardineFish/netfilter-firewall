use crate::kernel_bindings::bindings;
use crate::kernel_bindings::module;
use super::extern_bindings;
use super::msg::{ NetLinkMessge, NetLinkAddr, NetLinkHeader };
use crate::kernel_bindings::memory;
use super::super::printk;

pub struct NetLinkBuilder{
    callback: Option<fn(msg: &NetLinkMessge)>,
    unit: i32,
    cfg: bindings::netlink_kernel_cfg,
}

pub struct NetLinkSock {
    sock: *mut bindings::sock
}

impl NetLinkSock {
    fn new(socket: *mut bindings::sock) -> NetLinkSock {
        NetLinkSock {
            sock: socket 
        }
    }
    pub fn release(&self) {
        unsafe {
            bindings::netlink_kernel_release(self.sock);
        }
    }
}

static mut RUST_INPUT_CALLBACK: Option<fn(msg: &NetLinkMessge)> = None;

impl NetLinkBuilder {
    pub fn new() -> NetLinkBuilder{
        Self {
            callback: None,
            cfg: Default::default(),
            unit: 0,
        }
    }
    pub fn unit(mut self, unit:i32) -> NetLinkBuilder {
        self.unit = unit;
        self
    }
    pub fn callback(mut self, callback: fn(msg:&NetLinkMessge)) -> NetLinkBuilder {
        self.callback = Some(callback);
        self
    }
    pub fn create(mut self) -> Option<NetLinkSock> {
        unsafe {
            RUST_INPUT_CALLBACK = self.callback;
            self.cfg.input = Some(input_callback);
            let socket = extern_bindings::netlink_kernel_create(&mut extern_bindings::init_net, self.unit, &mut self.cfg);

            if socket == core::ptr::null_mut() {
                return None;
            }
            return Some(NetLinkSock::new(socket));
        }
    }
}

extern "C" fn input_callback(skbuf: *mut bindings::sk_buff) {
    unsafe {
        printk::printk("Receive packet.\n\0");
        if let Some(callback) = RUST_INPUT_CALLBACK {
            let header = (*skbuf).data as *mut bindings::nlmsghdr;
            let skb_params = bindings::netlink_cb(skbuf);

            let data = bindings::nlmsg_data_non_inline(header) as *mut u8;
            let msg = NetLinkMessge {
                addr: NetLinkAddr {
                    pid: skb_params.portid,
                    group: skb_params.dst_group,
                },
                header: *header,
                data: memory::RawData::from_raw(data, ((*header).nlmsg_len as usize - core::mem::size_of::<bindings::nlmsghdr>())),
            };
            
            callback(&msg);
        }
    }
}