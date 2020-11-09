#![no_std]
#![feature(panic_info_message)]

mod kernel_bindings;
use kernel_bindings::bindings;
use kernel_bindings::netlink;
use kernel_bindings::printk;
use kernel_bindings::net;
use kernel_bindings::netfilter;

static mut NETFILTER_HOOK: Option<netfilter::NetfilterHook> = None;
static mut socket: Option<netlink::NetLinkSock> = None;
const NETLINK_PROTOCOL: i32 = 17;

fn init() -> i32 {
    // printk::printk("init module!\n");
    println!("init module!");
    println!("1 + 1 = {}", 1 + 1);

    unsafe {
        let mut cfg = kernel_bindings::bindings::netlink_kernel_cfg {
            input: Some(input),
            ..Default::default()
        };
        // kernel_bindings::netlink::netlink_kernel_create(&mut netlink::init_net, 17, &mut cfg);
        unsafe {
            socket = kernel_bindings::netlink::NetLinkBuilder::new()
                .unit(NETLINK_PROTOCOL)
                .callback(msg_callback)
                .create();

            NETFILTER_HOOK = Some(netfilter::NetfilterHook::new());
            if let Some(hook) = &mut NETFILTER_HOOK {
                hook.hook_func(packet_callback)
                    .hook(netfilter::HookPoint::PreRouting)
                    .register();
            }

            // hook = Some(netfilter::NetfilterHook::new()
            //     .hook_func(packet_callback)
            //     .register());
        }
        match socket {
            None => println!("Failed to create netlink socket."),
            _ => (),
        };
        // extern_code();
    }

    return 0;
}

fn packet_callback(packet: net::IPv4Packet) -> netfilter::HookResponse {

    if(packet.header.protocol == bindings::IpProtocol_TCP as u8) {
        if let Some(tcp) = net::TcpPacket::from(&packet) {
            let tcp = tcp as net::TcpPacket;
            println!("TCP {} -> {}", tcp.header.source, tcp.header.dest);
        }
        // println!("TCP {} -> {} with total size {}", packet.header.saddr, packet.header.daddr, packet.header.tot_len);
    }
    else if(packet.header.protocol == bindings::IpProtocol_UDP as u8) {
        if let Some(udp) = net::UdpPacket::from(&packet) {
            // let udp = udp as net::UdpPacket;
            println!("UDP {} -> {}", udp.header.source, udp.header.dest);
        }
        // println!("UDP {} -> {} with total size {}", packet.header.saddr, packet.header.daddr, packet.header.tot_len);
    }

    return netfilter::HookResponse::Accept;
}

fn msg_callback(msg: &kernel_bindings::netlink::NetLinkMessge) {
    // kernel_bindings::printk(b"Received netlink packet %d", msg.data.len());
    println!("received netlink packet {}", msg.data.len());
}

extern "C" fn input(buf: *mut bindings::sk_buff) {
    unsafe {
        let header: *mut bindings::nlmsghdr = (*buf).data as *mut bindings::nlmsghdr;
    }
}

extern "C" {
    pub fn extern_code();
    pub fn extern_cleanup();
}

fn exit() {
    println!("exit module");
    unsafe {
        // extern_cleanup();
        match &socket {
            Some(s) => s.release(),
            _ => (),
        };
        
        match &NETFILTER_HOOK {
            Some(h) => {h.unregister();},
            _ => (),
        }
    }
}

module_init!(init);
module_exit!(exit);

module_license!("GPL-3");
module_author!("SardineFish");
module_description!("A kernel module written in rust.");
module_version!("0.0.1");

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    println!("error");
    let mut writer = printk::LogWriter::new();
    core::fmt::write(&mut writer, *_info.message().unwrap());
    printk::printk(writer.to_str());
    printk::printk("\n");
    loop {}
}
