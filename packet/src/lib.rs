// #![no_std]
#![cfg_attr(feature = "no_std", no_std, feature(alloc_error_handler))]
// #![feature(alloc)]
// #![feature(lang_items)]

extern crate alloc;

pub mod deserialize;
pub mod packets;
pub mod serialize;

#[cfg(test)]
mod test;

pub use deserialize::{deserialize, Deserialize};
pub use serialize::{serialize, Serialize};

// extern crate serde;

// use serde::{ Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct NetlinkHeader {
    nlmsg_len: u32,
    nlmsg_type: u16,
    nlmsg_flags: u16,
    nlmsg_seq: u32,
    nlmsg_pid: u32,
}

pub struct NetlinkMessage<T> {
    pub header: NetlinkHeader,
    pub payload: T,
}
impl<T> NetlinkMessage<T>
where
    T: Serialize,
{
    pub fn pack(&self, buffer: &mut [u8]) -> usize {
        serialize(&self.payload, buffer)
    }
}
impl<T> NetlinkMessage<T>
where
    T: Deserialize<T>,
{
    pub fn from(header: NetlinkHeader, payload_buffer: &[u8]) -> Self {
        NetlinkMessage {
            header: header,
            payload: deserialize::<T>(payload_buffer).unwrap(),
        }
    }
}

#[cfg(feature = "no_std")]
mod no_std {
    // #[panic_handler]
    // fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    //     loop {}
    // }

    // #[alloc_error_handler]
    // fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    //     loop {}
    // }

    struct GlobalAllocator {
        allocator: Option<&'static alloc::alloc::GlobalAlloc>,
    }

    unsafe impl alloc::alloc::GlobalAlloc for GlobalAllocator {
        unsafe fn alloc(&self, layout: alloc::alloc::Layout) -> *mut u8 {
            match self.allocator {
                Some(allocator) => allocator.alloc(layout),
                _ => core::ptr::null_mut(),
            }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::alloc::Layout) {
            match self.allocator {
                Some(allocator) => allocator.dealloc(ptr, layout),
                _ => ()
            }
        }
    }

    unsafe impl Sync for GlobalAllocator {
    }

    // #[global_allocator]
    // static mut GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator{
    //     allocator: None
    // };

    // pub fn set_global_allocator(allocator: &'static alloc::alloc::GlobalAlloc) {
    //     unsafe {
    //         GLOBAL_ALLOCATOR.allocator = Some(allocator);
    //     }
    // }
}
