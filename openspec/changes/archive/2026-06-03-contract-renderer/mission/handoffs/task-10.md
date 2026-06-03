# Handoff: Task 10 - Update ARCH.md for darkfactory-render (closing task: claude-arch)

## Task
Add a section to the repo-root `ARCH.md` describing the new `darkfactory-render` tool, how it relates to the planning pipeline, and add it to the tools/External-dependencies list. Do NOT modify the root `CLAUDE.md`.

## Commit SHA
3229d14bae2e7d2d99a1b2631e0cd7304f8a8521

## Approach
- Read `openspec/changes/contract-renderer/design.md` (Context, Architecture Decision, Module Inventory, Migration Plan) and the as-built crate to keep the section accurate. Verified against code: crate at `tools/darkfactory-render`, `version = "1.0.0"`, single dep `thiserror`, `src/core/{types,parse,render,shadow}.rs` plus `cli.rs`/`main.rs`/`lib.rs`, `templates/closing-task-checks.md`, and a `Makefile`.
- Added a new `## Contract renderer (darkfactory-render)` section to `ARCH.md`, placed after `## Planning constraints` and before `## Configuration` (it belongs with the planning-pipeline content). It covers: what it is (deterministic renderer of `contract.md` removing duplicated-artifact drift; Stage 1); architecture (Rust crate built/installed/tested like `darkfactory-dispatch`, pinned v1.0.0, `thiserror` only; `core` lib types/parse/render/shadow behind a thin CLI with `render`/`shadow` subcommands); how it relates to the planning pipeline (composes the contract from per-scenario `acceptance` entries grouped by `[task:N]` plus the standing closing-task fragment joined to each `Closing: <kind>` task, verbatim, never inventing checks; workers never see the contract, brief projects `behavior` only); and shadow mode/cutover (read-only shadow diff now, Phase 5 NOT wired, future gated cutover with parity/separate-session/seeded-failure+real-defect/rollback gate).
- Added `darkfactory-render` to the `## External dependencies` list alongside `darkfactory-dispatch`, both annotated `pinned at v1.0.0` (consistent with the Versioning section). Matched ARCH.md's terse high-level tone.
- Did not overstate: explicitly stated it is NOT wired into Phase 5 yet.

## Commands run + exit codes
- `grep -n 'darkfactory-render' ARCH.md` -> 0 (4 mentions: section heading, two body lines, deps line)
- `git status --short CLAUDE.md` -> 0 (empty: CLAUDE.md untouched)
- `git diff 5350696619e9a52fda4b7c98b92a13d21eba6311 -- CLAUDE.md` -> 0 (empty)
- `git add ARCH.md && git commit -m "[task-10] document darkfactory-render in ARCH.md"` -> 0 (1 file changed, 21 insertions(+), 1 deletion(-))
- `git rev-parse HEAD` -> 0 (3229d14bae2e7d2d99a1b2631e0cd7304f8a8521)
- `git show --stat --oneline HEAD` -> 0 (only ARCH.md committed)

## Concerns
None. ARCH.md is the only file staged/committed; CLAUDE.md and everything under `openspec/changes/` were left unstaged. The section is accurate to the as-built code and does not claim Phase 5 integration.
