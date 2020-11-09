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
        if let Some(tcp) = net::TcpPacket::from(&packet) {
            let tcp = tcp as net::TcpPacket;
            println!("TCP {} -> {}", tcp.header.source, tcp.header.dest);
            captured.source_port = tcp.header.source;
            captured.dest_port = tcp.header.dest;
            captured.payload = tcp.payload.to_vec();
        }
    } else if (packet.header.protocol == bindings::IpProtocol_UDP as u8) {
        if let Some(udp) = net::UdpPacket::from(&packet) {
            // let udp = udp as net::UdpPacket;
            println!("UDP {} -> {}", udp.header.source, udp.header.dest);

            captured.source_port = udp.header.source;
            captured.dest_port = udp.header.dest;
            captured.payload = udp.payload.to_vec();
        }
    }

    let mut buf = [0; 65536];
    let size = packet::serialize(&captured, &mut buf);
    let slice = &buf[..size];
    let mut portid = 0;

    unsafe {
        portid = CONNECTED_CLIENT_PORT;
        if let Some(socket) = &NETLINK_SOCKET {
            if portid != 0 {
                let msg = netlink::NetlinkMsgRaw::new(slice);
                socket.send(portid, &msg);
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

        CONNECTED_CLIENT_PORT = msg_recieved.addr.pid;

        if let Some(socket) = &NETLINK_SOCKET {
            let payload = alloc::format!(
                "received netlink packet from {} with {}bytes payload.",
                msg_recieved.addr.pid,
                msg_recieved.payload.len()
            );
            let msg = netlink::NetlinkMsgRaw::new(payload.as_bytes());
            // socket.send(msg_recieved.addr.pid, &msg);
        }
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

        match &NETFILTER_HOOK {
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
