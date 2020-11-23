"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const electron_1 = require("electron");
electron_1.app.on("ready", () => {
    const win = new electron_1.BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true,
        }
    });
    win.loadFile("./pages/index.html");
    const result = electron_1.globalShortcut.register("CommandOrControl+F5", () => {
        console.log("!");
        electron_1.app.relaunch();
        electron_1.app.exit(0);
    });
    if (!result)
        console.warn("Failed to register shortcut.");
});
//# sourceMappingURL=main.js.map