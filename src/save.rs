// save.rs — JSON persistence for pet state and theme.
//
// Save location: platform data directory / "terminal-pet" / "save.json"
//   • Windows: %APPDATA%\terminal-pet\save.json
//   • Linux/macOS: ~/.local/share/terminal-pet/save.json

use std::{
    fs,
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::pet::Pet;
use crate::theme::ThemeColor;

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
pub fn save_path() -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("terminal-pet");
    fs::create_dir_all(&dir).ok();
    dir.join("save.json")
}

// ─── Load / Save ──────────────────────────────────────────────────────────

/// Load a save file from disk.
/// Returns `None` if the file doesn't exist or is unreadable/corrupt.
pub fn load() -> Option<SaveFile> {
    let path = save_path();
    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
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
        let path = save_path();
        let _ = fs::write(path, json);
    }
}

/// Calculate how many real-world minutes elapsed since `timestamp`.
pub fn minutes_since(timestamp: DateTime<Utc>) -> f64 {
    let elapsed = Utc::now() - timestamp;
    elapsed.num_seconds() as f64 / 60.0
}
