mod config;

use tauri::{webview::PageLoadEvent, AppHandle, Manager};

#[tauri::command]
fn get_target_address() -> String {
    config::TARGET_ADDRESS.to_string()
}

#[tauri::command]
fn close_app(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        // this plugin needs to be loaded first
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder = builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .on_page_load(|window, payload| match payload.event() {
            PageLoadEvent::Started => {
                println!("{} started loading", payload.url());
                window.eval(include_str!("../../src/preload.js")).unwrap();
            }
            PageLoadEvent::Finished => {
                println!("{} finished loading", payload.url());
            }
        })
        .invoke_handler(tauri::generate_handler![get_target_address, close_app]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
