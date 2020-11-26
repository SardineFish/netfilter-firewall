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

pub fn add_rule(priority: usize, rule: GeneralFirewallRule) {
    unsafe {
        let rules = FIREWALL_RULES.as_mut().unwrap();
        rules.add_rule(rule, priority);
    }
}

pub fn list_rules() -> alloc::vec::Vec<GeneralFirewallRule> {
    let rules;
    unsafe {
        rules = FIREWALL_RULES.as_ref().unwrap();
    }
    rules.list_rules()
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
        if state.check_tcp(ip, tcp) {
            netfilter::HookResponse::Accept
        }
        else if tcp.syn() == 1 {
            if rules.permit_tcp(ip, tcp) {
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
                    "Deny a TCP connection {}:{} -> {}:{}", 
                    utils::ipv4_addr(ip.saddr), 
                    tcp.source.to_be(), 
                    utils::ipv4_addr(ip.daddr), 
                    tcp.dest.to_be());
                netfilter::HookResponse::Drop
            }
        }
        else {
            println!(
                "Ingore an unkown TCP packet {}:{} -> {}:{}", 
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
        let state;
        let rules;
        unsafe {
            state = CONNECTION_STATE.as_mut().unwrap();
            rules = FIREWALL_RULES.as_mut().unwrap();
        }

        let (iphdr, udphdr) = self.unwrap();

        if state.check_udp(iphdr, udphdr) {
            netfilter::HookResponse::Accept
        }
        else if rules.permit_udp(iphdr, udphdr) {
            state.establish_udp(iphdr, udphdr);
            println!(
                    "Accept new UDP session {}:{} -> {}:{}", 
                    utils::ipv4_addr(iphdr.saddr), 
                    udphdr.source.to_be(), 
                    utils::ipv4_addr(iphdr.daddr), 
                    udphdr.dest.to_be());
            netfilter::HookResponse::Accept
        }
        else {
            println!(
                    "Ingore a UDP packet {}:{} -> {}:{}", 
                    utils::ipv4_addr(iphdr.saddr), 
                    udphdr.source.to_be(), 
                    utils::ipv4_addr(iphdr.daddr), 
                    udphdr.dest.to_be());
            netfilter::HookResponse::Drop
        }
    }
}

impl PacketHandler for Packet<(&net::IPv4Header, &net::IcmpHeader)> {
    fn handle(self) -> netfilter::HookResponse {
        let states;
        let rules;
        unsafe {
            states = CONNECTION_STATE.as_mut().unwrap();
            rules = FIREWALL_RULES.as_mut().unwrap();
        }

        let (iphdr, icmphdr) = self.unwrap();

        if states.check_icmp(iphdr, icmphdr) {
            println!(
                "Existed ICMP session {} -> {}",
                utils::ipv4_addr(iphdr.saddr),
                utils::ipv4_addr(iphdr.daddr)
            );
            netfilter::HookResponse::Accept
        }
        else if icmphdr.type_ == net::icmp_code::ICMP_ECHO {
            if rules.permit_icmp(iphdr, icmphdr) {
                states.establish_icmp(iphdr, icmphdr);
                println!(
                    "Accept new ICMP session {} -> {}",
                    utils::ipv4_addr(iphdr.saddr),
                    utils::ipv4_addr(iphdr.daddr)
                );
                netfilter::HookResponse::Accept
            }
            else {
                println!(
                    "Deny an ICMP session {} -> {}",
                    utils::ipv4_addr(iphdr.saddr),
                    utils::ipv4_addr(iphdr.daddr)
                );
                netfilter::HookResponse::Drop
            }
        }
        else if icmphdr.type_ == net::icmp_code::ICMP_ECHOREPLY {
            println!(
                "Ignore an unkown ICMP packet {} -> {} type:{} code:{}",
                utils::ipv4_addr(iphdr.saddr),
                utils::ipv4_addr(iphdr.daddr),
                icmphdr.type_,
                icmphdr.code
            );
            netfilter::HookResponse::Drop
        }
        else {
            netfilter::HookResponse::Accept
        }
    }
}

