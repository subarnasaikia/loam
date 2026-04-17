# M1 — Tauri Shell + Local Storage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up a minimal Tauri 2 desktop app that can read, write, and list markdown journal entries on disk, initialize an SQLite index, and load/save a JSON settings file — all unit-tested, no UI beyond proof-of-life.

**Architecture:** Rust side exposes narrow Tauri commands (`ensure_loam_dir`, `write_entry`, `read_entry`, `list_entries`, `load_settings`, `save_settings`) backed by the standard library and `tauri-plugin-sql`. TypeScript side wraps each command in a strongly-typed client and exposes an `EntryStore` and `useSettings()` hook. Markdown files are the source of truth (`~/Documents/Loam/entries/YYYY-MM-DD.md`); SQLite (`index.sqlite`) is a rebuildable cache with schema created via an idempotent migration. Test-driven throughout — every module gets its failing test first.

**Tech Stack:** Tauri 2 (Rust), React 18 + Vite + TypeScript, `tauri-plugin-sql` (SQLite), `serde` / `serde_json`, Vitest (TS unit tests), `cargo test` (Rust unit tests), pnpm.

**Branch:** `feat/m1-tauri-shell-storage` (already created).
**PR target:** `main` (requires `chore/design-spec` merged first so the spec is visible on main).
**Reference spec:** [`docs/superpowers/specs/2026-04-17-loam-design.md`](../specs/2026-04-17-loam-design.md) — especially §6 (architecture), §7 (data model), and §8 (tech stack).

---

## Prerequisites (one-time, not counted as tasks)

Confirm the following are installed on the development machine before starting:

- Rust toolchain (`rustup`, stable ≥ 1.75): `rustc --version` should succeed.
- Node.js ≥ 20: `node --version`.
- pnpm ≥ 9: `pnpm --version`. Install with `corepack enable && corepack prepare pnpm@latest --activate` if missing.
- Tauri prerequisites for macOS: Xcode Command Line Tools (`xcode-select --install`).
- Tauri prerequisites for Linux: see https://v2.tauri.app/start/prerequisites/ (webkit2gtk, libssl-dev, etc.).
- Tauri prerequisites for Windows: WebView2 runtime (pre-installed on Windows 11), MSVC build tools.

---

## File Structure

After M1 completes, the repo looks like this (new paths marked `+`):

```
.
├── .github/                              (existing)
├── brand/                                (from chore/design-spec)
├── docs/
│   ├── superpowers/
│   │   ├── specs/                        (from chore/design-spec)
│   │   └── plans/                        +
│   │       └── 2026-04-17-m1-...md       + (this file)
│   └── WORKFLOW.md                       (existing)
├── package.json                          +
├── pnpm-lock.yaml                        +
├── tsconfig.json                         +
├── vite.config.ts                        +
├── vitest.config.ts                      +
├── index.html                            +
├── src/                                  +
│   ├── main.tsx                          + React entrypoint (proof-of-life only)
│   ├── App.tsx                           + Root component (proof-of-life only)
│   ├── lib/
│   │   ├── paths.ts                      + TS mirror of loam-dir resolution (pure)
│   │   ├── commands.ts                   + Thin wrappers over Tauri invoke()
│   │   ├── entry-store.ts                + EntryStore — read/write/list markdown
│   │   ├── settings.ts                   + Settings load/save + defaults
│   │   └── types.ts                      + Shared TypeScript types
│   └── lib/__tests__/
│       ├── paths.test.ts                 +
│       ├── entry-store.test.ts           +
│       └── settings.test.ts              +
├── src-tauri/                            +
│   ├── Cargo.toml                        +
│   ├── tauri.conf.json                   +
│   ├── build.rs                          +
│   ├── icons/                            + (Tauri-generated placeholders)
│   └── src/
│       ├── main.rs                       + Tauri entrypoint, command registration
│       ├── paths.rs                      + loam-dir resolution + ensure dirs
│       ├── entries.rs                    + File IO for markdown entries
│       ├── settings.rs                   + config.json read/write + defaults
│       ├── db.rs                         + SQLite schema + migration
│       └── error.rs                      + Unified error type + serde
└── .gitignore                            (already configured for node_modules/, target/, etc.)
```

**Boundaries (why this shape):**

- **`paths`** (Rust + TS mirror): resolves `~/Documents/Loam/...`. Pure; no filesystem side effects. Separately tested in both languages because both layers need it.
- **`entries`** (Rust): the only place that touches markdown files on disk. One responsibility: `write_entry`, `read_entry`, `list_entries`.
- **`settings`** (Rust): the only place that touches `config.json`. Load + save + defaults.
- **`db`** (Rust): SQLite schema + idempotent migration. Runs once at app start. Does not own query logic — queries ride `tauri-plugin-sql` at the TS layer for now, migrated to Rust queries in later milestones if needed.
- **`commands`** (TS): strongly-typed thin wrappers over `invoke()`. No business logic.
- **`entry-store`** (TS): combines commands into an API the rest of the app will consume (`getEntry(date)`, `saveEntry(date, body)`, `listEntries()`). Unit-tested by mocking `invoke`.
- **`settings`** (TS) / `useSettings()`: load once on boot, expose a reactive hook. Unit-tested with mocked invoke.

Every module is small (<150 LOC) and has one responsibility. Pure modules are unit-tested; the Rust command layer is tested via `cargo test` with tempdirs; the TS command layer is tested via mocked `invoke` (we do **not** boot a real Tauri harness in M1).

---

## Test Strategy

- **Rust unit tests** (`cargo test`) cover `paths`, `entries`, `settings`, `db` against a temporary directory (`tempfile` crate). No mocks; real filesystem inside `tempdir`.
- **TypeScript unit tests** (`vitest`) cover TS `paths` mirror, `entry-store`, and `settings` with a mocked `@tauri-apps/api/core` `invoke`. Keeps tests fast and hermetic.
- **No E2E Tauri tests in M1.** End-to-end Tauri runtime testing arrives in M2+ once there's a UI worth driving. The proof-of-life screen in M1 exists only to confirm the bundle runs; it is not under test.

**TDD rhythm for every coding task:** write the failing test → run to confirm the RED → implement the minimal code → run to confirm the GREEN → refactor if obvious → commit.

---

## Task 0: Scaffold Tauri 2 project structure

**Files:**
- Create: `package.json`, `pnpm-lock.yaml`, `index.html`, `vite.config.ts`, `tsconfig.json`, `src/main.tsx`, `src/App.tsx`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/icons/*`

- [ ] **Step 0.1: Bootstrap with the Tauri CLI**

Run in the project root:

```bash
pnpm create tauri-app@latest --template react-ts --manager pnpm --identifier com.loam.app --name loam -y
```

If the CLI refuses to scaffold into a non-empty directory, run:

```bash
pnpm create tauri-app@latest --template react-ts --manager pnpm --identifier com.loam.app --name loam --path . --force
```

Expected: directories `src/`, `src-tauri/`, files `package.json`, `index.html`, `vite.config.ts`, `tsconfig.json` appear. Does NOT overwrite `README.md`, `LICENSE`, `CONTRIBUTING.md`, `.gitignore`, `docs/`, `brand/`, `.github/` (Tauri scaffolder only writes into paths it owns).

- [ ] **Step 0.2: Verify scaffolded files**

Run:

```bash
ls -la src/ src-tauri/ && cat package.json | head -20
```

Expected: `src/main.tsx`, `src/App.tsx` exist; `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` exist; `package.json` has `@tauri-apps/api` and `@tauri-apps/cli` as deps.

- [ ] **Step 0.3: Pin Tauri 2.x and update identifier**

Edit `src-tauri/tauri.conf.json` — set:

```json
{
  "productName": "Loam",
  "version": "0.0.1",
  "identifier": "com.loam.app",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "pnpm build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Loam",
        "width": 1100,
        "height": 760,
        "minWidth": 900,
        "minHeight": 600
      }
    ],
    "security": { "csp": null }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

Edit `src-tauri/Cargo.toml` — ensure `tauri` and `tauri-build` dependencies are `^2`:

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

- [ ] **Step 0.4: Proof-of-life dev run**

Run:

```bash
pnpm install && pnpm tauri dev
```

Expected: Tauri window opens titled "Loam" showing the default React boilerplate. Close the window (Cmd-Q / Alt-F4).

- [ ] **Step 0.5: Replace proof-of-life UI with a minimal placeholder**

Edit `src/App.tsx` to:

```tsx
export default function App() {
  return (
    <main style={{ fontFamily: "Georgia, serif", padding: 48, color: "#3d2817", background: "#f4ecd8", minHeight: "100vh" }}>
      <h1>loam</h1>
      <p style={{ opacity: 0.6 }}>M1 — shell alive. Storage plumbing pending.</p>
    </main>
  );
}
```

Run `pnpm tauri dev` again. Expected: cream window with "loam" heading; no React logos or counter.

- [ ] **Step 0.6: Commit**

```bash
git add package.json pnpm-lock.yaml index.html vite.config.ts tsconfig.json src/ src-tauri/
git commit -m "chore(m1): scaffold tauri 2 + react + vite shell"
```

Verify: `git status` shows a clean working tree.

---

## Task 1: Vitest harness + TS config

**Files:**
- Create: `vitest.config.ts`
- Modify: `package.json`, `tsconfig.json`

- [ ] **Step 1.1: Install Vitest and test deps**

```bash
pnpm add -D vitest @vitest/ui happy-dom
```

Expected: `package.json` has `vitest`, `@vitest/ui`, `happy-dom` under `devDependencies`.

- [ ] **Step 1.2: Create `vitest.config.ts`**

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "happy-dom",
    globals: false,
    include: ["src/**/__tests__/**/*.test.ts", "src/**/__tests__/**/*.test.tsx"],
    coverage: {
      provider: "v8",
      reporter: ["text", "lcov"],
      include: ["src/lib/**/*.ts"]
    }
  }
});
```

- [ ] **Step 1.3: Add test scripts to `package.json`**

Under `"scripts"`, add:

```json
"test": "vitest run",
"test:watch": "vitest",
"test:coverage": "vitest run --coverage",
"typecheck": "tsc --noEmit"
```

- [ ] **Step 1.4: Add a canary test to prove the harness works**

Create `src/lib/__tests__/canary.test.ts`:

```ts
import { describe, it, expect } from "vitest";

describe("canary", () => {
  it("runs", () => {
    expect(2 + 2).toBe(4);
  });
});
```

Run:

```bash
pnpm test
```

Expected: `1 passed`.

- [ ] **Step 1.5: Commit**

```bash
git add vitest.config.ts package.json pnpm-lock.yaml src/lib/__tests__/canary.test.ts
git commit -m "chore(m1): add vitest harness with canary test"
```

---

## Task 2: Rust error type

**Files:**
- Create: `src-tauri/src/error.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 2.1: Write the failing test**

Create `src-tauri/src/error.rs`:

```rust
use serde::Serialize;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum LoamError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(String),

    #[error("path error: {0}")]
    Path(String),
}

impl Serialize for LoamError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type LoamResult<T> = Result<T, LoamError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_errors_convert() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "nope");
        let loam_err: LoamError = io_err.into();
        assert!(matches!(loam_err, LoamError::Io(_)));
        assert!(loam_err.to_string().contains("nope"));
    }

    #[test]
    fn serialize_as_string() {
        let err = LoamError::Path("bad".into());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"path error: bad\"");
    }
}
```

- [ ] **Step 2.2: Add `thiserror` to Cargo**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:

```toml
thiserror = "1"
```

- [ ] **Step 2.3: Register the module**

Edit `src-tauri/src/main.rs`, add at the top (below `cfg_attr` if present):

```rust
mod error;
```

- [ ] **Step 2.4: Run the tests to verify**

```bash
cd src-tauri && cargo test error:: -- --nocapture
```

Expected: `test result: ok. 2 passed; 0 failed`.

- [ ] **Step 2.5: Commit**

```bash
cd ..
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/error.rs src-tauri/src/main.rs
git commit -m "feat(m1): add unified LoamError type"
```

---

## Task 3: Path resolution (Rust)

**Files:**
- Create: `src-tauri/src/paths.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 3.1: Add `dirs` crate**

Edit `src-tauri/Cargo.toml`:

```toml
[dependencies]
dirs = "5"
```

- [ ] **Step 3.2: Write the failing test**

Create `src-tauri/src/paths.rs`:

```rust
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
        ensure_dirs(&root).unwrap(); // second call must not error
    }
}
```

- [ ] **Step 3.3: Add `tempfile` as a dev-dependency**

Edit `src-tauri/Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3.4: Register the module**

Add to `src-tauri/src/main.rs`:

```rust
mod paths;
```

- [ ] **Step 3.5: Run tests — expect RED for any still-unwritten pieces, then GREEN**

```bash
cd src-tauri && cargo test paths:: -- --nocapture
```

Expected: `test result: ok. 5 passed; 0 failed`.

- [ ] **Step 3.6: Commit**

```bash
cd ..
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/paths.rs src-tauri/src/main.rs
git commit -m "feat(m1): add path resolution with ensure_dirs"
```

---

## Task 4: Entry file IO (Rust)

**Files:**
- Create: `src-tauri/src/entries.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 4.1: Write the failing test for `write_entry`**

Create `src-tauri/src/entries.rs`:

```rust
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
    // YYYY-MM-DD, strict
    if s.len() != 10 { return false; }
    let b = s.as_bytes();
    b[4] == b'-' && b[7] == b'-'
        && b[0..4].iter().all(|c| c.is_ascii_digit())
        && b[5..7].iter().all(|c| c.is_ascii_digit())
        && b[8..10].iter().all(|c| c.is_ascii_digit())
}

pub fn write_entry(root: &Path, date: &str, body: &str) -> LoamResult<PathBuf> {
    let path = entry_path(root, date)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    // atomic write: tmp file + rename
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
```

- [ ] **Step 4.2: Register module**

Add to `src-tauri/src/main.rs`:

```rust
mod entries;
```

- [ ] **Step 4.3: Run tests — RED expected for any missing pieces, then GREEN**

```bash
cd src-tauri && cargo test entries:: -- --nocapture
```

Expected: `test result: ok. 6 passed; 0 failed`.

- [ ] **Step 4.4: Commit**

```bash
cd ..
git add src-tauri/src/entries.rs src-tauri/src/main.rs
git commit -m "feat(m1): add entry file IO with atomic writes"
```

---

## Task 5: Settings module (Rust)

**Files:**
- Create: `src-tauri/src/settings.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 5.1: Write failing tests**

Create `src-tauri/src/settings.rs`:

```rust
use crate::error::LoamResult;
use crate::paths::config_path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub aesthetic: String,         // "paper" | "nocturnal"
    pub typewriter_mode: bool,
    pub distraction_free: bool,
    pub ambient_sound: bool,
    pub ambient_volume: f32,
    pub classifier: String,        // "heuristic" | "llm"
    pub llm_model: Option<String>,
    pub prompt_packs_enabled: Vec<String>,
    pub loam_path: Option<String>, // None = default Documents/Loam
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
        assert_eq!(s.typewriter_mode, true); // default
        assert_eq!(s.autosave_debounce_ms, 300); // default
    }
}
```

- [ ] **Step 5.2: Register module**

Add to `src-tauri/src/main.rs`:

```rust
mod settings;
```

- [ ] **Step 5.3: Run tests**

```bash
cd src-tauri && cargo test settings:: -- --nocapture
```

Expected: `test result: ok. 4 passed; 0 failed`.

- [ ] **Step 5.4: Commit**

```bash
cd ..
git add src-tauri/src/settings.rs src-tauri/src/main.rs
git commit -m "feat(m1): add Settings struct with JSON load/save"
```

---

## Task 6: SQLite schema + migration

**Files:**
- Create: `src-tauri/src/db.rs`
- Modify: `src-tauri/Cargo.toml`, `src-tauri/src/main.rs`

**Note on SQLite choice:** The design spec calls out `tauri-plugin-sql`, which is designed for SQL access from the TS layer. This plan uses the `rusqlite` crate (with the `bundled` feature) directly from Rust instead, because all our queries run inside Rust commands (for performance, encapsulation, and because the TS layer only needs the index through higher-level command wrappers). Both achieve the same goal — a bundled SQLite with our schema. This deviation is intentional and compatible with the spec's intent; record it if you add a post-M1 ADR.

- [ ] **Step 6.1: Add `rusqlite` dep**

Edit `src-tauri/Cargo.toml`:

```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
```

- [ ] **Step 6.2: Write failing tests**

Create `src-tauri/src/db.rs`:

```rust
use crate::error::{LoamError, LoamResult};
use crate::paths::sqlite_path;
use rusqlite::{Connection, params};
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
    conn.execute_batch(SCHEMA_V1).map_err(|e| LoamError::Sqlite(e.to_string()))?;
    conn.execute(
        "INSERT OR REPLACE INTO _meta (key, value) VALUES ('schema_version', '1')",
        params![],
    ).map_err(|e| LoamError::Sqlite(e.to_string()))?;
    Ok(())
}

pub fn schema_version(conn: &Connection) -> LoamResult<Option<String>> {
    let mut stmt = conn
        .prepare("SELECT value FROM _meta WHERE key = 'schema_version'")
        .map_err(|e| LoamError::Sqlite(e.to_string()))?;
    let mut rows = stmt.query(params![]).map_err(|e| LoamError::Sqlite(e.to_string()))?;
    if let Some(row) = rows.next().map_err(|e| LoamError::Sqlite(e.to_string()))? {
        let v: String = row.get(0).map_err(|e| LoamError::Sqlite(e.to_string()))?;
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

        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type IN ('table','index') ORDER BY name"
        ).unwrap();
        let names: Vec<String> = stmt
            .query_map(params![], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        for expected in &[
            "entries", "entries_fts", "idx_biome", "idx_themes",
            "unlocks", "prompt_history", "_meta",
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
        migrate(&conn).unwrap(); // must not fail
        assert_eq!(schema_version(&conn).unwrap(), Some("1".into()));
    }

    #[test]
    fn schema_version_errors_before_meta_exists() {
        let (_tmp, root) = setup();
        let conn = open(&root).unwrap();
        // _meta doesn't exist until migrate() runs — confirm callers must migrate first.
        let result = schema_version(&conn);
        assert!(result.is_err());
    }
}
```

- [ ] **Step 6.3: Register module**

Add to `src-tauri/src/main.rs`:

```rust
mod db;
```

- [ ] **Step 6.4: Run tests**

```bash
cd src-tauri && cargo test db:: -- --nocapture
```

Expected: `test result: ok. 3 passed; 0 failed`. First compile may take a minute because `rusqlite` with `bundled` features compiles SQLite from source.

- [ ] **Step 6.5: Commit**

```bash
cd ..
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/db.rs src-tauri/src/main.rs
git commit -m "feat(m1): add sqlite schema migration"
```

---

## Task 7: Tauri commands — wire Rust modules to the frontend

**Files:**
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 7.1: Add `tauri-plugin-fs` is NOT needed — we handle FS in our own commands**

Skip. No change.

- [ ] **Step 7.2: Rewrite `src-tauri/src/main.rs` to register commands**

Replace the entire file content:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod entries;
mod error;
mod paths;
mod settings;

use error::{LoamError, LoamResult};
use settings::Settings;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

/// Application state — holds resolved Loam root directory.
struct AppState {
    root: Mutex<PathBuf>,
}

fn resolve_root_from_settings(user_override: Option<&str>) -> LoamResult<PathBuf> {
    let base = paths::default_base()?;
    Ok(paths::resolve_root(
        &base,
        user_override.map(std::path::Path::new),
    ))
}

#[tauri::command]
fn ensure_loam_dir(state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().unwrap().clone();
    paths::ensure_dirs(&root)?;
    let conn = db::open(&root)?;
    db::migrate(&conn)?;
    Ok(root.to_string_lossy().to_string())
}

#[tauri::command]
fn write_entry(date: String, body: String, state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().unwrap().clone();
    let path = entries::write_entry(&root, &date, &body)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn read_entry(date: String, state: State<AppState>) -> LoamResult<Option<String>> {
    let root = state.root.lock().unwrap().clone();
    entries::read_entry(&root, &date)
}

#[tauri::command]
fn list_entries(state: State<AppState>) -> LoamResult<Vec<String>> {
    let root = state.root.lock().unwrap().clone();
    entries::list_entries(&root)
}

#[tauri::command]
fn load_settings(state: State<AppState>) -> LoamResult<Settings> {
    let root = state.root.lock().unwrap().clone();
    settings::load_settings(&root)
}

#[tauri::command]
fn save_settings(new_settings: Settings, state: State<AppState>) -> LoamResult<()> {
    let mut root_guard = state.root.lock().unwrap();
    settings::save_settings(&root_guard, &new_settings)?;
    // If user changed loam_path, update in-memory root for subsequent commands.
    if let Some(new_path) = new_settings.loam_path.as_deref() {
        *root_guard = std::path::PathBuf::from(new_path);
    } else {
        *root_guard = resolve_root_from_settings(None)?;
    }
    Ok(())
}

fn main() {
    let initial_root = resolve_root_from_settings(None)
        .expect("could not resolve initial Loam directory");

    tauri::Builder::default()
        .manage(AppState {
            root: Mutex::new(initial_root),
        })
        .invoke_handler(tauri::generate_handler![
            ensure_loam_dir,
            write_entry,
            read_entry,
            list_entries,
            load_settings,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 7.3: Verify Rust compiles**

```bash
cd src-tauri && cargo check
```

Expected: `Finished dev [unoptimized + debuginfo] target(s)` with no errors. Warnings about unused code in tests are OK.

- [ ] **Step 7.4: Run full Rust test suite**

```bash
cargo test
```

Expected: all existing tests still pass (`test result: ok. N passed`).

- [ ] **Step 7.5: Commit**

```bash
cd ..
git add src-tauri/src/main.rs
git commit -m "feat(m1): expose Tauri commands for storage and settings"
```

---

## Task 8: TypeScript — shared types

**Files:**
- Create: `src/lib/types.ts`

- [ ] **Step 8.1: Write the module**

```ts
export type Aesthetic = "paper" | "nocturnal";
export type Classifier = "heuristic" | "llm";

export interface Settings {
  aesthetic: Aesthetic;
  typewriter_mode: boolean;
  distraction_free: boolean;
  ambient_sound: boolean;
  ambient_volume: number;
  classifier: Classifier;
  llm_model: string | null;
  prompt_packs_enabled: string[];
  loam_path: string | null;
  autosave_debounce_ms: number;
  onboarding_complete: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  aesthetic: "paper",
  typewriter_mode: true,
  distraction_free: true,
  ambient_sound: false,
  ambient_volume: 0.4,
  classifier: "heuristic",
  llm_model: null,
  prompt_packs_enabled: ["canon"],
  loam_path: null,
  autosave_debounce_ms: 300,
  onboarding_complete: false,
};

export const ISO_DATE = /^\d{4}-\d{2}-\d{2}$/;
export function isIsoDate(s: string): boolean {
  return ISO_DATE.test(s);
}
```

- [ ] **Step 8.2: Commit**

```bash
git add src/lib/types.ts
git commit -m "feat(m1): add shared TS types mirroring Rust Settings"
```

---

## Task 9: TypeScript — Tauri command wrappers

**Files:**
- Create: `src/lib/commands.ts`, `src/lib/__tests__/commands.test.ts`

- [ ] **Step 9.1: Write the failing test with a mocked invoke**

Create `src/lib/__tests__/commands.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import * as cmd from "../commands";
import { DEFAULT_SETTINGS } from "../types";

describe("commands", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("ensureLoamDir invokes ensure_loam_dir and returns root path", async () => {
    mockInvoke.mockResolvedValueOnce("/Users/me/Documents/Loam");
    const root = await cmd.ensureLoamDir();
    expect(mockInvoke).toHaveBeenCalledWith("ensure_loam_dir");
    expect(root).toBe("/Users/me/Documents/Loam");
  });

  it("writeEntry passes date and body", async () => {
    mockInvoke.mockResolvedValueOnce("/path/2026-04-17.md");
    const p = await cmd.writeEntry("2026-04-17", "hello");
    expect(mockInvoke).toHaveBeenCalledWith("write_entry", {
      date: "2026-04-17",
      body: "hello",
    });
    expect(p).toBe("/path/2026-04-17.md");
  });

  it("readEntry returns null for missing entry", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    const body = await cmd.readEntry("2026-04-17");
    expect(body).toBeNull();
  });

  it("readEntry returns body string when present", async () => {
    mockInvoke.mockResolvedValueOnce("hello");
    const body = await cmd.readEntry("2026-04-17");
    expect(body).toBe("hello");
  });

  it("listEntries returns string array", async () => {
    mockInvoke.mockResolvedValueOnce(["2026-04-17", "2026-04-18"]);
    const list = await cmd.listEntries();
    expect(list).toEqual(["2026-04-17", "2026-04-18"]);
  });

  it("loadSettings returns parsed Settings", async () => {
    mockInvoke.mockResolvedValueOnce(DEFAULT_SETTINGS);
    const s = await cmd.loadSettings();
    expect(s).toEqual(DEFAULT_SETTINGS);
  });

  it("saveSettings passes new_settings arg", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    await cmd.saveSettings(DEFAULT_SETTINGS);
    expect(mockInvoke).toHaveBeenCalledWith("save_settings", {
      newSettings: DEFAULT_SETTINGS,
    });
  });
});
```

- [ ] **Step 9.2: Run the test to confirm RED**

```bash
pnpm test commands
```

Expected: FAIL — `Cannot find module '../commands'` or similar.

- [ ] **Step 9.3: Implement `src/lib/commands.ts`**

```ts
import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "./types";

export async function ensureLoamDir(): Promise<string> {
  return invoke<string>("ensure_loam_dir");
}

export async function writeEntry(date: string, body: string): Promise<string> {
  return invoke<string>("write_entry", { date, body });
}

export async function readEntry(date: string): Promise<string | null> {
  return invoke<string | null>("read_entry", { date });
}

export async function listEntries(): Promise<string[]> {
  return invoke<string[]>("list_entries");
}

export async function loadSettings(): Promise<Settings> {
  return invoke<Settings>("load_settings");
}

export async function saveSettings(newSettings: Settings): Promise<void> {
  return invoke<void>("save_settings", { newSettings });
}
```

- [ ] **Step 9.4: Run tests — expect GREEN**

```bash
pnpm test commands
```

Expected: `7 passed`.

- [ ] **Step 9.5: Commit**

```bash
git add src/lib/commands.ts src/lib/__tests__/commands.test.ts
git commit -m "feat(m1): add typed tauri command wrappers"
```

---

## Task 10: EntryStore — higher-level TS API

**Files:**
- Create: `src/lib/entry-store.ts`, `src/lib/__tests__/entry-store.test.ts`

- [ ] **Step 10.1: Write the failing test**

Create `src/lib/__tests__/entry-store.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import { EntryStore } from "../entry-store";

describe("EntryStore", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("getEntry returns null when no file exists", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    const store = new EntryStore();
    const entry = await store.getEntry("2026-04-17");
    expect(entry).toBeNull();
  });

  it("getEntry returns { date, body } when file exists", async () => {
    mockInvoke.mockResolvedValueOnce("hello world");
    const store = new EntryStore();
    const entry = await store.getEntry("2026-04-17");
    expect(entry).toEqual({ date: "2026-04-17", body: "hello world" });
  });

  it("saveEntry rejects invalid date format", async () => {
    const store = new EntryStore();
    await expect(store.saveEntry("2026-4-17", "x")).rejects.toThrow(/invalid date/i);
  });

  it("saveEntry calls write_entry with correct args", async () => {
    mockInvoke.mockResolvedValueOnce("/path/2026-04-17.md");
    const store = new EntryStore();
    const path = await store.saveEntry("2026-04-17", "hi");
    expect(mockInvoke).toHaveBeenCalledWith("write_entry", {
      date: "2026-04-17",
      body: "hi",
    });
    expect(path).toBe("/path/2026-04-17.md");
  });

  it("listEntries returns sorted string array", async () => {
    mockInvoke.mockResolvedValueOnce(["2026-04-17", "2026-04-18"]);
    const store = new EntryStore();
    const list = await store.listEntries();
    expect(list).toEqual(["2026-04-17", "2026-04-18"]);
  });
});
```

- [ ] **Step 10.2: Run to confirm RED**

```bash
pnpm test entry-store
```

Expected: FAIL — module not found.

- [ ] **Step 10.3: Implement `src/lib/entry-store.ts`**

```ts
import { isIsoDate } from "./types";
import { writeEntry, readEntry, listEntries } from "./commands";

export interface Entry {
  date: string;
  body: string;
}

export class EntryStore {
  async getEntry(date: string): Promise<Entry | null> {
    if (!isIsoDate(date)) {
      throw new Error(`invalid date: ${date}`);
    }
    const body = await readEntry(date);
    if (body === null) return null;
    return { date, body };
  }

  async saveEntry(date: string, body: string): Promise<string> {
    if (!isIsoDate(date)) {
      throw new Error(`invalid date: ${date}`);
    }
    return writeEntry(date, body);
  }

  async listEntries(): Promise<string[]> {
    return listEntries();
  }
}
```

- [ ] **Step 10.4: Run tests — GREEN**

```bash
pnpm test entry-store
```

Expected: `5 passed`.

- [ ] **Step 10.5: Commit**

```bash
git add src/lib/entry-store.ts src/lib/__tests__/entry-store.test.ts
git commit -m "feat(m1): add EntryStore with date validation"
```

---

## Task 11: Settings TS module + `useSettings` hook

**Files:**
- Create: `src/lib/settings.ts`, `src/lib/__tests__/settings.test.ts`

- [ ] **Step 11.1: Write the failing test**

Create `src/lib/__tests__/settings.test.ts`:

```ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import { loadAppSettings, saveAppSettings } from "../settings";
import { DEFAULT_SETTINGS } from "../types";

describe("settings module", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("loadAppSettings returns Rust-provided settings", async () => {
    mockInvoke.mockResolvedValueOnce({ ...DEFAULT_SETTINGS, aesthetic: "nocturnal" });
    const s = await loadAppSettings();
    expect(s.aesthetic).toBe("nocturnal");
    expect(s.typewriter_mode).toBe(true); // default propagated
  });

  it("saveAppSettings passes through to save_settings command", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    const next = { ...DEFAULT_SETTINGS, onboarding_complete: true };
    await saveAppSettings(next);
    expect(mockInvoke).toHaveBeenCalledWith("save_settings", { newSettings: next });
  });

  it("saveAppSettings merges partial updates with defaults", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    await saveAppSettings({ aesthetic: "nocturnal" });
    const call = mockInvoke.mock.calls[0];
    expect(call[0]).toBe("save_settings");
    expect(call[1].newSettings.aesthetic).toBe("nocturnal");
    expect(call[1].newSettings.typewriter_mode).toBe(true); // default merged
  });
});
```

- [ ] **Step 11.2: Confirm RED**

```bash
pnpm test settings
```

Expected: FAIL (module not found).

- [ ] **Step 11.3: Implement `src/lib/settings.ts`**

```ts
import { loadSettings as loadCmd, saveSettings as saveCmd } from "./commands";
import { DEFAULT_SETTINGS, type Settings } from "./types";

export async function loadAppSettings(): Promise<Settings> {
  return loadCmd();
}

export async function saveAppSettings(partial: Partial<Settings>): Promise<void> {
  const merged: Settings = { ...DEFAULT_SETTINGS, ...partial };
  return saveCmd(merged);
}
```

- [ ] **Step 11.4: Confirm GREEN**

```bash
pnpm test settings
```

Expected: `3 passed`.

- [ ] **Step 11.5: Commit**

```bash
git add src/lib/settings.ts src/lib/__tests__/settings.test.ts
git commit -m "feat(m1): add settings load/save with partial merge"
```

---

## Task 12: Integrate — proof-of-life boot flow

**Files:**
- Modify: `src/App.tsx`

- [ ] **Step 12.1: Replace `App.tsx` with a storage-verifying proof-of-life**

```tsx
import { useEffect, useState } from "react";
import { ensureLoamDir } from "./lib/commands";
import { EntryStore } from "./lib/entry-store";
import { loadAppSettings } from "./lib/settings";
import type { Settings } from "./lib/types";

const store = new EntryStore();

export default function App() {
  const [root, setRoot] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [entries, setEntries] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const r = await ensureLoamDir();
        setRoot(r);
        const s = await loadAppSettings();
        setSettings(s);
        const list = await store.listEntries();
        setEntries(list);
      } catch (e) {
        setError(String(e));
      }
    })();
  }, []);

  return (
    <main
      style={{
        fontFamily: "Georgia, serif",
        padding: 48,
        color: "#3d2817",
        background: "#f4ecd8",
        minHeight: "100vh",
      }}
    >
      <h1>loam</h1>
      <p style={{ opacity: 0.6 }}>M1 — shell + storage</p>
      {error && <p style={{ color: "#a33" }}>error: {error}</p>}
      {root && (
        <section style={{ marginTop: 32 }}>
          <h3>root</h3>
          <code>{root}</code>
        </section>
      )}
      {settings && (
        <section style={{ marginTop: 24 }}>
          <h3>settings</h3>
          <pre style={{ fontSize: 12 }}>{JSON.stringify(settings, null, 2)}</pre>
        </section>
      )}
      {entries && (
        <section style={{ marginTop: 24 }}>
          <h3>entries ({entries.length})</h3>
          <ul>
            {entries.map((d) => (
              <li key={d}>{d}</li>
            ))}
          </ul>
        </section>
      )}
    </main>
  );
}
```

- [ ] **Step 12.2: Boot the app and verify**

```bash
pnpm tauri dev
```

Expected:

1. Cream window titled "Loam" opens.
2. "root" section shows `/Users/<you>/Documents/Loam` (or platform equivalent).
3. "settings" section shows the default JSON.
4. "entries" shows `0` (empty list on first boot).
5. Check disk: `ls ~/Documents/Loam/` shows `entries/`, `assets/`, `index.sqlite`.

Close the window.

- [ ] **Step 12.3: Manually verify round-trip via a one-off test entry**

Create a temporary file to verify the round-trip. Run in a scratch terminal (not committed):

```bash
echo "# hello" > ~/Documents/Loam/entries/2026-04-17.md
```

Re-launch `pnpm tauri dev`. Expected: entries list now shows `2026-04-17`. Delete the file when done:

```bash
rm ~/Documents/Loam/entries/2026-04-17.md
```

- [ ] **Step 12.4: Commit**

```bash
git add src/App.tsx
git commit -m "feat(m1): boot verifies storage, settings, and entry listing"
```

---

## Task 13: Type check + lint + final full test run

**Files:** none modified — verification only.

- [ ] **Step 13.1: TypeScript typecheck**

```bash
pnpm typecheck
```

Expected: no errors.

- [ ] **Step 13.2: Full Vitest suite**

```bash
pnpm test
```

Expected: all tests pass. Note the total count (should be ≥ 16: canary 1, commands 7, entry-store 5, settings 3).

- [ ] **Step 13.3: Full Rust test suite**

```bash
cd src-tauri && cargo test && cd ..
```

Expected: all tests pass (should be ≥ 20: error 2, paths 5, entries 6, settings 4, db 3).

- [ ] **Step 13.4: Release build sanity check**

```bash
pnpm tauri build --debug
```

Expected: build completes; a `.app` (macOS) or equivalent binary lands in `src-tauri/target/debug/bundle/`. This confirms the project bundles on the developer's platform. (Full `pnpm tauri build` without `--debug` takes much longer; the debug bundle is enough for M1 sanity.)

- [ ] **Step 13.5: Commit nothing, but note success**

Nothing to commit here — this is the green-checkmark gate.

---

## Task 14: Push + open PR

**Files:** none.

- [ ] **Step 14.1: Push the branch**

```bash
git push -u origin feat/m1-tauri-shell-storage
```

- [ ] **Step 14.2: Open the PR**

Visit the GitHub URL printed by `git push` (or `https://github.com/subarnasaikia/loam/pull/new/feat/m1-tauri-shell-storage`). Fill out the PR template using the suggested body below. PR title: `feat(m1): tauri shell + local storage foundation`.

Suggested PR body:

```markdown
## Summary

Lands the M1 foundation from the Loam design spec: Tauri 2 shell, React
proof-of-life UI, typed Tauri commands for markdown entry IO and settings,
and an idempotent SQLite schema migration. No user-facing features yet —
the window boots, verifies the `~/Documents/Loam` directory, and lists
entries on disk.

## Linked issue

Refs the M1 milestone in `docs/superpowers/plans/2026-04-17-m1-tauri-shell-storage.md`.

## Type

- [x] feat — new user-facing behavior (shell + storage plumbing)

## Checklist

- [x] Branch name follows the `type/topic` convention
- [x] Commit messages follow Conventional Commits
- [x] Tests added (Rust: 20+ unit tests across 5 modules; TS: 15+ unit tests across 3 modules)
- [x] Docs updated — plan lives at `docs/superpowers/plans/2026-04-17-m1-tauri-shell-storage.md`
- [x] Self-reviewed the diff
- [x] No secrets, credentials, or personal journal data committed
- [x] Respects Loam's principles — local-first, no network, no telemetry

## Screenshots

(attach a screenshot of the proof-of-life window showing root path, default settings, and empty entries list)

## Notes for the reviewer

- `rusqlite` is compiled with `bundled` to avoid a system SQLite dep. First Rust build is slow (~1 min).
- The `useSettings` React hook is deferred to M2 — only the functional `load/save` pair ships in M1.
- `.gitignore` already covers `target/`, `node_modules/`, user journal data, and SQLite files.
```

- [ ] **Step 14.3: Tick off the PR checklist on GitHub and mark for review**

If solo, self-review the diff and squash-merge once happy. If a reviewer exists, wait for approval.

---

## Definition of Done for M1

- [ ] All 13 tasks' steps ticked.
- [ ] `pnpm typecheck`, `pnpm test`, `cargo test`, `pnpm tauri build --debug` all succeed.
- [ ] `pnpm tauri dev` opens a window that shows the resolved Loam root path, default settings JSON, and an (initially empty) entries list.
- [ ] `~/Documents/Loam/` contains `entries/`, `assets/`, `index.sqlite`, `config.json` after first run.
- [ ] PR merged to `main` via squash-merge, per docs/WORKFLOW.md.
