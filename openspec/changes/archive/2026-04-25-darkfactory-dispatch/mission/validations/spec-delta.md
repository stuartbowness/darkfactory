# Spec Delta Check

Coverage audit: `openspec/changes/darkfactory-dispatch/coverage-audit.md`
Codebase: `~/.darkfactory/tools/darkfactory-dispatch/src/`
Skills: `~/.darkfactory/.claude/skills/`

## Verdict: CLEAN

All 14 user stories, 17 solution items, and 1 approved deviation verified present in the codebase.

---

## User Story Verification

| # | Requirement | Verdict | Evidence |
|---|-------------|---------|----------|
| 1 | Workers killed on stall | VERIFIED | `engine.rs:99-118` -- soft timeout fires, arms hard timer, then calls `graceful_stop` which sends SIGTERM then SIGKILL |
| 2 | Two-phase timeout | VERIFIED | `engine.rs:99-118` -- soft_sleep fires first (line 99), sets `soft_armed = false`, emits `timeout_warning`, then creates `hard_sleep` (line 103). Hard sleep branch (line 106) calls `graceful_stop(grace_sec.max(10))` |
| 3 | Structured JSON on stdout | VERIFIED | `types.rs:26-37` -- `DispatchResult` with Serialize/Deserialize. `cli.rs:233-244` -- `print_result` serializes to stdout when `--json`. `main.rs:84` -- calls `print_result` |
| 4 | NDJSON heartbeat events on stderr | VERIFIED | `event.rs:147-152` -- `emit_heartbeat` method. `event.rs:229-242` -- `emit` serializes JSON + newline to writer. `engine.rs:133-139` -- heartbeat interval in select loop. Default writer is stderr (`event.rs:105`) |
| 5 | Child in own process group | VERIFIED | `supervisor.rs:134-137` -- `pre_exec` calls `libc::setpgid(0, 0)`. `supervisor.rs:277-298` -- `signal_group` sends signals to `-pgid` (negative = process group) |
| 6 | Stdin closed on child | VERIFIED | `engine.rs:45` -- `SupervisorBuilder` uses `.stdin(StdinMode::Null)`. `supervisor.rs:117` -- `StdinMode::Null` maps to `Stdio::null()` |
| 7 | CLI flags match spec | VERIFIED | `cli.rs:22-64` -- Clap struct with `--model`, `--effort`, `--timeout`, `--grace`, `--sandbox`, `--config`, `--prompt`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json`, `--async` |
| 8 | Configurable timeout and grace | VERIFIED | `cli.rs:33-37` -- `--timeout` (default 900) and `--grace` flags. `cli.rs:98` -- grace defaults to `(timeout / 2).max(1)`. `engine.rs:35,696-702` -- `effective_grace_sec` applies same default |
| 9 | Capture exit code and signal | VERIFIED | `supervisor.rs:42-46` -- `ExitResult { code, signal }`. `supervisor.rs:176-180` -- populated from `status.code()` and `status.signal()` (via `ExitStatusExt`). `types.rs:30-31` -- `DispatchResult` includes `exit_code` and `signal` |
| 10 | Single static binary | VERIFIED | `Cargo.toml:22-27` -- release profile with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`. `Makefile:9-10` -- `install` target builds release and copies to `~/.local/bin/` |
| 11 | Async dispatch mode with dispatch ID | VERIFIED | `cli.rs:63` -- `--async` flag. `cli.rs:9` -- `ulid::Ulid` for dispatch IDs. `cli.rs:131-139` -- `prepare_async_store` creates `~/.darkfactory/dispatches/<id>/`. `main.rs:45-67` -- async mode branch sets up store, writes host.pid, prints async_started JSON |
| 12 | Poll dispatch status via subcommand | VERIFIED | `cli.rs:67-71` -- `Commands::Status`, `Commands::Result`, `Commands::Wait` subcommands. `cli.rs:191-231` -- `print_status`, `print_result_for_id`, `wait_for_id` implementations. `main.rs:90-102` -- `handle_lifecycle_command` routes subcommands |
| 13 | Token usage tracking | VERIFIED | `types.rs:73-78` -- `TokenUsage { input, output, reasoning }`. `engine.rs:289-296` -- `parse_codex_event` extracts tokens from `turn.completed` events. `engine.rs:549-555` -- tracker accumulates token counts across turns |
| 14 | Parent-death detection | VERIFIED | `supervisor.rs:301-344` -- macOS: kqueue with `EVFILT_PROC` + `NOTE_EXIT`, kills process group on parent exit. `supervisor.rs:139-145` -- Linux: `prctl(PR_SET_PDEATHSIG, SIGKILL)` in pre_exec. `supervisor.rs:346-351` -- Linux fallback also in `watch_parent_death`. `engine.rs:46` -- `.parent_death_watch(spec.store_dir.is_none())` disables in async mode |

## Solution Item Verification

| # | Item | Verdict | Evidence |
|---|------|---------|----------|
| S1 | --output PATH pass-through | VERIFIED | `cli.rs:48` -- `--output` flag. `engine.rs:200-202` -- passes `-o` to codex args. `engine.rs:596-600` -- checks `output_exists` in finalize. `types.rs:33` -- `output_exists: bool` in result |
| S2 | Activity as file path lists | VERIFIED | `types.rs:80-88` -- `Activity { files_changed: Vec<String>, files_read: Vec<String>, commands_run: Vec<String>, tool_calls, parse_errors }` |
| S3 | Generic --config pass-through | VERIFIED | `cli.rs:41-42` -- `--config` with `num_args = 1`. `engine.rs:197-199` -- each config passed as `-c <value>` to codex |
| S4 | Log routing to --log | VERIFIED | `engine.rs:52,209-227` -- `open_log` creates log file. `engine.rs:83,229-254` -- both stdout lines and stderr captured to shared log file |
| S5 | --effort maps to codex -c flag | VERIFIED | `engine.rs:192-195` -- effort mapped to `-c model_reasoning_effort="<effort>"` |
| S6 | Soft timeout: warning + arm hard timer | VERIFIED | `engine.rs:99-103` -- soft timeout emits `timeout_warning`, then creates `hard_sleep`. Hard timer is only armed after soft fires |
| S7 | Dispatch store at ~/.darkfactory/dispatches/ | VERIFIED | `cli.rs:126-129` -- `dispatch_store_root()` returns `$HOME/.darkfactory/dispatches`. `cli.rs:131-139` -- `prepare_async_store` creates per-dispatch directory |
| S8 | Parent-death disabled in async mode | VERIFIED | `engine.rs:46` -- `.parent_death_watch(spec.store_dir.is_none())` -- false when store_dir is set (async mode). `supervisor.rs:107-109` -- `parent_death_watch` setter. `supervisor.rs:168-169` -- only calls `watch_parent_death` when enabled |
| S9 | --json and --async mutually exclusive | VERIFIED | `cli.rs:59` -- `#[arg(long, conflicts_with = "async_mode")]` on json. `cli.rs:62` -- `#[arg(long = "async", conflicts_with = "json")]` on async_mode |
| S10 | --prompt and --prompt-file mutually exclusive | VERIFIED | `cli.rs:44` -- `conflicts_with = "prompt_file"` on prompt. `cli.rs:47` -- `conflicts_with = "prompt"` on prompt_file |
| S11 | Malformed NDJSON: 4MB buffer, raw_passthrough, parse error count | VERIFIED | `engine.rs:21` -- `STDOUT_BUFFER_BYTES = 4 * 1024 * 1024`. `engine.rs:55` -- `BufReader::with_capacity(STDOUT_BUFFER_BYTES, stdout)`. `engine.rs:276,309` -- non-JSON lines become `RawPassthrough`. `engine.rs:571-574` -- `RawPassthrough` increments `parse_errors`. `types.rs:87` -- `parse_errors: u64` in Activity |
| S12 | Atomic status.json via tmp+fsync+rename | VERIFIED | `engine.rs:656-688` -- `write_status_json_atomic`: creates tmp file, writes JSON, calls `sync_all()`, renames to final path, syncs parent dir. `cli.rs:405-426` -- `write_json_atomic` in CLI uses same pattern |
| S13 | Grace default: timeout/2, minimum 1s | VERIFIED | `cli.rs:98` -- `self.grace.unwrap_or_else(\|\| (self.timeout / 2).max(1))`. `engine.rs:696-702` -- `effective_grace_sec` applies same logic: `(timeout_sec / 2).max(1)` when grace is 0 |
| S14 | Hard-stop floor: 10s | VERIFIED | `engine.rs:115` -- `graceful_stop(grace_sec.max(10))` -- floor of 10 seconds on hard stop |
| S15 | Dispatch store cleanup: prune >7 days | VERIFIED | `cli.rs:14` -- `DISPATCH_RETENTION_SECS = 7 * 24 * 60 * 60`. `cli.rs:436-460` -- `prune_old_dispatches` removes dirs older than 7 days (by ULID timestamp or mtime). Called from `prepare_async_store` (line 135) |
| S16 | Mission skill calls darkfactory-dispatch | VERIFIED | `mission/SKILL.md` Phase 8 Step 2 -- worker dispatch script uses `darkfactory-dispatch` with `--model`, `--effort`, `--timeout`, `--sandbox`, `--config`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json` flags. Phase 9 adversarial reviewer also dispatches via `darkfactory-dispatch` |
| S17 | darkfactory-init checks for darkfactory-dispatch | VERIFIED | `darkfactory-init/SKILL.md` Step 4 -- checks `darkfactory-dispatch --version` as a required CLI tool. Reports install command: `cd ~/.darkfactory/tools/darkfactory-dispatch && make install` |

## Approved Deviation Verification

| # | Deviation | Verdict | Evidence |
|---|-----------|---------|----------|
| D1 | Codex-only, no Claude adapter trait | VERIFIED | No adapter trait exists in the codebase. `engine.rs` builds codex args directly in `build_codex_args` (line 176). `CLAUDE.md` rule: "Codex-only: No adapter trait, no engine flag." Claude subagents use the Agent tool (per mission skill Phase 8 Step 2), which provides its own lifecycle. No loss of functionality. |

## Items Verified: 32/32

- 14 user stories: all verified
- 17 solution items: all verified
- 1 approved deviation: verified correctly implemented

No discrepancies found.
