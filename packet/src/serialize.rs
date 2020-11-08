use core::cmp;
use core::mem::size_of;
use core::slice;

pub struct BinaryWriter<'p> {
    buffer: &'p mut [u8],
    pos: usize,
}

pub trait DataWriter<'w> {
    fn write(&mut self, data: &[u8]) -> usize;
}

impl<'p> BinaryWriter<'p> {
    pub fn from(ptr: &'p mut [u8]) -> Self {
        BinaryWriter {
            buffer: ptr,
            pos: 0,
        }
    }
}

impl<'p> DataWriter<'p> for BinaryWriter<'p> {
    fn write(&mut self, data: &[u8]) -> usize {
        let write_size = cmp::min(self.buffer.len(), self.pos + data.len());
        self.buffer[self.pos..self.pos + write_size].copy_from_slice(&data[..write_size]);
        self.pos += write_size;
        write_size
    }
}

pub struct Serializer<'w> {
    writer: &'w mut DataWriter<'w>,
    length: usize,
}

impl<'w> Serializer<'w> {
    pub fn new<T: DataWriter<'w>>(writer: &'w mut T) -> Serializer<'w> {
        Serializer {
            writer: writer,
            length: 0,
        }
    }
}

pub trait Serialize {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s>;
}

impl<'b> Serializer<'b> {
    pub fn serialize<T: Serialize>(self, data: &T) -> Self {
        data.serialize(self)
    }
}

macro_rules! impl_serialize {
    () => ();
    ($type: ident $(, $others: ident)*) => {
        impl Serialize for $type {
            fn serialize<'s>(&self, mut serializer: Serializer<'s>) -> Serializer<'s> {
                unsafe {
                    let ptr = self as *const $type as *const u8;
                    let slice = slice::from_raw_parts(ptr, size_of::<$type>());
                    serializer.length += serializer.writer.write(slice);

                    serializer
                }
            }
        }
        impl_serialize!($($others), *);
    }
}

impl_serialize!(bool, i8, i16, i32, i64, u8, u16, u32, u64, usize, isize);

impl<T: Serialize> Serialize for &[T]{
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        let mut serializer = serializer.serialize(&self.len());

        for i in 0..self.len() {
            serializer = serializer.serialize(&self[i]);
        }

        serializer
    }
}

pub fn serialize<'b, T>(target: &T, buffer: &mut [u8]) -> usize
where
    T: Serialize,
{
    let mut writer = BinaryWriter::from(buffer);
    let serializer = Serializer::new(&mut writer);

    target.serialize(serializer).length
}
