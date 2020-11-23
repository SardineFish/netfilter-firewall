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
const react_dom_1 = __importDefault(require("react-dom"));
const antd_1 = require("antd");
const packet_list_1 = require("../components/packet-list");
const action_1 = require("../components/action");
const { Sider, Content, Header } = antd_1.Layout;
function App() {
    const [capture, setCapture] = react_1.useState(false);
    return (react_1.default.createElement(react_1.default.Fragment, null,
        react_1.default.createElement(antd_1.Layout, { className: "layout-root" },
            react_1.default.createElement(Header, { className: "header" },
                react_1.default.createElement(action_1.ActionPanel, { onCaptureChanged: (capture) => setCapture(capture) })),
            react_1.default.createElement(antd_1.Layout, { className: "content" },
                react_1.default.createElement(Content, { className: "layout-capture" },
                    react_1.default.createElement(packet_list_1.PacketList, { capture: capture })),
                react_1.default.createElement(Sider, { className: "layout-packet", width: 400, theme: "light" })))));
}
const element = (react_1.default.createElement(App, null));
react_dom_1.default.render(element, document.querySelector("#root"));
//# sourceMappingURL=index.js.map