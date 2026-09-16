use tauri::{App, AppHandle, Manager, Window, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

/// Handles blur, focus and window lifecycle events
pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(false) = event {
        let _ = window.hide();
    }
}

// Handles single-instance toggle for wayland/linux
pub fn handle_single_instance(app: &AppHandle, _args: Vec<String>, _cwd: String) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

//Sets up global shortcuts for Mac and Windows
pub fn setup_shortcuts(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
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
}
