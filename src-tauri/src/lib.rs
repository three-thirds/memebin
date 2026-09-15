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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, ensure_storage, save_meme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
