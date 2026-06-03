# Adversarial Review A

Baseline: `cargo test` = 76 passed (8 suites); `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt --check` clean.

## Task 1 -- [task-1] crate skeleton, domain types, RenderError, Makefile
### Surface summary
- `Cargo.toml`: package `darkfactory-render` v1.0.0, edition 2021, dep `thiserror = "2"`, release profile (panic=abort).
- `Makefile`: targets build/release/install/test/lint/fmt/check/smoke. `check: fmt lint test`. lint = `cargo clippy --all-targets -- -D warnings`.
- `src/lib.rs`: `pub mod core { pub mod types; }`.
- `src/main.rs`: stub binary printing "CLI not yet implemented".
- `src/core/types.rs`: `VerificationMethod` (Execution/Output/Inspection + `parse`), `ScenarioStatus` (5 variants + `parse`/`parse_optional`/`is_deviated`), `SpecReference{capability,number}`, `AcceptanceEntry{statement,method,task_link}`, `RequirementScenario{spec_ref,status,acceptance}`, `ClosingTaskFragment{kinds}`, `ClosingTaskKind{name,checks}`, `Task{number,name,closing_kind}`, `TaskList{tasks}`, `RenderedContract{body,warnings}` + `to_markdown`/`warnings`, `RenderError` (10 variants).
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 2 -- [task-2] spec and acceptance parsing
### Surface summary
- `src/core/parse.rs`: `parse_specs(capability, spec_markdown) -> Result<Vec<RequirementScenario>, RenderError>`; helpers `scenario_blocks`, `parse_scenario`, `parse_acceptance`, `parse_acceptance_entry`, `bracketed`, `parse_task_link`, `comment_value`, `parse_spec_ref_number`, `scenario_identifier`. (parse_tasks/parse_fragment/validate_closing_kinds added in task-3.)
- `src/lib.rs`: adds `pub mod parse` under `core`.
- `tests/openspec_validate.rs`: 2 integration tests — `openspec validate` on an acceptance-carrying fixture exits 0 (fail-loud if openspec missing); fixture round-trips through `parse_specs`.
- `tests/fixtures/acceptance_spec.md`: widget spec with spec-ref 1, status proposed, 2 acceptance entries.
- Fix commit 8c103c6: `EmptyStatement` gained a `String` identifier field; openspec test changed from conditional skip to fail-loud `assert!`.
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | MEDIUM | src/core/parse.rs:307-309 | `parse_acceptance` breaks on the first blank line. An acceptance block with a blank line between entries silently DROPS every entry after the blank (confirmed by probe: 2 authored entries -> 1 parsed, no error). This violates the design's fail-loud guarantee (silent data loss): a dropped check vanishes from the rendered contract with no warning. Current migrated fixtures keep entries contiguous so it is latent, but any future blank-separated authoring loses checks silently. |
| 2 | LOW | src/core/parse.rs:290-294 | A non-numeric `spec-ref` value (e.g. `<!-- spec-ref: abc -->`) is collapsed into `MissingSpecRef`, not a distinct malformed-ref error. Documented as deliberate (no Stage-1 variant), but the message "missing a spec reference" misdescribes a present-but-malformed ref, which could mislead an author. |

## Task 3 -- [task-3] task-list + closing-task fragment parsing
### Surface summary
- `src/core/parse.rs`: adds `parse_tasks(md) -> Result<TaskList,_>`, `parse_fragment(md) -> Result<ClosingTaskFragment,_>`, `validate_closing_kinds(tasks, fragment) -> Result<(),_>`; helpers `closing_field`, `fragment_check`, `validate_check_method`, `task_heading`, `strip_checkbox`.
- `templates/closing-task-checks.md`: declares 5 kinds smoke-test/cleanup/module-shape/claude-arch/documentation with `- [<method>] <statement>` checks.
- `tests/closing_task_fragment.rs`: pins fragment to exactly the 5 kinds; validates real tasks_sample against fragment.
- `tests/fixtures/tasks_sample.md`: 11 tasks (1-6 feature, 7-11 closing).
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | src/core/parse.rs:132-137 | A fragment check bullet appearing before the first `### <kind>` heading is silently dropped (`kinds.last_mut()` is `None`). The committed fragment has none, so latent only. Method validation still fires first, so an unknown method before a heading does fail loud. |

## Task 4 -- [task-4] contract render
### Surface summary
- `src/core/render.rs`: `build_model(scenarios, tasks, fragment) -> Result<ContractModel,_>`, `render(&ContractModel) -> Result<RenderedContract,_>`; private `ContractModel{sections}`, `TaskSection`, `CheckLine`, `PlacedEntry`; helpers `placed_entries`, `task_by_number`, `feature_section`, `closing_section`, `specs_comment`, `spec_ref_key`, `method_token`, `fragment_check_method`, `inspection_warning`. Title const `# Contract`. Cap = strictly > 20% via integer math `inspection*100 > total*20`.
- `src/core/types.rs`: `RenderedContract` gains `warnings: Vec<String>` field + `to_markdown()`/`warnings()`.
- `src/lib.rs`: adds `pub mod render`.
- `tests/contract_render.rs`: pipeline test (parse->build->render shape) + DuplicateSpecReference surfaces through full pipeline.
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | src/core/render.rs:288-290 | Cross-capability ordering uses lexicographic capability-name sort. The real spec has two capabilities (`acceptance-field`, `contract-render`). No unit test exercises a single task receiving checks from two real capabilities; if any task mixed them, the rendered order would be alphabetical, which must match the hand-authored contract. The task-6 parity test is the only guard (verified separately). |
| 2 | LOW | src/core/render.rs:99,204 | `placed_entries(scenarios)` is rebuilt for the global link-validation loop and again inside every `feature_section`. Pure inefficiency on small inputs, not a correctness bug. |

## Task 5 -- [task-5] shadow compare + CLI
### Surface summary
- `src/core/shadow.rs`: `compare(rendered, existing) -> Diff`; `Diff{Identical, Different(String)}`; `NEWLINE_ONLY_MESSAGE` const; in-house LCS `line_diff`. Fix d5e5a4f adds explicit newline-only/CRLF handling so a byte-diff with identical line content never yields an empty `Different("")`.
- `src/cli.rs`: `run(args, out, err) -> i32`; subcommands `render`/`shadow`; `RenderArgs`/`ShadowArgs`/`Flags` stdlib arg parsing; `render_pipeline`, `read_scenarios` (sorts capability dirs), `read_to_string`, `emit_warnings`. Exit codes: `EXIT_USAGE=2`, `EXIT_RENDER_ERROR=2`, `EXIT_DIFFERENT=1`.
- `src/main.rs`: thin wrapper -> `cli::run`, exits with returned code.
- `src/lib.rs`: adds `pub mod shadow`, `pub mod cli`.
- `tests/cli.rs`: 10 e2e tests driving the real binary (render shape/byte-match, multi-cap ordering, render-error, inspection-cap warning, shadow identical/different/error, and a read-only test snapshotting bytes+mtime of the whole tree). Fixtures under `tests/fixtures/cli/`.
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | src/cli.rs:27,32 | `EXIT_USAGE` and `EXIT_RENDER_ERROR` are both 2, so a usage error and a render/IO error are indistinguishable by exit code. The contract only requires non-zero, and `EXIT_DIFFERENT=1` is correctly distinct, so this passes — but the two error classes can't be scripted apart. |

## Task 6 -- [task-6] stable-ref migration + archived-fixture test
### Surface summary
- `tests/migrated_contract.rs`: 3 integration tests — (1) renders migrated fixture equivalent to archived contract (feature tasks 1-3 verbatim + Specs comments; closing tasks 4-8 structural only); (2) inserting a scenario leaves existing spec-refs fixed; (3) migrated tokens resolve to the same scenarios as positional refs.
- `tests/fixtures/migrated/specs/{brief-domain-types,domain-type-boundary,type-name-audit,readme-updates}/spec.md` + `tasks.md`: the archived change migrated to v2 grammar (spec-ref tokens + acceptance). readme-updates is the authored D6 scenario for the archived Task-3 README checks that had no spec source.
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | MEDIUM | tools/darkfactory-render/tests/migrated_contract.rs:173-204 | The parity test proves equivalence only for the 3 FEATURE tasks. The 5 CLOSING tasks (4-8) render the GENERIC fragment checks, which materially differ from the archived contract's mission-specific closing checks (e.g. archived Task 4 smoke = "All modified markdown files parse..." vs rendered "an end-to-end test runs the project's primary user flow..."). So the contract Task-6 claim "equivalent... same checks under the same tasks" holds for feature tasks only; for 5/8 tasks the rendered checks are NOT the hand-authored ones. This is documented and defended in design D6/Test Strategy (closing checks standardized via fragment), so it is a deliberate weakening, not a defect — but a validator running the rendered contract would not reproduce the archived closing-task validator outcomes. Worth a human-gate eye before cutover. |

## Task 7 -- [task-7] end-to-end smoke test
### Surface summary
- `tests/smoketest.sh`: POSIX sh. Builds binary (`cargo build`, fail-loud exit 2), runs `render` and asserts format markers, runs `shadow` vs identical (exit 0, "no differences", checksum byte-unchanged), runs `shadow` vs different (exit 1, non-empty diff). 10/10 checks pass when run from crate root. Uses `cksum`/`cut`/`grep -qF` (cross-platform).
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 8 -- [task-8] cleanup
### Surface summary
- `src/core/render.rs`: converts two silent fallbacks to fail-loud — `closing_section` `.unwrap_or_default()` -> `.expect()` (kind guaranteed by `validate_closing_kinds`); `fragment_check_method` `.unwrap_or(Execution)` -> `.expect()` (method tag validated by `parse_fragment`).
- Verified: clippy `--all-targets -D warnings` clean; no `todo!()`/`unimplemented!()`/`#[allow(dead_code)]`; no `#[allow]` of any kind; remaining `unwrap_or` uses are legitimate (None=non-deviated; empty token still hits the fail-loud `expect`).
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 10 -- [task-10] document darkfactory-render in ARCH.md
### Surface summary
- `ARCH.md`: adds `## Contract renderer (darkfactory-render)` section (purpose, module map, two authored sources, shadow/cutover). Adds darkfactory-render to the tools list with v1.0.0 pin.
- Verified: root `CLAUDE.md` unchanged in base..HEAD and in the working tree (contract Task 10 check 2 satisfied).
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 11 -- [task-11] document darkfactory-render
### Surface summary
- `tools/darkfactory-render/README.md`: documents `render`/`shadow` usage, exit codes, acceptance authoring syntax, closing-task fragment, Cutover gate criteria, Rollback procedure. All 5 pasted command examples verified verbatim against the built binary (render, render-error, shadow-identical, shadow-different, no-subcommand) — outputs and exit codes match exactly.
- `README.md` (repo): adds `### darkfactory-render` section + link, and notes it in the Rust prerequisite.
### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Cross-task
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tools/darkfactory-render/src/core/render.rs:128 | The renderer hard-codes the title `# Contract`. The archived fixture uses `# Contract` (so the parity test matches), but THIS change's own hand-authored `contract.md` uses `# Contract: contract-renderer`. So `shadow` against any real mission contract that puts the change name in the title will always show a title-line diff. Diagnostic-only (shadow never blocks; cutover is gated on validator-outcome parity, not a clean diff), but operators should expect the title to be a permanent diff line until cutover normalizes it. |
| 2 | LOW | openspec/changes/contract-renderer/mission/validations/module-shape-audit.md | The Task-9 deliverable (and the whole `mission/validations/`, `briefs/`, `handoffs/`, `issues/` set) is UNTRACKED in git (not gitignored, just uncommitted). The contract Task-9 `[output]` check passes against the working tree, but the audit that justifies the design.md "Reconciled Task 9" edits is not yet in any commit, so it would be absent from a clean checkout / the PR diff unless committed at mission close. Normal mission-in-progress state, not a code defect. |
| 3 | INFO | (chain) types.rs Task::closing_kind -> parse::validate_closing_kinds -> render::closing_section | Verified the fail-loud chain is sound: `build_model` calls `validate_closing_kinds` BEFORE any `closing_section`, so `closing_section`'s `.expect()` (task-8) can only fire on a genuine invariant violation, never a user-input path. No finding. |
| 4 | INFO | render::spec_ref_key (task-4) consumed by migrated parity (task-6) | Task 2 of the migrated fixture mixes two capabilities (`domain-type-boundary`, `type-name-audit`); the renderer's alphabetical capability sort yields `domain-type-boundary/1, type-name-audit/1, type-name-audit/2`, which matches the archived contract's hand-authored order. Parity holds, but it depends on the archived author having listed capabilities alphabetically — coincidental alignment, not an enforced invariant. No finding. |

## Summary
Findings: 0 HIGH, 2 MEDIUM, 7 LOW, 2 INFO. The mission is in good shape and I would recommend shipping as-is, with two items flagged for the human gate. The codebase is clean and faithful to its design: `make check` exits 0 (fmt + clippy `--all-targets -D warnings` + 76 unit/integration tests), the smoke test passes 10/10, and every contract check I probed holds up under execution rather than just inspection — all five README command examples reproduce verbatim against the built binary, the openspec-validate compatibility test runs openspec 1.3.1 for real, and the task-8 cleanup genuinely converted two silent fallbacks to fail-loud `expect`s guarded by upstream validation. The biggest risk is conceptual, not a bug: the task-6 parity test proves equivalence to the hand-authored contract only for the 3 FEATURE tasks; the 5 CLOSING tasks render generic fragment checks that materially differ from the archived mission-specific closing checks (MEDIUM-1, documented and defended in design D6 but worth a human eye before cutover). The second flag is a latent fail-loud gap: `parse_acceptance` silently drops every acceptance entry after a blank line within a block (MEDIUM-2), which contradicts the stated fail-loud guarantee — currently harmless because no committed fixture uses blank-separated entries, but it should break loud rather than lose checks. The LOW items are mostly hardening notes (collapsed error variants, an aliased exit code, a hard-coded title that will perma-diff in shadow, and an untracked Task-9 artifact). Nothing here blocks the read-only Stage-1 shadow goal.
