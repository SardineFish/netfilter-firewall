import React, { useState } from "react";
import ReactDOM from "react-dom";
import antd, { Layout } from "antd";
import { PacketList } from "../components/packet-list";
import { ActionPanel } from "../components/action";

const { Sider, Content, Header } = Layout;

function App()
{
    const [capture, setCapture] = useState(false);
    return (<>
        <Layout className="layout-root">
            <Header className="header">
                <ActionPanel onCaptureChanged={(capture)=>setCapture(capture)}/>
            </Header>
            <Layout className="content">
                <Content className="layout-capture">
                    <PacketList capture={capture}></PacketList>
                </Content>
                <Sider className="layout-packet" width={400} theme="light">

                </Sider>
            </Layout>
        </Layout>
    </>);
}

const element = (<App />);
ReactDOM.render(element, document.querySelector("#root"));
