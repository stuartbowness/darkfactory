Implemented and committed Task 3.

Commit: `ac416608dd4c1cad746c505d59870a0a93f88d31`

Changed [skills/adversarial/SKILL.md](/home/ironbank/.darkfactory/skills/adversarial/SKILL.md) to use synchronous `darkfactory-dispatch --json` inside one backgrounded Bash call, with `.status` parsing and `REVIEW_COMPLETE` / `REVIEW_FAILED` markers. Step 4 now collects via `BashOutput`.

Wrote the uncommitted handoff to [task-3.md](/home/ironbank/.darkfactory/openspec/changes/dispatch-sync-only/mission/handoffs/task-3.md).

Checks run:
- Verified no `--async`, `async_started`, dispatch ID lookup, or lifecycle subcommands remain.
- Verified `--json`, `--log`, `.status`, `BashOutput`, `REVIEW_COMPLETE`, and `REVIEW_FAILED` are present.
- Ran `git diff --check`.
- Confirmed the commit stages only `skills/adversarial/SKILL.md`.

Skipped runtime dispatch because this is a skill markdown change, not executable Rust behavior.