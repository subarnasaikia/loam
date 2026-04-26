# Changelog

All notable changes to Loam are documented here.

## [0.1.0.0] - 2026-04-27

### Added

- **Tauri 2 shell** — macOS desktop app scaffold with React 19 + Vite + TypeScript frontend
- **Local markdown storage** — atomic read/write/list for daily entries under `~/Documents/Loam/entries/YYYY-MM-DD.md`; writes use tmp+rename to prevent corruption on crash
- **Settings persistence** — `config.json` load/save with partial-field defaults, survives app restarts; custom `loam_path` honored on boot
- **SQLite index** — idempotent schema migration creates `entries`, `entries_fts`, `unlocks`, `prompt_history`, and `_meta` tables; migration is transactional so a mid-crash leaves the DB clean
- **Typed Tauri command layer** — `ensure_loam_dir`, `write_entry`, `read_entry`, `list_entries`, `load_settings`, `save_settings` exposed to the frontend with `LoamError` serialized as a string for JS consumption
- **TypeScript API** — `EntryStore`, `loadAppSettings`, `saveAppSettings` wrappers with Vitest unit tests; all `invoke` calls mocked, no Tauri runtime required for TS tests
- **Proof-of-life UI** — boot screen shows resolved Loam root, default settings, and entry list to confirm the full stack is wired up
- **Date validation** — both Rust and TypeScript enforce ISO 8601 format with calendar range checks (month 01–12, day 01–31)
- **Path safety** — `loam_path` is validated as absolute and blocked from system-reserved prefixes before being committed
- **Test suite** — 20 Rust unit tests (error, paths, entries, settings, db) + 16 TypeScript unit tests (commands, entry-store, settings, canary)
