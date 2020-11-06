mod c_types;
mod module;
mod bindings;

pub fn printk(str: &str) {
    unsafe {
        bindings::printk(str.as_bytes().as_ptr());
    }
}