
extern crate packet;
extern crate netlink_sys;
extern crate netlink_packet_core;
extern crate regex;

mod protocol;
mod msg;

use packet::packets::*;
use packet::*;
use protocol::ip_protocol;
use std::{fmt::Display, net};

fn list_rules() {
    let msg = FirewallMessage::QueryRules;
    let socket = send_msg(msg);
    let reply = socket.recv::<FirewallMessage>();
    match reply {
        FirewallMessage::RuleList(rules) => {
            println!(
                "Priority\t{}\t{:<16} {:<16} {:<12} {:<16} {:<16} {:<10} {}",
                "Protocol",
                "Src Ip",
                "Src Mask",
                "Src Port",
                "Dest Ip",
                "Dest Mask",
                "Dest Port",
                "Action"
            );
            println!("----------------------------------------------------------------------------------------------------------------------------------");
            for (i, rule) in rules.iter().enumerate() {
                println!("{}\t\t{}",i , format_rule(&rule));
            }
        },
        _ => {
            println!("Invalid message from firewall kernel module.");
        }
    }
}

fn parse_endpoint(endpoint: &str) -> Result<(u32, u32, u16), String> {
    let reg = regex::Regex::new(r"^(\d+\.\d+\.\d+\.\d+)(/\d+)?(:\d+)?").unwrap();
    let caps = reg.captures(endpoint).unwrap();

    let ip: net::Ipv4Addr = caps.get(1).unwrap().as_str().parse().unwrap();
    let ip: u32 = ip.into();

    let mask_bits = if let Some(mask) = caps.get(2) {
        mask.as_str()[1..].parse::<u32>().unwrap()
    } else if ip == 0 {
        0
    } else { 
        32
    };
    let mask = match mask_bits {
        0 => 0,
        x => 0xFFFFFFFFu32.wrapping_shl(32 - x),
    };

    let port = if let Some(port) = caps.get(3) {
        port.as_str()[1..].parse::<u16>().unwrap()
    } else { 0 };
    Ok((ip, mask, port))
}

pub fn ipv4_addr(addr: u32) -> String {
    let bytes = addr.to_ne_bytes();
    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
}

fn format_rule(rule: &FirewallRule) -> String {
    let protocol = match rule.protocol {
        ip_protocol::TCP => "TCP",
        ip_protocol::UDP => "UDP",
        ip_protocol::ICMP => "ICMP",
        _ => "UNKOWN",
    };
    let action = match rule.action {
        FirewallAction::Allow => "Allow",
        FirewallAction::Deny => "Deny",
    };
    format!(
        "{}\t\t{:<16} {:<16} {:<12} {:<16} {:<16} {:<10} {}", 
        protocol,
        ipv4_addr(rule.source_ip), 
        ipv4_addr(rule.source_mask),
        rule.source_port,
        ipv4_addr(rule.dest_ip),
        ipv4_addr(rule.dest_mask),
        rule.dest_port,
        action
    )
}

fn send_rule(args: &[String], action: FirewallAction) {
    if args.len() < 3 {
        println!("Invalid arguments.");
        println!("Example: firewall allow TCP 127.0.0.1/32:0 192.168.0.0/24:80");
        std::process::exit(-1);
    }

    let protocol = &args[0].to_uppercase() as &str;
    let protocol = match protocol {
        "TCP" => ip_protocol::TCP,
        "UDP" => ip_protocol::UDP,
        "ICMP" => ip_protocol::ICMP,
        _ => {
            println!("Unknown protocol {}", args[0]);
            std::process::exit(-1);
        }
    };

    let (src_ip, src_mask, src_port) = parse_endpoint(args[1].as_str()).unwrap();
    let (dst_ip, dst_mask, dst_port) = parse_endpoint(args[2].as_str()).unwrap();

    let rule = FirewallRule {
        source_ip: src_ip,
        source_mask: src_mask,
        source_port: src_port,
        dest_ip: dst_ip,
        dest_mask: dst_mask,
        dest_port: dst_port,
        protocol: protocol,
        action: action,
        priority: 0,
    };
    send_msg(FirewallMessage::SetRule(rule));
    println!("Rule added.");
}

fn send_msg(msg: FirewallMessage) -> msg::Socket {
    let socket = msg::Socket::new(17);
    socket.send(0, 0, msg);
    socket
}

pub fn main() {

    let args:Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Invalid arguments.");
        std::process::exit(-1);
    }

    let command = &args[1][..];

    match command {
        "ls" | "list" => list_rules(),
        "allow" => send_rule(&args[2..], FirewallAction::Allow),
        "deny" => send_rule(&args[2..], FirewallAction::Deny),
        _ => {
            println!("Unknown command {}", command);
            std::process::exit(-1);
        }
    }

    return;
    
}
