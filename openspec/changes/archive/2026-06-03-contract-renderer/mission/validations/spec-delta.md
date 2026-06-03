# Spec Delta Check

## Result: CLEAN

All 26 COVERED requirements from the coverage audit are present in the built
codebase at HEAD `06d9c2e575b54e2f76150c60b71103b0055327ac`. The mission diff
(`main...HEAD`) touches only `tools/darkfactory-render/*` plus docs (`ARCH.md`,
`README.md`, `UBIQUITOUS_LANGUAGE.md`, `docs/darkfactory-2.md`) and the
`openspec/changes/contract-renderer/` change folder. No `skills/` file changed,
so the live mission Phase 5 and worker brief are untouched (structural rows
#24/#25 hold). The full `cargo test` suite passes (11 test binaries; the real
`openspec_validate` and `migrated_contract` equivalence tests ran and passed).

## Coverage Verification
| # | PRD Requirement | Audit Status | Verified | Notes |
|---|----------------|-------------|----------|-------|
| 1 | Author each spec-derived check once as scenario `acceptance` | COVERED | YES | `parse.rs` `parse_specs` reads the per-scenario `**Acceptance:**` block; `AcceptanceEntry` in `types.rs:101`. Tested `parses_a_single_acceptance_entry`. |
| 2 | Acceptance entry: method tag + task link + inherited Spec Reference + verbatim statement | COVERED | YES | `AcceptanceEntry { statement, method, task_link }` (`types.rs:101`); `VerificationMethod` enum (`types.rs:15`); ref inherited from scenario (`parse.rs:259`). |
| 3 | A scenario without acceptance renders nothing (not an error) | COVERED | YES | `parse_scenario` returns `Ok(None)` for an acceptance-less block (`parse.rs:246`). Tested `acceptance_less_scenario_parses_and_contributes_no_entries`. |
| 4 | Fail loud on schema inconsistencies (empty statement/list, missing/dup spec-ref, unknown method/status) | COVERED | YES | All variants in `RenderError` (`types.rs:192`); enforced in `parse.rs` (`EmptyStatement`, `EmptyAcceptanceList`, `MissingSpecRef`, `DuplicateSpecReference`, `UnknownVerificationMethod`, `UnknownStatus`). 9 fail-loud unit tests. |
| 5 | Stable Spec Reference: insertion does not renumber; duplicates fail | COVERED | YES | Ref is the explicit `<!-- spec-ref: N -->` token, never positional (`parse.rs:234`); dup -> `DuplicateSpecReference` (`parse.rs:61`). Tested `inserting_a_scenario_does_not_change_existing_spec_references` (`migrated_contract.rs:208`). |
| 6 | Legacy position-numbered references are migratable | COVERED | YES | `migrated_tokens_resolve_to_the_same_scenarios_as_positional_refs` (`migrated_contract.rs:284`) proves token N == archived positional ref N over the real archived change fixture. |
| 7 | Scenario `status`; `deviated` read for the marker; unknown fails loud | COVERED | YES | `ScenarioStatus` enum + `parse_optional` (omitted = non-deviated) + `is_deviated` (`types.rs:44`); `UnknownStatus` on bad token. Marker rendered in `specs_comment` (`render.rs:278`). |
| 8 | Deterministic renderer; byte-identical output; task-list + Spec Reference + source ordering | COVERED | YES | `build_model` fully orders (sort by spec-ref then source position, `render.rs:216`); `render` is a pure join (`render.rs:127`). Tested `repeat_render_is_byte_identical`, `feature_checks_order_by_spec_reference_then_source_order`, `tasks_render_in_task_list_order_not_link_order`. |
| 9 | Emit contract.md in the existing format | COVERED | YES | `render` emits `# Contract`, `## Task N: name`, `<!-- Specs: ... -->`, `- [ ] [method] statement` (`render.rs:127-139`). Smoke + render tests assert the shape. |
| 10 | Carry the check statement verbatim | COVERED | YES | Statement copied without rewording (`render.rs:227`). Tested `statements_render_verbatim_without_rewording` (backticks/brackets/percent survive). |
| 11 | One deduped/ordered Spec Reference comment per task; `(DEVIATED)` marking | COVERED | YES | `specs_comment` dedupes by ordered ref and appends ` (DEVIATED)` for deviated scenarios (`render.rs:269`). Tested `a_deviated_scenarios_ref_is_marked_and_deduped_once`. |
| 12 | Operational/closing-task checks from the standing fragment; no Spec Reference | COVERED | YES | `templates/closing-task-checks.md` (5 kinds) parsed by `parse_fragment`; `closing_section` emits fragment checks with `specs_comment: None` (`render.rs:242`). Tested `closing_task_fragment.rs` (committed fragment -> 5 kinds) and `a_closing_task_renders_fragment_checks_with_no_specs_comment`. |
| 13 | Render fail-loud (dangling link, acceptance→closing task, feature task zero checks, unknown closing kind) | COVERED | YES | `build_model` validates each: `DanglingTaskLink`, `AcceptanceLinksClosingTask`, `FeatureTaskNoChecks` (`render.rs:99-211`); `UnknownClosingKind` via `validate_closing_kinds` (`parse.rs:148`). 4 fail-loud render tests. |
| 14 | Warn when a task's `[inspection]` checks exceed 20% | COVERED | YES | `inspection_warning` uses integer `inspection*100 > total*20` (strictly greater) on `RenderedContract::warnings` (`render.rs:320`). Tested over-cap warns, at-cap (20%) does not, closing-task reads fragment tag. |
| 15 | Shadow mode renders + diffs an existing contract.md without modifying it | COVERED | YES | `shadow.rs` `compare` is pure LCS line diff; `cli.rs` `run_shadow` only reads `--existing` (`cli.rs:94-132`). Smoke test asserts `contract_identical.md` byte-unchanged via `cksum`. |
| 16 | Never auto-trigger cutover | COVERED | YES | Shadow is read-only, reports a diagnostic diff, writes nothing (`cli.rs:111`, `shadow.rs:1-12`). README "The renderer **never** does this on its own" (`README.md:226`). |
| 17 | Documented cutover gate + rollback procedure | COVERED | YES | Tool README "Cutover gate" (`README.md:222`) + "Rollback procedure" (`README.md:244`) with the four gate criteria and two rollback steps. |
| 18 | Parity judged by validator outcomes (equivalence), not text | COVERED (interpretation) | YES | Single-mission equivalence test `migrated_fixture_renders_equivalent_to_the_archived_contract` (`migrated_contract.rs:71`) checks "same checks under same tasks", not byte-identity; documented gate states "Parity is judged on validator outcomes, not on a clean diff" (`README.md:242`). Full cross-mission parity deferred to cutover per design. |
| 19 | `openspec validate` still passes on a spec carrying acceptance | COVERED | YES | `tests/openspec_validate.rs` is a real, non-skipped test: it `assert!`s openspec is on PATH (fails loud if missing) then runs `openspec validate`. Ran and passed in this verification. |
| 20 | Rust crate under `tools/`, built/installed/tested like darkfactory-dispatch | COVERED | YES | `tools/darkfactory-render/` crate (`Cargo.toml` v1.0.0, dep `thiserror`); `Makefile` build/release/install/test/lint/smoke, mirroring darkfactory-dispatch's layout. |
| 21 | Smoke test of the primary flow against the real binary | COVERED | YES | `tests/smoketest.sh` builds and runs the real `target/debug/darkfactory-render` (render + shadow identical/different), POSIX-portable, `make smoke`. |
| 22 | Documentation: tool README + repo README update | COVERED | YES | `tools/darkfactory-render/README.md` (10.3K, full usage + syntax + gate); repo `README.md:139` "darkfactory-render" section. |
| 23 | ARCH.md updated for the new tool | COVERED | YES | `ARCH.md:79` "Contract renderer (darkfactory-render)" section + external-dependencies entry (`ARCH.md:111`). |
| 24 | Worker never sees `acceptance` or the rendered contract | COVERED | YES | Structural: no `skills/` change on the branch; `worker-brief.md` unchanged (`git diff main...HEAD` empty for it). ARCH.md:94 "Workers still never see the contract; the worker brief projects `behavior` only." |
| 25 | v1 keeps shipping unchanged during shadow (Phase 5 untouched) | COVERED | YES | Structural: `git diff main...HEAD -- skills/mission/SKILL.md` is empty; the mission diff touches only `tools/darkfactory-render/*` + docs. Live Phase 5 hand-authoring is not wired to the renderer. |
| 26 | Fixes land on `acceptance` at source; never patch a generated file | COVERED | YES | The renderer makes the contract a pure projection of `acceptance` + fragment (`render.rs` `build_model`/`render`); README frames it as rendering "from a single source" (`README.md:6`) and the rollback restores hand-authoring without touching v1. Discipline enabled + documented, not a separate build artifact. |

## Discrepancies (if any)

None.

## APPROVED-DEVIATION Summary

None. The coverage audit lists no approved deviations (Substitutions or
Removals). Row #18 is classified as an interpretation, not a deviation: Stage 1
delivers the single-mission equivalence test plus the documented validator-
outcome gate, and full cross-mission parity is deferred to the later cutover
change per the design Migration Plan.
