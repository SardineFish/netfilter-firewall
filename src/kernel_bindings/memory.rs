use super::bindings;
use super::c_types;

use crate::println;

use crate::alloc::alloc;

pub struct RawData {
    ptr: *mut u8,
    size: usize,
}

impl Default for RawData {
    fn default() -> Self {
        RawData {
            ptr: core::ptr::null_mut(),
            size: 0,
        }
    }
}

impl RawData {
    pub fn from_raw(ptr: *mut u8, size: usize) -> Self {
        RawData {
            ptr: ptr,
            size: size,
        }
    }
    pub fn alloc(size: usize) -> Option<Self> {
        unsafe {
            let ptr = bindings::kmalloc_wrapped(size, bindings::GFP_KERNEL);
            if ptr == core::ptr::null_mut() {
                return None;
            }
            return Some(RawData {
                ptr: ptr as *mut u8,
                size: size
            });
        }
    }
    pub fn free(self){
        unsafe {
            bindings::kfree_wrapped(self.ptr as *mut c_types::c_void);
        }

    }
    pub fn len(&self) -> usize {
        self.size
    }
}

pub struct KernelAlloc {

}

unsafe impl alloc::GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        let ptr = bindings::kmalloc_wrapped(layout.size(), bindings::GFP_KERNEL);
        println!("Alloc {} bytes aligned {} in kernel at {:#x}", layout.size(), layout.align(), ptr as usize);
        ptr as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        println!("Dealloc {} bytes aligned {} in kernel", layout.size(), layout.align());
        bindings::kfree_wrapped(ptr as *const c_types::c_void);
    }
}

unsafe impl Sync for KernelAlloc{}