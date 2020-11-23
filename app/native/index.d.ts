export enum Protocol
{
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

export interface FilterRule
{
    sourceIP: number,
    sourceMask: number,
    sourcePort: number,
    destIP: number,
    destMask: number,
    destPort: number,
    protocol: Protocol,
}

export interface CapturedPacket
{
    sourceIP: number;
    destIP: number;
    sourcePort: number;
    destPort: number;
    protocol: number;
    payload: ArrayBuffer;
}

export type CaptureCallback = (packet: CapturedPacket) => void;

declare function connectKernel(): void;

declare function filterRules(rules: FilterRule): void;

declare function capturePacket(): CapturedPacket;

export {
    connectKernel,
    filterRules,
    capturePacket,
}