mod bindings;
mod runtime;
mod utils;

use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            bindings::get_library,
            bindings::decrypt,
            bindings::get_temp_book_path
        ])
        .setup(|app| {
            if let Err(_) = runtime::init_base_key() {
                app.dialog()
                    .message("Unable to retrieve the Ridibooks global key. Has Ridibooks been correctly installed and started?")
                    .kind(MessageDialogKind::Error)
                    .title("Fatal Error")
                    .blocking_show();
                panic!();
            }
            if let Err(_) = runtime::init_temp_dir() {
                app.dialog()
                    .message("Unable to initialise the temporary file path. Please check whether your antivirus software is blocking this operation.")
                    .kind(MessageDialogKind::Error)
                    .title("Fatal Error")
                    .blocking_show();
                panic!();
            }
            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Destroyed = event {
                        let _ = runtime::cleanup_temp_dir();
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
