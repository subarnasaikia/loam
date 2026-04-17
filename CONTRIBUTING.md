# Contributing to Loam

Thanks for your interest — every thoughtful contribution makes Loam better.

This is the short version. The full workflow lives in [docs/WORKFLOW.md](docs/WORKFLOW.md). Both documents evolve as the project does; PRs that improve them are welcome.

## Before you start

1. Read the [design spec](docs/superpowers/specs/2026-04-17-loam-design.md) — it is the source of truth for scope and principles.
2. Read the [Code of Conduct](CODE_OF_CONDUCT.md).
3. Browse [open issues](https://github.com/subarnasaikia/loam/issues). Issues tagged `good first issue` are a gentle entry point.
4. For non-trivial changes, open or comment on an issue **before** you start coding so we can align on scope.

## The short workflow

1. **Open or claim an issue** before starting non-trivial work.
2. **Branch from `main`:**
   - `feat/<topic>` — new user-facing behavior
   - `fix/<topic>` — bug fixes
   - `docs/<topic>` — documentation only
   - `chore/<topic>` — tooling, deps, non-user-facing
   - `refactor/<topic>` — no behavior change
   - `test/<topic>` — test-only changes
3. **Commit in [Conventional Commits](https://www.conventionalcommits.org/) style:**
   ```
   feat(world): place landmarks in correct biome (#42)
   fix(editor): preserve scroll on typewriter toggle (#87)
   docs(spec): clarify classifier confidence thresholds
   ```
4. **Open a PR** using the template. Reference the issue. Attach screenshots for UI changes.
5. **Address review feedback.** At least one approving review is required.
6. **Squash-merge** when approved. One PR = one commit on `main`.

Small docs or typo fixes don't need a prior issue — send the PR directly.

## Style

- **Code:** Prettier + ESLint (configuration lands in M1). Rust formatted with `cargo fmt`.
- **Comments:** minimal. Explain *why*, never *what*. Well-named identifiers carry the *what*.
- **Commit subjects:** imperative, lowercase after the type, no trailing period, under 72 characters.
- **PR titles:** same format as the main commit.

## Principles we will not trade away

Contributions that contradict these will be closed politely:

- No streaks, badges, points, fire emojis, or any social-pressure dopamine mechanics.
- No notifications.
- No cloud-by-default, no telemetry, no analytics phoning home.
- No social / sharing / public features.
- Markdown-on-disk remains the source of truth.

## License of contributions

By submitting a pull request you agree your contribution is licensed under the [Apache License 2.0](LICENSE), the project's license. There is no CLA.

## Contact

Open an issue or discussion for general questions. For anything sensitive (security or conduct concerns), use GitHub's private vulnerability reporting or contact a maintainer via the project's official channels listed on the repository page.
