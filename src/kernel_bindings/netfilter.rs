use crate::kernel_bindings::bindings;
use crate::kernel_bindings::c_types;
use crate::kernel_bindings::net;

#[no_mangle]
extern "C" fn netfilter_hook_input(
    priv_: *mut c_types::c_void,
    skb: *mut bindings::sk_buff,
    state: *const bindings::nf_hook_state,
) -> c_types::c_uint {
    unsafe {
        if let Some(func) = HOOK_FUNC_INPUT {
            let header = bindings::ip_hdr_wrapped(skb);
            if header == core::ptr::null_mut() || skb == core::ptr::null_mut() {
                return HookResponse::Accept as c_types::c_uint;
            }

            func(header.as_ref().unwrap(), skb.as_ref().unwrap()) as c_types::c_uint
        } else {
            HookResponse::Accept as c_types::c_uint
        }
    }
}

#[no_mangle]
extern "C" fn netfilter_hook_output(
    priv_: *mut c_types::c_void,
    skb: *mut bindings::sk_buff,
    state: *const bindings::nf_hook_state,
) -> c_types::c_uint {
    unsafe {
        if let Some(func) = HOOK_FUNC_OUTPUT {
            let header = bindings::ip_hdr_wrapped(skb);
            if header == core::ptr::null_mut() || skb == core::ptr::null_mut() {
                return HookResponse::Accept as c_types::c_uint;
            }

            func(header.as_ref().unwrap(), skb.as_ref().unwrap()) as c_types::c_uint
        } else {
            HookResponse::Accept as c_types::c_uint
        }
    }
}

pub type HookFuncType = fn(ip_header: &net::IPv4Header, skb: &bindings::sk_buff) -> HookResponse;

static mut HOOK_FUNC_INPUT: Option<HookFuncType> = None;
static mut HOOK_FUNC_OUTPUT: Option<HookFuncType> = None;

pub struct NetfilterHook {
    input_hook: bindings::nf_hook_ops,
    output_hook: bindings::nf_hook_ops,
    pub active: bool,
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
            input_hook: bindings::nf_hook_ops {
                hook: Some(netfilter_hook_input),
                hooknum: HookPoint::PreRouting as u32,
                pf: net::protocol_family::INET,
                priority: bindings::nf_ip_hook_priorities_NF_IP_PRI_FIRST,
                ..Default::default()
            },
            output_hook: bindings::nf_hook_ops {
                hook: Some(netfilter_hook_output),
                hooknum: HookPoint::PostRouting as u32,
                pf: net::protocol_family::INET,
                priority: bindings::nf_ip_hook_priorities_NF_IP_PRI_FIRST,
                ..Default::default()
            },
            active: false,
        }
    }
    pub fn protocol_family(&mut self, pf: u8) -> &mut Self {
        self.input_hook.pf = pf;
        self.output_hook.pf = pf;
        self
    }
    pub fn hook_input(&mut self, hook: HookPoint) -> &mut Self {
        self.input_hook.hooknum = hook as u32;
        self
    }
    pub fn hook_output(&mut self, hook: HookPoint) -> &mut Self {
        self.output_hook.hooknum = hook as u32;
        self
    }
    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.input_hook.priority = priority;
        self.output_hook.priority = priority;
        self
    }
    pub fn hook_input_func(&mut self, func: HookFuncType) -> &mut Self {
        unsafe {
            HOOK_FUNC_INPUT = Some(func);
        }
        self
    }
    pub fn hook_output_func(&mut self, func: HookFuncType) -> &mut Self {
        unsafe {
            HOOK_FUNC_OUTPUT = Some(func);
        }
        self
    }
    pub fn register(&mut self) -> &mut Self {
        if self.active {
            return self;
        }
        unsafe {
            bindings::nf_register_net_hook(
                &mut net::init_net,
                &self.input_hook as *const bindings::nf_hook_ops,
            );
            bindings::nf_register_net_hook(
                &mut net::init_net,
                &self.output_hook as *const bindings::nf_hook_ops,
            );
            self.active = true;
            self
        }
    }
    pub fn unregister(&mut self) -> &mut Self {
        if !self.active {
            return self;
        }
        unsafe {
            bindings::nf_unregister_net_hook(
                &mut net::init_net,
                &self.input_hook as *const bindings::nf_hook_ops,
            );
            bindings::nf_unregister_net_hook(
                &mut net::init_net,
                &self.output_hook as *const bindings::nf_hook_ops,
            );
        }
        self.active = false;
        self
    }
}
