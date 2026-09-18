use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binding {
    pub id: String,
    pub trigger: String,
    pub meme_id: String,
}

/// Normalize a trigger: trim and lowercase (aliases and chord strings).
pub fn normalize_trigger(trigger: &str) -> String {
    trigger.trim().to_lowercase()
}

pub fn load_bindings(path: &Path) -> Result<Vec<Binding>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw =
        fs::read_to_string(path).map_err(|e| format!("failed to read bindings.json: {e}"))?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(&raw).map_err(|e| format!("failed to parse bindings.json: {e}"))
}

pub fn save_bindings(path: &Path, bindings: &[Binding]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("failed to create bindings dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(bindings)
        .map_err(|e| format!("failed to serialize bindings: {e}"))?;
    fs::write(path, json).map_err(|e| format!("failed to write bindings.json: {e}"))?;
    Ok(())
}

pub fn upsert_binding(bindings: &mut Vec<Binding>, trigger: &str, meme_id: &str) -> Binding {
    let normalized = normalize_trigger(trigger);
    if let Some(existing) = bindings.iter_mut().find(|b| b.trigger == normalized) {
        existing.meme_id = meme_id.to_string();
        return existing.clone();
    }
    let binding = Binding {
        id: Uuid::new_v4().to_string(),
        trigger: normalized,
        meme_id: meme_id.to_string(),
    };
    bindings.push(binding.clone());
    binding
}

pub fn remove_binding_by_trigger(bindings: &mut Vec<Binding>, trigger: &str) -> bool {
    let normalized = normalize_trigger(trigger);
    let before = bindings.len();
    bindings.retain(|b| b.trigger != normalized);
    bindings.len() != before
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_trigger_trims_and_lowercases() {
        assert_eq!(normalize_trigger("  Ctrl+Shift+M  "), "ctrl+shift+m");
        assert_eq!(normalize_trigger("LUL"), "lul");
    }

    #[test]
    fn upsert_updates_existing_trigger() {
        let mut bindings = Vec::new();
        let first = upsert_binding(&mut bindings, "lul", "meme-a");
        let second = upsert_binding(&mut bindings, " LUL ", "meme-b");
        assert_eq!(bindings.len(), 1);
        assert_eq!(first.id, second.id);
        assert_eq!(bindings[0].meme_id, "meme-b");
    }
}
