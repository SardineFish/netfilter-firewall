import React, { useState } from "react";
import { Button, Input, Select, Switch } from "antd";
import { ApiOutlined, CameraOutlined, FilterFilled, FilterOutlined, ReloadOutlined } from "@ant-design/icons";
import Form from "antd/lib/form/Form";
import { Address4 } from "ip-address";
import { FilterRule } from "native";
import { Protocol } from "../misc/protocol";
import workerController from "../misc/worker-controller";

export function ActionPanel(props: { onCaptureChanged: (capture: boolean) => void }) {
    const [capture, setCapture] = useState(false);
    const [srcIp, setSrcIp] = useState("");
    const [dstIp, setDstIp] = useState("");
    const [srcMask, setSrcMask] = useState("");
    const [dstMask, setDstMask] = useState("");
    const [srcPort, setSrcPort] = useState("");
    const [dstPort, setDstPort] = useState("");
    const [protocol, setProtocol] = useState("");
    const [dirty, setDirty] = useState(false);
    
    

    const changeCapture = (checked: boolean) => {
        setCapture(checked);
        // props.onCaptureChanged(checked);
        if (checked)
            workerController.startCapture();
        else
            workerController.stopCapture();
    }

    const changeCallback = (callbackFunc: (value: string) => void) =>
        (e: React.ChangeEvent<HTMLInputElement>) => {
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
            workerController.startCapture();
    }
    
    const applyFilter = () => {
        let protocolNum = Protocol[protocol as any] as any as number;
        protocolNum = protocolNum === undefined ? Protocol.RAW : protocolNum;

        const rule: FilterRule = {
            sourceIP: Number(new Address4(!srcIp ? "0.0.0.0" : srcIp).bigInteger()),
            destIP: Number(new Address4(!dstIp ? "0.0.0.0" : dstIp).bigInteger()),
            sourceMask: Number(new Address4(!srcMask ? "0.0.0.0" : srcMask).bigInteger()),
            destMask: Number(new Address4(!dstMask ? "0.0.0.0" : dstMask).bigInteger()),
            sourcePort: parseInt(!srcPort ? "0" : srcPort),
            destPort: parseInt(!dstPort ? "0" : dstPort),
            protocol: protocolNum,
        }

        setDirty(false);
        console.log(rule);

        if (capture)
            workerController.setFilterRule(rule);
    }

    return (<Form className="filter-form" layout="inline">
        <Switch checked={capture} onChange={e => changeCapture(e)} checkedChildren={<CameraOutlined />} unCheckedChildren={<ApiOutlined />}></Switch>
        <Input.Group compact style={{ width: "30%"}}>
            <Input placeholder="Source IP" style={{ width: "40%" }} value={srcIp} onChange={changeCallback(setSrcIp)} />
            <Input placeholder="Source Mask" style={{ width: "40%" }} value={srcMask} onChange={changeCallback(setSrcMask)} />
            <Input placeholder="Port" style={{ width: "20%" }} value={srcPort} onChange={changeCallback(setSrcPort)} />
        </Input.Group >
        <Input.Group compact style={{ width: "30%" }}>
            <Input placeholder="Destination IP" style={{ width: "40%" }} value={dstIp} onChange={changeCallback(setDstIp)}/>
            <Input placeholder="Destination Mask" style={{ width: "40%" }} value={dstMask} onChange={changeCallback(setDstMask)} />
            <Input placeholder="Port" style={{ width: "20%" }} value={dstPort} onChange={changeCallback(setDstPort)} />
        </Input.Group>
        <Select style={{width: "10em"}} placeholder="Protocol" value={protocol} onChange={e=>setProtocol(e)}>
            {protocols.map((protocol) => (<Select.Option key={protocol} value={protocol}>{protocol}</Select.Option>))}
        </Select>
        <Button shape="circle" icon={<ReloadOutlined />} onClick={reset}/>
        {
            dirty
                ? <Button type="primary" onClick={applyFilter}>Apply</Button>
                : null
        }
    </Form>);
}

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
]