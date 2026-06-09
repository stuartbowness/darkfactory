# Tasks: computed-coverage

All tasks use `Skill: /tdd` (Rust crate; no frontend). `browser_tools: false`. Layered decomposition: 4 feature tasks then 4 closing tasks.

## Feature Tasks

- [x] Task 1: Scenario model and parser retention
  Depends on: none
  Consumes: core::types (RequirementScenario, SpecReference, AcceptanceEntry), core::parse::parse_specs, core::render::build_model, core::shadow
  Produces: extended `RequirementScenario` (adds scenario title + behavior text; `spec_ref` becomes `Option<SpecReference>`), `parse_specs` retaining every scenario incl. acceptance-less ones and capturing title + behavior, render/shadow call-site adaptation that keeps their output byte-identical
  Layers: types, parse
  Skill: /tdd

  The foundational refactor everything else imports. Extend `RequirementScenario` to carry the scenario title and behavior text (the verbatim `- **WHEN**`/`- **THEN**`/`- **AND**` bullets before `**Acceptance:**`, excluding comments and blank lines), and change `spec_ref` to `Option<SpecReference>` (None only for an acceptance-less scenario with no `<!-- spec-ref: N -->`). Extend `parse_specs` to retain every scenario, including acceptance-less ones (currently dropped). Adapt `build_model`/`render` and `shadow` to the `Option` WITHOUT changing their output: they process only acceptance-carrying scenarios, which always carry `Some(spec_ref)`; an acceptance-carrying scenario with no spec-ref still fails `MissingSpecRef`. Done = all existing render/shadow tests stay green (byte-identical) AND a new test proves acceptance-less scenarios are retained with their title and behavior text.

- [x] Task 2: Coverage analysis
  Depends on: Task 1
  Consumes: extended RequirementScenario (Task 1), core::types Task/TaskList, core::parse::parse_tasks
  Produces: `CoverageGap`, `GapKind` (NoAcceptance / DanglingTaskLink / ClosingTaskLink), and `CoverageReport` (ordered gaps + total/covered counts) added to `core::types` (per design Type Catalog); the `core::coverage` module owning `analyze` (scenarios + tasks -> CoverageReport) plus the private classification/ordering logic; `RenderError::DuplicateTaskNumber`; a duplicate-task-number check helper for the coverage path
  Layers: types, coverage
  Skill: /tdd

  The tolerant analysis. `analyze` classifies each scenario: a Coverage gap when none of its acceptance entries links to a valid Feature task; the gap's `GapKind`s are the distinct reasons present (no acceptance, dangling link, closing-task link) in that fixed order; a scenario with at least one valid Feature-task link is NOT a gap (unresolved sibling links are ignored — they surface at render time). Gaps ordered by capability (ascending) then document position. `analyze` is infallible — it never returns an error. An acceptance-less scenario lacking a Spec Reference is identified by title. Add `RenderError::DuplicateTaskNumber` and a `check_duplicate_tasks` helper used by the CLI path (NOT inside `parse_tasks`, which stays permissive so render/shadow are unaffected). Unit tests: each gap kind, partial-link-not-a-gap, gap-kind set + fixed order, ordering across/within capabilities, duplicate-task detection.

- [x] Task 3: Gap report rendering
  Depends on: Task 2
  Consumes: CoverageReport, CoverageGap, GapKind (Task 2)
  Produces: `core::coverage::render_report` (CoverageReport -> String) implementing the canonical Gap report format
  Layers: coverage
  Skill: /tdd

  Project the `CoverageReport` into the canonical markdown defined by the spec's "Canonical Gap report format" requirement: a `# Coverage Gap Report` title; a `## Summary` section with the single line `<T> scenarios, <C> covered, <G> gaps`; a `## Gaps` section that is `None.` when there are no gaps, otherwise one `### <identifier> — <scenario title>` block per gap (identifier is `<capability>/<N>`, or `<capability>` when no Spec Reference), a `- Gap kind: <kinds>` line (distinct kinds comma-separated in fixed order), and a `- Behavior:` line followed by each behavior bullet indented two spaces and rendered verbatim. Deterministic and byte-identical. Tests: the exact worked example from the spec, and the no-gaps form.

- [x] Task 4: Coverage CLI subcommand
  Depends on: Task 1, Task 2, Task 3
  Consumes: parse_specs (Task 1), parse_tasks + analyze + check_duplicate_tasks (Task 2), render_report (Task 3)
  Produces: `coverage` subcommand + `CoverageArgs` (`--specs`, `--tasks`), the parse -> duplicate-check -> analyze -> render pipeline glue, exit codes 0/3/2, archived Stage 1 change fixture
  Layers: cli
  Skill: /tdd

  Make coverage runnable. The CLI reads `--specs` and `--tasks`, calls `parse_specs` + `parse_tasks` (any `RenderError` -> exit `2` with an actionable stderr message), runs `check_duplicate_tasks` (`DuplicateTaskNumber` -> exit `2`), then `analyze` and writes `render_report` to stdout; exit `3` when gaps exist, `0` when none. Integration tests in `tests/coverage.rs`: exit `0` (no gaps), exit `3` (gaps, report on stdout), and exit `2` for every malformed-input case the spec lists — missing `--specs`/`--tasks`, an unreadable or missing input path, an unrecognised verification method, a malformed task link, a duplicate Spec Reference within a capability, an acceptance-carrying scenario with no Spec Reference, and a duplicate task number. Add Stage 1's migrated fixture (`tools/darkfactory-render/tests/fixtures/migrated/`, in acceptance-field format) as a realistic integration fixture — mostly covered, with the known acceptance-less scenarios (e.g. `domain-type-boundary`'s) as gaps. (NOT the archived change's spec deltas, which have no acceptance blocks.)

## Closing Tasks

- [x] Task 5: End-to-end smoke test
  Depends on: Task 4
  Consumes: full coverage pipeline (Tasks 1-4)
  Produces: extended `tests/smoketest.sh` (`make smoke`) exercising `coverage` end-to-end against a real fixture
  Layers: tests
  Skill: /tdd

  Mandatory smoke test. Extend `tests/smoketest.sh` so `make smoke` runs `darkfactory-render coverage --specs <dir> --tasks <md>` against a real on-disk fixture and asserts the produced report and exit code. Real dependencies, no project-internal mocks.

- [x] Task 6: Cleanup
  Depends on: Task 5
  Consumes: full tree
  Produces: dead code / scaffolding / debris removed; fail-loud guards verified
  Layers: cleanup
  Skill: /tdd

  Mandatory cleanup. Remove unused code, leftover scaffolding, temporary artifacts, and any backwards-compatibility shims introduced during the build. No shims, re-exports, or deprecation wrappers (project rule). Confirm no silent fallbacks were added.

- [x] Task 7: Module shape re-check
  Depends on: Task 6
  Consumes: design.md, committed code
  Produces: `mission/validations/module-shape-audit.md`
  Layers: audit
  Skill: /tdd

  Mandatory shape re-check (task-1's `spec_ref` change spans modules, so worth doing even though there are 4 feature tasks). Re-read design.md's Module Inventory and Type Catalog and compare to what the code exposes: `core::coverage` public functions (`analyze`, `render_report`), the `CoverageGap`/`GapKind`/`CoverageReport` types, the extended `RequirementScenario`, `RenderError::DuplicateTaskNumber`, and the `coverage` subcommand. Verify every Type Catalog name exists in code with a matching name. Reconcile any drift (update design.md for deliberate change, or refactor). Write findings to `mission/validations/module-shape-audit.md`.

- [x] Task 8: Documentation
  Depends on: Task 7
  Consumes: the built coverage feature
  Produces: updated `tools/darkfactory-render/README.md`, repo `README.md`, and `ARCH.md`
  Layers: docs
  Skill: /tdd

  Mandatory documentation. Document the `coverage` subcommand in the tool README (usage, `--specs`/`--tasks`, exit codes `0`/`3`/`2`, an example with real pasted output), add a Stage 2 / coverage mention to the repo `README.md`, and update `ARCH.md` to describe the new `core::coverage` module and `coverage` subcommand in the darkfactory-render architecture (the project rule requires ARCH.md updates for architectural changes). Any commands shown must be executed and their real output pasted. CLAUDE.md already exists — do not modify it.
