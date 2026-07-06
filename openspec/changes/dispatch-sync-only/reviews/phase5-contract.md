# Review Checkpoint — Phase 5: contract.md

Status: REVIEWED (clean)
Timestamp: 2026-06-30T01:00:00Z
Reviewer: Codex (gpt-5.5) via /artifact-review, 3 passes.

## Result
No CRITICAL/HIGH. 26 checks across 6 tasks. Testability: 17 [execution], 9 [output], 0 [inspection] (0% inspection — well under the 20%/task cap). Every dispatch-cli (1-7), synchronous-dispatch (1-9), and adversarial-sync-review (1-3) scenario is covered; traces verified by document order; exit codes match the code mapping.

## Findings resolved
Round 1 (4 HIGH): `--async --prompt x --json` could pass on old code (the `--async`/`--json` clap conflict masks removal) → now `--async --prompt x` requiring parser exit 2; `status`/`wait`/`result` exit-non-zero could pass on old runtime "not found" → now exit-2 "unrecognized subcommand"; spawn-error trace wrong (was /4) → /3; coverage missed synchronous-dispatch /2, /4, /8, /9 → added.
Round 2 (2 HIGH): "no files after dispatch" passes on old code (writes `status.json` mid-run, deletes on exit) → added an [output] source grep that the status/store/control internals are gone; missing-binary exit code was vague "non-zero" → pinned to exit 2 (`BinaryNotFound -> EXIT_CLI_ERROR`), exit 1 for other failures.
Round 3: clean.

The standout value: several checks would have passed against the *unchanged* code — exactly the false-negatives that let a non-conforming build through. Now every check provably fails against the old code.
