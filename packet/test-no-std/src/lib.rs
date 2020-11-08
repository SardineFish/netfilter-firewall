#![no_std]
#![feature(lang_items, start)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}

fn main() {

}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {

}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}