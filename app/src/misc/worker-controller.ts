import { CapturedPacket, FilterRule } from "native";

const worker = new Worker("../lib/worker-wrapper.js");
console.log("load web worker.");

function setPacketCaptureCallback(callback: (packet: CapturedPacket) => void)
{
    worker.addEventListener("message", (e) => callback(e.data));
}

function setFilterRule(rule: FilterRule)
{
    // filterRules(rule);
    worker.postMessage(rule);
}

const defaultRule: FilterRule = {
    sourceIP: 0,
    destIP: 0,
    sourceMask: 0,
    destMask: 0,
    sourcePort: 0,
    destPort: 0,
    protocol: 255
};

const stopRule: FilterRule = {
    sourceIP: 0,
    destIP: 0,
    sourceMask: 0,
    destMask: 0,
    sourcePort: 0,
    destPort: 0,
    protocol: 0
};

function startCapture(rule = defaultRule)
{
    setFilterRule(rule);
}

function stopCapture()
{
    setFilterRule(stopRule);
}

export default {
    setPacketCaptureCallback,
    setFilterRule,
    startCapture,
    stopCapture,
};