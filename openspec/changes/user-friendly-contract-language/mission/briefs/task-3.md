# Worker Brief: Task 3 - Rename contract "assertions" to "checks" in remaining living source files

## Your Task

Rename all contract-item uses of "assertion/assertions" to "check/checks" in 12 files. This completes the terminology rename started in Tasks 1 and 2.

**Files to modify (all paths relative to repo root):**

1. `skills/mission/prompts/scrutiny-validator.md` -- rename contract sections only (gate numbering was already done in Task 1)
   - "Contract Assertions for This Task" -> "Contract Checks for This Task"
   - "Contract Assertions" table heading -> "Contract Checks"
   - "Failed Assertions" -> "Failed Checks"
   - "Assertion" column header -> "Check"
   - "Mark each assertion PASS, FAIL, or SKIP" -> "Mark each check PASS, FAIL, or SKIP"
   - "each assertion" -> "each check" (contract context)
   - "assertion" -> "check" in behavioral conformance descriptions (contract context)
   - Preserve "test assertions" in gate 1 test presence description (code test context)
   - Preserve "tautological assertions" in gate 14 test quality description (code test context)
2. `skills/mission/prompts/usertest-validator.md` -- same contract section renames
3. `skills/mission/prompts/adversarial-reviewer.md` -- "assertions" in contract context
4. `skills/mission/templates/contract.md` -- template items say "check"
5. `skills/mission/templates/pr-body.md` -- "Total checks: <M>"
6. `skills/mission/templates/plan.md` -- "contract checks" in validator descriptions
7. `skills/mission/templates/worker-fix-brief.md` -- "Do NOT copy check text"
8. `skills/mission-resume/SKILL.md` -- "checks grouped by task"
9. `docs/provenance.md` -- "checks" for contract items
10. `docs/design-decisions.md` -- "checks" for contract items
11. `docs/reference_factory_talk.md` -- "checks" for contract items
12. `README.md` -- "checks" for contract items

**Do NOT modify these files:**
- `skills/test-quality/SKILL.md` (code test assertions)
- `skills/tdd/tests.md` (code test assertions)
- `skills/prd/SKILL.md` (general English)
- `skills/mission/SKILL.md` (already done in Task 2)

**Preservation rules:**
- "test assertions" in scrutiny-validator.md gate 1 (test presence) -> keep as-is
- "tautological assertions" in scrutiny-validator.md gate 14 (test quality) -> keep as-is
- "assertion" used as a verb or general English -> keep as-is

**Verification:** `grep -rni "assertion" skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md` should return only code-test-context hits (tautological assertions, test assertions) or zero hits per file.

## Project Context

Darkfactory is renaming user-facing mission contract language from "assertions" to "checks". Validator steps were renamed to "gates" in Task 1. SKILL.md was renamed in Task 2. This task finishes the remaining files.

## Architecture

The files in this task serve different roles:
- **Validator prompts** (scrutiny, usertest, adversarial): consumed by Claude subagents during validation
- **Templates** (contract, pr-body, plan, worker-fix-brief): consumed by the orchestrator when generating artifacts
- **Mission-resume skill**: consumed by the orchestrator when resuming an interrupted mission
- **Docs** (provenance, design-decisions, reference_factory_talk): consumed by human developers
- **README.md**: consumed by human developers

## Key Terms (from UBIQUITOUS_LANGUAGE.md)

- **Check**: A single verifiable statement in a contract (replaces "assertion" in contract context)
- **Gate**: A numbered step in the scrutiny validator's sequence (already renamed in Task 1)
- **Tautological assertion**: A test assertion that passes regardless of correctness (code test term, NOT renamed)

## Domain Types

- Check: contract item (glossary)
- Gate: validator step (glossary)
- Contract: set of checks grouped by task (glossary)

## What Previous Tasks Built

### Task 1: Rename scrutiny validator steps from "check" to "gate"
- Modified: skills/mission/prompts/scrutiny-validator.md, skills/mission/SKILL.md
- "Gate Sequence", "Gate Results" headers; "Gate N:" row labels; "14 gates" in SKILL.md

### Task 2: Rename contract "assertions" to "checks" in SKILL.md
- Modified: skills/mission/SKILL.md
- All contract-item "assertion/assertions" replaced with "check/checks"
- Zero "assertion" remaining in SKILL.md

## Downstream Consumers

Task 4 (smoke test) will verify all files are consistent after this task completes.

## Procedures
- Read each file before modifying it to understand context
- Make all changes across all 12 files in a single commit with message: "[task-3] Rename contract assertions to checks in prompts, templates, docs, and README"
- Write your handoff report to openspec/changes/user-friendly-contract-language/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Rename contract "assertions" to "checks" in remaining files
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
