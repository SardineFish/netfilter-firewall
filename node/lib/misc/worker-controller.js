"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const worker = new Worker("../lib/worker-wrapper.js");
console.log("load web worker.");
function setPacketCaptureCallback(callback) {
    worker.addEventListener("message", (e) => callback(e.data));
}
function setFilterRule(rule) {
    // filterRules(rule);
    worker.postMessage(rule);
}
const defaultRule = {
    sourceIP: 0,
    destIP: 0,
    sourceMask: 0,
    destMask: 0,
    sourcePort: 0,
    destPort: 0,
    protocol: 255
};
const stopRule = {
    sourceIP: 0,
    destIP: 0,
    sourceMask: 0,
    destMask: 0,
    sourcePort: 0,
    destPort: 0,
    protocol: 0
};
function startCapture(rule = defaultRule) {
    setFilterRule(rule);
}
function stopCapture() {
    setFilterRule(stopRule);
}
exports.default = {
    setPacketCaptureCallback,
    setFilterRule,
    startCapture,
    stopCapture,
};
//# sourceMappingURL=worker-controller.js.map