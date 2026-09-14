use tauri::Manager;
use tauri_plugin_global_shortcut::Shortcut;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
