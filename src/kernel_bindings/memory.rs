use super::bindings;
use super::c_types;

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