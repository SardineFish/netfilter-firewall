#![no_std]
// #![feature(alloc)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::GlobalAlloc;

struct StupidHeep {
}

impl StupidHeep {
}

unsafe impl GlobalAlloc for StupidHeep {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
    }
}

unsafe impl Sync for StupidHeep {}

#[global_allocator]
static STUPID_HEAP: StupidHeep = StupidHeep {
};

fn main() {

}

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