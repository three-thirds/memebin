//! # System Tray Integration stuff
//!
//! As memebin is floating window and does not have decorations...
//! WE need a way to gracefully exit the app, so I am writing this module
//! which initializes builds a tray, supposed to be called in Application init function

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

///Initializes the native system tray icon, menu items and event dispatcher stuff
///
///It sets up:
///1. Menu items to Toggle Memebin and close Memebin
///2. Handlers to exit the app gracefully, or toggle it
///3. Direct Left-Click for opening and closing the window(does not work on Linux
///   because GNOME knows better)
///
///# Arguments
///* app - Reference to [`App`] during setup
///
///# Errors
///Returns error if menu construction fails, or maybe tray icon does not register
///with the OS
pub fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "toggle", "Toggle Memebin", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit Memebin", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    let icon = app.default_window_icon().unwrap().clone();

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            println!("[DEBUG TRAY EVENT]: {:?}", event); // <--- ADD THIS LINE
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
