# Issue: Task 2 - Coverage analysis (Attempt 1)

## Failed Gates
None from the validators (both returned PASS). This is an orchestrator-found spec gap that the contract checks, as worded, were too lenient to catch.

## Observed Symptoms
An acceptance-less scenario with no `<!-- spec-ref: N -->` comment parses to `RequirementScenario { spec_ref: None, .. }`. `core::coverage` derives a gap's capability from `spec_ref` only, so for these scenarios the capability comes out as the empty string `""`. The gap report (Task 3) would then render an empty capability for exactly the most common gap: a scenario someone jotted (WHEN/THEN) with no acceptance and no spec-ref yet. The required behavior is that such a scenario is identified by its real capability (the spec directory name, which `parse_specs` already receives) and its title.

## Root Cause
`RequirementScenario` (added in Task 1) stores the capability only inside `spec_ref: Option<SpecReference>`. When `spec_ref` is `None`, the capability is lost, even though `parse_specs(capability, markdown)` knows it. There is no standalone capability field.

## Fix
Add a `capability: String` field to `RequirementScenario`, populated by `parse_specs` for EVERY scenario (= the `capability` argument), and have `core::coverage` use `scenario.capability` instead of deriving it from `spec_ref`. Keep `spec_ref` as-is (None stays None — do not fabricate a number). Render is unaffected (it uses `spec_ref` for acceptance-carrying scenarios, which always have it).
