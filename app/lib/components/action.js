"use strict";
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (Object.hasOwnProperty.call(mod, k)) result[k] = mod[k];
    result["default"] = mod;
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const react_1 = __importStar(require("react"));
const antd_1 = require("antd");
const icons_1 = require("@ant-design/icons");
const Form_1 = __importDefault(require("antd/lib/form/Form"));
const ip_address_1 = require("ip-address");
const protocol_1 = require("../misc/protocol");
const worker_controller_1 = __importDefault(require("../misc/worker-controller"));
function ActionPanel(props) {
    const [capture, setCapture] = react_1.useState(false);
    const [srcIp, setSrcIp] = react_1.useState("");
    const [dstIp, setDstIp] = react_1.useState("");
    const [srcMask, setSrcMask] = react_1.useState("");
    const [dstMask, setDstMask] = react_1.useState("");
    const [srcPort, setSrcPort] = react_1.useState("");
    const [dstPort, setDstPort] = react_1.useState("");
    const [protocol, setProtocol] = react_1.useState("");
    const [dirty, setDirty] = react_1.useState(false);
    const changeCapture = (checked) => {
        setCapture(checked);
        // props.onCaptureChanged(checked);
        if (checked)
            worker_controller_1.default.startCapture();
        else
            worker_controller_1.default.stopCapture();
    };
    const changeCallback = (callbackFunc) => (e) => {
        callbackFunc(e.target.value);
        setDirty(true);
    };
    const reset = () => {
        setCapture(false);
        setSrcIp("");
        setDstIp("");
        setSrcMask("");
        setDstMask("");
        setSrcPort("");
        setDstPort("");
        setProtocol("");
        setDirty(false);
        if (capture)
            worker_controller_1.default.startCapture();
    };
    const applyFilter = () => {
        let protocolNum = protocol_1.Protocol[protocol];
        protocolNum = protocolNum === undefined ? protocol_1.Protocol.RAW : protocolNum;
        const rule = {
            sourceIP: Number(new ip_address_1.Address4(!srcIp ? "0.0.0.0" : srcIp).bigInteger()),
            destIP: Number(new ip_address_1.Address4(!dstIp ? "0.0.0.0" : dstIp).bigInteger()),
            sourceMask: Number(new ip_address_1.Address4(!srcMask ? "0.0.0.0" : srcMask).bigInteger()),
            destMask: Number(new ip_address_1.Address4(!dstMask ? "0.0.0.0" : dstMask).bigInteger()),
            sourcePort: parseInt(!srcPort ? "0" : srcPort),
            destPort: parseInt(!dstPort ? "0" : dstPort),
            protocol: protocolNum,
        };
        setDirty(false);
        console.log(rule);
        if (capture)
            worker_controller_1.default.setFilterRule(rule);
    };
    return (react_1.default.createElement(Form_1.default, { className: "filter-form", layout: "inline" },
        react_1.default.createElement(antd_1.Switch, { checked: capture, onChange: e => changeCapture(e), checkedChildren: react_1.default.createElement(icons_1.CameraOutlined, null), unCheckedChildren: react_1.default.createElement(icons_1.ApiOutlined, null) }),
        react_1.default.createElement(antd_1.Input.Group, { compact: true, style: { width: "30%" } },
            react_1.default.createElement(antd_1.Input, { placeholder: "Source IP", style: { width: "40%" }, value: srcIp, onChange: changeCallback(setSrcIp) }),
            react_1.default.createElement(antd_1.Input, { placeholder: "Source Mask", style: { width: "40%" }, value: srcMask, onChange: changeCallback(setSrcMask) }),
            react_1.default.createElement(antd_1.Input, { placeholder: "Port", style: { width: "20%" }, value: srcPort, onChange: changeCallback(setSrcPort) })),
        react_1.default.createElement(antd_1.Input.Group, { compact: true, style: { width: "30%" } },
            react_1.default.createElement(antd_1.Input, { placeholder: "Destination IP", style: { width: "40%" }, value: dstIp, onChange: changeCallback(setDstIp) }),
            react_1.default.createElement(antd_1.Input, { placeholder: "Destination Mask", style: { width: "40%" }, value: dstMask, onChange: changeCallback(setDstMask) }),
            react_1.default.createElement(antd_1.Input, { placeholder: "Port", style: { width: "20%" }, value: dstPort, onChange: changeCallback(setDstPort) })),
        react_1.default.createElement(antd_1.Select, { style: { width: "10em" }, placeholder: "Protocol", value: protocol, onChange: e => setProtocol(e) }, protocols.map((protocol) => (react_1.default.createElement(antd_1.Select.Option, { key: protocol, value: protocol }, protocol)))),
        react_1.default.createElement(antd_1.Button, { shape: "circle", icon: react_1.default.createElement(icons_1.ReloadOutlined, null), onClick: reset }),
        dirty
            ? react_1.default.createElement(antd_1.Button, { type: "primary", onClick: applyFilter }, "Apply")
            : null));
}
exports.ActionPanel = ActionPanel;
const protocols = [
    "<Any>",
    "IP",
    "ICMP",
    "IGMP",
    "IPIP",
    "TCP",
    "EGP",
    "PUP",
    "UDP",
    "IDP",
    "TP",
    "DCCP",
    "IPV6",
    "RSVP",
    "GRE",
    "ESP",
    "AH",
    "MTP",
    "BEETPH",
    "ENCAP",
    "PIM",
    "COMP",
    "SCTP",
    "UDPLITE",
    "MPLS",
];
//# sourceMappingURL=action.js.map