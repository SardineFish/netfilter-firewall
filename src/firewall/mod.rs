use crate::kernel_bindings::bindings;
use crate::kernel_bindings::net;
use crate::kernel_bindings::netfilter;
use crate::println;

enum Packet<T> {
    Input(T),
    Output(T),
}

trait PacketHandler {
    fn handle(self) -> netfilter::HookResponse;
}

pub fn packet_input(header: &net::IPv4Header, skb: &bindings::sk_buff) -> netfilter::HookResponse {
    match header.protocol {
        net::ip_protocol::TCP => {
            let header = net::TcpHeader::from_skbuff(skb).unwrap();
            Packet::Input(header).handle()
        },
        net::ip_protocol::UDP => {
            let header = net::UdpHeader::from_skbuff(skb).unwrap();
            Packet::Input(header).handle()
        },
        net::ip_protocol::ICMP => {
            let header = net::IcmpHeader::from_skbuff(skb).unwrap();
            Packet::Input(header).handle()
        },
        _ => netfilter::HookResponse::Accept,
    }
}

pub fn packet_output(header: &net::IPv4Header, skb: &bindings::sk_buff) -> netfilter::HookResponse {
    match header.protocol {
        net::ip_protocol::TCP => {
            let header = net::TcpHeader::from_skbuff(skb).unwrap();
            Packet::Output(header).handle()
        },
        net::ip_protocol::UDP => {
            let header = net::UdpHeader::from_skbuff(skb).unwrap();
            Packet::Output(header).handle()
        },
        net::ip_protocol::ICMP => {
            let header = net::IcmpHeader::from_skbuff(skb).unwrap();
            Packet::Output(header).handle()
        },
        _ => netfilter::HookResponse::Accept,
    }
}

impl PacketHandler for Packet<&net::TcpHeader> {
    fn handle(self) -> netfilter::HookResponse {
        match self {
            Packet::Input(header) => {
                println!("Input TCP {} -> {}", header.source.to_be(), header.dest.to_be());
            },
            Packet::Output(header) => {
                println!("Output TCP {} -> {}", header.source.to_be(), header.dest.to_be());
            },
        }
        netfilter::HookResponse::Accept
    }
}

impl PacketHandler for Packet<&net::UdpHeader> {
    fn handle(self) -> netfilter::HookResponse {
        netfilter::HookResponse::Accept
    }
}

impl PacketHandler for Packet<&net::IcmpHeader> {
    fn handle(self) -> netfilter::HookResponse {
        netfilter::HookResponse::Accept
    }
}