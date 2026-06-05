# Handoff: Task 4 - Coverage CLI subcommand

- **Task:** Task 4 - Coverage CLI subcommand (`coverage --specs <dir> --tasks <md>`, exit 0/3/2)
- **Commit SHA:** `ab4cd6663e29c31e0621ea0318734beec7adbee7`

## Approach

Wired a `coverage` subcommand into the existing thin CLI (`src/cli.rs`), mirroring the
`render`/`shadow` structure exactly:

- Added `EXIT_GAPS = 3`, distinct from `EXIT_USAGE`/`EXIT_RENDER_ERROR` (`2`) and `shadow`'s
  `EXIT_DIFFERENT` (`1`).
- Added `CoverageArgs` (just `--specs` and `--tasks`, no `--fragment`/`--existing`), parsed
  with the same stdlib `Flags` parser the other subcommands use.
- `run_coverage` -> `coverage_pipeline` reuses the existing `read_scenarios` spec-dir reader
  (sorted capability dirs, each `<cap>/spec.md` parsed via `parse_specs(<cap>, content)`), so
  `coverage` and `render` agree on what they read. Pipeline:
  `read_scenarios -> parse_tasks -> check_duplicate_tasks -> analyze -> render_report`.
- Exit policy: `0` when `report.gaps` is empty, `3` when there are gaps (report still written
  to stdout), `2` for any usage error or `RenderError` (parse/IO).
- A dangling acceptance link stays a GAP (exit `3`), never a parse error: `analyze` is
  tolerant where `render`/`build_model` fail loud. A duplicate task number is a fail-loud
  input error mapped to exit `2` via `check_duplicate_tasks`.
- Extended the `USAGE` string with the `coverage` line. Did not touch `render`/`shadow`
  behavior, their exit codes, or `smoketest.sh` (Task 5 owns the coverage smoke case).

Added `tests/coverage.rs` (14 tests, driving the real built binary) and small fixtures under
`tests/fixtures/cli/` (one capability dir each). Coverage of the required cases:

- exit `0` (no gaps) + report on stdout; exit `3` (gaps) + gap on stdout.
- dangling acceptance link asserts exit `3` (gap), not `2`.
- byte-identical stdout across two runs (full-pipeline determinism).
- migrated fixture (`tests/fixtures/migrated/`) pinned at 3 gaps, all in
  `domain-type-boundary`.
- exit `2` for each of: missing `--specs`, missing `--tasks`, unreadable specs path, missing
  tasks path, unknown verification method, malformed task link, duplicate Spec Reference,
  acceptance-carrying scenario with no Spec Reference, duplicate task number.

## Commands run

- `cargo test --test coverage` -> 14 passed.
- `make check` (fmt + clippy `-D warnings` + full `cargo test`) -> exit `0`, green.
- `make smoke` -> 10/10 passed, exit `0` (still `render`/`shadow` only; Task 5 adds coverage).
- Direct binary run against the migrated fixture to capture the exact report (below).

## Concerns

- **Migrated-fixture gap count: 3** (`11 scenarios, 8 covered, 3 gaps`, exit `3`). All three
  are `NoAcceptance` gaps in the `domain-type-boundary` capability: its scenarios "Type
  Catalog definition includes glossary relationship", "Type naming chain in Relationships",
  and "Domain type term present" carry no `**Acceptance:**` block. Every other migrated
  scenario links to a feature task. The test `coverage_on_the_migrated_fixture_reports_a_
  stable_gap_count` pins both the count (3) and that all three are in `domain-type-boundary`.
- A non-numeric task link (`[task:two]`) surfaces as `UnknownVerificationMethod` (exit `2`),
  not a dedicated "malformed link" variant — this matches Stage 1's `parse_acceptance_entry`
  design (no malformed-entry variant exists). The test asserts exit `2` and empty stdout
  rather than a specific message.
- Out of scope and left untouched: a pre-existing `M UBIQUITOUS_LANGUAGE.md` at the repo root
  was already modified in the working tree before this task. It is outside
  `tools/darkfactory-render/` and was NOT staged or committed.
