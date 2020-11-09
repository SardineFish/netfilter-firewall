use crate::kernel_bindings::bindings;
use crate::kernel_bindings::memory;

pub struct NetLinkAddr {
    pub pid: u32,
    pub group: u32,
}

impl Default for NetLinkAddr {
    fn default() -> Self {
        NetLinkAddr { group: 0, pid: 0 }
    }
}

pub type NetLinkHeader = bindings::nlmsghdr;

pub struct NetlinkMsgRaw<'a> {
    pub addr: NetLinkAddr,
    pub header: NetLinkHeader,
    pub payload: &'a [u8],
}

impl<'a> NetlinkMsgRaw<'a> {
    pub fn new(payload: &'a [u8]) -> Self {
        NetlinkMsgRaw::<'a> {
            addr: Default::default(),
            header: Default::default(),
            payload: payload,
        }
    }
    pub unsafe fn from_sk_buf(skbuf: *mut bindings::sk_buff) -> Self {
        let header = (*skbuf).data as *mut bindings::nlmsghdr;
        let skb_params = bindings::netlink_cb(skbuf);
        let data_ptr = bindings::nlmsg_data_non_inline(header) as *mut u8;
        let size = ((*header).nlmsg_len as usize - core::mem::size_of::<bindings::nlmsghdr>());
        NetlinkMsgRaw {
            header: *header,
            addr: NetLinkAddr {
                pid: skb_params.portid,
                group: skb_params.dst_group,
            },
            payload: core::slice::from_raw_parts(data_ptr, size),
        }
    }
    pub unsafe fn to_sk_buf(&self) -> Option<*mut bindings::sk_buff> {
        let sk_buff = bindings::nlmsg_new_non_inline(self.payload.len(), bindings::GFP_KERNEL);
        if sk_buff == core::ptr::null_mut() {
            return None;
        }
        let header_ptr = bindings::nlmsg_put_wrapped(
            sk_buff,
            self.header.nlmsg_pid,
            self.header.nlmsg_seq,
            self.header.nlmsg_type,
            self.payload.len() as u32,
            self.header.nlmsg_flags,
        );
        let ptr = bindings::nlmsg_data_non_inline(header_ptr);
        let slice = core::slice::from_raw_parts_mut(ptr as *mut u8, self.payload.len());
        slice.copy_from_slice(self.payload);
        Some(sk_buff)
    }
}