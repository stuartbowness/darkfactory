# Coverage Audit: darkfactory-dispatch

PRD: https://github.com/layercodedev/darkfactory/issues/85

## Coverage Table

| # | PRD Requirement (User Story) | Status | Task(s) | Notes |
|---|------------------------------|--------|---------|-------|
| 1 | Codex workers killed automatically if they stall for N seconds | COVERED | 5.1 | Two-phase timeout in engine select! loop |
| 2 | Two-phase timeout (soft warning then hard kill) | COVERED | 2.1, 5.1 | Supervisor owns GracefulStop, engine owns timers |
| 3 | Structured JSON on stdout when dispatch completes | COVERED | 3.1, 5.1, 6.1 | Types defines DispatchResult, engine finalizes, CLI emits |
| 4 | NDJSON heartbeat events on stderr | COVERED | 4.1, 5.1 | Event emitter owns heartbeat ticker, engine updates activity |
| 5 | Child process in its own process group | COVERED | 2.1 | Supervisor sets setpgid, signals -pgid |
| 6 | Stdin closed on child immediately | COVERED | 5.1 | Engine passes empty reader to codex stdin |
| 7 | Dispatch codex workers with CLI interface | COVERED | 6.1 | Clap CLI with model/effort/timeout/sandbox/config flags |
| 8 | Configurable timeout and grace period per dispatch | COVERED | 6.1, 5.1 | CLI accepts --timeout/--grace, engine enforces |
| 9 | Capture child's exit code and signal information | COVERED | 2.1, 3.1, 5.1 | Supervisor detects signal via ExitStatusExt, types includes fields, engine populates |
| 10 | Single static binary with no runtime dependencies | COVERED | 1.1 | Cargo.toml + Makefile with release profile |
| 11 | Async dispatch mode with dispatch ID | COVERED | 6.2 | Async dispatch, ULID, dispatch store |
| 12 | Poll dispatch status via subcommand | COVERED | 6.2 | status/result/wait subcommands |
| 13 | Dispatch result includes token usage | COVERED | 3.1, 5.1 | Types defines TokenUsage, engine accumulates from turn.completed |
| 14 | Parent-death detection for orphan cleanup | COVERED | 2.1 | kqueue on macOS, prctl on Linux, disabled in async mode |

## PRD Solution Items (from revision comments)

| # | Solution Item | Status | Task(s) |
|---|---------------|--------|---------|
| S1 | --output PATH pass-through, child writes, dispatcher reports output_exists | COVERED | 5.1, 6.1 |
| S2 | Activity data as file path lists (string arrays) | COVERED | 3.1, 5.1 |
| S3 | Generic --config key=value pass-through | COVERED | 6.1 |
| S4 | Log routing: parse stdout pipe, stderr separate, both to --log | COVERED | 5.1 |
| S5 | --effort maps to metadata + codex -c flag | COVERED | 6.1, 5.1 |
| S6 | Soft timeout: warning + arm hard timer only | COVERED | 5.1 |
| S7 | Minimal dispatch store at ~/.darkfactory/dispatches/ | COVERED | 6.2 |
| S8 | Parent-death disabled in async mode | COVERED | 2.1, 6.2 |
| S9 | --json and --async mutually exclusive | COVERED | 6.1 |
| S10 | --prompt and --prompt-file mutually exclusive | COVERED | 6.1 |
| S11 | Malformed NDJSON: 4MB buffer, raw_passthrough, parse error count | COVERED | 5.1 |
| S12 | Atomic status.json via tmp+fsync+rename | COVERED | 5.1 |
| S13 | Grace default: timeout/2, minimum 1s | COVERED | 6.1 |
| S14 | Hard-stop floor: 10s | COVERED | 5.1 |
| S15 | Dispatch store cleanup: prune >7 days | COVERED | 6.2 |
| S16 | Mission skill integration | COVERED | 7.1 |
| S17 | darkfactory-init required dependency | COVERED | 8.1 |

## Gaps

None. All 14 user stories and 17 solution items are covered.

## Deviations

| # | Deviation | Classification | Notes |
|---|-----------|---------------|-------|
| D1 | PRD says "Engine adapters (codex and claude CLI argument construction)" -- we build codex-only, no adapter trait | APPROVED-DEVIATION (Substitution) | Claude subagents use Agent tool which provides its own lifecycle management. No loss of functionality. Decided during grill. |

## Closing Tasks Coverage

| Task | Purpose |
|------|---------|
| 9.1 Smoke test | End-to-end verification of the full dispatch flow |
| 9.2 Cleanup | Remove dead code, verify make check passes |
| 9.3 Module shape re-check | Verify Gallant boundary and module interfaces match design.md |
| 9.4 Documentation | Update README.md and CLAUDE.md |
