#pragma once
#include <linux/netlink.h>

void* nlmsg_data_non_inline(struct nlmsghdr* nlh);

struct netlink_skb_parms netlink_cb(struct sk_buff* skb);

struct sk_buff* nlmsg_new_non_inline(size_t payload, gfp_t flags);

struct nlmsghdr* nlmsg_put_wrapped(struct sk_buff* skb, u32 portid, u32 seq, u16 type, u32 len, u16 flags);

void* kmalloc_wrapped(size_t size, gfp_t flags);

void* kcalloc_wrapped(size_t n, size_t size, gfp_t flags);

void kfree_wrapped(const void* ptr);

struct iphdr* ip_hdr_wrapped(const struct sk_buff* skb);

struct tcphdr* tcp_hdr_wrapped(const struct sk_buff* skb);

struct GFP
{
    enum
    {
        KERNEL = GFP_KERNEL
    };
};

struct ProtocolFamily
{
    enum
    {
        UNSPEC = 0,
        LOCAL = 1,
        UNIX = LOCAL,
        FILE = LOCAL,
        INET = 2,
        AX25 = 3,
        IPX = 4,
        APPLETALK = 5,
        NETROM = 6,
        BRIDGE = 7,
        ATMPVC = 8,
        X25 = 9,
        INET6 = 10,
        ROSE = 11,
        DECnet = 12,
        NETBEUI = 13,
        SECURITY = 14,
        KEY = 15,
        NETLINK = 16,
        ROUTE = NETLINK,
        PACKET = 17,
        ASH = 18,
        ECONET = 19,
        ATMSVC = 20,
        RDS = 21,
        RXRPC = 33,
        ISDN = 34,
        PHONET = 35,
        IEEE802154 = 36,
        CAIF = 37,
        ALG = 38,
        NFC = 39,
        VSOCK = 40,
        KCM = 41,
        QIPCRTR = 42,
        SMC = 43,
        XDP = 44,
        MAX = 45,
    };
};


struct IpProtocol {
    enum
    {
        IP = 0, /* Dummy protocol for TCP		*/

        ICMP = 1, /* Internet Control Message Protocol	*/

        IGMP = 2, /* Internet Group Management Protocol	*/

        IPIP = 4, /* IPIP tunnels (older KA9Q tunnels use 94) */

        TCP = 6, /* Transmission Control Protocol	*/

        EGP = 8, /* Exterior Gateway Protocol		*/

        PUP = 12, /* PUP protocol				*/

        UDP = 17, /* User Datagram Protocol		*/

        IDP = 22, /* XNS IDP protocol			*/

        TP = 29, /* SO Transport Protocol Class 4	*/

        DCCP = 33, /* Datagram Congestion Control Protocol */

        IPV6 = 41, /* IPv6-in-IPv4 tunnelling		*/

        RSVP = 46, /* RSVP Protocol			*/

        GRE = 47, /* Cisco GRE tunnels (rfc 1701,1702)	*/

        ESP = 50, /* Encapsulation Security Payload protocol */

        AH = 51, /* Authentication Header protocol	*/

        MTP = 92, /* Multicast Transport Protocol		*/

        BEETPH = 94, /* IP option pseudo header for BEET	*/

        ENCAP = 98, /* Encapsulation Header			*/

        PIM = 103, /* Protocol Independent Multicast	*/

        COMP = 108, /* Compression Header Protocol		*/

        SCTP = 132, /* Stream Control Transport Protocol	*/

        UDPLITE = 136, /* UDP-Lite (RFC 3828)			*/

        MPLS = 137, /* MPLS in IP (RFC 4023)		*/

        RAW = 255, /* Raw IP packets			*/
    };
};