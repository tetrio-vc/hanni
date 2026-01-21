window.IS_ELECTRON = true;
window.IPC = {
    send: (...args) => {
        console.log("[IPC.send]", ...args);
        switch (args[0]) {
            case "close":
                window.__TAURI__.core.invoke("close_app");
                break;
        }
    },
    on: (...args) => {
        console.log("[IPC.on]", ...args);
    },
    invoke: (...args) => {
        console.log("[IPC.invoke]", ...args);
        return Promise.resolve(null);
    }
};
window.CLIENT_VERSION = 10;
