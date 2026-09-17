pub mod clipboard;
pub mod tray;
pub mod window;

use std::sync::Mutex;

use arboard::Clipboard;
use tauri::{App, Manager};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    window::setup_shortcuts(app)?;

    tray::setup_tray(app)?;

    let clipboard = Clipboard::new()?;
    app.manage(Mutex::new(clipboard));
    Ok(())
}
