# Loam — Design Spec

**Date:** 2026-04-17
**Status:** Approved for planning
**Author:** Brainstormed with Claude
**Project:** Loam (open source)

---

## 1. One-Line Description

A local, desktop-only, 3D explorable journal where daily writing builds a living world you inhabit — with no streak shame, no cloud, and no brain-rot mechanics.

## 2. Problem

Modern attention is fragmented by short-form content, infinite scroll, and reactive notifications. Existing journaling tools (Day One, Notion, Obsidian) are either too sterile, too feature-bloated, or adopt the same dopamine-exploitation patterns they claim to escape. A journaling habit dies when the ritual feels like either a chore or another feed.

The goal: a tool that pulls the writer back daily through intrinsic wonder — the compounding joy of watching a private world grow — rather than through guilt, streaks, or notifications.

## 3. Core Principles (Non-Negotiable)

1. **Intrinsic motivation only.** No streaks, badges, fire emojis, notifications, social features, public profiles, or sharing. Ever.
2. **Local-first.** Markdown files on disk are the source of truth. Any editor can read them forever. No cloud by default.
3. **Desktop-only v1.** Phone is the enemy of the ritual. Mobile read-only may come later; writing on phone is explicitly rejected.
4. **User autonomy (Self-Determination Theory).** Every behavior is toggleable. Opinionated defaults, total configurability.
5. **The world is the UI.** Open app → 3D world → camera fly-in to today's page → pull back on close. No "new entry" button. No modals. The world is the doorway.
6. **Never shame a missed day.** A skipped day means "the garden slept." Gardens don't die — they wait.

## 4. v1 Scope

### 4.1 Daily Page
- One markdown file per day: `entries/YYYY-MM-DD.md`.
- Opening **prompt** (from curated canon, contextually selected). Skippable — skipping yields pure blank page, no guilt.
- **Reflection surface** (top, primary): free-form rich-text editor with markdown output.
- **Rapid-log strip** (bottom, collapsible): classic bullet-journal items:
  - `·` task
  - `○` event
  - `—` note
- Either surface may be empty on a given day. Both write into the same daily markdown file under two sections.

### 4.2 3D World
- Low-poly, painterly aesthetic (A Short Hike / Alto's Odyssey lineage).
- **Six fixed biomes** (v1):
  - **Meadow** — gratitude, joy, small delights
  - **Grove** — reflection, memory, inner quiet
  - **Foothills** — ideas, ambitions, plans
  - **Lake** — grief, sadness, heavy emotions
  - **Desert** — fatigue, struggle, burnout
  - **Village / Workshop** — craft, work, doing
- Each entry plants a **landmark** in the biome matching its primary theme. Landmark type encodes word-count tier and entry intensity (e.g., small sapling → oak; pebble → monument).
- **Walk / pan / zoom** via mouse or trackpad. Click any past landmark to re-read that entry.
- Most-written biomes grow largest. The world literally reflects what you've thought about most.

### 4.3 Theme Classification
- **Default (v1):** Local TypeScript heuristic lexicons. No ML, no network. Scores text against word-lists for each biome, picks primary + optional secondary.
- **User override:** Always-visible chips below the entry. One click re-classifies. The world updates immediately.
- **v1.5 (deferred, design hooks present):** Local small LLM (Phi-3-mini / Gemma-2B via `ollama` or `llama.cpp`). User chooses from a curated list of supported models. Runs as Tauri sidecar. Classification + richer analytics reveal.

### 4.4 Aesthetic System
- **Paper & Ink** (default): cream background, warm serif (Fraunces or equivalent), letterpress warmth. Journal page feels like Moleskine paper.
- **Nocturnal Garden** (dark toggle): dark canvas, same serif, constellations overlay the same world. Moon phases visible.
- Toggle anytime from header. Transition is smooth, no page reload.
- World renders the same geometry; shaders/materials swap between day palette and night palette.

### 4.5 Writing Session Mechanics
All on by default, all toggleable:
- **Typewriter mode:** current line stays vertically centered.
- **Distraction-free:** chrome fades while actively writing. `Esc` or mouse-move restores.
- **Full edit history:** every save-state preserved. User can scrub past drafts of today's entry.
- **Auto-save per keystroke** (debounced ~300ms). No save button. Nothing ever lost.
- **Ambient sound** (off by default, opt-in): soft typewriter clack, pen-on-paper whisper.

### 4.6 Mastery Unlocks
Three parallel tracks. No item lost — not yet unlocked simply means "coming." No guilt mechanics. Everything cumulative (lifetime entries), never streak-based.

**Track A — Prompt Packs:**
- Week 1: Canon (~80 hand-curated prompts from Stoics, Rilke, Tara Brach, James Hollis, The Artist's Way).
- Week 3: "Hollis on Shadow" pack.
- Month 2: "Rilke letters adapted" pack.
- Month 6: "Seasonal Stoics" pack.
- (More packs added over product life; user-authored prompts deferred to v1.5.)

**Track B — World Features:**
- Day 14: seasons activate (spring/summer/autumn/winter cycle in world).
- Day 30: weather (rain, mist) activates and correlates softly with recent biome weighting.
- Day 60: moon phases visible in Nocturnal aesthetic.
- Day 90: "night walk" mode — dark world with ambient sound, meditative exploration.

**Track C — Deeper Seeing:**
- Day 14: monthly analytics reveal unlocks.
- Day 30: local LLM opt-in becomes available (justifies the ~4GB install by then).
- Day 365: yearly retrospective (themes, biome growth, arc-of-the-year).

### 4.7 Prompt System
- Curated canon (~80 prompts in v1, growing). Each tagged with: mode (morning/evening/any), tone (reflective/practical/difficult), biome hints.
- **Contextual rotation:**
  - Recent biome weighting — if user has been in Lake a lot, prompts acknowledge it gently without prying.
  - Season — summer/winter tonal adjustments.
  - Day of week — Sunday leans review, Monday leans intention.
- **Never-repeat window:** ~30 days between reuses.
- **Skip** always available. Skipping shows pure blank page — no guilt, no follow-up.

### 4.8 Settings Panel
All of the above surfaced as toggles or selections:
- Aesthetic (Paper / Nocturnal / Auto-by-system-theme).
- Writing: typewriter mode, distraction-free, ambient sound volume, edit-history visibility.
- Auto-save interval.
- Classifier (heuristic / LLM when installed).
- Prompt packs enabled.
- Loam path (default: `~/Documents/Loam`).
- Reindex from files (rebuilds SQLite from markdown).
- Export all / import.

### 4.9 Onboarding (First Launch)
- Minimal. 3 screens max:
  1. "This is Loam. Your writing stays on your computer."
  2. "Pick where your files live." (default folder shown, changeable)
  3. "Here's your world. Click the empty spot to begin."
- No account. No email. No "tour."

## 5. Explicitly Out-of-Scope for v1

- Mobile app (read-only view planned for later release).
- Cloud sync of any kind (design is B-ready — v1.5).
- Local LLM integration (hooks present; feature ships v1.5).
- User-authored prompt rotation (v1.5).
- Collections pages (books-to-read, ideas list, custom lists — v2).
- Reading-and-notes integration / book companion (separate spec, likely v2 or its own product).
- Social features (never).
- Public sharing of entries or worlds (never).
- Analytics / telemetry back to any server (never).

## 6. Architecture

### 6.1 Component Map

```
┌─ Tauri Shell (Rust) ──────────────────────────────┐
│  ├─ fs_cmd: read/write markdown, watch files      │
│  ├─ sqlite plugin: index, search, biome cache     │
│  ├─ settings io: config.json                      │
│  └─ llm sidecar hook (v1.5): ollama/llama.cpp     │
└──────────────────────┬────────────────────────────┘
                       │ Tauri IPC
┌──────────────────────┴────────────────────────────┐
│  React 18 + Vite + TypeScript                     │
│                                                    │
│  ┌─ World Layer (R3F) ────────────────────────┐   │
│  │  WorldCanvas · Biomes · Landmarks · Camera │   │
│  │  fly-in/out controller · day/night shader  │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ┌─ Page Layer (DOM) ─────────────────────────┐   │
│  │  JournalPage (TipTap) · PromptHeader       │   │
│  │  RapidLogStrip · Typewriter scroll hook    │   │
│  └────────────────────────────────────────────┘   │
│                                                    │
│  ┌─ Logic ─────────────────────────────────────┐  │
│  │  ThemeClassifier (heuristic) · UnlockRules  │  │
│  │  PromptSelector · BiomeCalculator           │  │
│  │  EntryStore (markdown + sqlite sync)        │  │
│  └─────────────────────────────────────────────┘  │
│                                                    │
│  ┌─ UI Shell ──────────────────────────────────┐  │
│  │  Settings · Reveal (monthly/yearly)         │  │
│  │  AestheticTheme (Paper/Nocturnal)           │  │
│  └─────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────┘
```

### 6.2 Module Boundaries

| Module | Owns | Does not know about |
|---|---|---|
| `EntryStore` | file IO + SQLite sync | world, prompts, UI |
| `ThemeClassifier` | `classify(text) → themes` (pure) | storage, world |
| `PromptSelector` | `select(today, stats) → prompt` (pure) | UI, world |
| `UnlockRules` | `evaluate(stats) → unlocks[]` (pure data + pure fn) | storage, UI |
| `BiomeCalculator` | theme → biome → landmark placement (pure) | UI |
| `WorldCanvas` | R3F scene, camera, landmark rendering | text editor, storage |
| `JournalPage` | TipTap editor, prompt display, rapid log | world, classifier |
| `Settings` | config.json read/write, toggle UI | storage, classifier |

Every pure module (classifier, selector, rules, biome calc) is independently unit-testable. World and editor are integration-tested via Playwright.

### 6.3 Daily-Write Data Flow

1. App launch → Tauri reads `index.sqlite` + last N entries → React renders World.
2. Camera fly-in to today's tile (empty pedestal if no entry yet). Page opens.
3. `PromptSelector` picks today's prompt from canon (season + recent biomes + weekday).
4. User writes. Each keystroke → debounced write to `entries/YYYY-MM-DD.md`.
5. On page close or explicit commit:
   - `ThemeClassifier.classify(body)` scores text against lexicons → primary + optional secondary theme.
   - Markdown frontmatter updated.
   - SQLite row upserted.
6. `BiomeCalculator` maps theme → biome → determines landmark type and world placement.
7. Camera pulls back. New/updated landmark settles into world with gentle animation.

## 7. Data Model

### 7.1 Markdown File (Source of Truth)

```markdown
---
date: 2026-04-17
prompt: "What surprised you today?"
prompt_id: canon.surprise.001
themes: [reflection, gratitude]
biome: grove
landmark: small-oak
word_count: 347
duration_ms: 612000
edit_sessions: 2
aesthetic_when_written: paper
---

# reflection

The rain started while I was making coffee and I just stood at the window
for ten minutes. I cannot remember the last time I did nothing like that.
...

# rapid log

· finish design doc
○ call with mira @ 3pm
— she mentioned that book about attention
```

Entries are human-readable forever. If the app is deleted, the user still has their journal.

### 7.2 SQLite Schema (Index, Rebuildable)

```sql
CREATE TABLE entries (
  date TEXT PRIMARY KEY,        -- ISO date: 2026-04-17
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
  created_at INTEGER,           -- unix seconds
  updated_at INTEGER
);

CREATE VIRTUAL TABLE entries_fts USING fts5(date UNINDEXED, body);

CREATE INDEX idx_biome ON entries(biome);
CREATE INDEX idx_themes ON entries(primary_theme);

CREATE TABLE unlocks (
  id TEXT PRIMARY KEY,          -- "pack.hollis.shadow", "world.seasons"
  unlocked_at INTEGER,
  kind TEXT                     -- prompt_pack | world_feature | insight
);

CREATE TABLE prompt_history (
  prompt_id TEXT,
  shown_on TEXT,                -- ISO date
  skipped INTEGER,              -- 0/1
  PRIMARY KEY (prompt_id, shown_on)
);
```

A `reindex` command rebuilds the DB from the markdown files on disk. The DB is never authoritative.

### 7.3 Config (`config.json`)

```json
{
  "aesthetic": "paper",
  "typewriterMode": true,
  "distractionFree": true,
  "ambientSound": false,
  "ambientVolume": 0.4,
  "classifier": "heuristic",
  "llmModel": null,
  "promptPacksEnabled": ["canon"],
  "loamPath": "~/Documents/Loam",
  "autosaveDebounceMs": 300,
  "onboardingComplete": true
}
```

### 7.4 Filesystem Layout

```
~/Documents/Loam/
  entries/
    2026-04-17.md
    2026-04-18.md
    ...
  config.json
  index.sqlite
  assets/
    prompt-packs/
      canon.json
      hollis-shadow.json
    world/
      (low-poly GLTFs bundled with app; user-local snapshots here)
```

## 8. Tech Stack

| Layer | Choice | Rationale |
|---|---|---|
| Shell | Tauri 2 | ~10MB binary, Rust, native FS |
| Frontend | React 18 + Vite + TypeScript | R3F ecosystem, type safety |
| 3D | React Three Fiber + drei | Declarative Three.js |
| Editor | TipTap (headless ProseMirror) | Typewriter-mode scroll, markdown out |
| Database | SQLite via `tauri-plugin-sql` | Index only; files = truth |
| Styling | Tailwind + CSS variables | Theme swap via root var flip |
| Audio | Howler.js | Optional ambient + clack |
| File watch | Rust `notify` via Tauri command | External edits trigger reindex |
| Build | pnpm + Tauri bundler | Standard |

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| 3D world becomes a game-dev rabbit hole | Hard scope: ~12 landmark types, 6 biomes, no characters, no physics, no quests. Milestone gate: if world work exceeds 25% of build, stop and simplify. |
| Prompt canon quality = product quality | Treat canon curation as a dedicated pass with research citations. Ship ~80 prompts initially, all hand-reviewed. No generic filler. |
| Heuristic classifier is wrong for nuanced writing | Always-visible correction chips. Show classifier confidence. Wrong biomes self-correct as user re-tags. LLM path for users who care, v1.5. |
| Tauri webview WebGL variance | Test R3F on macOS (WebKit), Windows (WebView2), Linux (WebKitGTK) in M4. If Linux is broken, document + ship mac/win first. |
| External edits cause SQLite desync | `notify`-based file watcher → incremental reindex. `reindex` command always available. |
| Large entries break typewriter-mode perf | Virtualize long documents in TipTap. Soft cap at 20k words/entry with a gentle warning. |
| World file grows unbounded over years | Entries paginated by year. Old years fade distant. Camera can jump by year past 365. |
| Disk corruption on crash mid-write | Atomic writes: write to `.tmp` file, fsync, rename. Standard Tauri FS idiom. |

## 10. Success Criteria (v1 Ship)

### Quantitative
- Tauri binary ≤ 15MB
- Cold start to interactive world ≤ 2s on M1 MacBook Air
- Zero network calls after first launch (verified with `tcpdump`)
- No data loss across 100 simulated updates (fuzz test harness)
- Entry save latency ≤ 50ms (debounced autosave round-trip)
- 60fps world rendering with 365 landmarks on M1

### Qualitative
- Opening the app feels like walking into a room, not launching software.
- Writing for 10 minutes leaves you feeling you actually wrote — not performed.
- The world 30 days in feels like *your* world, not a template.
- Settings feel respectful. No "are you sure?" guilt-trips.

## 11. Implementation Milestones

Guidance for the plan stage; ordering may shift.

1. **M1 — Shell + storage.** Tauri skeleton, React mount, markdown file IO via Rust command, SQLite plugin, settings store. No UI polish.
2. **M2 — Minimum viable journal.** Route `/today`, prompt header, TipTap editor, auto-save to `YYYY-MM-DD.md`, past-entry list.
3. **M3 — Theme classifier + biome mapping.** Heuristic lexicons (6 biomes). Frontmatter writes themes + biome. Correction chips UI.
4. **M4 — World scaffold.** R3F canvas, 6 biome tiles placed, one landmark type per biome, entries render as landmarks in correct biomes.
5. **M5 — Flow & camera.** Fly-in on day select, pull-back on close, walk/pan/zoom, click past landmark → open that day.
6. **M6 — Aesthetic theme swap.** Paper & Ink + Nocturnal CSS variables; world shader swap day/night; toggle in header.
7. **M7 — Prompt engine + canon.** Selector logic, ~80 curated prompts with metadata, rotation rules.
8. **M8 — Unlocks.** Rules engine, prompt-pack gating, world-feature gating (seasons, weather).
9. **M9 — Reveal.** Monthly summary view (themes, landmarks, word totals).
10. **M10 — Polish.** Typewriter mode, distraction-free, ambient sound, settings panel, first-run onboarding.

**Rough scale:** M1–M3 ≈ 2 weeks solo. M4–M6 ≈ 3 weeks (world is the biggest unknown). M7–M10 ≈ 2 weeks. **v1 ships in ~7–8 weeks of focused solo work.** This is a scale estimate, not a deadline.

## 12. Open Questions for Plan Stage

- Licensing / distribution model (free + donation? paid one-time? deferred.)
- Update mechanism — Tauri updater vs manual releases.
- Crash reporting — local-only log file? (No network telemetry, per principles.)
- Final prompt canon curation pipeline — where the 80 prompts come from, who vets them.
- Visual identity beyond logo — typography scale, color ramp, illustration style — separate pass.

---

*End of design spec.*
