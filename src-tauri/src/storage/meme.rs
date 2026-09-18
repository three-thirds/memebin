use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meme {
    pub id: String,
    pub name: String,
    pub filename: String,
    #[serde(default)]
    pub extension: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub size_bytes: u64,
    #[serde(default)]
    pub content_hash: String,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub use_count: u64,
    #[serde(default)]
    pub last_used_at: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

impl Meme {
    /// Fill fields that older sidecars may omit.
    pub fn normalize(mut self) -> Self {
        if self.extension.is_empty() {
            self.extension = self
                .filename
                .rsplit_once('.')
                .map(|(_, ext)| ext.to_lowercase())
                .unwrap_or_default();
        }
        if self.updated_at.is_empty() {
            self.updated_at = self.created_at.clone();
        }
        self
    }
}

pub fn write_sidecar(dir: &Path, meme: &Meme) -> Result<(), String> {
    let meta_path = dir.join(format!("{}.json", meme.id));
    let json = serde_json::to_string_pretty(meme)
        .map_err(|e| format!("failed to serialize meme metadata: {e}"))?;
    fs::write(&meta_path, json).map_err(|e| format!("failed to write meme metadata: {e}"))?;
    Ok(())
}

pub fn read_sidecar(path: &Path) -> Result<Meme, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("failed to read meme metadata: {e}"))?;
    let meme: Meme = serde_json::from_str(&raw)
        .map_err(|e| format!("failed to parse meme metadata: {e}"))?;
    Ok(meme.normalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_fills_extension_and_updated_at() {
        let meme = Meme {
            id: "abc".into(),
            name: "test".into(),
            filename: "abc.gif".into(),
            extension: String::new(),
            tags: vec![],
            size_bytes: 0,
            content_hash: String::new(),
            favorite: false,
            use_count: 0,
            last_used_at: None,
            created_at: "2024-01-01T00:00:00Z".into(),
            updated_at: String::new(),
        }
        .normalize();

        assert_eq!(meme.extension, "gif");
        assert_eq!(meme.updated_at, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn deserialize_old_sidecar_defaults() {
        let json = r#"{
            "id": "1",
            "name": "old",
            "filename": "1.png",
            "created_at": "100"
        }"#;
        let meme: Meme = serde_json::from_str(json).unwrap();
        let meme = meme.normalize();
        assert!(meme.tags.is_empty());
        assert!(!meme.favorite);
        assert_eq!(meme.use_count, 0);
        assert_eq!(meme.extension, "png");
    }
}
