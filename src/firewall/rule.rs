extern crate hashbrown;

use hashbrown::hash_map;
use crate::kernel_bindings::net;
use alloc::vec::{Vec};
use alloc::vec;
use core::convert::Into;

enum Direction {
    Out = 0,
    In = 1,
}

#[derive(Default, Clone)]
pub struct IcmpEndpoint {
    ip: u32,
    mask: u32,
}

#[derive(Default, Clone)]
pub struct Endpoint {
    pub ip: u32,
    pub mask: u32,
    pub port: u16,
}

#[derive(Eq, PartialEq, Copy, Clone)]
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
    pub protocol: net::IpProtocol,
    pub source: Endpoint,
    pub dest: Endpoint,
    pub action: RuleAction,
}

#[derive(Default, Clone)]
pub struct IcmpRuleEntry {
    source: IcmpEndpoint,
    dest: IcmpEndpoint,
    action: RuleAction,
}

#[derive(Default, Clone)]
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
    udp_default: RuleAction,
    icmp_default: RuleAction,
}

impl FirewallRules {
    pub fn new() -> Self {
        Self {
            tcp_rules: vec![],
            udp_rules: vec![],
            icmp_rules: vec![],
            tcp_default: RuleAction::Permit,
            udp_default: RuleAction::Permit,
            icmp_default: RuleAction::Permit,
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
    pub fn permit_udp(&self, iphdr: &net::IPv4Header, udphdr: &net::UdpHeader) -> bool {
        for rule in &self.udp_rules {
            let src_match = (iphdr.saddr & rule.source.mask) == (rule.source.ip & rule.source.mask);
            let dst_match = (iphdr.daddr & rule.dest.mask) == (rule.dest.ip & rule.dest.mask);
            let src_port_match = rule.source.port == 0 || (rule.source.port == udphdr.source);
            let dst_port_match = rule.dest.port == 0 || (rule.dest.port == udphdr.dest);

            if src_match && dst_match && src_port_match &&dst_port_match {
                return rule.action == RuleAction::Permit;
            }
        }

        self.udp_default == RuleAction::Permit
    }
    pub fn permit_icmp(&self, iphdr: &net::IPv4Header, icmphdr: &net::IcmpHeader) -> bool {
        for rule in &self.icmp_rules {
            let src_match = (iphdr.saddr & rule.source.mask) == (rule.source.ip & rule.source.mask);
            let dst_match = (iphdr.daddr & rule.dest.mask) == (rule.dest.ip & rule.dest.mask);

            if src_match && dst_match {
                return rule.action == RuleAction::Permit;
            }
        }

        self.icmp_default == RuleAction::Permit
    }
    pub fn add_rule(&mut self, mut rule: GeneralFirewallRule, mut priority: usize) {
        rule = handle_byte_order(rule);

        if priority >= self.tcp_rules.len() {
            priority = self.tcp_rules.len();
        }

        match rule.protocol {
            net::ip_protocol::TCP => self.tcp_rules.insert(priority, rule.into()),
            net::ip_protocol::UDP => self.udp_rules.insert(priority, rule.into()),
            net::ip_protocol::ICMP => self.icmp_rules.insert(priority, rule.into()),
            _=>(),
        }
    }

    pub fn set_default(&mut self, mut rule: GeneralFirewallRule) {
        rule = handle_byte_order(rule);

        match rule.protocol {
            net::ip_protocol::TCP => (self.tcp_default = rule.action),
            net::ip_protocol::UDP => (self.udp_default = rule.action),
            net::ip_protocol::ICMP => (self.icmp_default = rule.action),
            _=>(),
        }
    }

    pub fn delete_rule(&mut self, mut index: usize) -> Option<GeneralFirewallRule> {
        if index < self.tcp_rules.len() {
            return Some(self.tcp_rules.remove(index).into_tcp_rule());
        }
        index -= self.tcp_rules.len();
        if index < self.udp_rules.len() {
            return Some(self.udp_rules.remove(index).into_udp_rule());
        }
        index -= self.udp_rules.len();
        if index < self.icmp_rules.len() {
            return Some(self.icmp_rules.remove(index).into());
        }
        None
    }

    pub fn list_rules(&self) -> Vec<GeneralFirewallRule> {
        let mut list = Vec::<GeneralFirewallRule>::with_capacity(
            self.tcp_rules.len() + self.udp_rules.len() + self.icmp_rules.len());
        
        for rule in &self.tcp_rules {
            list.push(rule.clone().into_tcp_rule());
        }
        for rule in &self.udp_rules {
            list.push(rule.clone().into_udp_rule());
        }
        for rule in &self.icmp_rules {
            list.push(rule.clone().into());
        }
        list
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

impl From<GeneralFirewallRule> for TcpRuleEntry {
    fn from(mut rule: GeneralFirewallRule) -> Self {
        rule = handle_byte_order(rule);

        TcpRuleEntry {
            source: rule.source,
            dest: rule.dest,
            action: rule.action,
        }
    }
}


impl From<TcpRuleEntry> for GeneralFirewallRule {
    fn from(rule: TcpRuleEntry) -> Self {
        GeneralFirewallRule {
            source: rule.source.clone().to_be(),
            dest: rule.dest.clone().to_be(),
            action: rule.action,
            protocol: net::ip_protocol::IP,
        }
    }
}

impl TcpRuleEntry {
    fn into_tcp_rule(self) -> GeneralFirewallRule {
        GeneralFirewallRule {
            protocol: net::ip_protocol::TCP,
            ..self.into()
        }
    }
    fn into_udp_rule(self) -> GeneralFirewallRule {
        GeneralFirewallRule {
            protocol: net::ip_protocol::UDP,
            ..self.into()
        }
    }
}

impl From<GeneralFirewallRule> for IcmpRuleEntry {
    fn from(rule: GeneralFirewallRule) -> Self {
        let rule = handle_byte_order(rule);
        IcmpRuleEntry {
            source: IcmpEndpoint {
                ip: rule.source.ip,
                mask: rule.source.mask,
            },
            dest: IcmpEndpoint {
                ip: rule.dest.ip,
                mask: rule.dest.mask,
            },
            action: rule.action,
        }
    }
}

impl From<IcmpRuleEntry> for GeneralFirewallRule {
    fn from(rule: IcmpRuleEntry) -> Self {
        GeneralFirewallRule {
            source: Endpoint {
                ip: rule.source.ip.to_be(),
                mask: rule.source.mask.to_be(),
                port: 0,
            },
            dest: Endpoint {
                ip: rule.dest.ip.to_be(),
                mask: rule.dest.mask.to_be(),
                port: 0,
            },
            action: rule.action,
            protocol: net::ip_protocol::ICMP,
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
