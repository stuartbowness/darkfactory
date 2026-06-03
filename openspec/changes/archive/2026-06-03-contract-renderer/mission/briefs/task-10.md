# Worker Brief: Task 10 - Update ARCH.md for the new tool (closing task: claude-arch)

## Your Task

Add a section to the repo-root `ARCH.md` describing the new `darkfactory-render` tool and how it relates to the planning pipeline. Do NOT modify the root `CLAUDE.md`.

IMPORTANT override: the generic "claude-arch" closing task says "if ARCH.md exists, do not modify it." That does NOT apply here — the repo's own `CLAUDE.md` rule states *"After making architectural changes, update ARCH.md to reflect the change."* This mission added a whole new tool, so you MUST update the existing `ARCH.md`.

## What to add

Add a new section to `ARCH.md` (repo root: `/Users/home/.darkfactory-2-build/ARCH.md`) describing `darkfactory-render`. Match the existing terse, high-level style of ARCH.md (short prose + bullets; it is 93 lines, sections like "Three-role agent model", "Build loop", "Versioning", "External dependencies"). Cover:
- **What it is:** a deterministic tool that RENDERS `contract.md` from authored sources instead of an LLM re-authoring it — removing duplicated-artifact drift. (Stage 1 of the larger roadmap.)
- **Architecture:** a Rust crate `tools/darkfactory-render` with a pure `core` library (`types`, `parse`, `render`, `shadow`) behind a thin CLI (`render` and `shadow` subcommands). Built/installed/tested like `darkfactory-dispatch` (Makefile, `~/.local/bin`, `tests/`), pinned at v1.0.0.
- **How it relates to the planning pipeline:** it composes the contract from two authored sources — per-scenario `acceptance` entries (grouped by their `[task:N]` link) plus a standing closing-task-check fragment joined to each `Closing: <kind>` task — carrying check statements verbatim and never inventing checks. Workers still never see the contract; the worker brief projects `behavior` only.
- **Shadow mode / cutover:** in this stage the renderer runs READ-ONLY in shadow mode (`darkfactory-render shadow` renders and diffs against the existing hand-authored contract without changing anything). The live mission skill's Phase 5 hand-authoring is unchanged. A later, gated change wires Phase 5 to the renderer once the cutover gate holds (validator-outcome parity across diverse missions, separate-session authoring, a seeded failure + a real defect caught, a rollback path).

Also add `darkfactory-render` to ARCH.md's tools/External-dependencies list (alongside `darkfactory-dispatch`), noting it is pinned at v1.0.0 (consistent with the Versioning section).

Keep it accurate to the as-built code — `design.md` is the source of truth for the architecture (read it). Do not overstate (it is NOT wired into Phase 5 yet).

## Contract checks this task must satisfy (you don't see the contract)
- `ARCH.md` contains a section describing the `darkfactory-render` tool.
- `git diff` shows the root `CLAUDE.md` was NOT modified by this mission (so do not touch `CLAUDE.md`).

## Project Context

(repo CLAUDE.md) ARCH.md is the top-level architecture doc, updated when architecture changes. This mission added the `darkfactory-render` tool, so ARCH.md must describe it. (mission claude-arch closing task) Normally this task only creates CLAUDE.md/ARCH.md if absent; the repo rule overrides for ARCH.md specifically. CLAUDE.md already exists and no rule requires changing it — leave it alone.

## What Previous Tasks Built (source of truth for the section)
- `tools/darkfactory-render` (Tasks 1-8): crate with `core::types`, `core::parse` (`parse_specs`/`parse_tasks`/`parse_fragment`/`validate_closing_kinds`), `core::render` (`build_model` + `render`), `core::shadow` (`compare`), and a `cli` (render/shadow). Standing fragment at `tools/darkfactory-render/templates/closing-task-checks.md`. Pinned v1.0.0, dep `thiserror` only. `make check` green, `make smoke` green.
- The reconciled `openspec/changes/contract-renderer/design.md` (Task 9) is the accurate architecture reference — read its Context, Architecture Decision, Module Inventory, and Migration Plan sections.

## Procedures

**Before editing**, invoke the `/tdd` skill (Skill tool). This is a documentation task (no tests); apply /tdd's discipline by reading design.md + ARCH.md first and keeping the section accurate to the code. (Documentation closing tasks are exempt from the test-presence gate.)

### ENVIRONMENT — non-negotiable
Worktree at `/Users/home/.darkfactory-2-build`. The shell cwd RESETS to `/Users/home/.darkfactory` after EVERY Bash call; you can only WRITE into the worktree when cwd is inside it. So:
1. EVERY Bash command MUST begin with `cd /Users/home/.darkfactory-2-build` (the repo root — ARCH.md is at the repo root, NOT under tools/). Use absolute paths. Never assume cwd persisted.
2. Do NOT run Bash in the background; run git in the foreground.
3. Edit `ARCH.md` with the Edit tool using the absolute path `/Users/home/.darkfactory-2-build/ARCH.md`.

### Verify
- Confirm `ARCH.md` now has the `darkfactory-render` section: `cd /Users/home/.darkfactory-2-build && grep -n 'darkfactory-render' ARCH.md`.
- Confirm `CLAUDE.md` is untouched: `cd /Users/home/.darkfactory-2-build && git status --short CLAUDE.md` (must be empty) and the mission diff `git diff 5350696619e9a52fda4b7c98b92a13d21eba6311 -- CLAUDE.md` (empty).

### Commit
One commit. Stage ONLY `ARCH.md` (it is at the repo root — committable; it is NOT under `openspec/changes/`). Do NOT run `git add -A`/`git add .`. Do NOT stage `CLAUDE.md` or anything under `openspec/changes/`.
```
cd /Users/home/.darkfactory-2-build && git add ARCH.md && \
  git commit -m "[task-10] document darkfactory-render in ARCH.md"
```
Capture the SHA: `cd /Users/home/.darkfactory-2-build && git rev-parse HEAD`.

### Handoff
Write (do NOT commit) with the Write tool to:
`/Users/home/.darkfactory-2-build/openspec/changes/contract-renderer/mission/handoffs/task-10.md`
Format: Task; Commit SHA; Approach (what the section covers, where you placed it); Commands run + exit codes; Concerns.

## Final message
Reply with the commit SHA, a summary of the ARCH.md section you added, and confirmation that CLAUDE.md was not modified.
