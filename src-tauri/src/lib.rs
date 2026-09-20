pub mod system;

mod storage;

use storage::{
    Binding, ImportMode, LibraryStats, Manifest, Meme, MemeSort, RepairReport, SearchIndexCache,
    SearchOptions, SearchRankedResult,
};
use tauri::{AppHandle, Manager};

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
    storage::save_meme(&app, std::path::Path::new(&source_path), name,  tags)
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
fn search_ranked(
    app: AppHandle,
    query: String,
    opts: Option<SearchOptions>,
) -> Result<SearchRankedResult, String> {
    storage::search_ranked(&app, &query, opts.unwrap_or_default())
}

#[tauri::command]
fn list_tags(app: AppHandle) -> Result<Vec<String>, String> {
    storage::list_tags(&app)
}

#[tauri::command]
fn suggest_tags(
    app: AppHandle,
    prefix: String,
    limit: Option<usize>,
) -> Result<Vec<String>, String> {
    storage::suggest_tags(&app, &prefix, limit)
}

#[tauri::command]
fn update_meme(
    app: AppHandle,
    id: String,
    name: Option<String>,
    tags: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
) -> Result<Meme, String> {
    storage::update_meme(&app, &id, name, tags, aliases)
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
fn import_manifest(app: AppHandle, manifest: Manifest, mode: ImportMode) -> Result<u64, String> {
    storage::import_manifest(&app, manifest, mode)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(
            system::window::handle_single_instance,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
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
            search_ranked,
            list_tags,
            suggest_tags,
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
            system::clipboard::copy_to_clipboard,
            system::window::set_popup_shortcut,
        ])
        // Hide when someone clicks out of window
        .on_window_event(system::window::handle_window_event)
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                let window = app.get_webview_window("main").unwrap();
                window.with_webview(|webview| {
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

            app.manage(SearchIndexCache::new());
            app.manage(system::window::PopupShortcut(std::sync::Mutex::new(None)));
            system::setup(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

