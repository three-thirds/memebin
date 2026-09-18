mod bindings;
mod meme;

pub use bindings::Binding;
pub use meme::Meme;

use bindings::{
    load_bindings, normalize_trigger, remove_binding_by_trigger, save_bindings, upsert_binding,
};
use meme::{read_sidecar, write_sidecar};

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

const ALLOWED_EXTENSIONS: &[&str] = &["gif", "webp", "png", "jpg", "jpeg"];
const MANIFEST_VERSION: u32 = 1;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemeSort {
    Created,
    Recent,
    Favorites,
    Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LibraryStats {
    pub count: u64,
    pub favorites: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepairReport {
    pub removed_json: u64,
    pub removed_media: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub exported_at: String,
    pub memes: Vec<Meme>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportMode {
    Merge,
}

fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}

pub fn normalize_extension(ext: &str) -> Result<String, String> {
    let ext = ext.trim().trim_start_matches('.').to_lowercase();
    if ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        Ok(ext)
    } else {
        Err(format!(
            "unsupported extension '{ext}'; allowed: {}",
            ALLOWED_EXTENSIONS.join(", ")
        ))
    }
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))
}

/// Resolve the memes directory under the OS app data dir and create it if needed.
pub fn memes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?.join("memes");
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create memes dir: {e}"))?;
    Ok(dir)
}

fn bindings_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join("bindings.json"))
}

/// Ensure storage dirs exist; returns the memes directory path.
pub fn ensure_storage(app: &AppHandle) -> Result<PathBuf, String> {
    let _ = bindings_path(app)?;
    memes_dir(app)
}

fn find_by_hash(dir: &Path, hash: &str) -> Result<Option<Meme>, String> {
    for meme in list_memes_in_dir(dir)? {
        if meme.content_hash == hash {
            return Ok(Some(meme));
        }
    }
    Ok(None)
}

fn list_memes_in_dir(dir: &Path) -> Result<Vec<Meme>, String> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut memes = Vec::new();
    let entries = fs::read_dir(dir).map_err(|e| format!("failed to read memes dir: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to read dir entry: {e}"))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        memes.push(read_sidecar(&path)?);
    }
    Ok(memes)
}

fn sort_memes(mut memes: Vec<Meme>, sort: MemeSort) -> Vec<Meme> {
    match sort {
        MemeSort::Created => {
            memes.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }
        MemeSort::Recent => {
            memes.sort_by(|a, b| {
                b.last_used_at
                    .as_deref()
                    .unwrap_or("")
                    .cmp(a.last_used_at.as_deref().unwrap_or(""))
                    .then_with(|| b.use_count.cmp(&a.use_count))
            });
        }
        MemeSort::Favorites => {
            memes.sort_by(|a, b| {
                b.favorite
                    .cmp(&a.favorite)
                    .then_with(|| b.created_at.cmp(&a.created_at))
            });
        }
        MemeSort::Name => {
            memes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
    }
    memes
}

pub fn meme_matches_query(meme: &Meme, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    if meme.name.to_lowercase().contains(&q) {
        return true;
    }
    meme.tags.iter().any(|t| t.to_lowercase().contains(&q))
}

fn persist_new_meme(
    dir: &Path,
    bytes: &[u8],
    extension: &str,
    name: String,
    tags: Vec<String>,
) -> Result<Meme, String> {
    let hash = hash_bytes(bytes);
    if let Some(existing) = find_by_hash(dir, &hash)? {
        return Ok(existing);
    }

    let id = Uuid::new_v4().to_string();
    let filename = format!("{id}.{extension}");
    let dest = dir.join(&filename);
    fs::write(&dest, bytes).map_err(|e| format!("failed to write meme file: {e}"))?;

    let now = now_rfc3339();
    let meme = Meme {
        id: id.clone(),
        name,
        filename,
        extension: extension.to_string(),
        tags,
        size_bytes: bytes.len() as u64,
        content_hash: hash,
        favorite: false,
        use_count: 0,
        last_used_at: None,
        created_at: now.clone(),
        updated_at: now,
    };
    write_sidecar(dir, &meme)?;
    Ok(meme)
}

/// Copy a source file into the memes dir, write sidecar JSON metadata, return the Meme.
pub fn save_meme(
    app: &AppHandle,
    source_path: &Path,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    if !source_path.is_file() {
        return Err(format!(
            "source path is not a file: {}",
            source_path.display()
        ));
    }

    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| "source file has no extension".to_string())?;
    let extension = normalize_extension(ext)?;

    let bytes = fs::read(source_path).map_err(|e| format!("failed to read source file: {e}"))?;
    let display_name = name.unwrap_or_else(|| {
        source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("meme")
            .to_string()
    });

    let dir = memes_dir(app)?;
    persist_new_meme(
        &dir,
        &bytes,
        &extension,
        display_name,
        tags.unwrap_or_default(),
    )
}

pub fn save_meme_bytes(
    app: &AppHandle,
    bytes: Vec<u8>,
    extension: String,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    let extension = normalize_extension(&extension)?;
    if bytes.is_empty() {
        return Err("cannot save empty meme bytes".into());
    }
    let dir = memes_dir(app)?;
    persist_new_meme(
        &dir,
        &bytes,
        &extension,
        name.unwrap_or_else(|| "meme".into()),
        tags.unwrap_or_default(),
    )
}

pub fn list_memes(app: &AppHandle) -> Result<Vec<Meme>, String> {
    list_memes_sorted(app, MemeSort::Created)
}

pub fn list_memes_sorted(app: &AppHandle, sort: MemeSort) -> Result<Vec<Meme>, String> {
    let dir = memes_dir(app)?;
    Ok(sort_memes(list_memes_in_dir(&dir)?, sort))
}

pub fn get_meme(app: &AppHandle, id: &str) -> Result<Meme, String> {
    let path = memes_dir(app)?.join(format!("{id}.json"));
    if !path.exists() {
        return Err(format!("meme not found: {id}"));
    }
    read_sidecar(&path)
}

pub fn meme_path(app: &AppHandle, id: &str) -> Result<String, String> {
    let meme = get_meme(app, id)?;
    let path = memes_dir(app)?.join(&meme.filename);
    if !path.exists() {
        return Err(format!("meme media missing for id: {id}"));
    }
    Ok(path.to_string_lossy().into_owned())
}

pub fn delete_meme(app: &AppHandle, id: &str) -> Result<(), String> {
    let dir = memes_dir(app)?;
    let meme = get_meme(app, id)?;
    let media = dir.join(&meme.filename);
    let meta = dir.join(format!("{id}.json"));
    if media.exists() {
        fs::remove_file(&media).map_err(|e| format!("failed to delete media: {e}"))?;
    }
    if meta.exists() {
        fs::remove_file(&meta).map_err(|e| format!("failed to delete metadata: {e}"))?;
    }

    // Drop bindings pointing at deleted meme.
    let bpath = bindings_path(app)?;
    let mut bindings = load_bindings(&bpath)?;
    let before = bindings.len();
    bindings.retain(|b| b.meme_id != id);
    if bindings.len() != before {
        save_bindings(&bpath, &bindings)?;
    }
    Ok(())
}

pub fn search_memes(app: &AppHandle, query: &str) -> Result<Vec<Meme>, String> {
    let memes = list_memes(app)?;
    Ok(memes
        .into_iter()
        .filter(|m| meme_matches_query(m, query))
        .collect())
}

pub fn update_meme(
    app: &AppHandle,
    id: &str,
    name: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<Meme, String> {
    let dir = memes_dir(app)?;
    let mut meme = get_meme(app, id)?;
    if let Some(name) = name {
        meme.name = name;
    }
    if let Some(tags) = tags {
        meme.tags = tags;
    }
    meme.updated_at = now_rfc3339();
    write_sidecar(&dir, &meme)?;
    Ok(meme)
}

pub fn record_use(app: &AppHandle, id: &str) -> Result<Meme, String> {
    let dir = memes_dir(app)?;
    let mut meme = get_meme(app, id)?;
    meme.use_count = meme.use_count.saturating_add(1);
    meme.last_used_at = Some(now_rfc3339());
    meme.updated_at = now_rfc3339();
    write_sidecar(&dir, &meme)?;
    Ok(meme)
}

pub fn set_favorite(app: &AppHandle, id: &str, favorite: bool) -> Result<Meme, String> {
    let dir = memes_dir(app)?;
    let mut meme = get_meme(app, id)?;
    meme.favorite = favorite;
    meme.updated_at = now_rfc3339();
    write_sidecar(&dir, &meme)?;
    Ok(meme)
}

pub fn library_stats(app: &AppHandle) -> Result<LibraryStats, String> {
    let memes = list_memes(app)?;
    let stats = LibraryStats {
        count: memes.len() as u64,
        favorites: memes.iter().filter(|m| m.favorite).count() as u64,
        total_bytes: memes.iter().map(|m| m.size_bytes).sum(),
    };
    Ok(stats)
}

pub fn repair_orphans(app: &AppHandle) -> Result<RepairReport, String> {
    let dir = memes_dir(app)?;
    let mut removed_json = 0u64;
    let mut removed_media = 0u64;

    let entries = fs::read_dir(&dir).map_err(|e| format!("failed to read memes dir: {e}"))?;
    let paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    for path in &paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if let Some(stem) = name.strip_suffix(".json") {
            let meme = match read_sidecar(path) {
                Ok(m) => m,
                Err(_) => {
                    fs::remove_file(path).ok();
                    removed_json += 1;
                    continue;
                }
            };
            let media = dir.join(&meme.filename);
            if !media.exists() {
                fs::remove_file(path).ok();
                removed_json += 1;
            }
            let _ = stem;
        } else {
            // Media file: expect matching <stem>.json
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let meta = dir.join(format!("{stem}.json"));
            if !meta.exists() {
                fs::remove_file(path).ok();
                removed_media += 1;
            }
        }
    }

    Ok(RepairReport {
        removed_json,
        removed_media,
    })
}

// --- Bindings ---

pub fn list_bindings(app: &AppHandle) -> Result<Vec<Binding>, String> {
    load_bindings(&bindings_path(app)?)
}

pub fn set_binding(app: &AppHandle, trigger: &str, meme_id: &str) -> Result<Binding, String> {
    let _ = get_meme(app, meme_id)?;
    let path = bindings_path(app)?;
    let mut bindings = load_bindings(&path)?;
    let binding = upsert_binding(&mut bindings, trigger, meme_id);
    save_bindings(&path, &bindings)?;
    Ok(binding)
}

pub fn remove_binding(app: &AppHandle, trigger: &str) -> Result<bool, String> {
    let path = bindings_path(app)?;
    let mut bindings = load_bindings(&path)?;
    let removed = remove_binding_by_trigger(&mut bindings, trigger);
    if removed {
        save_bindings(&path, &bindings)?;
    }
    Ok(removed)
}

pub fn resolve_trigger(app: &AppHandle, trigger: &str) -> Result<Option<Meme>, String> {
    let normalized = normalize_trigger(trigger);
    let bindings = load_bindings(&bindings_path(app)?)?;
    let Some(binding) = bindings.into_iter().find(|b| b.trigger == normalized) else {
        return Ok(None);
    };
    match get_meme(app, &binding.meme_id) {
        Ok(meme) => Ok(Some(meme)),
        Err(_) => Ok(None),
    }
}

pub fn resolve_trigger_path(app: &AppHandle, trigger: &str) -> Result<Option<String>, String> {
    match resolve_trigger(app, trigger)? {
        Some(meme) => Ok(Some(meme_path(app, &meme.id)?)),
        None => Ok(None),
    }
}

// --- Manifest ---

pub fn export_manifest(app: &AppHandle) -> Result<Manifest, String> {
    Ok(Manifest {
        version: MANIFEST_VERSION,
        exported_at: now_rfc3339(),
        memes: list_memes(app)?,
    })
}

/// Merge mode: for each meme in the manifest, if a local meme with the same id or
/// content_hash already exists, skip. Does not download or write media bytes — only
/// writes sidecar metadata when the media file is already present on disk.
pub fn import_manifest(
    app: &AppHandle,
    manifest: Manifest,
    mode: ImportMode,
) -> Result<u64, String> {
    let ImportMode::Merge = mode;
    let dir = memes_dir(app)?;
    let existing = list_memes_in_dir(&dir)?;
    let mut imported = 0u64;

    for mut incoming in manifest.memes {
        let id_exists = existing.iter().any(|m| m.id == incoming.id);
        let hash_exists = !incoming.content_hash.is_empty()
            && existing
                .iter()
                .any(|m| m.content_hash == incoming.content_hash);
        if id_exists || hash_exists {
            continue;
        }

        let media = dir.join(&incoming.filename);
        if !media.exists() {
            // Backend must supply files separately; skip metadata-only orphans.
            continue;
        }

        incoming = incoming.normalize();
        if incoming.updated_at.is_empty() {
            incoming.updated_at = now_rfc3339();
        }
        write_sidecar(&dir, &incoming)?;
        imported += 1;
    }

    Ok(imported)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_extension_allows_gif() {
        assert_eq!(normalize_extension("GIF").unwrap(), "gif");
        assert_eq!(normalize_extension(".WebP").unwrap(), "webp");
    }

    #[test]
    fn normalize_extension_rejects_exe() {
        assert!(normalize_extension("exe").is_err());
    }

    #[test]
    fn search_matches_name_and_tags() {
        let meme = Meme {
            id: "1".into(),
            name: "Funny Cat".into(),
            filename: "1.gif".into(),
            extension: "gif".into(),
            tags: vec!["lol".into(), "animals".into()],
            size_bytes: 10,
            content_hash: "abc".into(),
            favorite: false,
            use_count: 0,
            last_used_at: None,
            created_at: "t".into(),
            updated_at: "t".into(),
        };
        assert!(meme_matches_query(&meme, "cat"));
        assert!(meme_matches_query(&meme, "LOL"));
        assert!(!meme_matches_query(&meme, "dog"));
    }
}
