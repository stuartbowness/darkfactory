# Worker Fix Brief: Task 2 - Coverage analysis (Attempt 2)

## What went wrong (behavior)
An acceptance-less scenario that carries no `<!-- spec-ref: N -->` comment parses to `RequirementScenario { spec_ref: None, .. }`. The coverage code derives a gap's capability from `spec_ref` only, so for these scenarios the capability comes out as the empty string `""`. The required behavior is that such a scenario is reported as a gap identified by its **real capability** (the spec directory name) and its title — an empty capability is wrong, and it is the most common gap case (a scenario with a WHEN/THEN jotted but no acceptance and no spec-ref yet).

## Diagnostic Triage (root cause)
`RequirementScenario` (from Task 1) stores the capability only inside `spec_ref: Option<SpecReference>`. When `spec_ref` is `None`, the capability is lost — even though `parse_specs(capability, markdown)` already knows it. In `tools/darkfactory-render/src/core/coverage.rs`, `capability_of` does `spec_ref.map(..).unwrap_or_default()`, yielding `""` for `None`.

## The fix
1. Add a `capability: String` field to `RequirementScenario` in `src/core/types.rs`.
2. In `src/core/parse.rs`, populate `capability` for EVERY scenario `parse_specs` returns, using the `capability` argument it already receives. Do this for both acceptance-carrying and acceptance-less scenarios. Leave `spec_ref` exactly as-is (do not fabricate a number for `None` cases).
3. In `src/core/coverage.rs`, use `scenario.capability` for a gap's capability instead of deriving it from `spec_ref`.
4. Update every `RequirementScenario` constructor (tests, helpers) for the new field.
5. Render is unaffected — it uses `spec_ref` for the `<!-- Specs: capability/N -->` comments on acceptance-carrying scenarios (which always have `Some(spec_ref)`). Confirm render/shadow output stays byte-identical (existing tests green).

### Done when
- A test constructs an acceptance-less scenario with `spec_ref: None` parsed from capability `"widget"` and asserts the resulting gap's capability is `"widget"` (not `""`), identified by title.
- All existing tests stay green; `make check` is green.

## Procedures
**Before writing any code**, invoke the `/tdd` skill (mandatory) and follow red-green-refactor.
- Work only inside `tools/darkfactory-render/`. Verify with `make check`.
- Commit once, code+tests only, message: `[task-2] fix: carry scenario capability so no-spec-ref gaps identify by capability`. Stage only files under `tools/darkfactory-render/`. Never `git add -A`/`.`; never stage `openspec/changes/computed-coverage/`.
- Write your handoff to `openspec/changes/computed-coverage/mission/handoffs/task-2-fix-v2.md` (write, do not commit).

## Handoff Report Format
- Task: Task 2 - Coverage analysis (fix v2)
- Commit SHA: <full SHA>
- Approach: <what you changed>
- Commands run: <incl. make check exit code>
- Concerns: <any>
