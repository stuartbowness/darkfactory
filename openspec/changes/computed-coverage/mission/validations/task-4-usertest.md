# User-Test Validation: Task 4

## Result: PASS

## Test Strategy

Task 4 adds the `coverage` CLI subcommand (`src/cli.rs`), a user-facing CLI. I
verified behavior by building and RUNNING the real binary
(`target/debug/darkfactory-render coverage --specs <dir> --tasks <md>`), then
checking exit codes (`echo $?`) and stdout, not by reading code. I forced a
clean rebuild after touching the source so the binary matches HEAD
(`ab4cd66`). I drove fixtures committed under `tests/fixtures/cli/`, one
pre-existing fixture (`bad_specs`) for the dangling-link check, and the
real-world tree at `tests/fixtures/migrated/`. No `.log` files were written
into the repo.

Exit-code contract (from `src/cli.rs`): `0` = no gaps, `3` = gaps found
(tolerant, report still printed), `2` = usage or parse/IO error. Stdout carries
the report on the `0`/`3` paths and is empty on the `2` error path (errors go to
stderr).

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `coverage` on `covered_specs` (lone scenario links to feature Task 2) | CLI run | PASS |
| 2 | Two runs on `gap_specs` produce identical stdout (verified by shasum) | CLI run x2 | PASS |
| 3 | `coverage` on `gap_specs` (one acceptance-less scenario) | CLI run | PASS |
| 4a | `--specs` omitted | CLI run | PASS |
| 4b | `--tasks` omitted | CLI run | PASS |
| 4c | `--specs` points at a missing dir | CLI run | PASS |
| 4d | `--tasks` points at a missing file | CLI run | PASS |
| 5 | Acceptance entry links to task 99 (not in tasks file) | CLI run | PASS |
| 6a | Acceptance-carrying scenario with no Spec Reference | CLI run | PASS |
| 6b | Unrecognised verification method (`[telepathy]`) | CLI run | PASS |
| 6c | Malformed task link (`[task:two]`) | CLI run | PASS |
| 6d | Duplicate Spec Reference within a capability | CLI run | PASS |
| 6e | Duplicate task number | CLI run | PASS |
| 7 | `coverage` on `tests/fixtures/migrated/` | CLI run + shasum | PASS |
| 8 | Full committed suite `cargo test` (107 tests, 9 suites) | cargo test | PASS |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | `coverage --specs <dir> --tasks <md>` exits `0` with a no-gaps report when all scenarios link to a valid feature task | PASS | `covered_specs` + `covered_tasks.md`: exit `0`; stdout `# Coverage Gap Report`, `1 scenarios, 1 covered, 0 gaps`, `None.` |
| 2 | Running it twice on the same inputs gives byte-identical stdout | PASS | `gap_specs` run twice: both exit `3`; `shasum` of stdout identical (`ae823728...`). Migrated fixture also identical across runs (`c9002201...`) |
| 3 | Exits `3` and writes the Gap report to stdout when gaps exist | PASS | `gap_specs`: exit `3`; stdout has `## Gaps`, `### widget/2 — Widget remembers the caller`, `- Gap kind: no acceptance`; no `None.` body |
| 4 | Exits `2` when `--specs` or `--tasks` is omitted, and when a path is unreadable/missing | PASS | omit `--specs`: exit `2`, stderr `missing required flag \`--specs\``. omit `--tasks`: exit `2`. missing specs dir: exit `2`, stderr `cannot read specs dir ...`, stdout 0 bytes. missing tasks file: exit `2`, stderr `cannot read ...`, stdout 0 bytes |
| 5 | A dangling acceptance link is a gap → exit `3`, not `2` | PASS | `bad_specs` (acceptance `[task:99]`, task 99 absent): exit `3`; stdout `- Gap kind: dangling task link`. Tolerant analyze path confirmed (not the fail-loud `2`) |
| 6 | Exits `2` on each malformed input (no Spec Reference, unrecognised method, malformed task link, duplicate Spec Reference, duplicate task number) | PASS | all five exit `2` with empty stdout: no-spec-ref `acceptance entry is missing a spec reference`; method `unknown verification method: telepathy`; malformed link exit `2` (stderr misattributes it to method, see Notes); dup spec-ref `duplicate spec reference: widget/1`; dup task `duplicate task number 2: Greet command again` |
| 7 | `coverage` on `tests/fixtures/migrated/` completes with a small stable gap count (3), not an error | PASS | exit `3` (gaps-found, not the `2` error code); stdout `11 scenarios, 8 covered, 3 gaps`; all 3 gaps under `### domain-type-boundary`; deterministic across two runs |

## Failed Checks (if any)

None.

## Notes (non-blocking)

Check 6c (malformed task link `[task:two]`): the binary exits `2` with empty
stdout, satisfying the contract, but the stderr message is
`unknown verification method: [task:two] [execution] the task link is not a
number`. The acceptance line fails to match the entry grammar and the whole tail
is then misparsed as a "method", so the message blames the method rather than
the non-numeric task link. Behavior (exit code, no stdout) is correct; only the
diagnostic wording is misleading. The committed test
`a_malformed_task_link_is_a_parse_error` asserts only exit `2` and empty stdout,
so it does not catch this. Worth a follow-up message fix, not a Task 4 failure.

The `bad_specs` fixture used for check 5 is pre-existing (not added by this
commit); the commit's own dangling-link coverage rides on it via
`tests/coverage.rs`. That is fine for validating user-facing behavior.

## Recommendations (if FAIL)

N/A — all checks pass.
