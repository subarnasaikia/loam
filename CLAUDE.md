# Loam — project instructions

Local-first, desktop-only, 3D explorable journal. Tauri 2 + React 19 + Rust. This file teaches Claude Code how to collaborate on this repo.

## Principles (non-negotiable)

- **No streaks, badges, notifications, social features.** Intrinsic motivation only.
- **Local-first.** Markdown files on disk are the source of truth. No network, no telemetry, no cloud.
- **Desktop-only v1.** No mobile.
- **The world is the UI.** No splash screens or modals.
- **User autonomy.** Opinionated defaults; everything toggleable.

Flag any change that drifts from these.

## Repo map

```
src/                  React + TS frontend (proof-of-life UI)
├── lib/
│   ├── types.ts          Shared TS types (Settings, isIsoDate)
│   ├── commands.ts       Thin typed wrappers over Tauri invoke()
│   ├── entry-store.ts    Higher-level entry API (getEntry/saveEntry/listEntries)
│   ├── settings.ts       loadAppSettings / saveAppSettings
│   └── __tests__/        Vitest (mocked invoke)
src-tauri/src/        Rust backend
├── error.rs              LoamError + LoamResult (serializable)
├── paths.rs              Pure path resolution + ensure_dirs
├── entries.rs            Atomic markdown file IO (YYYY-MM-DD.md)
├── settings.rs           config.json load/save
├── db.rs                 SQLite schema + idempotent migrate (rusqlite bundled)
└── lib.rs                Tauri command registration + AppState
docs/
├── WORKFLOW.md           Branch/commit/PR conventions (read before shipping)
└── superpowers/
    ├── specs/            Design specs (single source of truth for intent)
    └── plans/            Task-by-task implementation plans
```

### Module boundaries

| Module | Responsibility | Disallowed |
|---|---|---|
| `paths` | Pure path strings + `ensure_dirs` | No file IO beyond `ensure_dirs` |
| `entries` | The only place that reads/writes `entries/*.md` | No other module touches markdown |
| `settings` | The only place that reads/writes `config.json` | Same — encapsulated |
| `db` | SQLite open + schema migration | No query logic yet (rides TS later) |
| `commands.ts` | `invoke()` wrappers, typed | Zero business logic |
| `entry-store.ts` | Combines commands into app-level API | Does not call `invoke` directly |

Every module stays small (<150 LOC) with one responsibility.

## Loam on-disk layout

```
~/Documents/Loam/
├── entries/YYYY-MM-DD.md   markdown source of truth
├── assets/                 images, audio
├── config.json             user settings
└── index.sqlite            rebuildable search/index cache
```

Markdown files are the source of truth. SQLite is disposable and must be recreatable from `entries/`.

## Workflow rules

- **TDD is required.** Red → green → refactor for every feature and bugfix. Pure modules get unit tests; R3F/editor layers get Playwright integration tests.
- **Branches:** `<type>/<topic>` per `docs/WORKFLOW.md` (feat, fix, docs, chore, refactor, test, style).
- **Commits:** Conventional Commits, imperative, lowercase subject, under 72 chars. One logical change per commit.
- **Size:** target < 400 lines per PR. Split sequential PRs for larger work.
- **PR template lives in `.github/`.** Fill all sections. Link issue via `Closes #N`.
- **Never commit** personal journal data, secrets, SQLite dumps, or `node_modules`/`target`.

## Commands

```bash
pnpm install            # bootstrap
pnpm tauri dev          # run desktop app in dev
pnpm test               # vitest (one-shot)
pnpm test:watch         # vitest watch
pnpm typecheck          # tsc --noEmit
cd src-tauri && cargo test    # full Rust test suite
pnpm tauri build --debug      # produces Loam.app under target/debug/bundle
```

First `cargo build` compiles `rusqlite` (bundled SQLite) — slow (~1 min). DMG bundling on macOS may fail locally due to `hdiutil` permissions; the `.app` bundle alone is the sanity check.

## Plans and specs

- **Specs** in `docs/superpowers/specs/` describe intent. Read before architectural changes.
- **Plans** in `docs/superpowers/plans/` are executable task lists with checkbox steps. Execute with `superpowers:executing-plans` or `superpowers:subagent-driven-development`.
- After completing a plan's steps, tick the checkboxes and commit the edit alongside the work.

## What to avoid

- Do not add streak counters, badges, fire emojis, push notifications, or social features — even as scaffolding.
- Do not introduce network calls or telemetry.
- Do not touch markdown files outside `src-tauri/src/entries.rs`.
- Do not write tests that hit the real OS `~/Documents/Loam` — use `tempfile::tempdir()` (Rust) or mocked `invoke` (TS).
- Do not create docs/MD files unless the user asks.

## Tone

Terse, concrete. No marketing copy in code, comments, or commits. Explain **why** only when non-obvious.
