"use strict";
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (Object.hasOwnProperty.call(mod, k)) result[k] = mod[k];
    result["default"] = mod;
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const native_1 = __importStar(require("../native"));
const ip_address_1 = require("ip-address");
var Protocol;
(function (Protocol) {
    Protocol[Protocol["IP"] = 0] = "IP";
    Protocol[Protocol["ICMP"] = 1] = "ICMP";
    Protocol[Protocol["IGMP"] = 2] = "IGMP";
    Protocol[Protocol["IPIP"] = 4] = "IPIP";
    Protocol[Protocol["TCP"] = 6] = "TCP";
    Protocol[Protocol["EGP"] = 8] = "EGP";
    Protocol[Protocol["PUP"] = 12] = "PUP";
    Protocol[Protocol["UDP"] = 17] = "UDP";
    Protocol[Protocol["IDP"] = 22] = "IDP";
    Protocol[Protocol["TP"] = 29] = "TP";
    Protocol[Protocol["DCCP"] = 33] = "DCCP";
    Protocol[Protocol["IPV6"] = 41] = "IPV6";
    Protocol[Protocol["RSVP"] = 46] = "RSVP";
    Protocol[Protocol["GRE"] = 47] = "GRE";
    Protocol[Protocol["ESP"] = 50] = "ESP";
    Protocol[Protocol["AH"] = 51] = "AH";
    Protocol[Protocol["MTP"] = 92] = "MTP";
    Protocol[Protocol["BEETPH"] = 94] = "BEETPH";
    Protocol[Protocol["ENCAP"] = 98] = "ENCAP";
    Protocol[Protocol["PIM"] = 103] = "PIM";
    Protocol[Protocol["COMP"] = 108] = "COMP";
    Protocol[Protocol["SCTP"] = 132] = "SCTP";
    Protocol[Protocol["UDPLITE"] = 136] = "UDPLITE";
    Protocol[Protocol["MPLS"] = 137] = "MPLS";
    Protocol[Protocol["RAW"] = 255] = "RAW";
})(Protocol || (Protocol = {}));
let rules = {
    sourceIP: 0,
    sourceMask: 0,
    sourcePort: 0,
    destIP: 0,
    destMask: 0,
    destPort: 0,
    protocol: Protocol.TCP
};
native_1.default.connectKernel();
native_1.filterRules(rules);
while (true) {
    let packet = native_1.capturePacket();
    let srcAddr = ip_address_1.Address4.fromInteger(packet.sourceIP).toArray().join(".");
    let dstAddr = ip_address_1.Address4.fromInteger(packet.destIP).toArray().join(".");
    console.log(`TCP ${srcAddr}:${packet.sourcePort} -> ${dstAddr}:${packet.destPort}`);
}
//# sourceMappingURL=test.js.map