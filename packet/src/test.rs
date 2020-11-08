use super::deserialize::{self, Deserialize, Deserializer, DeserializeResult };
use super::serialize::{self, Serialize, Serializer};
use core::mem::size_of;

#[derive(Debug, PartialEq)]
struct Message<'a> {
    is_true: bool,
    num: i32,
    float: f64,
    str: &'a str,
}


impl<'a> Serialize for Message<'a> {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.is_true)
            .serialize(&self.num)
            .serialize(&self.float)
            .serialize(&self.str)
    }
}

impl<'b> Deserialize<Message<'b>> for Message<'b> {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<Message<'b>> {
        let mut msg = Message {
            is_true: deserializer.deserialize_bool()?,
            num: deserializer.deserialize_i32()?,
            float: deserializer.deserialize_f64()?,
            str: deserializer.deserialize_str(|size| {
                Some(alloc(size))
            })?
        };
        Ok(msg)
    }
}

const TEST_STRING: &str = "A dog jump out of a lazy fox.";
const TEST_MESSAGE: Message =  Message {
        is_true: true,
        num: 5,
        float: 3.1415,
        str: TEST_STRING,
    };
const TEST_MESSAGE_SIZE: usize = size_of::<bool>()
        + size_of::<i32>()
        + size_of::<f64>()
        + size_of::<usize>()
        + TEST_STRING.len();

static mut TEST_MEM_POOL: [u8; 8192] = [0; 8192];

#[test]
fn test_serialize() {
    
    let mut buf: [u8; 1024] = [0; 1024];
    let size = serialize::serialize(&TEST_MESSAGE, &mut buf);
    
    assert_eq!(size, TEST_MESSAGE_SIZE);
}

#[test]
fn test_deserialize() {
    let mut buf: [u8; 1024] = [0; 1024];

    let size = serialize::serialize(&TEST_MESSAGE, &mut buf);
    
    let mut data = &buf[..size];
    let msg = deserialize::deserialize::<Message>(data).unwrap();
    assert_eq!(msg, TEST_MESSAGE);

}


fn alloc(size: usize) -> &'static mut [u8] {
    unsafe {
        &mut TEST_MEM_POOL[..size]
    }
}
