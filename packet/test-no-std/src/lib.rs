#![no_std]
// #![feature(alloc)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate packet;
extern crate rand;

use core::alloc::GlobalAlloc;

struct ExternAlloc {}

impl ExternAlloc {}

unsafe impl GlobalAlloc for ExternAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        extern_alloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        extern_dealloc(ptr)
    }
}

unsafe impl Sync for ExternAlloc {}

#[global_allocator]
static STUPID_HEAP: ExternAlloc = ExternAlloc {};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: alloc::alloc::Layout) -> ! {
    loop {}
}

extern "C" {
    fn extern_alloc(size: usize) -> *mut u8;
    fn extern_dealloc(ptr: *mut u8);
}

use packet::deserialize;
use packet::packets::CapturedPacket;
use packet::serialize;

use rand::Rng;
use rand::SeedableRng;

#[no_mangle]
extern "C" fn rust_main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
    let vec = alloc::vec::Vec::<i32>::with_capacity(123);
    let mut payload = alloc::vec![0; 34];
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
