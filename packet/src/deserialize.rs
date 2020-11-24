
extern crate concat_idents;

use core::cmp;
use core::mem::size_of;

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

pub trait DataReader<'a> {
    fn read(&mut self, size: usize) -> &'a [u8];
}

pub struct BinaryReader<'b> {
    buffer: &'b [u8],
    pos: usize, 
}

impl<'b> BinaryReader<'b> {
    pub fn from(buffer: &'b [u8]) -> Self {
        BinaryReader {
            buffer: buffer,
            pos: 0,
        }
    }
}

impl<'b> DataReader<'b> for BinaryReader<'b> {
    fn read(&mut self, size: usize) -> &'b [u8] {
        let read_size = cmp::min(size, self.buffer.len() - self.pos);
        let data = &self.buffer[self.pos..self.pos + read_size];
        self.pos += read_size;
        data
    }
}

pub type DeserializeResult<T> = Result<T, DeserializeError>;
pub trait Deserialize<T> {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<T>;
}

#[derive(Debug)]
pub enum DeserializeError {
    InvalidSize,
    AllocationFaild,
}

pub struct Deserializer<'a> {
    reader: &'a mut DataReader<'a>,
}

macro_rules! impl_deserializer_fn {
    () => {};
    ($type: ident $(, $others: ident)*) => {
        concat_idents::concat_idents!(fn_name = deserialize_, $type, {
            pub fn fn_name(&mut self) -> DeserializeResult<$type> {
                <$type as Deserialize<$type>>::deserialize(self)
            }
        });
        impl_deserializer_fn!($($others), *);
    };
}

macro_rules! impl_deserialize {
    () => {};
    ($type: ident $(, $others: ident)*) => {
        impl Deserialize<$type> for $type {

            fn deserialize<'a>(
                deserializer: &mut Deserializer<'a>,
            ) -> DeserializeResult<$type> {
                let buffer = deserializer.reader.read(size_of::<$type>());
                if buffer.len() < size_of::<$type>() {
                    return Err(DeserializeError::InvalidSize);
                } else {
                    unsafe {
                        let ptr = buffer.as_ptr() as *const $type;
                        return Ok(*ptr);
                    }
                }
            }

        }

        impl_deserialize!($($others), *);

    };
}

impl_deserialize!(bool, i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl<'a> Deserializer<'a> {
    pub fn new(reader: &'a mut DataReader<'a>) -> Self {
        Deserializer {
            reader: reader
        }
    }
    impl_deserializer_fn!(bool, i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);
    pub fn deserialize<T: Deserialize<T>>(&mut self) -> DeserializeResult<T> {
        <T as Deserialize<T>>::deserialize(self)
    }
    pub fn deserialize_array<'b, T>(&mut self, allocator: fn(usize)-> Option<&'b mut [T]>) -> DeserializeResult<&'b mut [T]>
    where T : Deserialize<T> {
        let count = self.deserialize_usize().unwrap();
        let arr = allocator(count);

        match arr {
            None => return Err(DeserializeError::AllocationFaild),
            Some(a) => {
                for i in 0..count {
                    a[i] = self.deserialize::<T>()?
                }
            
                return Ok(a);
            }
        }
    }
    pub fn deserialize_vec<T>(&mut self, mut vec: alloc::vec::Vec<T>) -> DeserializeResult<alloc::vec::Vec<T>>  where T: Deserialize<T>
    {
        let len = self.deserialize_usize()?;
        for i in 0..len {
            vec.push(self.deserialize()?);
        }
        Ok(vec)
    }
    pub fn deserialize_u8_array(&mut self) -> DeserializeResult<&'a [u8]> {
        let size = self.deserialize_usize()?;
        let buffer = self.reader.read(size);
        if buffer.len() != size {
            return Err(DeserializeError::InvalidSize);
        }
        return Ok(buffer);
    }
    pub fn deserialize_u8_array_alloc<'b>(&mut self, allocator: fn(usize) -> Option<&'b mut [u8]>) -> DeserializeResult<&'b mut [u8]> {
        let buffer = self.deserialize_u8_array()?;
        match allocator(buffer.len()) {
            Some(buf) if buf.len() >= buffer.len() => {
                buf[..buffer.len()].copy_from_slice(buffer);
                Ok(buf)
            },
            _ => Err(DeserializeError::InvalidSize)
        }
    }
    pub fn deserialize_str<'b>(&mut self, allocator: fn(usize) -> Option<&'b mut [u8]>) -> DeserializeResult<&'b mut str> {
        let buffer = self.deserialize_u8_array()?;
        match allocator(buffer.len()) {
            Some(string) if string.len() >= buffer.len() => {
                unsafe {
                    string[..buffer.len()].copy_from_slice(buffer);
                }
                return Ok(core::str::from_utf8_mut(string).unwrap());
            },
            _ => Err(DeserializeError::AllocationFaild),
        }
    }
    pub fn deserialize_u8_vec(&mut self) -> DeserializeResult<Vec<u8>> {
        let buffer = self.deserialize_u8_array()?;
        let mut vec = Vec::<u8>::with_capacity(buffer.len());
        vec.extend_from_slice(buffer);
        Ok(vec)
    }
}

// impl Deserialize<i32> for i32 {
//     fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<i32> {
//         let buffer = deserializer.reader.read(4);
//         5
//     }
// }

pub fn deserialize<T: Deserialize<T>>(buffer: &[u8]) -> DeserializeResult<T> {
    let mut reader = BinaryReader::from(buffer);
    let mut deserializer = Deserializer::new(&mut reader);
    deserializer.deserialize()
}