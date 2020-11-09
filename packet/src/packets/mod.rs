use crate::deserialize::{Deserialize, DeserializeResult, Deserializer};
use crate::serialize::{Serialize, Serializer};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

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
        // unsafe {
        //     std::alloc::dealloc(
        //         self.payload.as_ptr() as *mut u8,
        //         std::alloc::Layout::from_size_align(self.payload.len(), 1).unwrap(),
        //     );
        // }
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
    use crate::packets::CapturedPacket;
    use crate::packets::FilterRule;
    use crate::deserialize;
    use crate::serialize;
    use rand::Rng;
    use std::alloc;

    #[test]
    fn test_captured_packet() {
        unsafe {
            let mut rng = rand::thread_rng();

            let alloc_buf = alloc::alloc(alloc::Layout::from_size_align(256, 1).unwrap());
            
            let mut payload = vec![0; 256];
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

            let mut buffer = [0; 8192];
            serialize::serialize(&packet, &mut buffer);
            let deserialized_packet = deserialize::deserialize::<CapturedPacket>(&buffer).unwrap();

            assert_eq!(packet, deserialized_packet);

        }
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
}
