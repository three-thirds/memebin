mod storage;

use storage::Meme;
use tauri::AppHandle;
use tauri::Manager;

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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                let window = app.get_webview_window("main").unwrap();
                window.with_webview(|webview| {
                    #[cfg(target_os = "macos")]
                    unsafe {
                        use objc2::msg_send;
                        use objc2::runtime::AnyObject;
                        let ns_window: *mut AnyObject = webview.ns_window() as *mut AnyObject;
                        let _: () = msg_send![ns_window, setOpaque: false];
                        let content_view: *mut AnyObject = msg_send![ns_window, contentView];
                        let _: () = msg_send![content_view, setWantsLayer: true];
                        let layer: *mut AnyObject = msg_send![content_view, layer];
                        let _: () = msg_send![layer, setCornerRadius: 16.0f64];
                        let _: () = msg_send![layer, setMasksToBounds: true];
                    }
                }).ok();
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Failed to apply vibrancy — macOS 10.13+ required");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, ensure_storage, save_meme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


