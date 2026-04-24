// save.rs — JSON persistence for pet state and theme.
//
// Save location: platform data directory / "terminal-pet" / "save_{name}.json"
//   • Windows: %APPDATA%\terminal-pet\save_{name}.json
//   • Linux/macOS: ~/.local/share/terminal-pet/save_{name}.json

use std::{
    fs,
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::pet::Pet;
use crate::core::theme::ThemeColor;

// ─── Save-file schema ─────────────────────────────────────────────────────

/// The complete save-file structure serialized to JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFile {
    /// Semantic version of the app that wrote this file.
    pub version: String,

    /// The pet's full state.
    pub pet: Pet,

    /// Currently selected color theme.
    pub theme: ThemeColor,

    /// When this file was last written.
    pub saved_at: DateTime<Utc>,
}

// ─── Path resolution ──────────────────────────────────────────────────────

/// Resolve the save-file path using platform conventions.
pub fn save_path(name: &str) -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("terminal-pet");
    fs::create_dir_all(&dir).ok();
    // Use alphanumeric pet name to avoid bad filenames
    let safe_name: String = name.chars().filter(|c| c.is_alphanumeric()).collect();
    let safe_name = if safe_name.is_empty() { "unnamed".to_string() } else { safe_name };
    dir.join(format!("save_{}.json", safe_name))
}

pub fn list_saves() -> Vec<SaveFile> {
    let base = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("terminal-pet");
    let mut saves = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(entry.path()) {
                    if let Ok(save) = serde_json::from_str::<SaveFile>(&json) {
                        saves.push(save);
                    }
                }
            }
        }
    }
    saves
}

// ─── Load / Save ──────────────────────────────────────────────────────────

/// Load the most recently saved file.
/// Returns `None` if no save files exist.
pub fn load_latest() -> Option<SaveFile> {
    let mut saves = list_saves();
    saves.sort_by_key(|s| s.saved_at);
    saves.pop()
}

/// Write the save file to disk.
/// Errors are silently ignored (we don't want to crash on save failure).
pub fn save(pet: &Pet, theme: ThemeColor) {
    let file = SaveFile {
        version:  env!("CARGO_PKG_VERSION").to_string(),
        pet:      pet.clone(),
        theme,
        saved_at: Utc::now(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&file) {
        let path = save_path(&pet.name);
        let _ = fs::write(path, json);
    }
}

/// Delete the save file for the given pet name.
pub fn delete_save(name: &str) {
    let path = save_path(name);
    let _ = fs::remove_file(path);
}

/// Calculate how many real-world minutes elapsed since `timestamp`.
pub fn minutes_since(timestamp: DateTime<Utc>) -> f64 {
    let elapsed = Utc::now() - timestamp;
    elapsed.num_seconds() as f64 / 60.0
}
