use crate::error::LoamResult;
use crate::paths::config_path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub aesthetic: String,
    pub typewriter_mode: bool,
    pub distraction_free: bool,
    pub ambient_sound: bool,
    pub ambient_volume: f32,
    pub classifier: String,
    pub llm_model: Option<String>,
    pub prompt_packs_enabled: Vec<String>,
    pub loam_path: Option<String>,
    pub autosave_debounce_ms: u32,
    pub onboarding_complete: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            aesthetic: "paper".into(),
            typewriter_mode: true,
            distraction_free: true,
            ambient_sound: false,
            ambient_volume: 0.4,
            classifier: "heuristic".into(),
            llm_model: None,
            prompt_packs_enabled: vec!["canon".into()],
            loam_path: None,
            autosave_debounce_ms: 300,
            onboarding_complete: false,
        }
    }
}

pub fn load_settings(root: &Path) -> LoamResult<Settings> {
    let path = config_path(root);
    if !path.exists() {
        return Ok(Settings::default());
    }
    let raw = fs::read_to_string(&path)?;
    let settings = serde_json::from_str::<Settings>(&raw)?;
    Ok(settings)
}

pub fn save_settings(root: &Path, settings: &Settings) -> LoamResult<()> {
    let path = config_path(root);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(settings)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, &path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::ensure_dirs;
    use tempfile::tempdir;

    fn setup() -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        ensure_dirs(&root).unwrap();
        (tmp, root)
    }

    #[test]
    fn load_returns_defaults_when_missing() {
        let (_tmp, root) = setup();
        let s = load_settings(&root).unwrap();
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let (_tmp, root) = setup();
        let mut s = Settings::default();
        s.aesthetic = "nocturnal".into();
        s.typewriter_mode = false;
        save_settings(&root, &s).unwrap();
        let loaded = load_settings(&root).unwrap();
        assert_eq!(loaded, s);
    }

    #[test]
    fn unknown_fields_in_json_are_tolerated() {
        let (_tmp, root) = setup();
        let raw = r#"{ "aesthetic": "paper", "future_field": 42 }"#;
        fs::write(config_path(&root), raw).unwrap();
        let s = load_settings(&root).unwrap();
        assert_eq!(s.aesthetic, "paper");
    }

    #[test]
    fn missing_fields_fill_defaults() {
        let (_tmp, root) = setup();
        let raw = r#"{ "aesthetic": "nocturnal" }"#;
        fs::write(config_path(&root), raw).unwrap();
        let s = load_settings(&root).unwrap();
        assert_eq!(s.aesthetic, "nocturnal");
        assert_eq!(s.typewriter_mode, true);
        assert_eq!(s.autosave_debounce_ms, 300);
    }
}
