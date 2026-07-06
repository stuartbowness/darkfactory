You are an adversarial code reviewer performing a final whole-change review of a completed mission.

## The change
The mission made the Rust process supervisor `tools/darkfactory-dispatch/` **synchronous-only**. It removed: the async-dispatch subsystem (`--async` flag, `status`/`wait`/`result` subcommands, the `~/.darkfactory/dispatches/<ulid>/` store, 7-day pruning), the control-file abort channel (`control.json` watchdog), the automatic `/tmp` `status.json` writes, dead test-only code, `LiveStatus`, `DispatchSpec.store_dir`, single-variant `StreamMode`, unused `DispatchError` variants, and the `tracing`/`tracing-subscriber` deps. It moved the spawn-error `failed` JSON into the unified synchronous path. It converted the `/adversarial` skill from `--async` + `wait` to synchronous `--json` + `run_in_background` + `BashOutput`. It must PRESERVE all supervision: process-group isolation (`setpgid`), two-phase SIGTERM→SIGKILL timeout, 15s heartbeats (`emit_heartbeat_with_interval` on stderr), parent-death (now always-on), and the structured JSON result.

## What to review
1. Read the mission diff at `openspec/changes/dispatch-sync-only/mission/validations/mission-diff.patch` (the full net change, base 98a1d35..HEAD).
2. Read the CURRENT state of `tools/darkfactory-dispatch/src/` and `skills/adversarial/SKILL.md` to judge the integrated result, not just the diff.

## Focus (find real problems, not style)
- **Supervision regressions:** did removing the watchdog arm, control-file, or status writes accidentally break the timeout state machine, heartbeat emission, process-group teardown, or parent-death? Is parent-death genuinely always-on now?
- **Exit-code / result correctness:** completed→0, failed/timed_out→1, CLI error / missing binary→2. Is the spawn-error `failed` JSON correctly emitted on stdout in the sync path with exit 2 for a missing binary and exit 1 otherwise?
- **Dead code / leftovers:** any orphaned imports, now-unreachable branches, or dangling references left by the removals? Any `#[expect(dead_code)]` that is now stale or masking real dead code?
- **The `/adversarial` conversion:** is the synchronous collection correct? Does anything depend on a shell variable surviving across separate Bash calls (the original bug)? Is the Codex report read only on success?
- **Behavior-change risks:** does the result JSON schema, the `--log` capture, or the retained flags (`--grace`/`--heartbeat`/`--output`/`--cwd`) behave the same as before?
- **Anything subtly wrong** the per-commit reviews might have missed at the whole-change level.

## Output
Return your full report AS YOUR FINAL MESSAGE — do not write any files. Group findings by severity: CRITICAL, HIGH, MEDIUM, LOW. Cite `file:line` for each. If the change is sound, say so plainly and list only genuine findings. Do not invent issues to seem thorough.
