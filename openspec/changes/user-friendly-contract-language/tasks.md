## 1. Validator Gate Rename

- [x] Task 1: Rename scrutiny validator steps from "check" to "gate"
  Depends on: none
  Consumes: current scrutiny-validator.md, SKILL.md validator step references
  Produces: updated scrutiny-validator.md with "gate N" numbering, updated SKILL.md validator step references
  Layers: scrutiny-prompt, mission-skill

  Rename all numbered validator step references from "check N" to "gate N" in `skills/mission/prompts/scrutiny-validator.md`. Update any references to validator steps in `skills/mission/SKILL.md` that use "check" in the validator-step sense (e.g., "check 10: contract assertions" becomes "gate 10: contract checks"). Do NOT rename "assertion" to "check" in this task -- only rename validator step "check" to "gate". Verify: `grep -n "check [0-9]" skills/mission/prompts/scrutiny-validator.md` returns zero hits.

## 2. Contract Check Rename -- Mission Skill

- [x] Task 2: Rename contract "assertions" to "checks" in skills/mission/SKILL.md
  Depends on: Task 1
  Consumes: updated SKILL.md from Task 1 (validator step references already say "gate")
  Produces: updated SKILL.md with "check" for all contract items
  Layers: mission-skill

  Rename all contract-item uses of "assertion/assertions" to "check/checks" in `skills/mission/SKILL.md`. This is the bulk of the work (~50+ occurrences). Preserve "assertion" where it refers to code test assertions (e.g., "test assertions" in the test presence gate description). Verify: `grep -ni "assertion" skills/mission/SKILL.md` returns only code-test-context hits.

## 3. Contract Check Rename -- Prompts, Templates, Docs

- [x] Task 3: Rename contract "assertions" to "checks" in remaining living source files
  Depends on: Task 1
  Consumes: current validator prompts, templates, docs, README
  Produces: updated prompts, templates, docs, README with "check" for contract items
  Layers: scrutiny-prompt, usertest-prompt, adversarial-prompt, templates, docs

  Rename all contract-item uses of "assertion/assertions" to "check/checks" in these files:
  - `skills/mission/prompts/scrutiny-validator.md` (contract sections, not gate numbering)
  - `skills/mission/prompts/usertest-validator.md`
  - `skills/mission/prompts/adversarial-reviewer.md`
  - `skills/mission/templates/contract.md`
  - `skills/mission/templates/pr-body.md`
  - `skills/mission/templates/plan.md`
  - `skills/mission/templates/worker-fix-brief.md`
  - `skills/mission-resume/SKILL.md`
  - `docs/provenance.md`
  - `docs/design-decisions.md`
  - `docs/reference_factory_talk.md`
  - `README.md`
  Do NOT modify `skills/test-quality/SKILL.md`, `skills/tdd/tests.md`, or `skills/prd/SKILL.md`. Verify: `grep -rni "assertion" <each modified file>` returns only code-test-context hits or zero hits.

## 4. Closing Tasks

- [x] Task 4: Smoke test -- verify terminology consistency across all in-scope files
  Depends on: Task 2, Task 3
  Consumes: all modified files from Tasks 1-3
  Produces: grep verification report confirming no stale "assertion" in contract context, no stale "check N" in validator step context
  Layers: glossary, mission-skill, scrutiny-prompt, usertest-prompt, adversarial-prompt, templates, docs

  Run comprehensive grep verification across all in-scope files:
  1. `grep -rni "assertion" skills/mission/SKILL.md skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md UBIQUITOUS_LANGUAGE.md` -- every hit must be code-test context
  2. `grep -rn "check [0-9]" skills/mission/prompts/scrutiny-validator.md` -- must return zero hits
  3. Verify preserved files are unchanged: `skills/test-quality/SKILL.md`, `skills/tdd/tests.md`, `skills/prd/SKILL.md`
  4. Read through each modified file to check for awkward phrasings introduced by the rename

- [x] Task 5: Cleanup -- remove dead references and leftover inconsistencies
  Depends on: Task 4
  Consumes: all files from Tasks 1-3
  Produces: clean codebase with no debris from the rename
  Layers: all in-scope

  Review all modified files for: unused backward-compatibility shims, inconsistent capitalization (Check vs check), orphaned cross-references, and any "assertion" that slipped through in a non-code-test context.

- [x] Task 6: Module shape re-check
  Depends on: Task 5
  Consumes: design.md Type Catalog and Module Inventory, all committed code
  Produces: mission/validations/module-shape-audit.md
  Layers: all in-scope

  Compare design.md module descriptions and Type Catalog entries (Check, Gate, Contract) against what the committed files actually contain. Verify that each Type Catalog term appears in UBIQUITOUS_LANGUAGE.md with a matching definition. Write findings to mission/validations/module-shape-audit.md.

- [x] Task 7: CLAUDE.md + ARCH.md generation
  Depends on: Task 5
  Consumes: all committed changes, design.md
  Produces: CLAUDE.md and/or ARCH.md if they don't already exist
  Layers: docs

  If the project root does not have a CLAUDE.md, create one using the built codebase. If CLAUDE.md already exists, do not modify it. If no ARCH.md exists, create one from design.md. If ARCH.md already exists, do not modify it.

- [x] Task 8: Documentation update
  Depends on: Task 7
  Consumes: all committed changes, CLAUDE.md and ARCH.md (if newly created)
  Produces: updated README.md reflecting the terminology change
  Layers: docs

  Verify README.md accurately describes the current terminology. Update any documentation that references the old "assertion" terminology for contract items. Ensure UBIQUITOUS_LANGUAGE.md terms are used consistently in all docs. Verify any newly generated CLAUDE.md or ARCH.md also use correct terminology.
