const { invoke } = window.__TAURI__.core;
const TARGET_ADDRESS = await invoke("get_target_address");
window.location = TARGET_ADDRESS;
