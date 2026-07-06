# Review Checkpoint — Phase 4.4: tasks.md

Status: REVIEWED (clean after fixes)
Timestamp: 2026-06-30T00:05:00Z
Reviewer: Codex (gpt-5.5) via /artifact-review, one pass.

## Result
No CRITICAL/HIGH. Codex confirmed Task 2 is genuinely atomic for the `store_dir`/engine status/control removal and the dependency order is sound.

## Findings resolved
- MEDIUM (coverage): the spec's new regression tests (`--async`/lifecycle rejection, no `/tmp` or `~/.darkfactory/dispatches` files, planted `control.json` ignored, spawn-error JSON) were not explicitly mapped → added as integration-test outputs of Task 2.
- MEDIUM (coverage): dropping `tracing`/`tracing-subscriber` must also clean `Cargo.lock` → `Cargo.lock` added to Task 2 (consume + regenerate).
- LOW (task quality): "line count reduced toward ~1,800-2,000" was not a clear done state → reworded to an informational `wc -l src/*.rs` target with `make check` as the binding gate.

6 tasks (4 feature + 2 closing). Closing-task coverage documented (module-shape re-check optional <5; CLAUDE.md/ARCH.md no-op as both exist; docs in Task 4; no CI).
