# Review: tasks.md

Phase: 4.4
Reviewed: 2026-06-02
Status: REVIEWED
Findings: 3 (1 HIGH, 1 MEDIUM, 1 LOW) — all applied at source

Reviewer note: the Codex `/artifact-review` dispatch failed twice with a Codex-side error (`image_generation_user_error: The model 'gpt-image-2' does not exist`), unrelated to the artifact. Per Rule 11 (non-blocking), the review was run via a Claude subagent against tasks.md + design.md + all three specs + PRD #200.

Findings, all applied:

- [HIGH] The inspection-cap warning is computed in Task 4 (`core::render`) but emitted in Task 5 (CLI) with no named surface linking them → added `RenderedContract::warnings` to Task 4 Produces and made Task 5 Consume it, so the two halves can't drift.
- [MEDIUM] The model-assembly glue (read files → 3 parsers → typed model) had no named home → added `core::build_model` to Task 4 Produces; Task 5's CLI routes through `parse_* → build_model → render`, with the file-reading glue covered by the smoke (Task 7) and integration (Task 6) tests.
- [LOW] Task 6 conflated the concept "Spec Reference" with the authoring token `spec-ref: N` → reworded to distinguish them.

Verified clean by the reviewer: all 10 RenderError variants have a home; all three capabilities covered; legacy-migration and the openspec-validate-compatibility test covered; all five mandatory closing tasks present and tagged with fragment-known `Closing:` kinds (smoke-test/cleanup/module-shape/claude-arch/documentation); the dependency DAG is acyclic (1→{2,3}→4→5→6→7→…→11) with no task consuming an unproduced surface; Task 4 kept whole; no task spans >2 layers.

`openspec validate`: PASS. Human approval pending.
