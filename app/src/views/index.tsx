import React, { useState } from "react";
import ReactDOM from "react-dom";
import antd, { Layout } from "antd";
import { PacketList } from "../components/packet-list";
import { ActionPanel } from "../components/action";

const { Sider, Content, Header } = Layout;

function App()
{
    return (<>
        <Layout className="layout-root">
            <Header className="header">
                <ActionPanel/>
            </Header>
            <Layout className="content">
                <Content className="layout-capture">
                    <PacketList></PacketList>
                </Content>
                <Sider className="layout-packet" width={400} theme="light">

                </Sider>
            </Layout>
        </Layout>
    </>);
}

const element = (<App />);
ReactDOM.render(element, document.querySelector("#root"));
