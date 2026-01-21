mod config;

use tauri::webview::PageLoadEvent;

#[tauri::command]
fn get_target_address() -> String {
    config::TARGET_ADDRESS.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .on_page_load(|window, payload| match payload.event() {
            PageLoadEvent::Started => {
                println!("{} started loading", payload.url());
                window.eval(include_str!("../../src/preload.js")).unwrap();
            }
            PageLoadEvent::Finished => {
                println!("{} finished loading", payload.url());
            }
        })
        .invoke_handler(tauri::generate_handler![get_target_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
