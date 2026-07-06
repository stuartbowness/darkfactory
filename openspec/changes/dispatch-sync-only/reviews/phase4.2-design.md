# Review Checkpoint — Phase 4.2: design.md

Status: REVIEWED (clean)
Timestamp: 2026-06-18T19:40:00Z
Reviewer: Codex (gpt-5.5) via /artifact-review, 4 passes (pass 1 cut off by a Codex usage limit, since lifted).

## Findings resolved
- (pass 1) status writes are at TWO `write_status_json_atomic` sites, not just the watchdog arm → design now names both.
- **HIGH (pass 2): compile-safe removal order.** `engine.rs` reads `spec.store_dir` for status-dir selection, cleanup, parent-death, and both status writes, so removing the field before its readers breaks the build → removal order now collapses the async-store + engine status/control removal into one compile-safe step ("every task compiles at its commit").
- MEDIUM: `LiveStatus` status-store type must be deleted → added to removals.
- MEDIUM: `/adversarial` must parse JSON stdout despite the dispatcher's non-zero exit → conversion note added.
- LOW: `orphaned` is an async-lifecycle state, not a `DispatchStatus` variant → removed from the Type Catalog.
- LOW: `RunState.pid` dead field → added to the supervisor removal list.
- **HIGH (pass 3): spawn-error JSON path.** The `failed`-result-on-spawn-error conversion existed only on the async branch (`cli::failed_start_result`); the sync path returned a bare `Err` with no JSON. Design now moves that conversion into the unified sync path, preserving "exactly one JSON result" and the README's `failed`-on-spawn-error semantics.

## Result (pass 4)
No CRITICAL/HIGH remaining. Six mandatory sections present; OpenSpec validates (`design: done`). Existing-codebase, no architecture change (no candidate scoring). Ready for the human gate.
