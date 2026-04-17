use crate::error::{LoamError, LoamResult};
use crate::paths::entries_dir;
use std::fs;
use std::path::{Path, PathBuf};

fn entry_path(root: &Path, date: &str) -> LoamResult<PathBuf> {
    if !is_iso_date(date) {
        return Err(LoamError::Path(format!("invalid date: {date}")));
    }
    Ok(entries_dir(root).join(format!("{date}.md")))
}

fn is_iso_date(s: &str) -> bool {
    if s.len() != 10 {
        return false;
    }
    let b = s.as_bytes();
    b[4] == b'-'
        && b[7] == b'-'
        && b[0..4].iter().all(|c| c.is_ascii_digit())
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[8..10].iter().all(|c| c.is_ascii_digit())
}

pub fn write_entry(root: &Path, date: &str, body: &str) -> LoamResult<PathBuf> {
    let path = entry_path(root, date)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("md.tmp");
    fs::write(&tmp, body)?;
    fs::rename(&tmp, &path)?;
    Ok(path)
}

pub fn read_entry(root: &Path, date: &str) -> LoamResult<Option<String>> {
    let path = entry_path(root, date)?;
    if !path.exists() {
        return Ok(None);
    }
    let body = fs::read_to_string(&path)?;
    Ok(Some(body))
}

pub fn list_entries(root: &Path) -> LoamResult<Vec<String>> {
    let dir = entries_dir(root);
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut dates: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(stem) = name.strip_suffix(".md") {
            if is_iso_date(stem) {
                dates.push(stem.to_string());
            }
        }
    }
    dates.sort();
    Ok(dates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::ensure_dirs;
    use tempfile::tempdir;

    fn setup() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        ensure_dirs(&root).unwrap();
        (tmp, root)
    }

    #[test]
    fn write_then_read_roundtrips() {
        let (_tmp, root) = setup();
        write_entry(&root, "2026-04-17", "hello world").unwrap();
        let body = read_entry(&root, "2026-04-17").unwrap();
        assert_eq!(body, Some("hello world".to_string()));
    }

    #[test]
    fn read_missing_returns_none() {
        let (_tmp, root) = setup();
        let body = read_entry(&root, "2026-04-17").unwrap();
        assert_eq!(body, None);
    }

    #[test]
    fn list_returns_sorted_dates() {
        let (_tmp, root) = setup();
        write_entry(&root, "2026-04-18", "b").unwrap();
        write_entry(&root, "2026-04-17", "a").unwrap();
        write_entry(&root, "2026-04-19", "c").unwrap();
        let list = list_entries(&root).unwrap();
        assert_eq!(list, vec!["2026-04-17", "2026-04-18", "2026-04-19"]);
    }

    #[test]
    fn list_ignores_non_date_files() {
        let (_tmp, root) = setup();
        write_entry(&root, "2026-04-17", "a").unwrap();
        fs::write(root.join("entries").join("notes.md"), "x").unwrap();
        fs::write(root.join("entries").join(".DS_Store"), "x").unwrap();
        let list = list_entries(&root).unwrap();
        assert_eq!(list, vec!["2026-04-17"]);
    }

    #[test]
    fn rejects_invalid_date() {
        let (_tmp, root) = setup();
        let err = write_entry(&root, "2026-4-17", "x").unwrap_err();
        assert!(matches!(err, LoamError::Path(_)));
        let err = write_entry(&root, "bad", "x").unwrap_err();
        assert!(matches!(err, LoamError::Path(_)));
    }

    #[test]
    fn write_is_atomic_no_tmp_leftover() {
        let (_tmp, root) = setup();
        write_entry(&root, "2026-04-17", "a").unwrap();
        let dir_entries: Vec<_> = fs::read_dir(entries_dir(&root))
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        assert_eq!(dir_entries, vec!["2026-04-17.md"]);
    }
}
