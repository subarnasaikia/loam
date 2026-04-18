---
description: Scaffold a new Loam module TDD-style — failing test first, then minimal impl
argument-hint: <rust|ts> <module-name> [description]
allowed-tools: Read, Write, Edit, Bash, Glob
---

Scaffold a new Loam pure module following project conventions. **Tests first; implementation second.**

Arguments: $ARGUMENTS (first word = `rust` or `ts`, second = module name, rest = description).

## For `rust <name>`

1. Create `src-tauri/src/<name>.rs` containing the `#[cfg(test)] mod tests` block **with real failing tests** reflecting the stated description. Use `tempfile::tempdir()` for any filesystem work. Declare `use crate::error::{LoamError, LoamResult};` if the module can fail.
2. Register the module alphabetically in `src-tauri/src/lib.rs` (`mod <name>;`).
3. Run `cd src-tauri && cargo test <name>::` — confirm RED.
4. Stop. Ask the user to author the minimal implementation (or offer to draft one from the test body).

## For `ts <name>`

1. Create `src/lib/<name>.ts` as an empty stub export.
2. Create `src/lib/__tests__/<name>.test.ts` with failing tests. Use the mocked-invoke pattern:
   ```ts
   const mockInvoke = vi.fn();
   vi.mock("@tauri-apps/api/core", () => ({ invoke: (...a) => mockInvoke(...a) }));
   ```
3. Run `pnpm test <name>` — confirm RED.
4. Stop. Ask before writing the impl.

## Rules

- Module must be under 150 LOC when finished.
- One responsibility per module. If the description covers two things, refuse and propose splitting.
- Never read or write the real `~/Documents/Loam` from tests.
