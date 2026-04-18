---
description: Start the next Loam milestone — read spec, pick next uncompleted milestone, write a plan under docs/superpowers/plans/
argument-hint: [milestone-id, e.g. M2]
allowed-tools: Read, Write, Glob, Grep, Bash
---

Scope: write the next implementation plan for Loam. Do **not** write code yet.

Steps:

1. Read `docs/superpowers/specs/` for the live design spec — identify milestone $ARGUMENTS (or the next uncompleted one if unspecified).
2. Survey `docs/superpowers/plans/` — enumerate shipped milestones by filename.
3. Use `superpowers:writing-plans`. Produce a new plan at `docs/superpowers/plans/YYYY-MM-DD-<slug>.md` with:
   - Goal, architecture, tech stack, branch name, PR target (always `main`).
   - File structure diff showing new paths.
   - Test strategy (Rust: `cargo test` + `tempfile`. TS: `vitest` with mocked `invoke`. E2E deferred unless the milestone requires UI).
   - Tasks as numbered sections, each with `- [ ]` checkbox steps. Every code task starts with a failing test (per the project's TDD rule).
   - "Definition of done" checklist at the end.
4. Commit the plan on its own with `docs(mX): add implementation plan for <topic>`.
5. Stop. Do not implement.
