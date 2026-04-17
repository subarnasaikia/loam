use crate::error::{LoamError, LoamResult};
use std::path::{Path, PathBuf};

/// Default sub-folder under the user's Documents directory.
pub const DEFAULT_FOLDER: &str = "Loam";

/// Resolves the Loam root for a given override (or default) against a base.
/// Pure — no filesystem access. Exposed for testing and for dependency injection.
pub fn resolve_root(base: &Path, override_path: Option<&Path>) -> PathBuf {
    match override_path {
        Some(p) => p.to_path_buf(),
        None => base.join(DEFAULT_FOLDER),
    }
}

pub fn entries_dir(root: &Path) -> PathBuf {
    root.join("entries")
}

pub fn assets_dir(root: &Path) -> PathBuf {
    root.join("assets")
}

pub fn config_path(root: &Path) -> PathBuf {
    root.join("config.json")
}

pub fn sqlite_path(root: &Path) -> PathBuf {
    root.join("index.sqlite")
}

/// Default base — the user's Documents directory. Fallible because no documents
/// dir exists on some headless systems.
pub fn default_base() -> LoamResult<PathBuf> {
    dirs::document_dir()
        .ok_or_else(|| LoamError::Path("could not resolve Documents directory".into()))
}

/// Creates root/entries/ and root/assets/ if missing. Idempotent.
pub fn ensure_dirs(root: &Path) -> LoamResult<()> {
    std::fs::create_dir_all(entries_dir(root))?;
    std::fs::create_dir_all(assets_dir(root))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn default_root_is_base_plus_loam() {
        let base = PathBuf::from("/tmp/fake-base");
        let root = resolve_root(&base, None);
        assert_eq!(root, PathBuf::from("/tmp/fake-base/Loam"));
    }

    #[test]
    fn override_root_wins() {
        let base = PathBuf::from("/tmp/fake-base");
        let override_path = PathBuf::from("/custom/place");
        let root = resolve_root(&base, Some(&override_path));
        assert_eq!(root, override_path);
    }

    #[test]
    fn subpaths_are_correct() {
        let root = PathBuf::from("/x");
        assert_eq!(entries_dir(&root), PathBuf::from("/x/entries"));
        assert_eq!(assets_dir(&root), PathBuf::from("/x/assets"));
        assert_eq!(config_path(&root), PathBuf::from("/x/config.json"));
        assert_eq!(sqlite_path(&root), PathBuf::from("/x/index.sqlite"));
    }

    #[test]
    fn ensure_dirs_creates_structure() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("Loam");
        ensure_dirs(&root).unwrap();
        assert!(root.join("entries").is_dir());
        assert!(root.join("assets").is_dir());
    }

    #[test]
    fn ensure_dirs_is_idempotent() {
        let tmp = tempdir().unwrap();
        let root = tmp.path().join("Loam");
        ensure_dirs(&root).unwrap();
        ensure_dirs(&root).unwrap();
    }
}
