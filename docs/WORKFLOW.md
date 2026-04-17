# Loam Workflow

How work flows from idea to shipped code. This document evolves — propose changes via PR.

## The pipeline

```
idea → issue → triage → branch → PR → review → merge → release
```

Each step below expands one stage.

---

## 1. Idea

- **Informal thoughts → Discussions.** Half-formed ideas, design questions, "does anyone else want X?".
- **Concrete asks → Issues.** Use one of the issue templates below.

## 2. Issue

Three templates:

- **Bug report** — something is broken.
- **Feature request** — a concrete change is proposed.
- **RFC (design-level proposal)** — open a regular issue, add the `rfc` label yourself, and write the proposal in the body. An RFC describes a significant design choice that needs discussion before any code is written.

Every issue enters with the `triage` label.

## 3. Triage

A maintainer reviews open triage-labeled issues within **7 days** and replaces `triage` with one of:

| Label | Meaning |
|---|---|
| `accepted` | Go ahead and build. Anyone may claim it. |
| `needs-design` | Scope/shape needs a short design proposal before code. |
| `out-of-scope` | Closed with an explanation. Thanks for the idea. |
| `good first issue` | Small and approachable for a new contributor. |
| `duplicate` | Closed, linked to the original. |
| `blocked` | Waiting on an earlier issue or external change. |

## 4. Branch

Always branch from the latest `main`.

Branch name format: `<type>/<short-topic>`

- `feat/...` — new user-facing behavior
- `fix/...` — bug fix
- `docs/...` — documentation only
- `chore/...` — tooling, deps, non-user-facing
- `refactor/...` — no behavior change
- `test/...` — test-only changes
- `style/...` — formatting only

Topic part: kebab-case, under ~40 characters, descriptive. Examples:

```
feat/world-landmark-placement
fix/typewriter-scroll-jump
docs/classifier-confidence
chore/upgrade-tauri-2.1
```

## 5. Commits

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body — optional, explains why>

<footer — optional, references issues>
```

Examples:

```
feat(world): place landmarks in correct biome (#42)

Maps each entry's primary theme to its biome via BiomeCalculator.
Previously all landmarks defaulted to the meadow biome regardless
of theme classification.

Closes #42
```

```
fix(editor): preserve scroll on typewriter toggle (#87)
```

```
docs(spec): clarify classifier confidence thresholds
```

Rules:

- Subject: imperative mood, lowercase after the type, no trailing period, under 72 characters.
- Body: wrap at 80 columns. Explain **why**, not what.
- One logical change per commit. PRs are squashed on merge, so granular commits cost nothing and help review.

Valid types: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `style`, `perf`, `build`, `ci`, `revert`.

## 6. PR

- **Title format:** same as the main commit (`feat(scope): subject`).
- **Fill out the PR template** completely.
- **Size:** aim for under ~400 lines of diff. Split larger work into sequential PRs that each pass review independently.
- **Draft PRs** are welcome for early feedback (`gh pr create --draft` or the GitHub UI toggle).
- **Link the issue** in the PR body using `Closes #XX` so it auto-closes on merge.

## 7. Review

Reviewers check:

- [ ] Matches the linked issue's accepted scope
- [ ] Aligns with the design spec — flag drift
- [ ] Tests exist and pass
- [ ] No new lint or type errors
- [ ] Docs updated if user-facing
- [ ] No sensitive data, secrets, or personal journal contents
- [ ] Respects Loam's principles (local-first, no streaks, no telemetry, no social)

### Review comment conventions

Prefix review comments to signal weight:

- `nit:` — minor, non-blocking, purely preference
- `question:` — genuinely asking, not a disguised request
- `suggestion:` — proposed change; author may decline with reason
- `blocking:` — must address before merge

### Expectations

- **Reviewer turnaround:** 7 days. Ping politely if longer.
- **Author turnaround:** address feedback within 14 days or close the PR.
- **At least one approving review** is required for merge.
- **Stale approvals dismissed** on new commits (enforced by branch protection).

## 8. Merge

- **Strategy: squash.** One PR = one commit on `main`.
- Final commit message = PR title + cleaned-up PR body.
- Delete the branch after merge.
- Force-push to `main` is never allowed.

## 9. Release

- **Semver:** `vMAJOR.MINOR.PATCH`.
- **Pre-1.0:** minor bumps may include breaking changes. Release notes will call them out.
- **Release notes:** generated from squashed commit messages since the previous tag, grouped by type.
- **Tagging:**
  ```
  git tag -a v0.1.0 -m "v0.1.0 — short summary"
  git push origin v0.1.0
  ```

---

## Maintainer setup

The following are recommended for `main` branch protection (GitHub → Settings → Branches):

- [x] Require a pull request before merging
- [x] Require 1 approving review
- [x] Dismiss stale pull request approvals when new commits are pushed
- [x] Require status checks to pass (once CI exists)
- [x] Require linear history
- [x] Disallow force pushes
- [x] Disallow deletions

`CODEOWNERS` (at `.github/CODEOWNERS`) will be added as the maintainer team grows. For now all areas are owned by @subarnasaikia.

## This document evolves

If something here is outdated or unclear, open a PR. Good process is living process.
