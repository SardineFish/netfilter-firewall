extern crate hashbrown;

use crate::kernel_bindings::net;
use hashbrown::hash_map;

enum Direction {
    In, 
    Out,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Endpoint {
    pub ip: u32,
    pub port: u16,
}

struct TcpSession {
    pub src: Endpoint,
    pub dst: Endpoint,
    pub time: u64,
}

type UdpSession = TcpSession;

struct IcmpSession {
    src: u32,
    dst: u32,
    time: u64,
}

pub struct ConnectionState {
    tcp_sessions: hashbrown::HashMap<Endpoint, TcpSession>,
    udp_sessions: hashbrown::HashMap<Endpoint, UdpSession>,
    icmp_sessions: hashbrown::HashMap<u32, IcmpSession>,
}

trait TcpState {
    fn establish(&mut self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> &mut Self;
    fn close(&mut self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> &mut Self;
    fn check(&self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> Option<&TcpSession>;
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            tcp_sessions: hashbrown::HashMap::new(),
            udp_sessions: hashbrown::HashMap::new(),
            icmp_sessions: hashbrown::HashMap::new(),
        }
    }
}

impl ConnectionState {
    pub fn establish_tcp(&mut self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> &mut Self {
        let src = Endpoint {
            ip: ip.saddr,
            port: tcp.source,
        };
        let dst = Endpoint {
            ip: ip.daddr,
            port: tcp.dest,
        };
        let session = TcpSession {
            src: src.clone(),
            dst: dst.clone(),
            time: 0,
        };

        self.tcp_sessions.insert(src, session);

        self
    }
    pub fn close_tcp(&mut self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> &mut Self {
        let src;
        if let Some(session) = self.get_tcp_session(ip, tcp) {
            src = Some(session.src.clone());
        }
        else {
            src = None;
        }
        if let Some(src) = src {
            self.tcp_sessions.remove(&src);
        }

        self
    }
    fn get_tcp_session(&self, ip: &net::IPv4Header, tcp: &net::TcpHeader) -> Option<&TcpSession> {
        let src = Endpoint {
            ip: ip.saddr,
            port: tcp.source
        };
        let dst = Endpoint {
            ip: ip.daddr,
            port: tcp.dest,
        };

        if let Some(session) = self.tcp_sessions.get(&src) {
            if session.dst == dst {
                return Some(session);
            }
        }
        else if let Some(session) = self.tcp_sessions.get(&dst) {
            if session.dst == src {
                return Some(session);
            }
        }
        
        return None;
    }
    pub fn check_tcp(&self, iphdr: &net::IPv4Header, tcphdr: &net::TcpHeader) -> bool {
        if let Some(_) = self.get_tcp_session(iphdr, tcphdr) {
            true
        }
        else {
            false
        }
    }

    pub fn establish_udp(&mut self, iphdr: &net::IPv4Header, udphdr: &net::UdpHeader) -> &mut Self {
        let src = Endpoint {
            ip: iphdr.saddr,
            port: udphdr.source,
        };
        let dst = Endpoint {
            ip: iphdr.daddr,
            port: udphdr.dest,
        };
        let session = UdpSession {
            src: src.clone(),
            dst: dst.clone(),
            time: 0,
        };
        
        self.udp_sessions.insert(src, session);

        self
    }
    fn get_udp_session(&self, iphdr: &net::IPv4Header, udphdr: &net::UdpHeader) -> Option<&UdpSession> {
        let src = Endpoint {
            ip: iphdr.saddr,
            port: udphdr.source
        };
        let dst = Endpoint {
            ip: iphdr.daddr,
            port: udphdr.dest,
        };

        if let Some(session) = self.udp_sessions.get(&src) {
            if session.dst == dst {
                return Some(session);
            }
        }
        else if let Some(session) = self.udp_sessions.get(&dst) {
            if session.dst == src {
                return Some(session);
            }
        }
        
        return None;
    }
    pub fn check_udp(&self, iphdr: &net::IPv4Header, udphdr: &net::UdpHeader) -> bool {
        match self.get_udp_session(iphdr, udphdr) {
            Some(_) => true,
            _ => false
        }
    }

    pub fn establish_icmp(&mut self, iphdr: &net::IPv4Header, icmphdr: &net::IcmpHeader) -> &mut Self {
        let session = IcmpSession {
            src: iphdr.saddr,
            dst: iphdr.daddr,
            time: 0
        };

        self.icmp_sessions.insert(iphdr.saddr, session);

        self
    }
    fn get_icmp_session(&self, iphdr: &net::IPv4Header, icmphdr: &net::IcmpHeader) -> Option<&IcmpSession> {
        if let Some(session) = self.icmp_sessions.get(&iphdr.saddr) {
            if session.dst == iphdr.daddr {
                return Some(session);
            }
        }
        else if let Some(session) = self.icmp_sessions.get(&iphdr.daddr) {
            if session.dst == iphdr.saddr {
                return Some(session);
            }
        }

        None
    }
    pub fn check_icmp(&self, iphdr: &net::IPv4Header, icmphdr: &net::IcmpHeader) -> bool {
        self.get_icmp_session(iphdr, icmphdr).is_some()
    }
    
}