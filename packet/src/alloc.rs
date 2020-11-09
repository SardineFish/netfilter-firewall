pub fn alloc_u8_array<'a>(
    size: usize,
    allocator: &dyn core::alloc::GlobalAlloc,
) -> Option<&'a mut [u8]> {
    unsafe {
        let ptr = allocator.alloc(core::alloc::Layout::from_size_align(size, 1).unwrap());
        return Some(core::slice::from_raw_parts_mut(ptr, size));
    }
}

pub fn dealloc_u8_array<'a>(buffer: &'a mut [u8], allocator: &dyn core::alloc::GlobalAlloc) {
    unsafe {
        allocator.dealloc(
            buffer.as_mut_ptr(),
            core::alloc::Layout::from_size_align(buffer.len(), 1).unwrap(),
        );
    }
}
