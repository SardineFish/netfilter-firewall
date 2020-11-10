use crate::kernel_bindings::bindings;
use crate::kernel_bindings::c_types;
use crate::kernel_bindings::net;

#[no_mangle]
extern "C" fn netfilter_hook_func(
    priv_: *mut c_types::c_void,
    skb: *mut bindings::sk_buff,
    state: *const bindings::nf_hook_state,
) -> c_types::c_uint {
    unsafe {
        match HOOK_FUNC_RUST {
            Some(func) => {
                let header = bindings::ip_hdr_wrapped(skb);
                if header == core::ptr::null_mut() {
                    return HookResponse::Accept as c_types::c_uint;
                }

                if let Some(pre_filter) = PRE_FILTER_FUNC_RUST {
                    match pre_filter(header.as_ref().unwrap(), skb.as_ref().unwrap()) {
                        false => {return HookResponse::Accept as c_types::c_uint;},
                        _ => ()
                    }
                }

                let total_len = (*header).tot_len.to_be() as usize;
                let mut buffer: alloc::vec::Vec<u8> = alloc::vec![0; total_len];
                let result = bindings::skb_copy_bits(
                    skb,
                    0,
                    buffer.as_mut_ptr() as *mut c_types::c_void,
                    total_len as i32,
                );

                let mut packet = net::IPv4Packet::from_buf(&buffer);

                let packet = packet;

                func(packet) as c_types::c_uint
            }
            _ => HookResponse::Accept as c_types::c_uint,
        }
    }
}

pub type HookFuncType = fn(packet: net::IPv4Packet) -> HookResponse;
pub type PreFilterFuncType = fn(header: &net::IPv4Header, skb: &bindings::sk_buff) -> bool;

static mut HOOK_FUNC_RUST: Option<HookFuncType> = None;
static mut PRE_FILTER_FUNC_RUST: Option<PreFilterFuncType> = None;

pub struct NetfilterHook {
    hook_ops: bindings::nf_hook_ops,
}

#[allow(dead_code)]
pub enum HookResponse {
    Drop = 0,
    Accept = 1,
    Stolen = 2,
    Queue = 3,
    Repeat = 4,
    Stop = 5,
}

#[allow(dead_code)]
pub enum HookPoint {
    PreRouting = bindings::nf_inet_hooks_NF_INET_PRE_ROUTING as isize,
    LocalIn = bindings::nf_inet_hooks_NF_INET_LOCAL_IN as isize,
    Forward = bindings::nf_inet_hooks_NF_INET_FORWARD as isize,
    LocalOut = bindings::nf_inet_hooks_NF_INET_LOCAL_OUT as isize,
    PostRouting = bindings::nf_inet_hooks_NF_INET_POST_ROUTING as isize,
}

pub enum Privilege {}

impl NetfilterHook {
    pub fn new() -> NetfilterHook {
        NetfilterHook {
            hook_ops: bindings::nf_hook_ops {
                hook: Some(netfilter_hook_func),
                hooknum: HookPoint::PostRouting as u32,
                pf: net::protocol_family::INET,
                priority: bindings::nf_ip_hook_priorities_NF_IP_PRI_FIRST,
                ..Default::default()
            },
        }
    }
    pub fn protocol_family(&mut self, pf: u8) -> &mut Self {
        self.hook_ops.pf = pf;
        self
    }
    pub fn hook(&mut self, hook: HookPoint) -> &mut Self {
        self.hook_ops.hooknum = hook as u32;
        self
    }
    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.hook_ops.priority = priority;
        self
    }
    pub fn hook_func(&mut self, func: HookFuncType) -> &mut Self {
        unsafe {
            HOOK_FUNC_RUST = Some(func);
            self
        }
    }
    pub fn pre_filter(&mut self, func: PreFilterFuncType) -> &mut Self {
        unsafe {
            PRE_FILTER_FUNC_RUST = Some(func);
            self
        }
    }
    pub fn register(&mut self) -> &mut Self {
        unsafe {
            self.hook_ops.hook = Some(netfilter_hook_func);
            bindings::nf_register_net_hook(
                &mut net::init_net,
                &self.hook_ops as *const bindings::nf_hook_ops,
            );
            self
        }
    }
    pub fn unregister(&self) {
        unsafe {
            bindings::nf_unregister_net_hook(
                &mut net::init_net,
                &self.hook_ops as *const bindings::nf_hook_ops,
            );
        }
    }
}
