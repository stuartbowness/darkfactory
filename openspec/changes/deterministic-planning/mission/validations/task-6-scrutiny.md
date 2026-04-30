# Scrutiny Validation: Task 6

**Commit:** 796084ab83f0fd345664a7b6a36bdf9f32a806f3
**Validator:** scrutiny (manual contract assertion review)
**Result:** PASS

## Contract Assertions

- [x] **[output] SKILL.md Phase 8 Step 7 contains a foundation audit step that runs only after task 1 passes both validators.**
  PASS. The new step 4a is inserted inside "Step 7: Track Progress" (line 698). It opens with: "If this is task 1 and it passed both validators, run a foundation check before proceeding to task 2." This is within Phase 8's build loop. Location confirmed: between sub-step 4 ("Update the plan") and sub-step 5 ("If more tasks remain").

- [x] **[output] The foundation audit extracts module and type names from task 1's Produces topology field.**
  PASS. Line 700: "Read task 1's Produces field from its topology metadata in tasks.md". Lines 701-702 then extract module names and type names from that field. The field name "Produces" matches the topology metadata format defined at line 236: `Produces: <surfaces this task creates -- types, modules, interfaces, endpoints>`.

- [x] **[output] The audit compares extracted module names against design.md's Module Inventory.**
  PASS. Line 701: "Extract module names from the Produces field. Look up those modules in design.md's Module Inventory." "Module Inventory" is a required section heading in design.md (confirmed at line 172).

- [x] **[output] The audit compares extracted type names against design.md's Type Catalog.**
  PASS. Line 702: "Extract type names from the Produces field. Look up those types in design.md's Type Catalog." "Type Catalog" is a required section heading in design.md (confirmed at line 172).

- [x] **[output] When drift is detected, the audit generates a corrective task 1a: inserts it in tasks.md, adds assertions to contract.md, and presents both to the human for approval.**
  PASS. Lines 704-709 specify all three steps in the corrective flow:
  1. "Generate a corrective task 1a" (line 705)
  2. "Insert it in tasks.md between task 1 and task 2" (line 706)
  3. "Add assertions for it to contract.md" (line 707)
  4. "Present both to the human for approval" (line 708)
  All three required actions (insert task, add assertions, present to human) are present. An additional fourth bullet ("The corrective task goes through the normal build loop before task 2 starts") ensures the corrective task is actually executed.

- [x] **[output] When no drift is detected, the audit logs "Foundation audit PASSED" to progress.md.**
  PASS. Line 711: "If no drift: log to progress.md: `<timestamp> -- Foundation audit PASSED. Module structure and type names confirmed for task 1 produces.`"

- [x] **[output] The audit explicitly does not run for any task other than task 1.**
  PASS. The "task 1 only" condition appears in two places:
  1. Opening sentence (line 698): "If this is task 1 and it passed both validators"
  2. Closing sentence (line 713): "This audit runs only after task 1. It does not run after any other task."

- [x] **[execution] `grep -c "foundation audit" skills/mission/SKILL.md` returns at least 1 (case-insensitive match acceptable).**
  PASS. Case-insensitive grep returns 2 matches: one at the step header ("Foundation audit (after task 1 only)") and one at the pass log ("Foundation audit PASSED").

## Summary

All 8 contract assertions pass. The commit adds 17 lines to `skills/mission/SKILL.md` as sub-step 4a within Phase 8's Step 7. The foundation audit is correctly scoped to task 1 only, references the correct field name ("Produces" from topology metadata), compares against the correct design.md sections ("Module Inventory" and "Type Catalog"), and includes all three corrective-task actions (insert, add assertions, present to human) plus the no-drift logging path.

No issues found.
