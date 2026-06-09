# Adversarial Review B

Base SHA: `e93a76898950346e816d0b1639d37b5c5bf3475c`
Branch: `mission/computed-coverage`

## Task 1 -- scenario model + parser retention (title, behavior, spec_ref Option)
### Surface summary
- `core/types.rs`: `RequirementScenario` gains `title: String`, `behavior: Vec<String>`; `spec_ref` changed `SpecReference` -> `Option<SpecReference>`.
- `core/parse.rs`: `parse_scenario` now returns `RequirementScenario` (was `Option<..>`); acceptance-less scenarios retained; new `is_behavior_bullet`; duplicate-ref detection now keyed on `Option` spec_ref.
- `core/render.rs`: `placed_entries` skips `spec_ref == None` scenarios; test helper sets `title`/`behavior`.
- tests: `migrated_contract.rs`, `openspec_validate.rs` updated for `Option` spec_ref accessor.

### Verdict on render/shadow invariance
- Render is neutral: `placed_entries` `continue`s on `spec_ref == None`; sort key is `(spec_ref, scenario_index, entry_index)` and retained acceptance-less scenarios only widen the `scenario_index` gaps without changing the relative order of placed entries (stable sort). Shadow is a pure string diff over render output, so it is also neutral.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tools/darkfactory-render/src/core/parse.rs:60-67 | Duplicate-ref detection now runs on any scenario carrying a spec_ref, including a retained acceptance-less scenario with a `<!-- spec-ref: N -->` comment. Pre-change, acceptance-less scenarios were dropped before this check, so such a scenario could not collide. New behavior: an acceptance-less scenario whose spec-ref comment number duplicates an acceptance-carrying one now triggers `DuplicateSpecReference`. No existing fixture exercises this, so render/shadow output is unaffected today, but it is a real behavior change not called out in the spec. Arguably correct (a duplicate ref is a duplicate ref) but worth noting. |

## Task 2 -- coverage analysis (analyze, CoverageGap/GapKind/CoverageReport)
### Surface summary
- `core/coverage.rs` (new): `analyze(&[RequirementScenario], &TaskList) -> CoverageReport`; `classify`; `render_report`; `gap_identifier`; `gap_kinds`; `check_duplicate_tasks`.
- `core/types.rs`: adds `GapKind`, `CoverageGap`, `CoverageReport`, `RenderError::DuplicateTaskNumber(u32,String)`; `RequirementScenario.capability` (fix commit).
- `core/parse.rs` / `render.rs`: set/seed `capability` field (fix commit).
- `lib.rs`: `pub mod coverage`.

### Notes verified correct
- "covered iff any acceptance links to a feature task": `classify` early-returns `None` on the first feature-task hit, so a covered scenario with unresolved siblings is correctly not a gap. Matches spec scenarios.
- Distinct kinds in fixed order: `dangling` then `closing` flags pushed in that order; NoAcceptance is the early-return sole kind. Matches the spec's fixed order.
- `covered = total - gaps.len()` is exact: every scenario yields exactly one `Some`/`None`.
- No tie nondeterminism: sort key `(capability, global_position)`; `global_position` is the unique enumerate index, so keys are unique.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tools/darkfactory-render/src/core/coverage.rs:47-50 | "Document position within capability" relies on the CLI feeding each capability's scenarios as a CONTIGUOUS block (the sort's secondary key is the GLOBAL enumerate index). VERIFIED OK for the only caller: `cli.rs:read_scenarios` sorts capabilities by name and `extend`s the list one whole capability at a time, so per-capability blocks are contiguous and the global index is monotonic within a capability. Note remains as a coupling: `analyze`'s correctness silently depends on this caller invariant, undocumented in `analyze`. A future caller passing an interleaved list would mis-order without any failure. |
| 2 | LOW | tools/darkfactory-render/src/core/coverage.rs:50 | `analyze` re-sorts by capability even though the CLI already groups by capability name. Harmless and arguably defensive, but it means `analyze` does NOT match the spec's "document position within that capability's spec.md" for any caller that DOESN'T pre-group by name (the sort would reorder capabilities lexically while keeping the now-wrong within-capability positions). Today's single caller pre-groups, so output is correct. |

## Task 3 -- gap report rendering (render_report, canonical format)
### Surface summary
- `core/coverage.rs`: adds `render_report`, `gap_identifier`, `gap_kinds` (em dash U+2014 heading separator) + canonical-format tests.

### Notes verified correct
- The worked-example canonical layout test (`render_report_emits_the_canonical_gaps_layout`) asserts byte-for-byte against the spec's pasted block, and the END-TO-END migrated-fixture run (verified live) reproduces the same per-gap shape through parse->analyze->render. Not gamed.
- No-gaps body is exactly `None.`; summary ends `0 gaps`. Verified live.
- A gap with empty `behavior` renders `- Behavior:` with no following bullets — allowed by spec, not malformed.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 4 -- coverage CLI subcommand (--specs/--tasks, exit 0/3/2)
### Surface summary
- `cli.rs`: adds `EXIT_GAPS=3`, `run_coverage`, `coverage_pipeline`, `CoverageArgs`; `read_scenarios` shared with render (sorts capabilities by name, extends contiguously).
- `main.rs`: `ExitCode::from(code as u8)` propagates 0/1/2/3.

### Notes verified correct (live)
- Exit 0 no-gaps, 3 gaps-found, 2 usage/parse/IO all verified by the `tests/coverage.rs` integration suite (16 cases) and live runs.
- Dangling link -> exit 3 (gap), not 2: verified (`bad_specs`).
- `[task:0]` is a normal link resolved against the task list; no special-casing.
- Empty/file-as-dir `--specs`, missing `--tasks` -> exit 2 via `fs::read_dir`/`read_to_string` Err.
- Duplicate flag (`--specs a --specs b`) -> `finish()` leftover -> exit 2 (fail-loud).

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tools/darkfactory-render/src/core/parse.rs:363-369 | A malformed task link (`[task:two]`, `[task:abc]`) exits `2` as the contract requires, but the error MESSAGE is `unknown verification method: [task:two] [execution] ...` — it mislabels a bad task link as a bad method. Pre-existing Stage 1 behavior (documented in the fn doc), not introduced here; exit code is correct. Confusing for an author debugging a coverage failure. |

## Task 5 -- smoke test for coverage subcommand
### Surface summary
- `tests/smoketest.sh`: adds Test 4 (coverage gaps, exit 3, pinned summary `11 scenarios, 8 covered, 3 gaps`) and Test 5 (coverage no-gaps, exit 0, `0 gaps`/`None.`) against real on-disk fixtures via the built binary.

### Notes
- `make smoke` passes 16/16 live. POSIX-portable (`cksum`/`cut`, `grep -qF -e`), no GNU-only flags. Satisfies Task 5 contract.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 6 -- cleanup
### Surface summary
- `.gitignore`: `target/`, `*.log`. `types.rs`: 5-line doc note on `SpecReference::capability` duplication.

### Notes
- `make check` (fmt + clippy `-D warnings` + tests) passes clean on a forced non-cached run; no dead-code/unused-dep warnings. No tracked `.log` files are masked by the new ignore. Satisfies Task 6 contract.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Task 8 -- documentation (tool README, repo README, ARCH)
### Surface summary
- `tools/darkfactory-render/README.md`: `coverage` section (flags, exit codes 0/3/2, two pasted example outputs).
- repo `README.md`: Stage 2 coverage subcommand + exit codes (line 141).
- `ARCH.md`: `core::coverage` module + `coverage` subcommand (lines 88-89, 97).

### Notes verified correct (live)
- All three README pasted outputs (render w/ templates fragment, coverage no-gaps, coverage gaps-found migrated) reproduce BYTE-FOR-BYTE with exit codes 0/0/3. Not gamed.
- Migrated gaps render in document order (Type Catalog def, Type naming chain, Domain type term) matching the spec.md scenario order — within-capability ordering correct on a real fixture.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|

## Cross-task

### Interactions checked and cleared
- **`spec_ref: Option` (T1) -> `analyze`/`render_report` (T2/T3):** every consumer handles `None` (`gap_identifier` falls back to bare capability; `placed_entries` skips `None`). No unwrap on the Option in a production path. Cleared.
- **`RequirementScenario.capability` (T2 fix) vs render:** the fix added the field only to a render TEST helper; render's production sort still keys on `spec_ref.capability`/`number`, never the new field. Render output proven byte-for-byte identical to the base SHA on `cli/specs`, `multi_specs`, and `inspection_specs` (stdout AND stderr). Render/shadow invariance HOLDS.
- **Identifier consistency:** for a gap with a spec-ref, `gap.capability == gap.spec_ref.capability` (both from the same parser arg), so `gap_identifier`'s `<capability>/<N>` equals render's identifier. No divergence.
- **Closing-task definition parity:** coverage `classify` and render both gate on `task.closing_kind.is_some()`. Same semantics.
- **Acceptance-but-no-spec-ref still errors (T1) across the coverage path (T4):** `parse_specs` runs before `analyze` in `coverage_pipeline`, so a `MissingSpecRef` surfaces as exit 2, not a gap. Verified live (`missing_specref_specs`).
- **Determinism end to end:** `read_scenarios` sorts capabilities before parsing; `analyze` sorts by `(capability, unique global index)`; no `HashMap` iteration feeds output. Byte-identical across runs verified live.
- **Within-capability document order:** depends on the CLI feeding contiguous per-capability blocks; the one caller does. Verified on the real migrated fixture (3 gaps in spec.md document order). See Task 2 findings #1/#2 for the latent coupling.

### Findings
| # | Severity | File | Finding |
|---|----------|------|---------|
| 1 | LOW | tools/darkfactory-render/src/core/coverage.rs:28-58 | `analyze` is a `pub fn` whose correctness silently depends on the caller passing scenarios grouped contiguously by capability AND already ordered by capability name (otherwise the secondary global-index sort yields wrong within-capability order and the capability re-sort yields lexical order that may not match document order). The only caller (`cli::read_scenarios`) satisfies both, so shipping behavior is correct, but the invariant is undocumented on `analyze`. A future second caller is the failure mode. Not a bug today. |

## Summary
This is a tight, well-tested change. Findings: 0 HIGH, 0 MEDIUM, 5 LOW (one per Task 1/4, two on Task 2, one cross-task). The single biggest risk is the latent coupling in `analyze` (Task 2 / Cross-task): its document-position ordering is correct only because the lone caller pre-groups and name-sorts scenarios, and that contract is undocumented on the public `analyze` signature — but no shipping path falsifies it. Everything the prompt flagged checks out: render/shadow output is provably byte-for-byte unchanged from the base SHA (stdout and stderr, three fixtures) because the new `behavior`/`capability` fields and the `Option<SpecReference>` are never read on the render path and acceptance-less scenarios are `continue`d in `placed_entries`; "covered iff any acceptance links to a feature task" is correct for multi-entry scenarios (early-return on first feature hit); the Gap report is deterministic (sorted keys, no HashMap iteration, unique sort keys, no empty-capability case since `capability` is always the directory name); CLI exit codes 0/1/2/3 are distinct and correct on every tested edge; and no contract check is gamed — the canonical-format, migrated-fixture, and all three README example outputs reproduce byte-for-byte through the full parse->analyze->render pipeline, and `make check`/`make smoke` pass live (107 unit/integration tests + 16 smoke checks, clippy `-D warnings` clean). Ship: **YES, ship as-is.** The 5 LOW findings are documentation/robustness notes, not defects; the one worth a follow-up is documenting `analyze`'s caller-ordering precondition.
