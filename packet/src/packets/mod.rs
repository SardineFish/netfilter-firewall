use crate::deserialize::{Deserialize, DeserializeResult, Deserializer};
use crate::serialize::{Serialize, Serializer};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, PartialEq, Default)]
pub struct CapturedPacket {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Default)]
pub struct FilterRule {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_mask: u32,
    pub dest_mask: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FirewallAction {
    Deny = 0,
    Allow = 1,
}
impl  Default for FirewallAction {
    fn default() -> Self {
        FirewallAction::Allow
    }
}
impl From<u8> for FirewallAction {
    fn from(t: u8) -> Self {
        match t {
            0 => FirewallAction::Deny,
            1 => FirewallAction::Allow,
            _ => FirewallAction::Allow,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FirewallRule {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_mask: u32,
    pub dest_mask: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub priority: usize,
    pub action: FirewallAction,
}

#[derive(Debug, PartialEq)]
pub struct FirewallRuleList {
    pub rules: Vec<FirewallRule>,
}

#[derive(Debug, PartialEq)]
pub enum FirewallMessage {
    /* 0 */ Error, 
    /* 1 */ QueryRules, 
    /* 2 */ SetDefault(FirewallRule),
    /* 3 */ SetRule(FirewallRule),
    /* 4 */ RuleList(Vec<FirewallRule>),
}

impl Drop for CapturedPacket {
    fn drop(&mut self) {
    }
}

impl CapturedPacket {
    pub fn total_size(&self) -> usize {
        return core::mem::size_of::<u32>() * 2 
            + core::mem::size_of::<u16>() * 2
            + core::mem::size_of::<u8>() * 1
            + core::mem::size_of::<usize>()
            + self.payload.len();
    }
}

impl Serialize for CapturedPacket {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.source_ip)
            .serialize(&self.dest_ip)
            .serialize(&self.source_port)
            .serialize(&self.dest_port)
            .serialize(&self.protocol)
            .serialize(&self.payload.as_slice())
    }
}

impl Deserialize<CapturedPacket> for CapturedPacket {
    fn deserialize<'a>(
        deserializer: &mut Deserializer<'a>,
    ) -> DeserializeResult<CapturedPacket> {
        Ok(CapturedPacket {
            source_ip: deserializer.deserialize_u32()?,
            dest_ip: deserializer.deserialize_u32()?,
            source_port: deserializer.deserialize_u16()?,
            dest_port: deserializer.deserialize_u16()?,
            protocol: deserializer.deserialize_u8()?,
            payload: deserializer.deserialize_u8_vec()?,
        })
    }
}

impl Serialize for FilterRule {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.source_ip)
            .serialize(&self.source_mask)
            .serialize(&self.dest_ip)
            .serialize(&self.dest_mask)
            .serialize(&self.source_port)
            .serialize(&self.dest_port)
            .serialize(&self.protocol)
    }
}

impl Deserialize<FilterRule> for FilterRule {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FilterRule> {
        Ok(FilterRule {
            source_ip: deserializer.deserialize_u32()?,
            source_mask: deserializer.deserialize_u32()?,
            dest_ip: deserializer.deserialize_u32()?,
            dest_mask: deserializer.deserialize()?,
            source_port: deserializer.deserialize()?,
            dest_port: deserializer.deserialize()?,
            protocol: deserializer.deserialize()?
        })
    }
}

impl Serialize for FirewallRule {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.source_ip)
            .serialize(&self.source_mask)
            .serialize(&self.dest_ip)
            .serialize(&self.dest_mask)
            .serialize(&self.source_port)
            .serialize(&self.dest_port)
            .serialize(&self.protocol)
            .serialize(&self.priority)
            .serialize(&(self.action as u8))
    }
}

impl Deserialize<FirewallRule> for FirewallRule {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallRule> {
        Ok(FirewallRule {
            source_ip: deserializer.deserialize_u32()?,
            source_mask: deserializer.deserialize_u32()?,
            dest_ip: deserializer.deserialize_u32()?,
            dest_mask: deserializer.deserialize()?,
            source_port: deserializer.deserialize()?,
            dest_port: deserializer.deserialize()?,
            protocol: deserializer.deserialize()?,
            priority: deserializer.deserialize()?,
            action: FirewallAction::from(deserializer.deserialize_u8()?),
        })
    }
}

impl Serialize for FirewallRuleList {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer.serialize(&&self.rules[..])
    }
}

impl Deserialize<FirewallRuleList> for FirewallRuleList {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallRuleList> {
        Ok(FirewallRuleList {
            rules: deserializer.deserialize_vec(vec![])?
        })
    }
}

impl Serialize for FirewallMessage {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        match self {
            FirewallMessage::Error => serializer.serialize(&0),
            FirewallMessage::QueryRules => serializer.serialize(&1),
            FirewallMessage::SetDefault(rule) => serializer.serialize(&2).serialize(rule),
            FirewallMessage::SetRule(rule) => serializer.serialize(&3).serialize(rule),
            FirewallMessage::RuleList(rules) => serializer.serialize(&4).serialize(&&rules[..])
        }
    }
}

impl Deserialize<FirewallMessage> for FirewallMessage {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallMessage> {
        let msg_type = deserializer.deserialize_i32()?;
        let msg = match msg_type {
            1 => FirewallMessage::QueryRules,
            2 => FirewallMessage::SetDefault(deserializer.deserialize()?),
            3 => FirewallMessage::SetRule(deserializer.deserialize()?),
            4 => FirewallMessage::RuleList(deserializer.deserialize_vec(vec![])?),
            _ => FirewallMessage::Error,
        };
        Ok(msg)
    }
}

#[cfg(test)]
mod test {

    extern crate rand;
    use crate::packets::*;
    use crate::deserialize;
    use crate::serialize;
    use rand::Rng;
    use std::alloc;

    #[test]
    fn test_captured_packet() {
        unsafe {
            let mut rng = rand::thread_rng();

            let alloc_buf = alloc::alloc(alloc::Layout::from_size_align(256, 1).unwrap());
            
            let mut payload = vec![0; rng.gen_range(64, 1024)];
            for element in &mut payload {
                *element = rng.gen();
            }

            let packet = CapturedPacket {
                source_ip: rng.gen(),
                dest_ip: rng.gen(),
                source_port: rng.gen(),
                dest_port: rng.gen(),
                protocol: rng.gen(),
                payload: payload,
            };

            let mut buffer = vec![0; packet.total_size()];
            let serialized_size = serialize::serialize(&packet, &mut buffer);

            assert_eq!(packet.total_size(), serialized_size);

            let deserialized_packet = deserialize::deserialize::<CapturedPacket>(&buffer).unwrap();

            assert_eq!(packet, deserialized_packet);

        }
    }

    #[test]
    fn test_captured_packet_zero_payload() {
        let packet: CapturedPacket = Default::default();

        let mut buffer = [0; 8192];
        serialize::serialize(&packet, &mut buffer);
        let deserialized_packet = deserialize::deserialize::<CapturedPacket>(&buffer).unwrap(); 

        assert_eq!(deserialized_packet.payload.len(), 0);
        assert_eq!(deserialized_packet, packet);
    }

    #[test]
    fn test_filter_rule() {
        let mut rng = rand::thread_rng();

        let packet = FilterRule {
            source_ip: rng.gen(),
            source_mask: rng.gen(),
            dest_ip: rng.gen(),
            dest_mask: rng.gen(),
            source_port: rng.gen(),
            dest_port: rng.gen(),
            protocol: rng.gen()
        };

        let mut buffer = [0; 8192];
        serialize::serialize(&packet, &mut buffer);
        let deserialized_packet = deserialize::deserialize::<FilterRule>(&buffer).unwrap();

        assert_eq!(deserialized_packet, packet);
    }

    fn rand_firewall_rule(rng: &mut rand::rngs::ThreadRng) -> FirewallRule {
        FirewallRule {
                source_ip: rng.gen(),
                source_mask: rng.gen(),
                dest_ip: rng.gen(),
                dest_mask: rng.gen(),
                source_port: rng.gen(),
                dest_port: rng.gen(),
                protocol: rng.gen(),
                priority: rng.gen(),
                action: FirewallAction::from(rng.gen::<u8>()),
            }
    }

    #[test]
    fn test_firewall_rules() {
        let mut rng = rand::thread_rng();

        let mut rules: Vec<FirewallRule> = vec![];

        let len = rng.gen_range::<usize, usize, usize>(2, 8);

        for _ in 0..len {
            rules.push(rand_firewall_rule(&mut rng));
        }

        let msg = FirewallMessage::RuleList(rules);

        let mut buffer = [0; 8192];
        serialize::serialize(&msg, &mut buffer);
        let deserialized_msg = deserialize::deserialize::<FirewallMessage>(&buffer).unwrap();

        assert_eq!(deserialized_msg, msg);


        let rule = rand_firewall_rule(&mut rng);
        let msg = FirewallMessage::SetRule(rule);
        serialize::serialize(&msg, &mut buffer);
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);

        let rule = rand_firewall_rule(&mut rng);
        let msg = FirewallMessage::SetDefault(rule);
        serialize::serialize(&msg, &mut buffer);
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);

        let msg = FirewallMessage::QueryRules;
        serialize::serialize(&msg, &mut buffer);
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);
    }

}
