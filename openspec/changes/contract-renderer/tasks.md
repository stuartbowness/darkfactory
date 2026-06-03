# Tasks: contract-renderer

All tasks use `Skill: /tdd` (Rust crate; no frontend). `browser_tools: false`.

## Feature Tasks

- [ ] Task 1: Crate skeleton, domain types, RenderError, Makefile
  Depends on: none
  Consumes: none
  Produces: `tools/darkfactory-render` crate (Cargo.toml pinned 1.0.0, lib + thin bin), `core::types` (AcceptanceEntry, VerificationMethod, SpecReference, ScenarioStatus, RequirementScenario, ClosingTaskFragment, TaskList, RenderedContract), `RenderError` enum, Makefile (build/release/install/test/lint/fmt/check)
  Layers: types, build
  Skill: /tdd

  Scaffold the crate following `tools/darkfactory-dispatch` (Cargo workspace conventions, lib + bin, `tests/`, Makefile whose `check` target runs `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` + `cargo test`). Define the Type Catalog types and the `thiserror` `RenderError` enum. Implement `VerificationMethod` and `ScenarioStatus` value parsing here (small enums), including the loud-error path for an unrecognized verification method and an unrecognized status, and "omitted status = non-deviated".

- [ ] Task 2: Spec and acceptance parsing
  Depends on: Task 1
  Consumes: core::types (Task 1)
  Produces: `core::parse::parse_specs` (specs/**/*.md -> RequirementScenario list), acceptance-block grammar parser, schema fail-loud errors (EmptyStatement, EmptyAcceptanceList, MissingSpecRef, DuplicateSpecReference), openspec-validate-compatibility test
  Layers: parse
  Skill: /tdd

  Parse the acceptance authoring syntax (design D3/D4): `<!-- spec-ref: N -->` and optional `<!-- status: ... -->` under each `#### Scenario:`, and a `**Acceptance:**` block of `- [task:N] [<method>] <statement>` entries. Entries inherit the scenario's Spec Reference. Fail loudly on empty/whitespace statement, present-but-empty acceptance list, missing spec-ref on a scenario with acceptance, and duplicate spec-ref within a capability. Add a test asserting `openspec validate` still passes on a spec file carrying an acceptance block.

- [ ] Task 3: Task-list and closing-task fragment parsing
  Depends on: Task 1
  Consumes: core::types (Task 1)
  Produces: `core::parse::parse_tasks` (tasks.md -> TaskList with task numbers, names, `Closing: <kind>`), `core::parse::parse_fragment` (templates/closing-task-checks.md -> ClosingTaskFragment), `templates/closing-task-checks.md`, UnknownClosingKind detection
  Layers: parse, templates
  Skill: /tdd

  Parse tasks.md for `Task N: <name>` and the optional `Closing: <kind>` topology field. Parse the closing-task fragment (kinds -> `[<method>] <statement>` checks). Author `templates/closing-task-checks.md` declaring the mandatory closing-task kinds (smoke-test, cleanup, module-shape, claude-arch, documentation) and their checks. A `Closing:` kind absent from the fragment fails loudly (UnknownClosingKind).

- [ ] Task 4: Contract render
  Depends on: Task 2, Task 3
  Consumes: core::types (Task 1), parse_specs (Task 2), parse_tasks + parse_fragment (Task 3)
  Produces: `core::build_model` (assemble the typed model from parsed specs/tasks/fragment), `core::render::render` (model -> RenderedContract), `RenderedContract::to_markdown`, `RenderedContract::warnings` (named warning surface carrying the inspection-cap warning), grouping by task, Spec Reference comment grammar (dedupe/order/DEVIATED), ordering (task-list, then Spec Reference, then source tie-break), closing-kind->task join, render fail-loud (DanglingTaskLink, AcceptanceLinksClosingTask, FeatureTaskNoChecks), inspection-cap warning, existing-format fidelity
  Layers: render
  Skill: /tdd

  The deep core. Compose `contract.md` from spec `acceptance` grouped by `[task:N]` plus the closing-task fragment joined to each `Closing: <kind>` task, in the existing contract format (`## Task N: <name>`, one deduped/ordered `<!-- Specs: ... -->` per task with `(DEVIATED)` marking, `- [ ] [method] statement` checks). Deterministic ordering and byte-identical output. Fail loud on a dangling task link, an acceptance entry linking to a closing task, and a feature task with zero checks. Emit a non-fatal warning when a task's `[inspection]` checks exceed 20%.

- [ ] Task 5: Shadow compare mode and CLI
  Depends on: Task 4
  Consumes: core::build_model + core::render + RenderedContract::warnings (Task 4), core::parse (Tasks 2, 3)
  Produces: `core::shadow::compare` (rendered vs existing -> diff), `cli` binary (`render` and `shadow` subcommands, file IO, exit codes, stderr warnings), read-only shadow that never auto-cutovers
  Layers: shadow, cli
  Skill: /tdd

  Make the tool runnable. The CLI reads files, calls `parse_*` then `core::build_model` then `core::render`; this file-reading assembly glue is covered by the integration (Task 6) and smoke (Task 7) tests. `render` writes/prints the rendered contract; `shadow` renders and line-diffs against an existing `contract.md` for inspection without modifying it or any pipeline state. Map RenderError to a non-zero exit with an actionable stderr message; emit `RenderedContract::warnings` (including the inspection-cap breach) to stderr with exit 0.

- [ ] Task 6: Stable-ref migration and archived-fixture integration test
  Depends on: Task 5
  Consumes: full pipeline (Tasks 1-5)
  Produces: legacy position-numbered reference migration (procedure + helper/test), migrated fixture under `tests/`, integration equivalence test
  Layers: migration, tests
  Skill: /tdd

  Migrate the archived fixture `openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/`: assign a stable Spec Reference (via the `spec-ref: N` token) to its scenarios, author acceptance for each spec-derived check, and author a scenario for any feature-task check that had no spec source (design D6). Prove migrated references resolve to the same scenarios as before. Add an integration test that renders the migrated fixture and asserts the result is equivalent to the hand-authored contract (same checks under the same tasks / validator-outcome parity), not necessarily byte-identical.

## Closing Tasks

- [ ] Task 7: Smoke test
  Depends on: Task 6
  Consumes: cli (Task 5), migrated fixture (Task 6)
  Produces: `tests/smoketest.sh`
  Layers: tests
  Skill: /tdd
  Closing: smoke-test

  End-to-end smoke test of the primary flow against the real built binary, no internal mocks: build `darkfactory-render`, run `render` over a fixture, assert it emits a contract in the expected format, and run `shadow` and assert it leaves the existing contract unchanged.

- [ ] Task 8: Cleanup
  Depends on: Task 7
  Consumes: none
  Produces: removed dead code, scaffolding, temporary artifacts, unused dependencies
  Layers: cleanup
  Skill: /tdd
  Closing: cleanup

  Remove leftover scaffolding, dead code, and unused dependencies before the shape re-check.

- [ ] Task 9: Module shape re-check
  Depends on: Task 8
  Consumes: design.md
  Produces: `mission/validations/module-shape-audit.md`
  Layers: audit
  Skill: /tdd
  Closing: module-shape

  Compare design.md's Module Inventory and Type Catalog to what the committed crate exposes; verify every domain type exists with a matching name. Record findings (no drift / design updated / refactored).

- [ ] Task 10: Update ARCH.md for the new tool
  Depends on: Task 9
  Consumes: design.md
  Produces: ARCH.md updated to describe `darkfactory-render`
  Layers: docs
  Skill: /tdd
  Closing: claude-arch

  The repo's CLAUDE.md requires updating ARCH.md after architectural changes (this overrides the mission's generic "don't modify ARCH.md" default). Add a section describing the new `darkfactory-render` tool and how it relates to the planning pipeline. Do not modify the existing root CLAUDE.md (no project rule requires it).

- [ ] Task 11: Documentation
  Depends on: Task 10
  Consumes: cli (Task 5)
  Produces: `tools/darkfactory-render/README.md`, cutover-gate + rollback documentation (shadow-comparison spec requirement), updated repo `README.md`
  Layers: docs
  Skill: /tdd
  Closing: documentation

  Write the tool README (usage of `render`/`shadow`, the acceptance authoring syntax, the closing-task fragment) and the cutover-gate + rollback procedure the shadow-comparison spec requires. Update the repo README.md to mention the new tool (CLAUDE.md rule). Any commands shown must be run against the built binary with real output pasted.
