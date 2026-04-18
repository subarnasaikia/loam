use crate::error::{LoamError, LoamResult};
use crate::paths::sqlite_path;
use rusqlite::{params, Connection};
use std::path::Path;

const SCHEMA_V1: &str = r#"
CREATE TABLE IF NOT EXISTS entries (
  date TEXT PRIMARY KEY,
  word_count INTEGER,
  duration_ms INTEGER,
  primary_theme TEXT,
  secondary_theme TEXT,
  biome TEXT,
  landmark_type TEXT,
  landmark_x REAL,
  landmark_y REAL,
  landmark_z REAL,
  prompt_id TEXT,
  created_at INTEGER,
  updated_at INTEGER
);

CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(date UNINDEXED, body);

CREATE INDEX IF NOT EXISTS idx_biome ON entries(biome);
CREATE INDEX IF NOT EXISTS idx_themes ON entries(primary_theme);

CREATE TABLE IF NOT EXISTS unlocks (
  id TEXT PRIMARY KEY,
  unlocked_at INTEGER,
  kind TEXT
);

CREATE TABLE IF NOT EXISTS prompt_history (
  prompt_id TEXT,
  shown_on TEXT,
  skipped INTEGER,
  PRIMARY KEY (prompt_id, shown_on)
);

CREATE TABLE IF NOT EXISTS _meta (
  key TEXT PRIMARY KEY,
  value TEXT
);
"#;

pub fn open(root: &Path) -> LoamResult<Connection> {
    let path = sqlite_path(root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(&path).map_err(|e| LoamError::Sqlite(e.to_string()))?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> LoamResult<()> {
    conn.execute_batch(SCHEMA_V1)
        .map_err(|e| LoamError::Sqlite(e.to_string()))?;
    conn.execute(
        "INSERT OR REPLACE INTO _meta (key, value) VALUES ('schema_version', '1')",
        params![],
    )
    .map_err(|e| LoamError::Sqlite(e.to_string()))?;
    Ok(())
}

pub fn schema_version(conn: &Connection) -> LoamResult<Option<String>> {
    let mut stmt = conn
        .prepare("SELECT value FROM _meta WHERE key = 'schema_version'")
        .map_err(|e| LoamError::Sqlite(e.to_string()))?;
    let mut rows = stmt
        .query(params![])
        .map_err(|e| LoamError::Sqlite(e.to_string()))?;
    if let Some(row) = rows
        .next()
        .map_err(|e| LoamError::Sqlite(e.to_string()))?
    {
        let v: String = row
            .get(0)
            .map_err(|e| LoamError::Sqlite(e.to_string()))?;
        Ok(Some(v))
    } else {
        Ok(None)
    }
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
    fn migrate_creates_all_tables() {
        let (_tmp, root) = setup();
        let conn = open(&root).unwrap();
        migrate(&conn).unwrap();

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type IN ('table','index') ORDER BY name")
            .unwrap();
        let names: Vec<String> = stmt
            .query_map(params![], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        for expected in &[
            "entries",
            "entries_fts",
            "idx_biome",
            "idx_themes",
            "unlocks",
            "prompt_history",
            "_meta",
        ] {
            assert!(
                names.iter().any(|n| n == expected),
                "missing {expected} in {names:?}"
            );
        }
    }

    #[test]
    fn migrate_is_idempotent() {
        let (_tmp, root) = setup();
        let conn = open(&root).unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();
        assert_eq!(schema_version(&conn).unwrap(), Some("1".into()));
    }

    #[test]
    fn schema_version_errors_before_meta_exists() {
        let (_tmp, root) = setup();
        let conn = open(&root).unwrap();
        let result = schema_version(&conn);
        assert!(result.is_err());
    }
}
