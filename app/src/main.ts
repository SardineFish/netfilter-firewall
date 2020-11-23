import electron, { app, BrowserWindow, globalShortcut } from "electron";

app.on("ready", () => {
    const win = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true,
            
        }
    });

    win.loadFile("./pages/index.html");

    const result = globalShortcut.register("CommandOrControl+F5", () => {
        console.log("!");
        app.relaunch();
        app.exit(0);
    });

    if (!result)
        console.warn("Failed to register shortcut.");

    
});