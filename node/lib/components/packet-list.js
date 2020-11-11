"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const react_1 = __importDefault(require("react"));
const antd_1 = require("antd");
const ip_address_1 = require("ip-address");
const protocol_1 = require("../misc/protocol");
let worker = null;
const columns = [
    {
        title: "Source",
        key: "src",
        dataIndex: "src",
    },
    {
        title: "Destination",
        key: "dest",
        dataIndex: "dest",
    },
    {
        title: "Protocol",
        key: "protocol",
        dataIndex: "protocol",
    },
    {
        title: "Payload",
        key: "payloadSize",
        dataIndex: "payloadSize",
    }
];
class PacketList extends react_1.default.Component {
    constructor(props) {
        super(props);
        this.packetID = 0;
        this.state = {
            packets: []
        };
    }
    componentDidMount() {
        if (!worker) {
            worker = new Worker("../lib/worker-wrapper.js");
            worker.addEventListener("message", (e) => this.onPacket(e));
            console.log("load web worker.");
        }
    }
    onPacket(e) {
        let packet = e.data;
        let srcIP = ip_address_1.Address4.fromInteger(packet.sourceIP).toArray().join(".");
        let dstIP = ip_address_1.Address4.fromInteger(packet.destIP).toArray().join(".");
        let info = {
            key: ++this.packetID,
            src: srcIP,
            dest: dstIP,
            payloadSize: packet.payload.byteLength.toString(),
            protocol: protocol_1.Protocol[packet.protocol].toString(),
        };
        this.setState({
            packets: [...this.state.packets, info]
        });
    }
    render() {
        return (react_1.default.createElement(antd_1.Table, { columns: columns, dataSource: this.state.packets, pagination: false }));
    }
}
exports.PacketList = PacketList;
//# sourceMappingURL=packet-list.js.map