use crate::deserialize::{Deserialize, DeserializeResult, Deserializer};
use crate::serialize::{Serialize, Serializer, EvalSize};

mod firewall;

pub use firewall::*;

#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use alloc::vec;
use core::mem::size_of;

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
        let size =serialize::serialize(&msg, &mut buffer);
        assert_eq!(size, msg.eval_size());
        let deserialized_msg = deserialize::deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(deserialized_msg, msg);
        


        let rule = rand_firewall_rule(&mut rng);
        let msg = FirewallMessage::SetRule(rule);
        serialize::serialize(&msg, &mut buffer);
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);

        let rule = rand_firewall_rule(&mut rng);
        let msg = FirewallMessage::SetDefault(rule);
        let size = serialize::serialize(&msg, &mut buffer);
        assert_eq!(size, msg.eval_size());
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);

        let msg = FirewallMessage::QueryRules;
        let size = serialize::serialize(&msg, &mut buffer);
        assert_eq!(size, msg.eval_size());
        let deserialized_msg = deserialize::<FirewallMessage>(&buffer).unwrap();
        assert_eq!(msg, deserialized_msg);
    }

}
