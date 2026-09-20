//! # Window Lifecycle Management
//!
//! This module manages window visibility, hiding window on blur,
//! cross-platform global shortcuts, and Wayland single-instance IPC toggles.
//!
//! ## Architectural Invariants:
//! 1. Raycast-Style Focus and Blur: The launcher window automatically hides whenever it loses focus.
//! 2. Wayland Compatibility: On Linux/Wayland, global key grabbing is restricted by design.
//!    Window toggling is handled through single-instance IPC thingy, and keybind ot be added by
//!    compositor configs.
//! 3. Desktop Native Shortcuts: On Windows and macOS, native global shortcuts (`Ctrl+Shift+M`)
//!    are registered to toggle window state.
use std::sync::Mutex;
use tauri::{App, AppHandle, Manager, Window, WindowEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// Handles blur, focus and window lifecycle events
///
/// Checks for WindowEvent and hides window if user clicks out of window
///
/// # Arguments
/// * `window` - A reference to the active [`Window`] instance.
/// * `event` - The specific event that happened on window
pub fn handle_window_event(window: &Window, event: &WindowEvent) {
    if let WindowEvent::Focused(false) = event {
        let _ = window.hide();
    }
}

/// Handles secondary process launches when the user tries to start Memebin while it is already running.
///
/// When a second process starts, the single-instance plugin intercepts execution, delivers this
/// callback to the running primary process, and immediately terminates the secondary instance.
///
/// # Behavior:
/// * If the main window is currently visible: hides the window.
/// * If the main window is hidden: summons the window and grabs focus.
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

/// Registers the global keyboard shortcut (`Ctrl+Shift+M`) for desktop platforms.
///
/// # Arguments
/// * `app` - Mutable reference to the initializing [`App`] during setup.
///
/// # Errors
/// Returns an error if:
/// * The shortcut string fails to parse into a valid key combination.
/// * The operating system fails to bind the shortcut (e.g. key combination already claimed).
/// 
// pub fn setup_shortcuts(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
//     let window = app.get_webview_window("main").unwrap();

//     let hotkey: Shortcut = "Ctrl+Shift+M".parse().unwrap();
//     println!("{:?}", hotkey);

//     let win = window.clone();
//     app.global_shortcut()
//         .on_shortcut(hotkey, move |_app, _shortcut, event| {
//             println!("RAW EVENT DETECTED! {:?}", event);
//             use tauri_plugin_global_shortcut::ShortcutState;

//             if event.state == ShortcutState::Pressed {
//                 if win.is_visible().unwrap_or(false) {
//                     let _ = win.hide();
//                     println!("Shortcut Pressed");
//                 } else {
//                     let _ = win.show();
//                     let _ = win.set_focus();
//                 }
//             }
//         })?;
//     Ok(())
// }


pub struct PopupShortcut(pub Mutex<Option<Shortcut>>);

fn toggle_main(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}

pub fn register_popup(app: &AppHandle, accel: &str) -> Result<(), String> {
    let new: Shortcut = accel.parse().map_err(|e| format!("Failed to parse shortcut: {}", e))?;
    let state = app.state::<PopupShortcut>();
    let mut current = state.0.lock().unwrap();

    if *current == Some(new) {
        return Ok(());
    }

    app.global_shortcut()
        .on_shortcut(new, |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_main(app);
            }
        })
        .map_err(|e| e.to_string())?;

    if let Some(old) = current.take() {
        let _ = app.global_shortcut().unregister(old);
    }
    *current = Some(new);
    Ok(())
}

#[tauri::command]
pub fn set_popup_shortcut(app: AppHandle, shortcut: String) -> Result<(), String> {
    register_popup(&app, &shortcut)
}

pub fn setup_shortcuts(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // default only; the frontend overrides it with the saved binding on startup
    let _ = register_popup(app.handle(), "CommandOrControl+Shift+M");
    Ok(())
}

