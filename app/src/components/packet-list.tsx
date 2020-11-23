import React from "react";
import { CapturedPacket, connectKernel } from "native";
import { Table } from "antd";
import { ColumnType } from "antd/lib/list";
import { ColumnsType } from "antd/lib/table";
import { Address4 } from "ip-address";
import { Protocol } from "../misc/protocol";
import workerController from "../misc/worker-controller";

interface PacketListProps {
    className?: string;
    onPacketSelect?: (packet: CapturedPacket) => void;
    capture: boolean;
}

let worker: Worker | null = null;

interface PacketInfo {
    key: number,
    src: string;
    dest: string;
    protocol: string;
    payloadSize: string;
}

interface PacketListState {
    packets: PacketInfo[];
}

const columns: ColumnsType<PacketInfo> = [
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

export class PacketList extends React.Component<PacketListProps, PacketListState>
{
    packetID = 0;
    constructor(props: PacketListProps) {
        super(props);
        this.state = {
            packets: []
        };
    }
    componentDidMount() {
        // if (!worker) {
        //     worker = new Worker("../lib/worker-wrapper.js");
        //     worker.addEventListener("message", (e) => this.onPacket(e));
        //     console.log("load web worker.");
        // }

        workerController.setPacketCaptureCallback(packet => this.onPacket(packet));

    }
    onPacket(packet: CapturedPacket) {
        let srcIP = Address4.fromInteger(packet.sourceIP).toArray().join(".");
        let dstIP = Address4.fromInteger(packet.destIP).toArray().join(".");
        let info: PacketInfo = {
            key: ++this.packetID,
            src: `${srcIP}:${packet.sourcePort}`,
            dest: `${dstIP}:${packet.destPort}`,
            payloadSize: packet.payload.byteLength.toString(),
            protocol: Protocol[packet.protocol].toString(),
        }
        this.setState({
            packets: [...this.state.packets, info]
        });
    }
    render() {
        return (
            <Table columns={columns} dataSource={this.state.packets} pagination={false} />
        );
    }
}