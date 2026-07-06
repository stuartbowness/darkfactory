# Review Checkpoint — Phase 4.1: specs/dispatch-cli

Status: REVIEWED (clean after fixes)
Timestamp: 2026-06-18T19:10:00Z
Reviewer: Codex (gpt-5.5), 3 combined passes.

## Findings resolved (this spec)
- MEDIUM: retained-flag behavior under-covered → added a `--output` scenario asserting the `output`/`output_exists` result fields.
- MEDIUM (security): the no-files rule didn't cover the removed control channel → added a scenario that a planted `control.json` cannot abort a dispatch.
- MEDIUM/LOW: help-text ban was too literal → scoped to the flag/subcommand listing, not arbitrary prose.

4 requirements, 7 scenarios. No CRITICAL/HIGH open.
