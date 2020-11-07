#include <linux/netlink.h>
#include <linux/skbuff.h>
#include <linux/slab.h>
#include <net/netlink.h>
#include <net/net_namespace.h>
// #include "./helper.h"

void* nlmsg_data_non_inline(struct nlmsghdr* nlh)
{
    return NLMSG_DATA(nlh);
}

struct netlink_skb_parms netlink_cb(struct sk_buff* skb)
{
    return NETLINK_CB(skb);
}

struct sk_buff* nlmsg_new_non_inline(size_t payload, gfp_t flags)
{
    return kmalloc(nlmsg_total_size(payload), flags);
}

void* kmalloc_wrapped(size_t size, gfp_t flags) 
{
    return kmalloc(size, flags);
}

void* kcalloc_wrapped(size_t n, size_t size, gfp_t flags) 
{
    return kcalloc(n, size, flags);
}

void kfree_wrapped(const void* ptr) 
{
    kfree(ptr);
}

static void test_nl_receive_message(struct sk_buff *skb) {
    printk("Receive pakcet in C.\n");
}

extern void extern_code(void)
{
    struct netlink_kernel_cfg config = {
        .input = test_nl_receive_message,
    };
    struct sock *socket = netlink_kernel_create(&init_net, 25, &config);
    if(!socket)
        printk("Failed to create netlink socket\n");
}