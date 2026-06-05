# User-Test Validation: Task 3

## Result: PASS

## Test Strategy

Task 3 adds an INTERNAL pure formatter `core::coverage::render_report(&CoverageReport) -> String` to the Rust crate at `tools/darkfactory-render/`. There is no CLI, HTTP, or UI surface yet (that is Task 4), so the change is infrastructure-only and is verified by building the crate and running its bundled unit tests. The commit ships three behavior tests for `render_report` plus a no-spec-ref / multi-kind pair, all of which assert exact string output. I built the crate, ran the full test suite, ran the `render_report` tests in isolation, and cross-checked the asserted output bytes against the spec's worked example and the three contract checks.

Commit: `2a234e765f41078dfa9ca86ec1eaaf44c21c4ed3` on branch `mission/computed-coverage`. The diff touches only `tools/darkfactory-render/src/core/coverage.rs` (+204).

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo build` compiles the crate | Manual (cargo) | PASS |
| 2 | `cargo test` full suite | Manual (cargo) | PASS (93 passed, 0 failed) |
| 3 | `cargo test coverage::tests::render_report` (4 render_report tests) | Manual (cargo) | PASS (4 passed) |
| 4 | Em dash in code and fixtures is genuine U+2014, not a hyphen | Manual (byte inspection) | PASS |
| 5 | Spec worked-example layout (spec.md L68-85) equals test fixture (coverage.rs L437-456) | Manual (diff) | PASS |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | For the worked-example input, `render_report` produces byte-for-byte the canonical layout (title, summary, per-gap heading `### <id> — <title>`, `- Gap kind:`, `- Behavior:` with bullets indented two spaces verbatim). | PASS | Test `render_report_emits_the_canonical_gaps_layout` (coverage.rs L411-459) feeds the identical worked-example input (gadget/1 dangling "Sync gadget", widget/2 no-acceptance "Delete widget", total 3, covered 1) and asserts `render_report(&report) == expected`. The `expected` string (L437-456) is byte-identical to the spec's canonical layout (spec.md L68-85). Code emits the heading via `format!("### {} — {}\n", ...)` (L131) with em dash U+2014 (confirmed 0x2014), `- Gap kind:` (L132), `- Behavior:` (L133), and each behavior bullet via `format!("  {line}\n")` — exactly two leading spaces, verbatim line (L134-136). Test passes. |
| 2 | When no gaps, `## Gaps` contains exactly `None.` and the summary ends with `0 gaps`. | PASS | Code L123-127: when `report.gaps.is_empty()`, it pushes one blank line then `None.\n` and returns. Summary uses `report.gaps.len()` (= 0), yielding `0 gaps`. Test `render_report_emits_the_canonical_no_gaps_layout` (L461-481) asserts the full output ending `## Gaps\n\nNone.\n` with summary line `3 scenarios, 3 covered, 0 gaps`. Test passes. |
| 3 | The summary line reads exactly `<T> scenarios, <C> covered, <G> gaps`. | PASS | Code L114-119: `format!("{} scenarios, {} covered, {} gaps\n", report.total, report.covered, report.gaps.len())`. Asserted in both layout tests as `3 scenarios, 1 covered, 2 gaps` and `3 scenarios, 3 covered, 0 gaps`. Tests pass. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A — all contract checks pass.
