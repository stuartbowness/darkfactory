# Adversarial Review A

Ground-truth check on `mission/computed-coverage` @ HEAD `cafa105..9f0bf79`:
`cargo build` OK, `cargo fmt --check` OK, `cargo clippy --all-targets -- -D warnings` OK,
`cargo test` = 107 passed (9 suites), `make smoke` = 16/16 PASS.

## Task 1 -- [task-1] scenario model + parser retention (title, behavior, spec_ref Option)

### Surface summary
- `src/core/types.rs`: `RequirementScenario` gains `title: String`, `capability: String`, `behavior: Vec<String>`; `spec_ref` changes from `SpecReference` to `Option<SpecReference>`. `SpecReference.capability` retained (duplicated with scenario).
- `src/core/parse.rs`: `parse_scenario` now returns `RequirementScenario` (not `Option`), retains acceptance-less scenarios, captures behavior bullets via new `is_behavior_bullet`. Duplicate-spec-ref detection guarded behind `Some(spec_ref)`. `MissingSpecRef` still raised for acceptance-carrying scenarios with no ref (parse.rs:271).
- `src/core/render.rs`: placement loop skips scenarios with `spec_ref == None` (render.rs:~169 `let Some(spec_ref) = ... else { continue }`), deref change `&scenario.spec_ref` -> `spec_ref`.
- `tests/migrated_contract.rs`, `tests/openspec_validate.rs`: mechanical accessor updates for `Option<SpecReference>`.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Model change is byte-identity-safe: acceptance-less scenarios were dropped before and now place nothing (no entries), so render/shadow output is unchanged. `MissingSpecRef` invariant preserved at parse.rs:271. Behavior loop correctly breaks at `**Acceptance:**` (parse.rs:239-241). Test edits are mechanical accessor changes, no output assertion weakened. |

## Task 2 -- [task-2] coverage analysis (analyze, CoverageGap/GapKind/CoverageReport, DuplicateTaskNumber) + fix

### Surface summary
- `src/core/types.rs`: adds `GapKind {NoAcceptance, DanglingTaskLink, ClosingTaskLink}`, `CoverageGap`, `CoverageReport`, `RenderError::DuplicateTaskNumber(u32,String)`. Fix adds `RequirementScenario.capability`.
- `src/core/coverage.rs` (new): `pub fn analyze(&[RequirementScenario], &TaskList) -> CoverageReport` (infallible), `fn classify`, `pub fn render_report(&CoverageReport) -> String`, `pub fn check_duplicate_tasks(&TaskList) -> Result<(), RenderError>`.
- Fix commit 45e0efd: replaced `capability_of(scenario)` (which derived capability from spec_ref, yielding "" for no-spec-ref gaps) with `scenario.capability.clone()`; threaded new `capability` field through parse.rs and the render.rs test helper.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Verified against contract: NoAcceptance returns early so it never co-occurs with dangling/closing (consistent with the no-acceptance-vs-empty-entries semantics). `classify` returns `None` on first feature-task hit -> covered-despite-unresolved-sibling holds (coverage.rs:82). Sort is total (capability, then global position) so intra-capability document order is preserved even if capability dirs interleave (coverage.rs:50). Fix correctly sources capability from the scenario, not spec_ref, so no-spec-ref gaps identify by real capability. `check_duplicate_tasks` kept out of `parse_tasks`, preserving render/shadow permissiveness. |

## Task 3 -- [task-3] gap report rendering (render_report, canonical format)

### Surface summary
- `src/core/coverage.rs` (+204): adds `render_report`, `gap_identifier`, `gap_kinds`, and 5 unit tests asserting byte-equal canonical layout (gaps, no-gaps, bare-capability heading, multi-kind, summary line).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. `render_report` output matches the spec worked example byte-for-byte (verified via test + live run against migrated fixture). Em dash is U+2014 (e2 80 94) confirmed in actual binary output. Single-blank-line section separation holds. Empty-behavior gap renders `- Behavior:` with zero bullets, no trailing-blank artifact. Gap-kind labels exactly `no acceptance` / `dangling task link` / `closing-task link`. |

## Task 4 -- [task-4] coverage CLI subcommand (--specs/--tasks, exit 0/3/2)

### Surface summary
- `src/cli.rs`: adds `EXIT_GAPS=3`, `run_coverage`, `coverage_pipeline`, `CoverageArgs`. `coverage` wired into `run` dispatch. Pipeline: `read_scenarios -> parse_tasks -> check_duplicate_tasks -> analyze`. `read_scenarios` sorts capability dirs (deterministic).
- `tests/coverage.rs` (new, 340 lines): 15 integration tests driving the real binary -- exit 0/3/2 matrix, determinism, migrated-fixture pinned count, all 5 malformed-input cases + dangling-link-is-gap.
- 7 new CLI fixtures under `tests/fixtures/cli/` (bad_method, covered, dup_specref, dup_task_number, gap, malformed_link, missing_specref).
- `src/lib.rs`: `pub mod coverage;` (actually added in task-2 e644153).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Full 0/3/2 exit matrix tested against the real binary. `EXIT_USAGE` and `EXIT_RENDER_ERROR` both equal 2, satisfying "omitted flag" and "unreadable/missing path" -> 2. Dangling link -> gap/exit 3 (test + bad_specs fixture w/ task:99). All 5 malformed cases (bad method, malformed link, dup spec-ref, missing spec-ref w/ acceptance, dup task number) exit 2 with empty stdout where asserted. Coverage shares the same acceptance parser as render, so `[task:0]` resolves identically (no render/coverage disagreement). `read_scenarios` sorts dirs for determinism. |

## Task 5 -- [task-5] smoke test for coverage subcommand

### Surface summary
- `tests/smoketest.sh` (+34): appends Test 4 (coverage over migrated fixture -> exit 3 + pinned summary `11 scenarios, 8 covered, 3 gaps`) and Test 5 (coverage over `covered_specs` -> exit 0 + `0 gaps` / `None.`), driving the real built binary against on-disk fixtures.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tests/smoketest.sh:140 | The smoke test pins the migrated-fixture summary string `11 scenarios, 8 covered, 3 gaps` in two places (here and tests/coverage.rs:174). Any intentional future edit to the migrated fixture must update both. Not a bug -- pinning is correct and the count matches the live binary -- just a duplicated magic string to maintain. `make smoke` = 16/16 PASS. |

## Task 6 -- [task-6] cleanup: gitignore logs, doc-note duplicated capability, remove debris

### Surface summary
- `.gitignore`: adds `*.log`.
- `src/core/types.rs`: doc comment on `SpecReference.capability` explaining the documented duplication with `RequirementScenario.capability` (set from the same source in parse.rs, can't diverge; kept because render reads `spec_ref.capability` directly at render.rs:283/295).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. `make check` (fmt + clippy -D warnings + test) passes clean: no dead-code or unused-dependency warnings. Verified the duplicated-capability doc note is accurate -- render.rs genuinely reads `spec_ref.capability` (render.rs:283, 295), so the field is load-bearing, not dead. Commit subject says "remove debris" but the diff deletes no tracked files; no tracked debris exists (the removal was of untracked logs, now gitignored). Cosmetic subject/diff mismatch, not a code issue. |

## Task 8 -- [task-8] document coverage subcommand (tool README, repo README, ARCH.md)

### Surface summary
- `tools/darkfactory-render/README.md` (+87): `## coverage` section documents `--specs`/`--tasks`, exit codes 0/3/2, and two verbatim-output examples (no-gaps and gaps-found).
- `README.md` (repo): adds Stage 2 `coverage` subcommand mention at line 141.
- `ARCH.md`: documents `core::coverage` module (line 88) and the `coverage` subcommand (lines 89, 97).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| - | - | - | None. Both pasted README examples match the live binary byte-for-byte: no-gaps (`covered_specs` -> `1 scenarios, 1 covered, 0 gaps` / `None.`) and gaps-found (migrated -> `11 scenarios, 8 covered, 3 gaps` + 3 domain-type-boundary gaps with U+2014 em dash). Exit codes documented (0/3/2). Repo README + ARCH.md both updated. Task 7's module-shape-audit.md exists (2.2K); `rg` confirms CoverageGap/GapKind/CoverageReport/DuplicateTaskNumber/coverage all defined and compiling. |

## Cross-task

| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tests/smoketest.sh:140 + tests/coverage.rs:174 + README.md | The migrated-fixture gap count `11 scenarios, 8 covered, 3 gaps` is hard-pinned in three places (smoke test, integration test, tool README example). A deliberate future edit to `tests/fixtures/migrated/` must update all three. Verified consistent today (count derived from 11 `#### Scenario:` headings, 3 acceptance-less in domain-type-boundary). Maintenance hazard only, not a defect. |

Cross-task invariants verified (no findings):
- **Model change byte-identity (Task 1 -> render/shadow):** the `spec_ref: SpecReference -> Option<SpecReference>` change keeps render/shadow output byte-identical. Render skips `spec_ref == None` scenarios (render.rs), and those carry no acceptance entries, so they place nothing -- exactly as when they were dropped pre-change. The 19 pre-existing render/shadow/golden tests (contract_render, migrated_contract, cli, closing_task_fragment, openspec_validate) pass unchanged; task-1's test edits are accessor-only, no output assertion weakened.
- **Tolerant analyze vs strict build_model boundary (Task 2/4):** `analyze` is infallible (dangling/closing links -> gap kinds), `build_model` still fails loud; they share the same `parse_specs`/acceptance parser, so coverage and render never disagree about a link. A dangling link is exit 3 (gap), a malformed link/dup-ref/dup-task/bad-method is exit 2 -- the strict/tolerant split lands on the right side of every contract case.
- **Per-capability scoping (Task 1 parser + Task 4 CLI):** `seen_refs` is local per `parse_specs` call, and `read_scenarios` calls it once per capability dir, so DuplicateSpecReference is correctly "within a capability"; the same N across two capabilities is allowed.
- **Count accounting across model+analysis+CLI:** total/covered/gaps reconcile (11 = 8 + 3 on the migrated fixture; total = scenario-heading count). Retained acceptance-less scenarios (Task 1) flow into `analyze` (Task 2) as NoAcceptance gaps and reach the CLI report (Task 4) coherently.
- **Gap-kind fixed order (Task 2 enum -> Task 3 render):** enum declaration order, classify push order, and `gap_kinds` stored-order rendering all agree on no-acceptance / dangling task link / closing-task link.
- **Duplicated capability field (Task 1/2-fix/6):** `SpecReference.capability` and `RequirementScenario.capability` are both set from the same source in parse.rs and cannot diverge; render reads the former (render.rs:283/295), coverage reads the latter -- both load-bearing, documented, no data-integrity risk.
- **Per-commit compilability:** `pub mod coverage;` lands in task-2 alongside coverage.rs, so each `[task-N]` commit compiles independently.

## Summary

Findings: 0 HIGH, 0 MEDIUM, 2 LOW (both are maintenance hazards from a hard-pinned migrated-fixture gap count duplicated across the smoke test, integration test, and README; not defects -- all three agree today). The change is in strong shape: `make check` (fmt + clippy `-D warnings` + 107 tests) and `make smoke` (16/16) both pass clean. The five risk areas the dispatcher flagged all hold up under adversarial scrutiny: (1) the `spec_ref -> Option<SpecReference>` model change keeps render/shadow byte-identical because retained acceptance-less scenarios place no entries, and the 19 pre-existing golden tests pass with only accessor-level edits; (2) the tolerant `analyze`/strict `build_model` boundary is correct -- dangling/closing links become gaps (exit 3) while malformed input fails loud (exit 2), and both paths share one parser so they never disagree; (3) the canonical Gap report is deterministic and matches the spec worked example byte-for-byte, em dash = U+2014, verified live against the built binary; (4) the CLI 0/3/2 exit policy is fully tested against the real binary across the whole matrix; (5) the duplicated `capability` on RequirementScenario and SpecReference is genuinely load-bearing (render reads one, coverage the other), set from one source so it cannot diverge, and now documented. The biggest residual risk is the triple-pinned magic count (LOW) -- a future fixture edit could leave the three copies out of sync, but that fails loudly in CI rather than shipping a silent bug. Recommendation: SHIP as-is.
