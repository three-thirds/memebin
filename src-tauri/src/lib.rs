mod storage;

use storage::{
    Binding, ImportMode, LibraryStats, Manifest, Meme, MemeSort, RepairReport,
};
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
fn save_meme(
    app: AppHandle,
    source_path: String,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    storage::save_meme(&app, std::path::Path::new(&source_path), name, tags)
}

#[tauri::command]
fn save_meme_bytes(
    app: AppHandle,
    bytes: Vec<u8>,
    extension: String,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    storage::save_meme_bytes(&app, bytes, extension, name, tags)
}

#[tauri::command]
fn list_memes(app: AppHandle) -> Result<Vec<Meme>, String> {
    storage::list_memes(&app)
}

#[tauri::command]
fn list_memes_sorted(app: AppHandle, sort: MemeSort) -> Result<Vec<Meme>, String> {
    storage::list_memes_sorted(&app, sort)
}

#[tauri::command]
fn get_meme(app: AppHandle, id: String) -> Result<Meme, String> {
    storage::get_meme(&app, &id)
}

#[tauri::command]
fn meme_path(app: AppHandle, id: String) -> Result<String, String> {
    storage::meme_path(&app, &id)
}

#[tauri::command]
fn delete_meme(app: AppHandle, id: String) -> Result<(), String> {
    storage::delete_meme(&app, &id)
}

#[tauri::command]
fn search_memes(app: AppHandle, query: String) -> Result<Vec<Meme>, String> {
    storage::search_memes(&app, &query)
}

#[tauri::command]
fn update_meme(
    app: AppHandle,
    id: String,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    storage::update_meme(&app, &id, name, tags)
}

#[tauri::command]
fn record_use(app: AppHandle, id: String) -> Result<Meme, String> {
    storage::record_use(&app, &id)
}

#[tauri::command]
fn set_favorite(app: AppHandle, id: String, favorite: bool) -> Result<Meme, String> {
    storage::set_favorite(&app, &id, favorite)
}

#[tauri::command]
fn library_stats(app: AppHandle) -> Result<LibraryStats, String> {
    storage::library_stats(&app)
}

#[tauri::command]
fn repair_orphans(app: AppHandle) -> Result<RepairReport, String> {
    storage::repair_orphans(&app)
}

#[tauri::command]
fn list_bindings(app: AppHandle) -> Result<Vec<Binding>, String> {
    storage::list_bindings(&app)
}

#[tauri::command]
fn set_binding(app: AppHandle, trigger: String, meme_id: String) -> Result<Binding, String> {
    storage::set_binding(&app, &trigger, &meme_id)
}

#[tauri::command]
fn remove_binding(app: AppHandle, trigger: String) -> Result<bool, String> {
    storage::remove_binding(&app, &trigger)
}

#[tauri::command]
fn resolve_trigger(app: AppHandle, trigger: String) -> Result<Option<Meme>, String> {
    storage::resolve_trigger(&app, &trigger)
}

#[tauri::command]
fn resolve_trigger_path(app: AppHandle, trigger: String) -> Result<Option<String>, String> {
    storage::resolve_trigger_path(&app, &trigger)
}

#[tauri::command]
fn export_manifest(app: AppHandle) -> Result<Manifest, String> {
    storage::export_manifest(&app)
}

#[tauri::command]
fn import_manifest(
    app: AppHandle,
    manifest: Manifest,
    mode: ImportMode,
) -> Result<u64, String> {
    storage::import_manifest(&app, manifest, mode)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            ensure_storage,
            save_meme,
            save_meme_bytes,
            list_memes,
            list_memes_sorted,
            get_meme,
            meme_path,
            delete_meme,
            search_memes,
            update_meme,
            record_use,
            set_favorite,
            library_stats,
            repair_orphans,
            list_bindings,
            set_binding,
            remove_binding,
            resolve_trigger,
            resolve_trigger_path,
            export_manifest,
            import_manifest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
