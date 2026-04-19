---
description: Run Loam's full verification gate — typecheck, vitest, cargo test
argument-hint: (no args)
allowed-tools: Bash
---

Verify the working tree passes every gate before commit/PR. Run the three checks sequentially, report pass/fail for each, and stop at the first failure.

1. `pnpm typecheck` — TypeScript strict check.
2. `pnpm test` — Vitest (all `src/**/__tests__/**`).
3. `cd src-tauri && cargo test` — full Rust suite (pure modules + filesystem tests via tempdir).

At the end print a single summary line: `check: N/M gates passed` and list any that failed with the exact failing test name.

Do not modify code. If tests fail, surface the failure for the user to fix — do not attempt a fix inside this command.
