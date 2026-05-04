# Worker Brief: Task 2 - Rename contract "assertions" to "checks" in skills/mission/SKILL.md

## Your Task

Rename all contract-item uses of "assertion/assertions" to "check/checks" in `skills/mission/SKILL.md`. This is the bulk of the rename (~50+ occurrences).

**Rules:**
- "assertion" referring to a contract item becomes "check" (lowercase) or "Check" (matching original capitalization)
- "assertions" referring to contract items becomes "checks"
- "Assertion" at the start of a sentence or in a title becomes "Check"
- Preserve "assertion" where it refers to code test assertions. Specifically:
  - "test assertions" in the test presence gate description stays as-is
  - "tautological assertion" references stay as-is
  - Any reference to code-level test assertions (assert_eq!, expect().toBe()) stays as-is

**Specific replacements to make (non-exhaustive, find all):**
- "validation assertions" -> "validation checks"
- "contract assertions" -> "contract checks"  
- "Write the minimum set of assertions" -> "Write the minimum set of checks"
- "Group assertions by task" -> "Group checks by task"
- "Assertions should describe observable behavior" -> "Checks should describe observable behavior"
- "assertion count" -> "check count"
- "inspection assertions" -> "inspection checks"
- "executable assertions" -> "executable checks"
- "output assertions" -> "output checks"
- "assertions in contract" -> "checks in contract"
- "contract assertion text" -> "contract check text"
- "assertion text" (when referring to contract) -> "check text"
- "SKIP'd assertion" -> "SKIP'd check"
- "assertion is demonstrably true/false" -> "check is demonstrably true/false" (if present)
- "Add assertions for it to contract.md" -> "Add checks for it to contract.md"
- "Write assertions" -> "Write checks" (contract context)
- "each assertion" -> "each check" (contract context)
- "per-task validator" phrasing referencing assertions -> referencing checks
- "assertion" in "Do NOT copy contract assertion text" -> "Do NOT copy contract check text"

**What NOT to rename:**
- "test assertions" in the test presence gate (gate 1) description -- this refers to code test assertions like assert_eq!
- Any occurrence of "assertion" that clearly refers to code-level test assertions
- Do not change "gate" references -- those were already renamed in Task 1
- Do not touch any file other than `skills/mission/SKILL.md`

**Verification:** `grep -ni "assertion" skills/mission/SKILL.md` should return only code-test-context hits (e.g., "test assertions" in the test presence gate description).

## Project Context

Darkfactory is renaming user-facing mission contract language from "assertions" to "checks" to make it friendlier to non-developers. Validator steps were already renamed from "checks" to "gates" in Task 1 to avoid collision.

## Architecture

`skills/mission/SKILL.md` is the orchestrator's operating manual (~900 lines). It defines all mission phases: planning, contract writing, build loop, validation, and completion. Contract items are referenced throughout -- in contract rules (Phase 5), launch confirmation (Phase 7), build loop (Phase 8), and completion (Phase 9).

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Check**: A single verifiable statement in a contract that a validator can verify from outside the code (replaces "assertion" in contract context)
- **Gate**: A numbered step in the scrutiny validator's sequence (already renamed from "check" in Task 1)
- **Contract**: A set of checks written during planning that define "done" independently of implementation
- **Tautological assertion**: A test assertion that passes regardless of whether the code works correctly (code test term, NOT renamed)

## Domain Types

- Check: A single verifiable statement in a contract (owning module: glossary)
- Gate: A numbered step in the scrutiny validator's sequence (owning module: glossary)
- Contract: A set of checks grouped by task (owning module: glossary)

## What Previous Tasks Built

### Task 1: Rename scrutiny validator steps from "check" to "gate"
- Modified: skills/mission/prompts/scrutiny-validator.md, skills/mission/SKILL.md
- Section headers renamed: "Check Sequence" -> "Gate Sequence", "Check Results" -> "Gate Results"
- Report template rows now prefixed with "Gate N:" (Gate 1 through Gate 14)
- "Code standards check" renamed to "Code standards gate"
- SKILL.md updated "11 checks" to "14 gates" with full gate list
- "check [0-9]" pattern eliminated from both files

## Downstream Consumers

Task 3 will rename "assertions" to "checks" in the remaining files (prompts, templates, docs). It depends on Task 1 (not Task 2), so it operates independently on different files.

## Procedures
- Read skills/mission/SKILL.md thoroughly before making changes
- Make all changes in a single commit with message: "[task-2] Rename contract assertions to checks in mission SKILL.md"
- Write your handoff report to openspec/changes/user-friendly-contract-language/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2 - Rename contract "assertions" to "checks" in SKILL.md
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
