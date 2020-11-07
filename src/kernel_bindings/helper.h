#pragma once
#include <linux/netlink.h>

void* nlmsg_data_non_inline(struct nlmsghdr* nlh);

struct netlink_skb_parms netlink_cb(struct sk_buff* skb);

struct sk_buff* nlmsg_new_non_inline(size_t payload, gfp_t flags);

void* kmalloc_wrapped(size_t size, gfp_t flags);

void* kcalloc_wrapped(size_t n, size_t size, gfp_t flags);

void kfree_wrapped(const void* ptr);

struct GFP {
    enum
    {
        KERNEL = GFP_KERNEL
    };
};
