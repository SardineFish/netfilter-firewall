use crate::kernel_bindings::bindings;
use crate::kernel_bindings::net;
use crate::kernel_bindings::c_types;

#[no_mangle]
extern "C" fn netfilter_hook_func(
        priv_: *mut c_types::c_void,
        skb: *mut bindings::sk_buff,
        state: *const bindings::nf_hook_state,
    ) -> c_types::c_uint {
        unsafe {
            match hook_func_rust {
                Some(func) => {
                    let header = bindings::ip_hdr_wrapped(skb);
                    use crate::println;
                    println!("header at {}", (*header).tot_len);
                    // let packet = net::IPv4Packet::from(header);
                    // func(packet) as c_types::c_uint
                    HookResponse::Accept as c_types::c_uint
                },
                _=> HookResponse::Accept as c_types::c_uint
            }
        }
    }

pub type HookFuncType = fn(packet: net::IPv4Packet) -> HookResponse;

static mut hook_func_rust: Option<HookFuncType> = None;

// pub fn register_hook() {
//     unsafe {
//         bindings::nf_register_net_hook(&net::init_net);
//     }
// }

pub struct NetfilterHook {
    hook_ops: bindings::nf_hook_ops,
}

#[allow(dead_code)]
pub mod protocol_family {
    pub const UNSPEC : u8 = 0;
    pub const LOCAL : u8 = 1;
    pub const UNIX : u8 = LOCAL;
    pub const FILE : u8 = LOCAL;
    pub const INET : u8 = 2;
    pub const AX25 : u8 = 3;
    pub const IPX : u8 = 4;
    pub const APPLETALK : u8 = 5;
    pub const NETROM : u8 = 6;
    pub const BRIDGE : u8 = 7;
    pub const ATMPVC : u8 = 8;
    pub const X25 : u8 = 9;
    pub const INET6 : u8 = 10;
    pub const ROSE : u8 = 11;
    pub const DECnet : u8 = 12;
    pub const NETBEUI : u8 = 13;
    pub const SECURITY : u8 = 14;
    pub const KEY : u8 = 15;
    pub const NETLINK : u8 = 1;
    pub const ROUTE : u8 = NETLINK;
    pub const PACKET : u8 = 17;
    pub const ASH : u8 = 18;
    pub const ECONET : u8 = 19;
    pub const ATMSVC : u8 = 20;
    pub const RDS : u8 = 21;
    pub const SNA : u8 = 22;
    pub const IRDA : u8 = 23;
    pub const PPPOX : u8 = 24;
    pub const WANPIPE : u8 = 25;
    pub const LLC : u8 = 26;
    pub const IB : u8 = 27;
    pub const MPLS : u8 = 28;
    pub const CAN : u8 = 29;
    pub const TIPC : u8 = 30;
    pub const BLUETOOTH : u8 = 31;
    pub const IUCV : u8 = 32;
    pub const RXRPC : u8 = 33;
    pub const ISDN : u8 = 34;
    pub const PHONET : u8 = 35;
    pub const IEEE802154 : u8 = 36;
    pub const CAIF : u8 = 37;
    pub const ALG : u8 = 38;
    pub const NFC : u8 = 39;
    pub const VSOCK : u8 = 40;
    pub const KCM : u8 = 41;
    pub const QIPCRTR : u8 = 42;
    pub const SMC : u8 = 43;
    pub const XDP : u8 = 44;
    pub const MAX : u8 = 45;
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

pub enum Privilege {

}

impl NetfilterHook {
    pub fn new() -> NetfilterHook {
        NetfilterHook {
            hook_ops: bindings::nf_hook_ops{
                hook: Some(netfilter_hook_func),
                hooknum: HookPoint::PostRouting as u32,
                pf: protocol_family::INET,
                priority: bindings::nf_ip_hook_priorities_NF_IP_PRI_FIRST,
                ..Default::default()
            }
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
    pub fn hook_func(&mut self, func: HookFuncType) -> &mut Self{
        unsafe {
            hook_func_rust = Some(func);
            self
        }
    }
    pub fn register(&mut self) -> &mut Self {
        unsafe {
            self.hook_ops.hook = Some(netfilter_hook_func);
            bindings::nf_register_net_hook(&mut net::init_net, &self.hook_ops as *const bindings::nf_hook_ops);
            self
        }
    }
    pub fn unregister(&self) {
        unsafe {
            bindings::nf_unregister_net_hook(&mut net::init_net, &self.hook_ops as *const bindings::nf_hook_ops);
        }
    }
}
