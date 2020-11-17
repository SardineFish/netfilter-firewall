#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate packet;

mod kernel_bindings;
use kernel_bindings::bindings;
use kernel_bindings::net;
use kernel_bindings::netfilter;
use kernel_bindings::netlink;
use kernel_bindings::printk;

use alloc::*;
use packet::{deserialize, packets, serialize};

static mut NETFILTER_HOOK: Option<netfilter::NetfilterHook> = None;
static mut NETLINK_SOCKET: Option<netlink::NetLinkSock> = None;
static mut CONNECTED_CLIENT_PORT: u32 = 0;
static mut PACKET_FILTER_RULE: Option<packet::packets::FilterRule> = None;
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
            NETLINK_SOCKET = kernel_bindings::netlink::NetLinkBuilder::new()
                .unit(NETLINK_PROTOCOL)
                .callback(msg_callback)
                .create();

            NETFILTER_HOOK = Some(netfilter::NetfilterHook::new());
            if let Some(hook) = &mut NETFILTER_HOOK {
                hook
                    .pre_filter(packet_filter)
                    .hook_func(packet_callback)
                    .hook(netfilter::HookPoint::PreRouting)
                    .register();
            }
        }
        match NETLINK_SOCKET {
            None => println!("Failed to create netlink socket."),
            _ => (),
        };
        // extern_code();
    }

    return 0;
}

fn packet_callback(packet: net::IPv4Packet) -> netfilter::HookResponse {
    let mut captured: packet::packets::CapturedPacket = Default::default();

    captured.protocol = packet.header.protocol;
    captured.source_ip = packet.header.saddr;
    captured.dest_ip = packet.header.daddr;

    if (packet.header.protocol == bindings::IpProtocol_TCP as u8) {
        if let Some(tcp) = net::TcpPacket::from_lower(&packet) {
            let tcp = tcp as net::TcpPacket;
            // println!("TCP {} -> {}", tcp.header.source.to_be(), tcp.header.dest.to_be());
            captured.source_port = tcp.header.source;
            captured.dest_port = tcp.header.dest;
            captured.payload = tcp.payload.to_vec();
        }
    } else if (packet.header.protocol == bindings::IpProtocol_UDP as u8) {
        if let Some(udp) = net::UdpPacket::from_lower(&packet) {
            // let udp = udp as net::UdpPacket;
            // println!("UDP {} -> {}", udp.header.source, udp.header.dest);

            captured.source_port = udp.header.source;
            captured.dest_port = udp.header.dest;
            captured.payload = udp.payload.to_vec();
        }
    }

    let mut buf = vec![0; captured.total_size() + 64];
    let size = packet::serialize(&captured, &mut buf);
    // println!("serialize {} bytes with total size {}.", size, packet.header.tot_len.to_be());

    let slice = &buf[..size];
    let mut portid = 0;

    unsafe {
        portid = CONNECTED_CLIENT_PORT;
        if let Some(socket) = &NETLINK_SOCKET {
            if portid != 0 {
                let msg = netlink::NetlinkMsgRaw::new(slice);
                socket.send(portid, &msg);
                println!("sent");
            }
        }
    }
    return netfilter::HookResponse::Accept;
}

fn msg_callback(msg_recieved: &kernel_bindings::netlink::NetlinkMsgRaw) {
    println!(
        "received netlink packet from {} with {}bytes payload.",
        msg_recieved.addr.pid,
        msg_recieved.payload.len()
    );

    unsafe {

        if let Some(hook) = &mut NETFILTER_HOOK {
            if let Ok(rule) = deserialize::<packets::FilterRule>(msg_recieved.payload) {
                CONNECTED_CLIENT_PORT = msg_recieved.addr.pid;

                println!("Received filter rules from {}", msg_recieved.addr.pid);
                
                if rule.protocol == 0 && hook.active {
                    hook.unregister();
                    println!("Stop capture.");
                }
                else if rule.protocol != 0 && !hook.active {
                    hook.register();
                    println!("Start capture.");
                }

                PACKET_FILTER_RULE = Some(rule);
            }
            else {
                println!("Received non-rules packet from {}, stop capture", msg_recieved.addr.pid);
            }
        
        }

    }

    // unsafe {
    //     CONNECTED_CLIENT_PORT = msg_recieved.addr.pid;

    //     if let Some(socket) = &NETLINK_SOCKET {
    //         let payload = alloc::format!(
    //             "received netlink packet from {} with {}bytes payload.",
    //             msg_recieved.addr.pid,
    //             msg_recieved.payload.len()
    //         );
    //         let msg = netlink::NetlinkMsgRaw::new(payload.as_bytes());
    //         // socket.send(msg_recieved.addr.pid, &msg);
    //     }
    // }
}

fn packet_filter(ip_header: &net::IPv4Header, sk_buff: &kernel_bindings::bindings::sk_buff,) -> bool {
    let mut rule: &Option<packet::packets::FilterRule>;
    unsafe {
        rule = &PACKET_FILTER_RULE;
    }
    if let Some(rule) = rule {
        let src_addr_matched =
            (ip_header.saddr.to_be() & rule.source_mask) == (rule.source_ip & rule.source_mask);
        let dest_addr_matched =
            (ip_header.daddr.to_be() & rule.dest_mask) == (rule.dest_ip & rule.dest_mask);
        if !src_addr_matched || !dest_addr_matched {
            return false;
        }

        if rule.protocol != 255 && rule.protocol != ip_header.protocol {
            return false;
        }

        let (src_port, dest_port) = match ip_header.protocol {
            net::ip_protocol::UDP => match net::UdpHeader::from_skbuff(sk_buff) {
                Some(header) => (header.source.to_be(), header.dest.to_be()),
                None => (0, 0),
            },
            net::ip_protocol::TCP => match net::TcpHeader::from_skbuff(sk_buff) {
                Some(header) => (header.source.to_be(), header.dest.to_be()),
                None => (0, 0),
            },
            _ => (0, 0),
        };

        let src_port_mismatched = src_port != 0 && rule.source_port != 0 && rule.source_port != src_port;
        let dest_port_mismatched = dest_port != 0 && rule.dest_port != 0 && rule.dest_port != dest_port;

        if src_port_mismatched ||dest_port_mismatched {
            return false;
        }

        return true;
    } else {
        return false;
    }
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
        match &NETLINK_SOCKET {
            Some(s) => s.release(),
            _ => (),
        };

        match &mut NETFILTER_HOOK {
            Some(h) => {
                h.unregister();
            }
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

#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    println!(
        "Alloc Error, size={}, align={}",
        _layout.size(),
        _layout.align()
    );

    loop {}
}

#[global_allocator]
static KERNEL_ALLOC: kernel_bindings::memory::KernelAlloc = kernel_bindings::memory::KernelAlloc {};
