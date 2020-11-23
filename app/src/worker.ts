console.log("worker");

// onmessage = () => {
//     postMessage("fuck up");
// }


import addon, { CapturedPacket, capturePacket, connectKernel, FilterRule, filterRules } from "../native";
import ipAddr, { Address4 } from "ip-address";

enum Protocol {
    IP = 0,
    ICMP = 1,
    IGMP = 2,
    IPIP = 4,
    TCP = 6,
    EGP = 8,
    PUP = 12,
    UDP = 17,
    IDP = 22,
    TP = 29,
    DCCP = 33,
    IPV6 = 41,
    RSVP = 46,
    GRE = 47,
    ESP = 50,
    AH = 51,
    MTP = 92,
    BEETPH = 94,
    ENCAP = 98,
    PIM = 103,
    COMP = 108,
    SCTP = 132,
    UDPLITE = 136,
    MPLS = 137,
    RAW = 255,
}

let initialRules: FilterRule = {
    sourceIP: 0,
    sourceMask: 0,
    sourcePort: 0,
    destIP: 0,
    destMask: 0,
    destPort: 0,
    protocol: 255
};

console.log("Setup worker.");

addon.connectKernel();
filterRules(initialRules);

console.log("Connected to kernel.");

onmessage = (e: MessageEvent<FilterRule>) => {
    const rule = e.data;
    filterRules(rule);
}

(async () => {
    while (true) {
        let packet = capturePacket();
        let srcAddr = Address4.fromInteger(packet.sourceIP).toArray().join(".");
        let dstAddr = Address4.fromInteger(packet.destIP).toArray().join(".");
        console.log(packet.payload);
        postMessage(packet, [packet.payload]);
        // console.log(`TCP ${srcAddr}:${packet.sourcePort} -> ${dstAddr}:${packet.destPort}`);
    }
})();
