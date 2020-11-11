import React from "react";
import ReactDOM from "react-dom";
import antd, { Layout } from "antd";
import { PacketList } from "../components/packet-list";

const { Sider, Content, Header } = Layout;

function App()
{
    return (<>
        <Layout className="layout-root">
            <Header>
                Packet Capture
            </Header>
            <Content className="content" >
                <Content className="layout-capture">
                    <PacketList></PacketList>
                </Content>
                <Sider className="layout-packet" width={400}>

                </Sider>
            </Content>
        </Layout>
    </>);
}

const element = (<App />);
ReactDOM.render(element, document.querySelector("#root"));
