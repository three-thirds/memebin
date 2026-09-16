pub mod system;

use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_global_shortcut::Shortcut;
mod storage;

use storage::Meme;
use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn ensure_storage(app: AppHandle) -> Result<String, String> {
    storage::ensure_storage(&app).map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
fn save_meme(app: AppHandle, source_path: String, name: Option<String>) -> Result<Meme, String> {
    storage::save_meme(&app, std::path::Path::new(&source_path), name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(
            arboard::Clipboard::new().expect("Failed to initialize clipboard thingy"),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::Manager;
            use tauri_plugin_global_shortcut::GlobalShortcutExt;

            let window = app.get_webview_window("main").unwrap();

            let hotkey: Shortcut = "Ctrl+Shift+M".parse().unwrap();
            println!("{:?}", hotkey);

            let win = window.clone();
            app.global_shortcut()
                .on_shortcut(hotkey, move |_app, _shortcut, event| {
                    println!("RAW EVENT DETECTED! {:?}", event);
                    use tauri_plugin_global_shortcut::ShortcutState;

                    if event.state == ShortcutState::Pressed {
                        if win.is_visible().unwrap_or(false) {
                            let _ = win.hide();
                            println!("Shortcut Pressed");
                        } else {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            system::clipboard::copy_to_clipboard
        ])
        .invoke_handler(tauri::generate_handler![greet, ensure_storage, save_meme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
