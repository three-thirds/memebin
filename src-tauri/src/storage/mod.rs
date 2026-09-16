mod meme;

pub use meme::Meme;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Manager};
use uuid::Uuid;

/// Resolve the memes directory under the OS app data dir and create it if needed.
pub fn memes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))?;

    let dir = app_data.join("memes");
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create memes dir: {e}"))?;
    Ok(dir)
}

/// Ensure storage dirs exist; returns the memes directory path.
pub fn ensure_storage(app: &AppHandle) -> Result<PathBuf, String> {
    memes_dir(app)
}

/// Unix epoch seconds as a string (stand-in until we add chrono for RFC3339).
fn now_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into())
}

/// Copy a source file into the memes dir, write sidecar JSON metadata, return the Meme.
pub fn save_meme(
    app: &AppHandle,
    source_path: &Path,
    name: Option<String>,
) -> Result<Meme, String> {
    if !source_path.is_file() {
        return Err(format!(
            "source path is not a file: {}",
            source_path.display()
        ));
    }

    let dir = memes_dir(app)?;
    let id = Uuid::new_v4().to_string();

    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let filename = format!("{id}.{ext}");
    let dest = dir.join(&filename);

    fs::copy(source_path, &dest).map_err(|e| format!("failed to copy meme file: {e}"))?;

    let display_name = name.unwrap_or_else(|| {
        source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("meme")
            .to_string()
    });

    let meme = Meme {
        id: id.clone(),
        name: display_name,
        filename,
        created_at: now_timestamp(),
    };

    let meta_path = dir.join(format!("{id}.json"));
    let json = serde_json::to_string_pretty(&meme)
        .map_err(|e| format!("failed to serialize meme metadata: {e}"))?;
    fs::write(&meta_path, json).map_err(|e| format!("failed to write meme metadata: {e}"))?;

    Ok(meme)
}
