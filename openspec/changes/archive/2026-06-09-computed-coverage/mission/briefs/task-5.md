# Worker Brief: Task 5 - End-to-end smoke test (closing task)

## Your Task

Extend the crate's smoke test so `make smoke` exercises the new `coverage` subcommand end-to-end against a real on-disk fixture.

1. Read `tools/darkfactory-render/tests/smoketest.sh` to see how the existing `render`/`shadow` smoke checks are written (they build the release binary and run it against fixtures, asserting output and exit codes). Mirror that style.
2. Add coverage smoke checks that run the built binary as `darkfactory-render coverage --specs <dir> --tasks <md>` against a real fixture on disk and assert:
   - A **gaps** case: run against `tools/darkfactory-render/tests/fixtures/migrated/specs` + `tests/fixtures/migrated/tasks.md`. Assert exit code `3`, that stdout contains the title `# Coverage Gap Report` and the summary line `11 scenarios, 8 covered, 3 gaps`.
   - A **no-gaps** case: run against `tests/fixtures/cli/covered_specs` + `tests/fixtures/cli/covered_tasks.md` (added in Task 4). Assert exit code `0` and that stdout contains `0 gaps` and `None.`.
3. Keep the existing render/shadow smoke checks intact. Update the check count in the script's summary if it prints one (e.g. "N/N").

### Done when
- `make smoke` passes (from `tools/darkfactory-render/`) and its run now includes the coverage checks above.
- `make check` still green.

## Project Context
Closing task: the mandatory end-to-end smoke test for the mission. It proves the whole coverage pipeline works against real files via the real binary, no mocks.

## What Previous Tasks Built
- Task 4 (commit ab4cd66): the `coverage` subcommand. `darkfactory-render coverage --specs <dir> --tasks <md>` exits `0` (no gaps) / `3` (gaps) / `2` (usage/parse error) and prints the Gap report to stdout. The migrated fixture (`tests/fixtures/migrated/`) reports `11 scenarios, 8 covered, 3 gaps` (exit 3). `tests/fixtures/cli/covered_specs` + `covered_tasks.md` is a no-gaps fixture (exit 0).

## Procedures
**Before writing any code**, invoke the `/tdd` skill (mandatory).
- Read `tests/smoketest.sh` first and mirror its existing structure exactly (same helper/assert pattern, same binary it uses).
- Work only inside `tools/darkfactory-render/`. Verify with `make smoke` AND `make check`. Do NOT leave `.log` files in the repo.
- Commit once, message: `[task-5] smoke test for coverage subcommand`. Stage only files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/computed-coverage/`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-5.md` (write, do not commit).

## Handoff Report Format
- Task: Task 5 - End-to-end smoke test
- Commit SHA: <full SHA>
- Approach: <what you added>
- Commands run: <incl. make smoke + make check exit codes and the smoke pass count>
- Concerns: <any>
