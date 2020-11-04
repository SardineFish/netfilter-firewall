#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("SardineFish");
MODULE_DESCRIPTION("A linux kernel module for test.");
MODULE_VERSION("0.0.1");

int add(int x, int y);
int gcd_rust(int x, int y);

static int init(void)
{
    printk("Load kmod-test.\n");
    printk("gcd(48, 64) = %d, called from C & calculated in Rust.\n", gcd_rust(48, 64));
    return 0;
}

static void exit(void)
{
    printk("Unload kmod-test.\n");
}

module_init(init);
module_exit(exit);