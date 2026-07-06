# adversarial-sync-review

## ADDED Requirements

### Requirement: Adversarial reviewer dispatched synchronously
The `/adversarial` skill SHALL dispatch its Codex reviewer with a synchronous `--json` call run via Bash `run_in_background` and collected with `BashOutput`. It SHALL NOT use the `--async` flag or any `darkfactory-dispatch` lifecycle subcommand (`status`/`wait`/`result`).

#### Scenario: Codex reviewer uses the synchronous pattern
- **WHEN** the `/adversarial` skill dispatches its Codex reviewer
- **THEN** the skill's instructions use a synchronous `--json` dispatch via `run_in_background` + `BashOutput`, and contain no `--async` flag and no `darkfactory-dispatch` lifecycle subcommand (`status`/`wait`/`result`) or dispatch-id lookup

### Requirement: Review outcome reported correctly
The `/adversarial` skill SHALL report a reviewer that completed successfully as complete, reading the dispatch result from the captured background output rather than from a dispatch id that does not survive the background shell.

#### Scenario: A passing review is reported complete
- **WHEN** the Codex reviewer completes successfully
- **THEN** the skill reads the dispatch result's `status` (equal to `"completed"`) from the captured `BashOutput` and emits a `REVIEW_COMPLETE` marker, rather than reporting a false failure

#### Scenario: A failed or timed-out review is reported as a failure
- **WHEN** the Codex reviewer's dispatch result has `status` equal to `"failed"` or `"timed_out"`
- **THEN** the skill emits a `REVIEW_FAILED` marker rather than silently treating it as success
