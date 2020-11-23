#include <linux/netfilter.h>
#include <linux/netfilter_ipv4.h>
#include <linux/netlink.h>
#include <linux/skbuff.h>
#include <linux/slab.h>
#include <net/net_namespace.h>
#include <net/netlink.h>
#include <linux/ip.h>
#include <linux/tcp.h>
#include <linux/udp.h>
#include <linux/icmp.h>
// #include "./helper.h"

void* nlmsg_data_non_inline(struct nlmsghdr* nlh)
{
    return NLMSG_DATA(nlh);
}

struct netlink_skb_parms netlink_cb(struct sk_buff* skb)
{
    return NETLINK_CB(skb);
}

struct nlmsghdr* nlmsg_put_wrapped(struct sk_buff* skb, u32 portid, u32 seq, u16 type, u32 len, u16 flags)
{
    return nlmsg_put(skb, portid, seq, type, len, flags);
}

struct sk_buff* nlmsg_new_non_inline(size_t payload, gfp_t flags)
{
    struct sk_buff* skb = nlmsg_new(payload, flags);
    return skb;
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

static void test_nl_receive_message(struct sk_buff* skb)
{
    printk("Receive pakcet in C.\n");
}

struct iphdr* ip_hdr_wrapped(const struct sk_buff* skb)
{
    return ip_hdr(skb);
}

struct tcphdr* tcp_hdr_wrapped(const struct sk_buff* skb)
{
    return tcp_hdr(skb);
}

struct icmphdr* icmp_hdr_wrapped(const struct sk_buff* skb)
{
    return icmp_hdr(skb);
}

struct udphdr* udp_hdr_wrapped(const struct sk_buff* skb)
{
    return udp_hdr(skb);
}

unsigned int hook_func(void* priv, struct sk_buff* skb, const struct nf_hook_state* state)
{
    register struct iphdr* ip_header = ip_hdr(skb);
    if(ip_header->protocol == IPPROTO_TCP) {
        register struct tcphdr* tcp_header = tcp_hdr(skb);
        printk("TCP %d -> %d\n", htons(tcp_header->source), htons(tcp_header->dest));
        printk("TCP == IPv4 + 20 ? %d\n", ((void*)tcp_header) == ((void*)ip_header + 20));
    }
    return NF_ACCEPT;
}

static struct nf_hook_ops nfho;

extern void extern_code(void)
{
    nfho.hook = hook_func;
    nfho.pf = PF_INET;
    nfho.hooknum = NF_INET_PRE_ROUTING;
    nfho.priority = NF_IP_PRI_FIRST;

    nf_register_net_hook(&init_net, &nfho);
    printk("registered net hook\n");
}

extern void extern_cleanup(void)
{
    nfho.hook = hook_func;
    nfho.pf = PF_INET;
    nfho.hooknum = NF_INET_PRE_ROUTING;
    nfho.priority = NF_IP_PRI_FIRST;
    nf_unregister_net_hook(&init_net, &nfho);
    printk("unregister net hook\n");
}