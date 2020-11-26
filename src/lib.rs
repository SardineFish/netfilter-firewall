#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(warnings)]

extern crate alloc;
extern crate packet;

mod kernel_bindings;
mod firewall;
mod utils;


use kernel_bindings::bindings;
use kernel_bindings::net;
use kernel_bindings::netfilter;
use kernel_bindings::netlink;
use kernel_bindings::printk;

use alloc::*;
use packet::{deserialize, packets, serialize, EvalSize};

static mut NETFILTER_HOOK: Option<netfilter::NetfilterHook> = None;
static mut NETLINK_SOCKET: Option<netlink::NetLinkSock> = None;
static mut CONNECTED_CLIENT_PORT: u32 = 0;
static mut PACKET_FILTER_RULE: Option<packet::packets::FilterRule> = None;
const NETLINK_PROTOCOL: i32 = 17;

fn init() -> i32 {
    // printk::printk("init module!\n");
    println!("init module!");
    println!("1 + 1 = {}", 1 + 1);

    firewall::init_firewall();

    unsafe {
        let cfg = kernel_bindings::bindings::netlink_kernel_cfg {
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
                    .hook_input(netfilter::HookPoint::PreRouting)
                    .hook_input_func(firewall::packet_input)
                    .hook_output(netfilter::HookPoint::PostRouting)
                    .hook_output_func(firewall::packet_output)
                    .register();
                println!("Setup Netfilter hook.");
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

fn msg_callback(msg_recieved: &kernel_bindings::netlink::NetlinkMsgRaw) {
    println!(
        "received netlink packet from {} with {}bytes payload.",
        msg_recieved.addr.pid,
        msg_recieved.payload.len()
    );
    
    let socket;
    unsafe {
        socket = NETLINK_SOCKET.as_ref().unwrap();
    }

    if let Ok(msg) = deserialize::<packets::FirewallMessage>(msg_recieved.payload) {
        match msg {
            packets::FirewallMessage::SetRule(rule) => {
                firewall::add_rule(
                    rule.priority,
                    firewall::GeneralFirewallRule {
                    source: firewall::Endpoint{
                        ip: rule.source_ip,
                        mask: rule.source_mask,
                        port: rule.source_port,
                    },
                    dest: firewall::Endpoint {
                        ip: rule.dest_ip,
                        mask: rule.dest_mask,
                        port: rule.dest_port,
                    },
                    action: match rule.action {
                        packets::FirewallAction::Allow => firewall::RuleAction::Permit,
                        packets::FirewallAction::Deny => firewall::RuleAction::Drop,
                    },
                    protocol: rule.protocol,
                });
                println!("Added rule into the firewall.");
            },
            packets::FirewallMessage::SetDefault(rule) => {
                firewall::set_default(firewall::GeneralFirewallRule {
                    source: firewall::Endpoint{
                        ip: rule.source_ip,
                        mask: rule.source_mask,
                        port: rule.source_port,
                    },
                    dest: firewall::Endpoint {
                        ip: rule.dest_ip,
                        mask: rule.dest_mask,
                        port: rule.dest_port,
                    },
                    action: match rule.action {
                        packets::FirewallAction::Allow => firewall::RuleAction::Permit,
                        packets::FirewallAction::Deny => firewall::RuleAction::Drop,
                    },
                    protocol: rule.protocol,
                });
                println!("Default rule set.");
            },
            packets::FirewallMessage::DeleteRule(index) => {
                println!("Delete rule at {}", index);
                let msg;
                if let Some(rule) = firewall::delete_rule(index) {
                    msg = packets::FirewallMessage::SetRule(packets::FirewallRule {
                        source_ip: rule.source.ip,
                        source_mask: rule.source.mask,
                        source_port: rule.source.port,
                        dest_ip: rule.dest.ip,
                        dest_mask: rule.dest.mask,
                        dest_port: rule.dest.port,
                        action: match rule.action {
                            firewall::RuleAction::Permit => packets::FirewallAction::Allow,
                            firewall::RuleAction::Drop => packets::FirewallAction::Deny,
                        },
                        protocol: rule.protocol,
                        priority: index,
                    });
                }
                else {
                    msg = packets::FirewallMessage::Error;
                }
                send_msg(msg, msg_recieved.addr.pid);
            },
            packets::FirewallMessage::QueryRules => {
                println!("Query rules list.");
                let list = firewall::list_rules();
                let mut rules = alloc::vec::Vec::<packets::FirewallRule>::with_capacity(list.len());
                for (i, rule) in list.iter().enumerate() {
                    rules.push(packets::FirewallRule {
                        source_ip: rule.source.ip,
                        source_mask: rule.source.mask,
                        source_port: rule.source.port,
                        dest_ip: rule.dest.ip,
                        dest_mask: rule.dest.mask,
                        dest_port: rule.dest.port,
                        action: match rule.action {
                            firewall::RuleAction::Permit => packets::FirewallAction::Allow,
                            firewall::RuleAction::Drop => packets::FirewallAction::Deny,
                        },
                        protocol: rule.protocol,
                        priority: i,
                    });
                }
                let msg = packets::FirewallMessage::RuleList(rules);
                send_msg(msg, msg_recieved.addr.pid);
            }
            _ => {
                println!("Invalid message");
            }
        }
    }
}

fn send_msg(msg: packets::FirewallMessage, pid: u32) {
    let socket;
    unsafe {
        socket = NETLINK_SOCKET.as_ref().unwrap();
    }
    let mut buffer: alloc::vec::Vec<u8> = vec![0; msg.eval_size() + 64];
    let size = serialize(&msg, &mut buffer);
    let msg = netlink::NetlinkMsgRaw::new(&buffer[..size]);
    socket.send(pid, msg);
    println!("Send {} bytes message to netlink pid {}", size, pid);
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
                println!("Cleanup Netfilter hook.")
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
    println!("Error");
    let mut writer = printk::LogWriter::new();
    core::fmt::write(&mut writer, *_info.message().unwrap());
    printk::printk(writer.to_str());
    printk::printk("\n");
    if let Some(location) = _info.location() {
        println!("Panic at {} line:{}", location.file(), location.line());
    }
    
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    println!(
        "Alloc Error, size={}, align={}",
        _layout.size(),
        _layout.align()
    );
    
    loop {
        core::sync::atomic::spin_loop_hint();
    }
}

#[global_allocator]
static KERNEL_ALLOC: kernel_bindings::memory::KernelAlloc = kernel_bindings::memory::KernelAlloc {};
