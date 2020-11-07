#![no_std]
#![feature(panic_info_message)]

mod kernel_bindings;
use kernel_bindings::bindings;
use kernel_bindings::netlink;
use kernel_bindings::printk;

fn init() -> i32 {
    // printk::printk("init module!\n");
    println!("init module!");
    println!("1 + 1 = {}", 1 + 1);

    unsafe {
        let mut cfg = kernel_bindings::bindings::netlink_kernel_cfg {
            input: Some(input),
            ..Default::default()
        };
        // kernel_bindings::netlink::netlink_kernel_create(&mut netlink::init_net, 17, &mut cfg);
        kernel_bindings::netlink::NetLinkBuilder::new()
            .unit(16)
            .callback(msg_callback)
            .create();
    }

    return 0;
}

fn msg_callback(msg: &kernel_bindings::netlink::NetLinkMessge) {
    // kernel_bindings::printk(b"Received netlink packet %d", msg.data.len());
    println!("received netlink packet {}", msg.data.len());
}

extern "C" fn input(buf: *mut bindings::sk_buff) {
    unsafe {
        let header: *mut bindings::nlmsghdr = (*buf).data as *mut bindings::nlmsghdr;

    }
}

fn exit() {
    // kernel_bindings::printk("exit module!\n");
    println!("exit module");
    // kernel_bindings::printk::printk("exit module\n");

}

module_init!(init);
module_exit!(exit);

module_license!("GPL-3");
module_author!("SardineFish");
module_description!("A kernel module written in rust.");
module_version!("0.0.1");

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! 
{
    println!("error");
    let mut writer = printk::LogWriter::new();
    core::fmt::write(&mut writer,* _info.message().unwrap());
    printk::printk(writer.to_str());
    printk::printk("\n");
    loop {}
}
