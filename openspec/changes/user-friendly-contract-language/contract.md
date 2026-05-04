# Contract

## Task 1: Rename scrutiny validator steps from "check" to "gate"
<!-- Specs: validator-gate-rename/2, validator-gate-rename/5, validator-gate-rename/6 -->
- [ ] [output] `grep -n "check [0-9]" skills/mission/prompts/scrutiny-validator.md` returns zero results -- all numbered validator steps use "gate N" instead
- [ ] [output] The scrutiny validator prompt report template labels each step as "Gate N" (e.g., "Gate 1: Test presence", "Gate 10: Contract checks")
- [ ] [output] `grep -n "check [0-9]" skills/mission/SKILL.md` returns zero results -- all validator step references in SKILL.md use "gate"

## Task 2: Rename contract "assertions" to "checks" in skills/mission/SKILL.md
<!-- Specs: contract-check-rename/3, contract-check-rename/7 -->
- [ ] [output] `grep -ni "assertion" skills/mission/SKILL.md` returns only code-test-context hits (e.g., "test assertions" in test presence gate description) -- no contract-item uses of "assertion" remain
- [ ] [output] The contract rules section says "Group checks by task" (not "Group assertions by task")
- [ ] [output] The contract rules section says "Write the minimum set of checks needed to verify the task" (not "assertions")
- [ ] [output] The launch confirmation section references "total contract checks" and "<N> tasks, <M> checks in contract"
- [ ] [output] The testability pre-flight section uses "Inspection checks" and "Executable checks" (not "Inspection assertions" or "Executable assertions")
- [ ] [output] The contract review guidance says "the check count feels right" (not "assertion count")

## Task 3: Rename contract "assertions" to "checks" in remaining living source files
<!-- Specs: contract-check-rename/4, contract-check-rename/5, contract-check-rename/6, contract-check-rename/8, contract-check-rename/9, contract-check-rename/10 -->
- [ ] [output] `skills/mission/prompts/scrutiny-validator.md` uses "Contract Checks for This Task" as the section heading (not "Contract Assertions")
- [ ] [output] `skills/mission/prompts/scrutiny-validator.md` report template uses "Contract Checks" table heading and "Check" column header, with a "Failed Checks" section
- [ ] [output] `skills/mission/prompts/usertest-validator.md` uses "Contract Checks for This Task" as the section heading, "Check" column header, and "Failed Checks" section
- [ ] [output] `skills/mission/prompts/adversarial-reviewer.md` uses "checks" where it previously said "assertions" for contract items
- [ ] [output] `skills/mission/templates/contract.md` template items say "check" (e.g., `[execution] <check capturing specific spec scenario detail>`)
- [ ] [output] `skills/mission/templates/pr-body.md` says "Total checks: <M>" (not "Total assertions")
- [ ] [output] `skills/mission/templates/plan.md` uses "contract checks" for validator step descriptions (not "contract assertions")
- [ ] [output] `skills/mission/templates/worker-fix-brief.md` says "Do NOT copy check text from the contract" (not "assertion text")
- [ ] [output] `skills/mission-resume/SKILL.md` describes contract contents as "checks grouped by task" (not "assertions")
- [ ] [output] `docs/provenance.md`, `docs/design-decisions.md`, and `docs/reference_factory_talk.md` use "checks" for contract items
- [ ] [output] `README.md` uses "checks" for contract item references (not "assertions")
- [ ] [output] `grep -rni "assertion" skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md` returns only code-test-context hits or zero hits

## Task 4: Smoke test -- verify terminology consistency
<!-- Specs: contract-check-rename/3-15, validator-gate-rename/2-6 -->
- [ ] [execution] `grep -rni "assertion" skills/mission/SKILL.md skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md` -- every hit is in code-test context (tautological assertion, test assertions, assertion quality). UBIQUITOUS_LANGUAGE.md is excluded because it intentionally contains "Assertion (in contract context)" as an alias-to-avoid and resolved ambiguity text.
- [ ] [execution] `grep -rn "check [0-9]" skills/mission/prompts/scrutiny-validator.md` returns zero hits
- [ ] [inspection] `skills/test-quality/SKILL.md` contains "assertion quality" and "tautological assertion" unchanged from before the mission
- [ ] [inspection] `skills/tdd/tests.md` contains "one logical assertion per test" unchanged
- [ ] [inspection] No awkward phrasings like "check each check" exist in any modified file -- verb forms use "verify" or "mark" instead

## Task 5: Cleanup
<!-- Specs: contract-check-rename/3-10 -->
- [ ] [inspection] No inconsistent capitalization (e.g., "Check" mid-sentence where "check" is appropriate, or vice versa)
- [ ] [inspection] No orphaned cross-references to old terminology
- [ ] [output] `grep -rni "assertion" skills/mission/SKILL.md skills/mission/prompts/ skills/mission/templates/ skills/mission-resume/ docs/ README.md` returns only code-test-context hits (re-verified after cleanup)

## Task 6: Module shape re-check
<!-- Specs: contract-check-rename/1-2, validator-gate-rename/1, contract-check-rename/11 -->
- [ ] [output] `mission/validations/module-shape-audit.md` exists and documents the comparison of design.md Type Catalog entries (Check, Gate, Contract) against UBIQUITOUS_LANGUAGE.md
- [ ] [inspection] Each Type Catalog entry (Check, Gate, Contract) has a matching definition in UBIQUITOUS_LANGUAGE.md

## Task 7: CLAUDE.md + ARCH.md generation
<!-- No spec scenarios -- mandatory closing task -->
- [ ] [output] If CLAUDE.md did not previously exist, it now exists at the project root with accurate content reflecting the built codebase
- [ ] [output] If ARCH.md did not previously exist, it now exists at the project root with architecture decisions from design.md
- [ ] [inspection] If CLAUDE.md or ARCH.md already existed, they were not modified

## Task 8: Documentation update
<!-- Specs: contract-check-rename/9, contract-check-rename/10 -->
- [ ] [output] README.md accurately describes the current terminology with no references to "assertions" in contract-item context
- [ ] [output] All documentation files use UBIQUITOUS_LANGUAGE.md terms consistently
