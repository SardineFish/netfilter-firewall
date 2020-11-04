#![no_std]

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

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}