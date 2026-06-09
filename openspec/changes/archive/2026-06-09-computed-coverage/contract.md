# Contract

## Task 1: Scenario model and parser retention
<!-- Specs: computed-coverage/1, computed-coverage/20, computed-coverage/24, computed-coverage/25 -->
- [ ] [execution] `cargo test` passes, including every pre-existing `render` and `shadow` test.
- [ ] [output] No existing `render`/`shadow` byte-output assertion or golden fixture is weakened or changed. Mechanical updates to test helpers/accessors for the new struct shape (`spec_ref` now `Option`, new `title`/`behavior` fields) are allowed; every render/shadow output assertion remains unchanged.
- [ ] [execution] A test demonstrates `parse_specs` retains a scenario that has no `**Acceptance:**` block and exposes that scenario's title and its verbatim behavior bullets.
- [ ] [output] `RequirementScenario.spec_ref` is `Option<SpecReference>`, and parsing a scenario that declares acceptance but no Spec Reference still produces a `MissingSpecRef` error.

## Task 2: Coverage analysis
<!-- Specs: computed-coverage/1, computed-coverage/2, computed-coverage/3, computed-coverage/4, computed-coverage/5, computed-coverage/6, computed-coverage/7, computed-coverage/8, computed-coverage/9, computed-coverage/11, computed-coverage/22 -->
- [ ] [execution] `analyze` reports a scenario with no acceptance as a gap whose only kind is "no acceptance".
- [ ] [execution] `analyze` reports an acceptance-less scenario that also has no Spec Reference as a gap, identified by its capability and scenario title.
- [ ] [execution] `analyze` reports a scenario whose only acceptance link points to a non-existent task as a gap (dangling task link), and one whose only link points to a closing task as a gap (closing-task link).
- [ ] [execution] `analyze` does NOT report a scenario that has at least one acceptance entry linking to a valid feature task, even when another of its entries has an unresolved link.
- [ ] [execution] A gap whose entries are invalid for more than one reason lists each distinct gap kind once, in the fixed order no-acceptance, dangling task link, closing-task link.
- [ ] [execution] Gaps are ordered by capability name ascending, then by document position within the capability.
- [ ] [output] A task list with two tasks sharing a number is rejected by the coverage duplicate-task check (`DuplicateTaskNumber`), while `parse_tasks` itself still accepts it.

## Task 3: Gap report rendering
<!-- Specs: computed-coverage/12, computed-coverage/13, computed-coverage/14, computed-coverage/15 -->
- [ ] [output] For the worked-example input in the spec, `render_report` produces byte-for-byte the canonical layout shown in that scenario (title, summary line, per-gap heading/Gap kind/Behavior, two-space-indented verbatim bullets).
- [ ] [output] When there are no gaps, the `## Gaps` section contains exactly the line `None.` and the summary line ends with `0 gaps`.
- [ ] [output] The summary line reads exactly `<T> scenarios, <C> covered, <G> gaps` for the report's counts.

## Task 4: Coverage CLI subcommand
<!-- Specs: computed-coverage/10, computed-coverage/16, computed-coverage/17, computed-coverage/18, computed-coverage/19, computed-coverage/20, computed-coverage/21, computed-coverage/22, computed-coverage/23 -->
- [ ] [execution] `darkfactory-render coverage --specs <dir> --tasks <md>` exits `0` and writes a no-gaps report when every scenario links to a valid feature task.
- [ ] [execution] Running `coverage` twice on the same `--specs` and `--tasks` produces byte-identical stdout (full parse -> analyze -> render path is deterministic).
- [ ] [execution] The same command exits `3` and writes the Gap report to stdout when one or more gaps exist.
- [ ] [execution] The command exits `2` when `--specs` or `--tasks` is omitted, and when an `--specs`/`--tasks` path is unreadable or missing.
- [ ] [execution] A dangling acceptance link yields a gap and exit `3`, not exit `2`.
- [ ] [execution] The command exits `2` on each malformed input: an acceptance-carrying scenario with no Spec Reference, an unrecognised verification method, a malformed task link, a duplicate Spec Reference within a capability, and a duplicate task number.
- [ ] [output] Running `coverage` against Stage 1's migrated fixture (`tools/darkfactory-render/tests/fixtures/migrated/`, which is in the acceptance-field format) completes and reports a small, stable gap count — the acceptance-less scenarios in that fixture (e.g. `domain-type-boundary`'s) — rather than erroring.

## Task 5: End-to-end smoke test
- [ ] [execution] `make smoke` passes and its script invokes `darkfactory-render coverage` against a real on-disk fixture, asserting the produced report and exit code.

## Task 6: Cleanup
- [ ] [execution] `make check` (cargo fmt --check, clippy with `-D warnings`, and the full test suite) passes with no dead-code or unused-dependency warnings.

## Task 7: Module shape re-check
- [ ] [output] `mission/validations/module-shape-audit.md` exists and records, per module, whether drift was found and what was done about it.
- [ ] [execution] The crate compiles and `rg` confirms the committed code defines `CoverageGap`, `GapKind`, `CoverageReport`, `RenderError::DuplicateTaskNumber`, and the `coverage` subcommand by those exact names (the build proves the types exist; `rg` proves the names).

## Task 8: Documentation
- [ ] [output] `tools/darkfactory-render/README.md` documents the `coverage` subcommand: the `--specs`/`--tasks` flags, the `0`/`3`/`2` exit codes, and an example whose pasted output matches the tool's real output.
- [ ] [output] The repo `README.md` mentions the `coverage` subcommand / Stage 2 computed coverage.
- [ ] [output] `ARCH.md` describes the new `core::coverage` module and the `coverage` subcommand within the darkfactory-render architecture.
