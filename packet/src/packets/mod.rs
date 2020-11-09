use crate::deserialize::{Deserialize, DeserializeResult, Deserializer};
use crate::serialize::{Serialize, Serializer};

#[derive(Debug, PartialEq)]
pub struct CapturedPacket<'a> {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct FilterRule {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_mask: u32,
    pub dest_mask: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
}

impl<'a> Drop for CapturedPacket<'a> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.payload.as_ptr() as *mut u8,
                std::alloc::Layout::from_size_align(self.payload.len(), 1).unwrap(),
            );
        }
    }
}

impl<'a> Serialize for CapturedPacket<'a> {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.source_ip)
            .serialize(&self.dest_ip)
            .serialize(&self.source_port)
            .serialize(&self.dest_port)
            .serialize(&self.protocol)
            .serialize(&self.payload)
    }
}

impl<'b> Deserialize<CapturedPacket<'b>> for CapturedPacket<'b> {
    fn deserialize<'a>(
        deserializer: &mut Deserializer<'a>,
    ) -> DeserializeResult<CapturedPacket<'b>> {
        Ok(CapturedPacket {
            source_ip: deserializer.deserialize_u32()?,
            dest_ip: deserializer.deserialize_u32()?,
            source_port: deserializer.deserialize_u16()?,
            dest_port: deserializer.deserialize_u16()?,
            protocol: deserializer.deserialize_u8()?,
            payload: deserializer.deserialize_u8_array_alloc(|size| unsafe {
                let ptr = std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap());
                Some(core::slice::from_raw_parts_mut(ptr, size))
            })?,
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

            let packet = CapturedPacket {
                source_ip: rng.gen(),
                dest_ip: rng.gen(),
                source_port: rng.gen(),
                dest_port: rng.gen(),
                protocol: rng.gen(),
                payload: core::slice::from_raw_parts(alloc_buf, 256),
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
