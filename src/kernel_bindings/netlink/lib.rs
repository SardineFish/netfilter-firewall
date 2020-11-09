use super::super::printk;
use super::extern_bindings;
use super::msg::{NetLinkAddr, NetLinkHeader, NetlinkMsgRaw};
use crate::kernel_bindings::bindings;
use crate::kernel_bindings::memory;
use crate::kernel_bindings::module;
use crate::kernel_bindings::net;

pub struct NetLinkBuilder {
    callback: Option<fn(msg: &NetlinkMsgRaw)>,
    unit: i32,
    cfg: bindings::netlink_kernel_cfg,
}

pub struct NetLinkSock {
    sock: *mut bindings::sock,
}

pub enum SocketError{
    AllocFailure,
}

impl NetLinkSock {
    fn new(socket: *mut bindings::sock) -> NetLinkSock {
        NetLinkSock { sock: socket }
    }
    pub fn release(&self) {
        unsafe {
            bindings::netlink_kernel_release(self.sock);
        }
    }
    pub fn send(&self, portid: u32, msg: &NetlinkMsgRaw) -> Result<(), SocketError> {
        unsafe {
            if let Some(sk_buff) = msg.to_sk_buf() {
                let ptr = (&(*sk_buff).cb).as_ptr() as *mut bindings::netlink_skb_parms;
                (*ptr).dst_group = 0;
                bindings::netlink_unicast(self.sock, sk_buff, portid, 1);
                Ok(())
            }
            else {
                Err(SocketError::AllocFailure)
            }
        }
    }
}

static mut RUST_INPUT_CALLBACK: Option<fn(msg: &NetlinkMsgRaw)> = None;

impl NetLinkBuilder {
    pub fn new() -> NetLinkBuilder {
        Self {
            callback: None,
            cfg: Default::default(),
            unit: 0,
        }
    }
    pub fn unit(mut self, unit: i32) -> NetLinkBuilder {
        self.unit = unit;
        self
    }
    pub fn callback(mut self, callback: fn(msg: &NetlinkMsgRaw)) -> NetLinkBuilder {
        self.callback = Some(callback);
        self
    }
    pub fn create(mut self) -> Option<NetLinkSock> {
        unsafe {
            RUST_INPUT_CALLBACK = self.callback;
            self.cfg.input = Some(input_callback);
            let socket = extern_bindings::netlink_kernel_create(
                &mut net::init_net,
                self.unit,
                &mut self.cfg,
            );

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
            let raw_msg = NetlinkMsgRaw::from_sk_buf(skbuf);

            callback(&raw_msg);
        }
    }
}
