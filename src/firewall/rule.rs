extern crate hashbrown;

use hashbrown::hash_map;
use crate::kernel_bindings::net;
use alloc::vec::{Vec};
use alloc::vec;

enum Direction {
    Out = 0,
    In = 1,
}

#[derive(Default)]
pub struct IcmpEndpoint {
    ip: u32,
    mask: u32,
}

#[derive(Default)]
pub struct Endpoint {
    ip: u32,
    mask: u32,
    port: u16,
}

#[derive(Eq, PartialEq)]
pub enum RuleAction {
    Permit,
    Drop,
}

impl Default for RuleAction {
    fn default() -> Self {
        RuleAction::Permit
    }
}

pub struct GeneralFirewallRule {
    protocol: net::IpProtocol,
    source: Endpoint,
    dest: Endpoint,
    action: RuleAction,
}

#[derive(Default)]
pub struct IcmpRuleEntry {
    source: IcmpEndpoint,
    dest: IcmpEndpoint,
    action: RuleAction,
}

#[derive(Default)]
pub struct TcpRuleEntry {
    source: Endpoint,
    dest: Endpoint,
    action: RuleAction,
}
type UdpRuleEntry = TcpRuleEntry;

pub struct FirewallRules {
    tcp_rules: Vec<TcpRuleEntry>,
    udp_rules: Vec<UdpRuleEntry>,
    icmp_rules: Vec<IcmpRuleEntry>,
    tcp_default: RuleAction,
}

impl FirewallRules {
    pub fn new() -> Self {
        Self {
            tcp_rules: vec![],
            udp_rules: vec![],
            icmp_rules: vec![],
            tcp_default: RuleAction::Permit,
        }
    }
    pub fn permit_tcp(&self, iphdr: &net::IPv4Header, tcphdr: &net::TcpHeader) -> bool {
        for rule in self.tcp_rules.iter() {
            let src_match = (iphdr.saddr & rule.source.mask) == (rule.source.ip & rule.source.mask);
            let dst_match = (iphdr.daddr & rule.dest.mask) == (rule.dest.ip & rule.dest.mask);
            let src_port_match = rule.source.port == 0 || (rule.source.port == tcphdr.source);
            let dst_port_match = rule.dest.port == 0 || (rule.dest.port == tcphdr.dest);

            if src_match && dst_match && src_port_match && dst_port_match {
                return rule.action == RuleAction::Permit
            }
        }

        self.tcp_default == RuleAction::Permit
    }
    pub fn add_rule(&mut self, mut rule: GeneralFirewallRule, mut priority: usize) {
        rule = handle_byte_order(rule);

        if priority >= self.tcp_rules.len() {
            priority = self.tcp_rules.len();
        }

        match rule.protocol {
            net::ip_protocol::TCP => self.tcp_rules.insert(priority, TcpRuleEntry {
                    source: rule.source,
                    dest: rule.dest,
                    action: rule.action,
                }),
            net::ip_protocol::UDP => self.udp_rules.insert(priority, UdpRuleEntry {
                source: rule.source,
                dest: rule.dest,
                action: rule.action,
            }),
            net::ip_protocol::ICMP => self.icmp_rules.insert(priority, IcmpRuleEntry {
                source: IcmpEndpoint {
                    ip: rule.source.ip,
                    mask: rule.source.mask,
                },
                dest: IcmpEndpoint {
                    ip: rule.dest.ip,
                    mask: rule.dest.mask,
                },
                action: rule.action,
            }),
            _=>(),
        }
    }
}

impl Endpoint {
    pub const fn to_be(self) -> Self {
        Endpoint {
            ip: self.ip.to_be(),
            mask: self.mask.to_be(),
            port: self.port.to_be(),
        }
    }
}

fn handle_byte_order(rule: GeneralFirewallRule) -> GeneralFirewallRule {
    GeneralFirewallRule {
        source: rule.source.to_be(),
        dest: rule.dest.to_be(),
        ..rule
    }
}
