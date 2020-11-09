use crate::kernel_bindings::module;
use crate::kernel_bindings::bindings;


pub fn netlink_kernel_create(net: *mut bindings::net, unit:i32, cfg: *mut bindings::netlink_kernel_cfg) -> *mut bindings::sock {
    unsafe {
        return bindings::__netlink_kernel_create(net, unit, module::this_module(), cfg);
    }
}