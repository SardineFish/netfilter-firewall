#![no_std]

mod kernel_bindings;

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    return x + y;   
}

#[no_mangle]
pub extern "C" fn gcd_rust(x: i32, y: i32) -> i32 {
    if x % y == 0 {
       return y; 
    }
    return gcd_rust(y, x % y);
}

// #[no_mangle]
// pub extern "C" fn init_module() -> i32 {
//     kernel_bindings::printk("init module from rust!\n");
//     return 0;
// }

fn init() -> i32 {

    kernel_bindings::printk("init module!\n");
    return 0;
}

fn exit() {
    kernel_bindings::printk("exit module!\n");
}


module_init!(init);
module_exit!(exit);

module_license!("GPL-3");
module_author!("SardineFish");
module_description!("A kernel module written in rust.");
module_version!("0.0.1");

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}