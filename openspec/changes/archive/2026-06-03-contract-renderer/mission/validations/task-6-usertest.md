# User-Test Validation: Task 6

## Result: PASS

## Test Strategy

Task 6 is a CLI/library change (a Rust crate, `darkfactory-render`) plus test
fixtures, no HTTP or browser surface. So I verified behavior two independent
ways:

1. Ran the task's own integration suite: `cargo test --test migrated_contract`
   (3 tests, one per contract check).
2. Drove the REAL binary myself: `cargo build`, then
   `./target/debug/darkfactory-render render --specs tests/fixtures/migrated/specs
   --tasks tests/fixtures/migrated/tasks.md --fragment templates/closing-task-checks.md`,
   and diffed the rendered FEATURE-task sections (Tasks 1-3) byte-for-byte against
   the archived `contract.md`. I also cross-checked the migrated `spec-ref` tokens
   against the archived scenarios' positional order (read from the read-only
   archived change) so the equivalence and resolution claims are confirmed against
   the real archived source, not just asserted by the test.

The archived source consulted for cross-check:
`openspec/changes/archive/2026-04-29-domain-vocabulary-type-names/`
(contract.md + specs/). The contract-renderer change's own contract.md was NOT read.

## Tests Run

| # | Test Description | Method | Result |
|---|-----------------|--------|--------|
| 1 | `cargo test --test migrated_contract` (all 3 tests) | Cargo | PASS (3 passed) |
| 2 | Build the binary (`cargo build`) | Cargo | PASS (exit 0) |
| 3 | Render migrated fixture via the real binary | CLI | PASS (exit 0, full 8-task contract emitted) |
| 4 | Diff rendered Task 1 vs archived contract.md Task 1 | Manual diff | PASS (byte-identical) |
| 5 | Diff rendered Task 2 vs archived contract.md Task 2 | Manual diff | PASS (byte-identical) |
| 6 | Diff rendered Task 3 vs archived contract.md Task 3 | Manual diff | PASS (only diff: added `<!-- Specs: readme-updates/1 -->`, per design D6) |
| 7 | Closing Tasks 4-8 render the standing fragment's checks, no `<!-- Specs -->` | Manual grep | PASS (7/7 fragment statements present; Specs comments only on Tasks 1-3) |
| 8 | No SKIP markers; all 18 checks rendered | Manual grep | PASS (0 SKIP, 18 `- [ ]` bullets) |
| 9 | Cross-check migrated `spec-ref` tokens vs archived positional scenario order | Manual (awk over archived specs) | PASS (tokens N match the Nth archived scenario) |

## Contract Checks

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | [execution] An integration test renders the migrated archived fixture and asserts the result is equivalent to the hand-authored contract (same checks under the same tasks). | PASS | `migrated_fixture_renders_equivalent_to_the_archived_contract` passes: drives the full pipeline (`parse_specs`/`parse_tasks`/`parse_fragment` -> `build_model` -> `render`) on `tests/fixtures/migrated/`, asserts all 8 task sections in archived order, verbatim feature-task checks, and correct `<!-- Specs -->` comments. Independently confirmed by driving the real binary: rendered Tasks 1 and 2 are BYTE-IDENTICAL to the archived `contract.md`; Task 3 differs only by the one added `<!-- Specs: readme-updates/1 -->` line (its 3 README checks are byte-identical). Closing Tasks 4-8 render the standing fragment's generic checks (7/7 present), which the v2 design standardizes and which therefore differ from the archived mission-specific closing checks. 0 SKIP, 18 checks total. |
| 2 | [execution] A test shows that inserting a new scenario does not change the Spec References of existing scenarios. | PASS | `inserting_a_scenario_does_not_change_existing_spec_references` passes: parses migrated `brief-domain-types` (refs 1-4), inserts a NEW scenario carrying `spec-ref: 5` between scenarios 2 and 3 (a position shift), re-parses through the real `parse_specs`, and asserts the pre-existing refs 1-4 are unchanged and ref 3 resolves to the same scenario despite moving later in the file. Confirmed against the fixture: each scenario carries an explicit `<!-- spec-ref: N -->` token, so the reference is the token, not the position. |
| 3 | [execution] A test shows a migrated position-numbered capability yields stable Spec References that resolve to the same scenarios as before. | PASS | `migrated_tokens_resolve_to_the_same_scenarios_as_positional_refs` passes: for each archived positional reference (brief-domain-types/1-4, domain-type-boundary/1, type-name-audit/1-2) it asserts the migrated token N sits under the SAME scenario heading the Nth archived scenario had, and that every acceptance entry links to the archived task. I independently confirmed the archived positional order by reading the read-only archived specs (awk listing): brief-domain-types positions 1-4 = produces / consumes / no-domain-types / separate-from-Key-Terms; domain-type-boundary position 1 = "Domain type boundary in design template"; type-name-audit positions 1,2 = "All Type Catalog names present" / "Type name drift detected". These match the test's `expected_titles` exactly, so the migrated stable tokens resolve to the same scenarios the positional references named. |

## Failed Checks (if any)

None.

## Recommendations (if FAIL)

N/A.

## Notes

- The render emits 3 advisory stderr warnings (Tasks 5, 7, 8 exceed the 20%
  `[inspection]` cap). These come from the STANDING closing-task fragment, are
  non-fatal (exit code 0, full contract still rendered), and are outside Task 6's
  scope (the fragment is shared, not authored for this fixture). They do not
  affect any contract check.
