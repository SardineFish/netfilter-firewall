"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const react_1 = __importDefault(require("react"));
const react_dom_1 = __importDefault(require("react-dom"));
const antd_1 = require("antd");
const packet_list_1 = require("../components/packet-list");
const { Sider, Content, Header } = antd_1.Layout;
function App() {
    return (react_1.default.createElement(react_1.default.Fragment, null,
        react_1.default.createElement(antd_1.Layout, { className: "layout-root" },
            react_1.default.createElement(Header, null, "Packet Capture"),
            react_1.default.createElement(Content, { className: "content" },
                react_1.default.createElement(Content, { className: "layout-capture" },
                    react_1.default.createElement(packet_list_1.PacketList, null)),
                react_1.default.createElement(Sider, { className: "layout-packet", width: 400 })))));
}
const element = (react_1.default.createElement(App, null));
react_dom_1.default.render(element, document.querySelector("#root"));
//# sourceMappingURL=index.js.map