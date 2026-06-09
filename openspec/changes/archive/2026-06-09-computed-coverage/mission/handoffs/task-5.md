# Handoff: Task 5 - End-to-end smoke test (closing task)

- **Task:** Task 5 - End-to-end smoke test for the `coverage` subcommand
- **Commit SHA:** f778f97ce7b476edc93486dc131b37dccc41c44a
- **Branch:** mission/computed-coverage (parent: ab4cd66, the Task 4 commit)

## Approach

Extended `tools/darkfactory-render/tests/smoketest.sh` with two new test blocks that run the real built debug binary against on-disk fixtures, mirroring the existing `render`/`shadow` `check`/`contains` helper pattern exactly. No mocks.

- Added four fixture-path variables near the top: `GAP_SPECS`, `GAP_TASKS`, `COVERED_SPECS`, `COVERED_TASKS`.
- **Test 4 (gaps path):** runs `coverage --specs tests/fixtures/migrated/specs --tasks tests/fixtures/migrated/tasks.md`. Asserts exit code `3`, stdout contains the title `# Coverage Gap Report`, and the summary line `11 scenarios, 8 covered, 3 gaps`.
- **Test 5 (no-gaps path):** runs `coverage --specs tests/fixtures/cli/covered_specs --tasks tests/fixtures/cli/covered_tasks.md`. Asserts exit code `0`, stdout contains `0 gaps` and `None.`.
- Updated the header doc comment to list the two new checks.
- The summary line (`=== Results: $PASS/$TOTAL passed ===`) is computed dynamically, so no hardcoded count needed updating.

Existing render/shadow checks left intact. Per-test stdout/stderr captured into the script's `$TMPDIR` (cleaned by the `trap cleanup EXIT`), so no `.log` or temp files are left in the repo.

## Commands run

- `make smoke` (from `tools/darkfactory-render/`): exit `0`, **16/16 passed** (10 pre-existing + 6 new coverage checks). All PASS.
- `make check` (fmt + clippy + test): exit `0`, all green (fmt clean, no clippy warnings, all unit/integration tests pass).
- Verified `git ls-files --others` shows no stray `.log` files.

## Staging / scope

- Staged and committed only `tools/darkfactory-render/tests/smoketest.sh` (1 file, +34 lines). No `git add -A`/`.`.
- Pre-existing unstaged change `UBIQUITOUS_LANGUAGE.md` (from earlier mission/domain-model work) was deliberately left untouched and unstaged.
- `openspec/changes/computed-coverage/` left untracked/unstaged as required.

## Concerns

- The smoke script builds and runs the **debug** binary (`./target/debug/darkfactory-render` via `cargo build`), matching the existing script's convention. The brief's prose mentioned a "release binary," but I mirrored the script's actual existing behavior as instructed rather than changing the build target. No functional difference for these assertions.
- None blocking.
