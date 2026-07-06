# Worker Brief: Task 6 - Cleanup and closing checks

## Your Task
Final cleanup of `tools/darkfactory-dispatch/` after the synchronous-only conversion: remove any leftover debris, apply the small safe simplifications the earlier reviews flagged, close two test-coverage gaps, and confirm `make check` is green.

### Cleanup
- Remove any orphaned imports, unused items, dead references, or leftover scaffolding introduced or exposed by Tasks 1-5. `cargo clippy -- -D warnings` must have zero warnings.

### Safe simplifications (flagged by prior reviews; apply only if clearly safe)
- **Single-variant `StreamMode`:** after earlier removals, `StreamMode` in `event.rs` likely has only `Normal` left. If so, remove the now-pointless enum (and its filtering branch) and inline the single behavior. Do not change what's emitted — only remove the dead abstraction.
- **Duplicated `status_name`:** if `status_name` (or an equivalent status-to-string helper) is duplicated, dedupe to a single definition.
- **Stale `#[expect(dead_code)]` in `types.rs`:** review the `types.rs` `#[expect(dead_code)]`. If some `DispatchError` variants it covers are now actually constructed (no longer dead) or are truly unused, tighten or remove the annotation so it reflects reality. If removing a genuinely-unused variant is clean and safe, do so; if a variant is load-bearing for the result schema, keep it. `cargo build`/`clippy` must stay green (an unfulfilled `#[expect]` fails the build).

### Close two test-coverage gaps (flagged P1 by test-quality review)
Add CLI-level tests (in `tests/cli.rs`) for:
- The **exit-1 failed-dispatch** path: a worker that exits non-zero yields a JSON result with `status` `"failed"` carrying the exit code, and the dispatcher process exits 1.
- The **non-`--json` output branch**: a dispatch run WITHOUT `--json` produces the expected human/plain output (whatever `print_result` emits in non-json mode) and the correct exit code.

### Closing checks
- **Module-shape note (optional, <5 feature tasks):** add a one-line confirmation in your handoff that the module inventory still matches `design.md` (modules: `main.rs`, `cli.rs`, `engine.rs`, `supervisor.rs`, `event.rs`, `types.rs`; the async/status/control machinery is gone). This is a note, not a code change.
- **`make check`** (fmt + clippy `-D warnings` + test) is green.

## Acceptance criteria
- `make check` (fmt + clippy `-D warnings` + test) is green from `tools/darkfactory-dispatch/`.
- No orphaned imports/dead references/scaffolding remain.
- The two new CLI tests (exit-1 failed path, non-json branch) pass.
- Any simplifications made keep behavior identical and the suite green.

### Running tests in this sandbox (required)
The mock `codex` will not exec from `/tmp`. Run cargo/make with a repo-local temp and cargo home:
```
mkdir -p target/tmp target/cargo-home
export TMPDIR="$PWD/target/tmp" CARGO_HOME="$PWD/target/cargo-home"
make check    # or: cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test
```

## Project Context
The dispatcher is now synchronous-only (Tasks 1-5). This is the final closing task: tidy up, apply the safe simplifications the reviews flagged, close the two coverage gaps, and confirm the whole crate is clean.

## Architecture
Crate at `tools/darkfactory-dispatch/` (NO root Cargo.toml). Modules: `main.rs`, `cli.rs`, `engine.rs`, `supervisor.rs`, `event.rs`, `types.rs`. Follow the crate CLAUDE.md rules.

## Key Terms
`Dispatch result` — the JSON object on stdout with `status` (`completed`/`failed`/`timed_out`), `exit_code`, `output`, `output_exists`. Exit codes: 0 completed, 1 failed/timed_out, 2 CLI error.

## What Previous Tasks Built
- Task 1 (07833a0): dead code removed. Task 2 (eece9ea): synchronous-only dispatcher (async/store/control/LiveStatus/tracing removed; failed_start_result moved into the sync path). Task 3 (ac41660): /adversarial converted. Task 4 (070c683): docs aligned. Task 5 (05a1c67): smoke-test verified + supervision/combined-log tests added.
- Suite is green with the TMPDIR/CARGO_HOME workaround (engine 36, supervisor 7, cli 14).

## Downstream Consumers
None — this is the last task. Phase 9 runs whole-repo final gates, a spec-delta check, and a draft PR.

## Procedures
**Before writing any change**, invoke the `/tdd` skill. Add the two CLI tests first (red), then make them pass; keep the whole suite green while doing the cleanup/simplifications.
- Run cargo/make with the `TMPDIR`/`CARGO_HOME` workaround above.
- Commit in a single commit: `[task-6] Cleanup, simplifications, and closing test coverage`. Stage only files under `tools/darkfactory-dispatch/`. Do NOT `git add -A`; never stage anything under `openspec/changes/dispatch-sync-only/`.
- Write your handoff to `openspec/changes/dispatch-sync-only/mission/handoffs/task-6.md` (write the file; do not commit it), including the one-line module-shape note.

## Handoff Report Format
- Task: Task 6 - Cleanup and closing checks
- Commit SHA: <the full SHA>
- Approach: <what you cleaned/simplified; the two new tests; module-shape note>
- Commands run: <commands and exit codes, incl the make check invocation>
- Concerns: <anything uncertain>
