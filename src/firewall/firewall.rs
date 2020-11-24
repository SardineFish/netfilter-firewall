use crate::kernel_bindings::bindings;
use crate::kernel_bindings::net;
use crate::kernel_bindings::netfilter;
use crate::println;
use crate::utils;

use super::rule::*;
use super::state::{ConnectionState, *};

static mut CONNECTION_STATE: Option<ConnectionState> = None;
static mut FIREWALL_RULES: Option<FirewallRules> = None;

enum Packet<T> {
    Input(T),
    Output(T),
}

trait PacketHandler {
    fn handle(self) -> netfilter::HookResponse;
}

pub fn packet_input(ip_header: &net::IPv4Header, skb: &bindings::sk_buff) -> netfilter::HookResponse {
    match ip_header.protocol {
        net::ip_protocol::TCP => {
            let tcp_header = net::TcpHeader::from_skbuff(skb).unwrap();
            Packet::Input((ip_header, tcp_header)).handle()
        }
        net::ip_protocol::UDP => {
            let udp_header = net::UdpHeader::from_skbuff(skb).unwrap();
            Packet::Input((ip_header, udp_header)).handle()
        }
        net::ip_protocol::ICMP => {
            let icmp_header = net::IcmpHeader::from_skbuff(skb).unwrap();
            Packet::Input((ip_header, icmp_header)).handle()
        }
        _ => netfilter::HookResponse::Accept,
    }
}

pub fn packet_output(ip_header: &net::IPv4Header, skb: &bindings::sk_buff) -> netfilter::HookResponse {
    match ip_header.protocol {
        net::ip_protocol::TCP => {
            let tcp_header = net::TcpHeader::from_skbuff(skb).unwrap();
            Packet::Output((ip_header, tcp_header)).handle()
        }
        net::ip_protocol::UDP => {
            let udp_header = net::UdpHeader::from_skbuff(skb).unwrap();
            Packet::Output((ip_header, udp_header)).handle()
        }
        net::ip_protocol::ICMP => {
            let icmp_header = net::IcmpHeader::from_skbuff(skb).unwrap();
            Packet::Output((ip_header, icmp_header)).handle()
        }
        _ => netfilter::HookResponse::Accept,
    }
}

pub fn init_firewall() {
    unsafe {
        CONNECTION_STATE = Some(ConnectionState::new());
        FIREWALL_RULES = Some(FirewallRules::new());
    }
}

pub fn add_rule(rule: GeneralFirewallRule, priority: usize) {
    unsafe {
        let rules = FIREWALL_RULES.as_mut().unwrap();
        rules.add_rule(rule, priority);
    }
}

impl<T> Packet<T> {
    pub fn unwrap(self) -> T {
        match self {
            Packet::Input(packet) => packet,
            Packet::Output(packet) => packet,
        }
    }
}

impl PacketHandler for Packet<(&net::IPv4Header, &net::TcpHeader)> {
    fn handle(self) -> netfilter::HookResponse {
        let state;
        let rules;
        unsafe {
            state = CONNECTION_STATE.as_mut().unwrap();
            rules = FIREWALL_RULES.as_mut().unwrap();
        }
        let (ip, tcp) = self.unwrap();
        if state.is_valid_tcp(ip, tcp) {
            netfilter::HookResponse::Accept
        }
        else if tcp.syn() == 1 && rules.permit_tcp(ip, tcp) {
            state.establish_tcp(ip, tcp);
            println!(
                "Accept new TCP connection {}:{} -> {}:{}", 
                utils::ipv4_addr(ip.saddr), 
                tcp.source.to_be(), 
                utils::ipv4_addr(ip.daddr), 
                tcp.dest.to_be());
            netfilter::HookResponse::Accept
        }
        else {
            println!(
                "Drop a TCP packet {}:{} -> {}:{}", 
                utils::ipv4_addr(ip.saddr), 
                tcp.source.to_be(), 
                utils::ipv4_addr(ip.daddr), 
                tcp.dest.to_be());
            netfilter::HookResponse::Drop
        }
    }
}

impl PacketHandler for Packet<(&net::IPv4Header, &net::UdpHeader)> {
    fn handle(self) -> netfilter::HookResponse {
        netfilter::HookResponse::Accept
    }
}

impl PacketHandler for Packet<(&net::IPv4Header, &net::IcmpHeader)> {
    fn handle(self) -> netfilter::HookResponse {
        netfilter::HookResponse::Accept
    }
}

