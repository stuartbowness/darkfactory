## Why

Every mission hand-authors the **Coverage audit** (Phase 4a) — a table answering "did every requirement reach a task?" That is a join already implied by the **Acceptance** task links, re-authored by an LLM each mission, so it can drift from the specs and tasks it derives from and costs orchestrator effort to re-derive. This is the **Duplicated-artifact drift** EPIC #199 targets, applied to coverage. Stage 2 computes the answer instead of authoring it.

## What Changes

- Add a `coverage` subcommand to `darkfactory-render` (`darkfactory-render coverage --specs <dir> --tasks <md>`, no `--fragment`) that renders the **Gap report** from the Scenarios and their **Acceptance** task links: a tolerant, scenario-level pass listing every **Scenario** with no acceptance line pointing to a valid **Feature task**, each tagged with its **Gap kind**, plus a one-line total/covered/gaps summary. Markdown to stdout; three-way exit code (`0` no gaps, `3` gaps found, `2` usage/parse error).
- Extend `parse_specs` (additively) to retain *every* scenario — including those with no `**Acceptance:**` block — and to capture each scenario's behavior text. The strict contract **Renderer** keeps its current fail-loud behavior unchanged.
- Introduce coverage domain types (a `CoverageGap` value carrying the scenario reference and gap kind, and a coverage report model) following the existing deterministic, fail-loud patterns.
- Runs in **shadow mode** only: not wired into the live mission Phase 4a, the hand-authored Coverage audit is not retired, and v1 mission flow is unchanged. No **Cutover**.

## Capabilities

### New Capabilities
- `computed-coverage`: tolerant, deterministic computation of the coverage gap report — uncovered scenarios, their gap kinds, and a summary — from acceptance task-links, exposed as the `coverage` subcommand. Includes the parser retention needed to see all scenarios and their behavior text, the tolerance boundary (which conditions are gaps vs. fatal parse errors), and the canonical Gap report output format.

### Modified Capabilities
<!-- None. The parser retention is additive (new scenarios surfaced, no existing acceptance-field requirement changed) and the strict renderer is untouched. If the specs phase finds the retention changes an existing acceptance-field requirement, a delta spec will be added then. -->

## Impact

- `tools/darkfactory-render`: new coverage analysis module + `coverage` CLI subcommand; `parse_specs` extended additively; new domain types. The shared parser also feeds `render`/`shadow`, so the extension must not alter their behavior — guarded by Stage 1's existing tests staying green (`make check`, `make smoke`).
- New fixture-based equivalence tests cover the projection: one fixture per **Gap kind** (no acceptance, dangling link, closing-task link), the no-gap case, the exit codes (`0`/`2`/`3`), and the real Stage 1 archived change — following Stage 1's archived-fixture test pattern.
- No changes to `darkfactory-dispatch`.
- No changes to the live mission flow, the hand-authored coverage audit, or any other skill — this lands additively on `main`.
