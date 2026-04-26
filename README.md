<p align="center">
  <img src="brand/logo.svg" alt="Loam" width="280"/>
</p>

<p align="center">
  <em>A local, desktop-only, 3D explorable journal<br/>
  where daily writing builds a living world.</em>
</p>

---

## What is Loam?

Loam is a journal that pulls you back daily through intrinsic wonder, not guilt.

Every entry plants a landmark in a private 3D world you can walk through. Six biomes — meadow, grove, foothills, lake, desert, village — shape themselves around what you write. No streaks. No notifications. No cloud. Your markdown files live on your own disk, always readable, always yours.

## Status

**Alpha — M1 complete.** The local storage foundation is wired and running. A proof-of-life window opens, creates `~/Documents/Loam/`, reads/writes markdown entries, and persists settings across restarts. No 3D world yet — that's M2+.

Full design: [`docs/superpowers/specs/2026-04-17-loam-design.md`](docs/superpowers/specs/2026-04-17-loam-design.md).

## Principles

- **Intrinsic motivation only.** No streaks, badges, fire emojis, notifications, social features.
- **Local-first.** Markdown files on disk are the source of truth. Any editor can read them forever.
- **Desktop-only v1.** Phone is the enemy of the ritual. Mobile read-only may come later.
- **User autonomy.** Opinionated defaults; everything toggleable.
- **The world is the UI.** Open the app and you arrive in your world. No splash screens, no modals.
- **Never shame a missed day.** The garden slept. It didn't die.

## Roadmap

- [x] Design spec
- [x] Brand marks
- [x] Implementation plan
- [x] M1 — Tauri shell + local storage
- [ ] M2 — Minimum viable journal page
- [ ] M3 — Theme classifier + biomes
- [ ] M4–M5 — 3D world + camera flow
- [ ] M6 — Paper & Nocturnal aesthetic swap
- [ ] M7 — Prompt engine + canon
- [ ] M8 — Mastery unlocks
- [ ] M9 — Analytics reveal
- [ ] M10 — Polish + first-run onboarding

See the design spec §11 for scope details.

## Development

```bash
pnpm install          # install dependencies
pnpm tauri dev        # run the desktop app (hot-reload)
pnpm test             # vitest unit tests
pnpm typecheck        # tsc --noEmit
cd src-tauri && cargo test   # Rust unit tests
```

First `cargo build` compiles `rusqlite` (bundled SQLite) — takes ~1 min. Subsequent builds are fast.

On first launch, Loam creates `~/Documents/Loam/` with `entries/`, `assets/`, `config.json`, and `index.sqlite`. All data stays there; nothing is written anywhere else.

## Tech

**Shipping:** Tauri 2 · Rust · React 19 · TypeScript · Vite · SQLite (rusqlite bundled)

**Planned:** React Three Fiber · TipTap · Tailwind

## Contributing

This is an open-source project. Contributions are very welcome.

Start with [CONTRIBUTING.md](CONTRIBUTING.md) and the full [docs/WORKFLOW.md](docs/WORKFLOW.md). Read the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## License

[Apache 2.0](LICENSE).

---

> *"Loam" is the rich, crumbly soil where seeds actually grow. It is what's underfoot when a writer sits down with a blank page.*
