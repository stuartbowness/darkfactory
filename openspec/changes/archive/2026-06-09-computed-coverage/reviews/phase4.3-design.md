# Review: design.md

Phase: 4.3
Reviewed: 2026-06-04
Status: REVIEWED
Findings: 3 (2 HIGH, 1 MEDIUM)

Codex (gpt-5.5, read-only) reviewed design.md WITH read access to the actual crate, asked to stress-test the `spec_ref -> Option` regression claim against render.rs. It judged the design mostly feasible; two boundaries needed tightening. All 3 addressed (in design.md + spec.md):

- HIGH (duplicate-task detection breaks render/shadow): `parse_tasks` is shared and currently accepts duplicate task numbers; adding `DuplicateTaskNumber` there would make render/shadow exit 2 on inputs they accept today, violating "render/shadow unchanged." FIXED: duplicate-task rejection moved to the coverage CLI path; `parse_tasks` stays permissive. Design Module Inventory + Error Strategy updated.
- HIGH (missing-Spec-Reference tolerance contradictory): coverage uses the strict `parse_specs`, so an acceptance-carrying scenario without a spec-ref exits 2 before analysis. FIXED: tolerance scoped to ACCEPTANCE-LESS scenarios only; acceptance-carrying-without-spec-ref stays exit 2 (MissingSpecRef). Spec tolerance requirement + a new scenario added; design Error Strategy updated.
- MEDIUM (`task:0` undefined): `u32` accepts 0 but spec said "positive integer". FIXED: spec now says the task token must resolve to a `u32` (matching `parse_task_link`); `[task:0]` is a normal link resolved against the task list, no special case.

`openspec validate` passes. The core `spec_ref -> Option` decision stands: render only processes acceptance-carrying scenarios (always `Some`), so render output is byte-identical — guarded by the existing render/shadow tests in the build loop.
