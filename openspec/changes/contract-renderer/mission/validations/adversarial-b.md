# Adversarial Review B

Scope: `540e1953cff20bb58e9e759412dab91306dd9ac3..HEAD` (12 task commits). Reviewed against the full validation contract at `openspec/changes/contract-renderer/contract.md`. Baseline probes: `cargo test --all-targets` = 76 passed, `make check` (fmt + clippy `-D warnings` + test) = clean, `tests/smoketest.sh` = 10/10, all README example commands reproduced verbatim against the built binary, `openspec validate` test exercised against real `openspec` 1.3.1.

## Task 1 -- [task-1] crate skeleton, domain types, RenderError, Makefile

### Surface summary
- `src/core/types.rs`: `VerificationMethod` (+`parse`), `ScenarioStatus` (+`parse`/`parse_optional`/`is_deviated`), `SpecReference`, `AcceptanceEntry`, `RequirementScenario`, `ClosingTaskFragment`, `ClosingTaskKind`, `Task`, `TaskList`, `RenderedContract` (+`to_markdown`/`warnings`), `RenderError` (10 variants).
- `src/lib.rs`: `pub mod core { pub mod types; }`. `src/main.rs`: stub. `Cargo.toml`: `version = "1.0.0"`, dep `thiserror`. Makefile: `check = fmt lint test`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Cargo.toml is v1.0.0 (contract T1.4). `make check` runs fmt-check, clippy `-D warnings`, test (T1.1). Tag/status parsing + error variants tested (T1.2, T1.3). |

## Task 2 -- [task-2] spec and acceptance parsing (+ fix commit 8c103c6)

### Surface summary
- `src/core/parse.rs`: `parse_specs(capability, md) -> Vec<RequirementScenario>`; private scenario/acceptance grammar (`scenario_blocks`, `parse_scenario`, `parse_acceptance`, `parse_acceptance_entry`, `bracketed`, `comment_value`, `parse_spec_ref_number`).
- `tests/openspec_validate.rs`: runs real `openspec validate` (fail-loud if absent) + parse round-trip. `tests/fixtures/acceptance_spec.md`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | MEDIUM | src/core/parse.rs:303-318 | `parse_acceptance` stops at the FIRST blank line. A blank line placed immediately after `**Acceptance:**` (before the first entry) yields zero entries and surfaces as `EmptyAcceptanceList` ("scenario X has no acceptance entries"), even though valid entries follow. Reproduced via the binary: a spec with a blank line after the marker errors `scenario widget/1 has no acceptance entries` and exits 2. OpenSpec markdown commonly puts a blank line after a bold label, so authors will hit this; the error misdescribes the cause (entries exist; a blank line broke parsing). Fails loud (no silent drop), so it is brittleness/UX, not a correctness hole. |
| 2 | LOW | src/core/parse.rs:206-216 | `scenario_blocks` matches `line.trim_start().starts_with("#### Scenario:")`, so an indented `#### Scenario:` inside a fenced code block would be treated as a real scenario heading. Specs are unlikely to embed such fences; low impact. |
| 3 | LOW | src/core/parse.rs:290-294 | A non-numeric or malformed `spec-ref` value maps to `MissingSpecRef` rather than a dedicated "malformed spec-ref" error. Documented as a deliberate Stage-1 choice (no malformed-ref variant); fails loud, just with a slightly imprecise message. |

## Task 3 -- [task-3] task-list + closing-task fragment parsing

### Surface summary
- `src/core/parse.rs`: `parse_tasks(md) -> TaskList`, `parse_fragment(md) -> ClosingTaskFragment`, `validate_closing_kinds(tasks, fragment) -> Result<()>`; helpers `task_heading`, `strip_checkbox`, `closing_field`, `fragment_check`, `validate_check_method`.
- `templates/closing-task-checks.md`: declares `smoke-test`, `cleanup`, `module-shape`, `claude-arch`, `documentation`. `tests/closing_task_fragment.rs`, `tests/fixtures/tasks_sample.md`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Fragment declares exactly the five mandatory kinds (T3.3). `parse_tasks` extracts number/name/`Closing:` (T3.1); `parse_fragment` maps kinds to checks (T3.2); `validate_closing_kinds` returns `UnknownClosingKind` for an absent kind (T3.4). Several of the standing fragment's closing kinds (`cleanup` 50%, `module-shape`/`claude-arch` 100%, `documentation` 50%) trip the Task-4 inspection-cap warning when rendered — expected non-fatal behavior, not a defect. |

## Task 4 -- [task-4] contract render (+ task-8 cleanup amends this module)

### Surface summary
- `src/core/render.rs`: `ContractModel`, `build_model(scenarios, tasks, fragment) -> Result<ContractModel>`, `render(&ContractModel) -> Result<RenderedContract>`; private `PlacedEntry`, `feature_section`, `closing_section`, `specs_comment`, `spec_ref_key`, `inspection_warning`, `fragment_check_method`.
- `types.rs`: `RenderedContract` gains `warnings`. `tests/contract_render.rs`: full `parse->build->render` pipeline + error surfacing.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 4 | LOW | src/core/render.rs:269-285 | `specs_comment` dedups Spec References by `(capability, number)` only and takes the `deviated` flag from the FIRST entry of each key. In the real pipeline `parse_specs` rejects duplicate refs within a capability, and cross-capability refs cannot collide, so one key maps to one scenario / one deviated flag — no inconsistency reachable through the CLI. Only the `pub build_model` API, fed two hand-built scenarios with the same ref, could expose a stale flag. Unreachable in practice. |
| 5 | LOW | src/core/render.rs:242-254, 305-312 | `closing_section`/`fragment_check_method` `expect(...)` (panic) on an invalid closing kind or method tag. Guaranteed unreachable after `validate_closing_kinds` + `parse_fragment` in the CLI; only a caller bypassing those via the `pub` API and `panic="abort"` would abort. Documented as an invariant violation; fail-loud-by-design (task-8 deliberately replaced the prior silent `unwrap_or_default`/`unwrap_or(Execution)` fallbacks with these). Acceptable. |

Contract T4 verified: checks render under the linked task `- [ ] [method] statement` (T4.1); verbatim, incl. backticks/`%`/brackets (T4.2, tested); ordered by Spec Reference then source order (T4.3); one deduped/ordered `<!-- Specs -->` per feature task (T4.4); `(DEVIATED)` marks the ref (T4.5); closing tasks carry fragment checks and no Specs comment (T4.6); `AcceptanceLinksClosingTask` (T4.7); byte-identical repeat render (T4.8); task-list order (T4.9); dangling link / zero-check feature / duplicate ref each fail loud with no partial output (T4.10); format fidelity (T4.11); strict-`>`-20% inspection cap, integer math, exact 20% does not warn (T4.12).

## Task 5 -- [task-5] shadow compare + render/shadow CLI (+ fix commit d5e5a4f)

### Surface summary
- `src/core/shadow.rs`: `Diff::{Identical,Different(String)}`, `compare(rendered, existing) -> Diff`, `NEWLINE_ONLY_MESSAGE`, in-house LCS `line_diff`.
- `src/cli.rs`: `run(args, out, err) -> i32`, `EXIT_USAGE=2`, `EXIT_RENDER_ERROR=2`, `EXIT_DIFFERENT=1`; `RenderArgs`/`ShadowArgs`/`Flags` (`--specs/--tasks/--fragment[/--existing]`). `src/main.rs`: thin. `tests/cli.rs`: drives the real binary incl. a byte+mtime read-only snapshot.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 6 | LOW | contract.md:48 vs src/cli.rs:39-40 | Contract T5.2 wording shows `shadow <existing-contract>` (positional), but the binary takes `--existing <contract_md>` (flagged). Wording mismatch only; the observable behavior (diff emitted, existing byte-unchanged) holds and the README documents the real flag syntax. Not a code defect. |
| 7 | LOW | src/cli.rs / contract.md:51 | T5.5 ("a shadow run modifies no mission-skill or pipeline files") is verified only against the copied fixture tree (byte+mtime snapshot), not against real mission/pipeline paths. Shadow is read-only by construction (the sole FS touch is `read_to_string(--existing)`), so the guarantee holds, but no test exercises an actual mission path. |

Contract T5 verified: `render` over a fixture emits the contract and exits 0 (T5.1, byte-matched to fixture); `shadow` emits a diff and leaves the existing contract byte-unchanged (T5.2, snapshot test); identical -> "no differences" exit 0 (T5.3); `RenderError` -> non-zero + stderr, inspection-cap -> warning on stderr + exit 0 (T5.4); read-only snapshot incl. mtime (T5.5). The fix commit adds explicit `NEWLINE_ONLY_MESSAGE` so a trailing-newline / CRLF-only difference is never a blank `Different("")` — a real edge correctly closed.

## Task 6 -- [task-6] stable-ref migration + archived-fixture equivalence test

### Surface summary
- `tests/migrated_contract.rs`: 3 tests (equivalence to archived contract; insertion stability; positional->token resolution). `tests/fixtures/migrated/` (4 capability specs + tasks.md).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 8 | LOW | tests/migrated_contract.rs:122-204 | The equivalence test hardcodes the archived feature-task check strings rather than reading the archived `contract.md` at test time. I cross-checked the hardcoded strings against `openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/contract.md`: the feature-task (1-3) checks and Specs comments match verbatim. Closing tasks 4-8 are intentionally compared STRUCTURALLY (fragment-sourced, no Specs comment) because v2 standardizes them — the test documents this honestly. Equivalence claim is accurate; the only risk is future archived-contract drift going undetected because the assertion is a copy, not a live read. |

Contract T6 verified: migrated fixture renders equivalent (same checks under same tasks) to the hand-authored archived contract (T6.1); inserting a scenario leaves prior Spec References fixed, incl. ref 3 resolving to the same scenario (T6.2); migrated tokens resolve to the same scenarios as the positional refs (T6.3). The migrated tasks.md is real and tags all five closing kinds.

## Task 7 -- [task-7] end-to-end smoke test

### Surface summary
- `tests/smoketest.sh`: builds the binary (fail-loud), runs `render` (asserts `# Contract`/`## Task `/`<!-- Specs: `/`- [ ] [`), `shadow` identical (no differences + `cksum` byte-unchanged), `shadow` different (exit 1 + non-empty diff). POSIX `sh`, `cksum`/`cut`, `grep -qF -e`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Ran it: 10/10 pass. Builds the real binary, runs `render` asserting a contract is emitted, runs `shadow` asserting the existing contract is byte-unchanged, exits 0 (T7.1). Cross-platform-clean: no `sed -i`/`date -d`/`stat -c`/`readlink -f`/`grep -P`; uses `mktemp -d /tmp/render-smoke.XXXXXX`. |

## Task 8 -- [task-8] cleanup: fail-loud render guards; remove debris

### Surface summary
- `src/core/render.rs`: replaced `closing_section`'s `.map(...).unwrap_or_default()` with `.expect(...)` and `fragment_check_method`'s `unwrap_or(Execution)` with `.expect(...)`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. clippy `--all-targets -D warnings` exits 0 (T8.1). `grep -rn` over `src` finds no `todo!()`/`unimplemented!()`/`#[allow(dead_code)]` (T8.2, confirmed; no `#[allow(...)]` at all). The change removes two silent fallbacks in favor of fail-loud `expect` (see Task 4 finding 5) — a genuine quality improvement, not gaming. |

## Task 9 -- module shape re-check (no source commit; artifact under mission/validations)

### Surface summary
- `mission/validations/module-shape-audit.md`: per-module (5) and per-type (9 + 4 additions) drift assessment with resolution.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 9 | LOW | mission/validations/module-shape-audit.md / design.md | The audit repeatedly states "design.md UPDATED" for the as-built widenings (`validate_closing_kinds`, `build_model`/`ContractModel`, cli lib-module, 4 catalog additions). design.md does contain those "Reconciled Task 9 / added Task 9" notes, but `git log` shows design.md was last touched in the BASE commit `540e195` itself, so those updates appear in the base, not within base..HEAD. Provenance quirk (the reconciliation is folded into the planning-artifacts commit), not a code defect. The contract check is `[output] the audit file exists and records drift` (T9.1), which holds. |

## Task 10 -- [task-10] document darkfactory-render in ARCH.md

### Surface summary
- `ARCH.md`: new "## Contract renderer (darkfactory-render)" section + dependency list entry.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. ARCH.md has a darkfactory-render section (T10.1). `git diff base..HEAD -- CLAUDE.md` is empty — root CLAUDE.md unmodified by the mission (T10.2, confirmed). |

## Task 11 -- [task-11] tool README + repo README + cutover/rollback

### Surface summary
- `tools/darkfactory-render/README.md`: build/install, `render`, `shadow`, acceptance syntax, closing-task fragment, cutover gate, rollback. `README.md`: adds a darkfactory-render subsection + toolchain note.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Tool README documents `render`/`shadow` and the acceptance authoring syntax (T11.1); describes the cutover gate criteria and a 2-step rollback (regenerate hand-authored contract, restore prior validator flow) (T11.2); repo README mentions the tool (T11.3). T11.4: I ran all four documented commands (render, shadow identical, shadow different, render error) against the built debug binary — each produced the pasted output verbatim with the stated exit codes (0/0/1/2). |

## Cross-task

| # | Severity | File | Finding |
|---|----------|------|---------|
| 10 | LOW | src/core/parse.rs:165-176 vs src/core/render.rs:305-312 | The fragment method tag is extracted by two different code paths: `validate_check_method` (parse side, via `bracketed`) and `fragment_check_method` (render side, via `strip_prefix('[')` + `split(']').next()`). I traced both on bracketed/nested-bracket inputs and they agree, so a tag `parse_fragment` accepts never panics in `fragment_check_method`. Duplicated extraction logic is a maintenance hazard (a future change to one and not the other could reintroduce a panic), but currently consistent — no live defect. |
| 11 | LOW | src/cli.rs:140-189 | Whole-pipeline composition (`read_scenarios` sorts capabilities -> `parse_*` -> `build_model` -> `render`/`compare`) is integration-tested end to end via the real binary (cli.rs, smoketest.sh, multi-capability ordering). Upstream fail-loud errors (`DuplicateSpecReference`, `DanglingTaskLink`) surface through the full pipeline (contract_render.rs, cli.rs). No integration gap found between any task's outputs and the next task's expectations. |

## Summary

Findings: 0 HIGH, 1 MEDIUM, 10 LOW. The mission is in strong shape: every contract check I could verify holds, the test suite is genuinely exercised (76 tests + 10 smoke checks + real `openspec validate` + real binary for every README example, all reproduced verbatim), and there is no contract gaming — no NullBridge/test-double leakage, no smoke test that bypasses real wiring, and the task-8 cleanup deliberately converted silent fallbacks to fail-loud `expect`. The single MEDIUM is parsing brittleness: a blank line between `**Acceptance:**` and its first entry truncates the block to zero entries and reports a misleading `EmptyAcceptanceList`, which authors using normal OpenSpec markdown spacing will hit (it fails loud, so it is UX/robustness, not a correctness hole). The LOW findings are documented-invariant panics that are unreachable through the CLI, a copy-rather-than-live-read equivalence assertion, and a design.md/audit provenance quirk. I would recommend shipping as-is for this read-only Stage-1 shadow tool; the acceptance-block blank-line tolerance is the one item worth fixing (or documenting in the authoring syntax) before authors lean on it.
