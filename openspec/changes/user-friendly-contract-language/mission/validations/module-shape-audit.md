# Module Shape Audit

## Type Catalog Verification

### Check
- **UBIQUITOUS_LANGUAGE.md:** Present in "Planning and contracts" section. Definition: "A single verifiable statement in a contract that a validator can verify from outside the code." Aliases to avoid: "Assertion (in contract context), test case, requirement."
- **Type Catalog (design.md):** "A single verifiable statement in a contract" (owning module: glossary).
- **Usage in code:** Used consistently in SKILL.md (contract rules, Phase 5), contract.md template, usertest-validator.md, adversarial-reviewer.md, plan.md template, docs/provenance.md, docs/design-decisions.md. No stale "assertion" found in contract context.
- **Drift:** None.

### Gate
- **UBIQUITOUS_LANGUAGE.md:** Present in "Validation" section. Definition: "A numbered step in the scrutiny validator's sequence (e.g., gate 3: tests, gate 10: contract checks). Gates report PASS, FAIL, WARN, SKIP, or N/A." Aliases to avoid: "Check (in validator-step context), step, stage."
- **Type Catalog (design.md):** "A numbered step in the scrutiny validator's sequence" (owning module: glossary).
- **Usage in code:** Used consistently in scrutiny-validator.md (14 gates labeled "Gate 1" through "Gate 14" in the results table, "gate sequence" heading). No stale "check N" pattern found in scrutiny validator. SKILL.md uses "gate" correctly for validator steps (e.g., "final gates").
- **Drift:** None.

### Contract
- **UBIQUITOUS_LANGUAGE.md:** Present in "Planning and contracts" section. Definition: "A set of checks written during planning that define 'done' independently of implementation, consumed only by validators." Aliases to avoid: "Test plan, acceptance criteria, spec."
- **Type Catalog (design.md):** "A set of checks grouped by task, consumed by validators" (owning module: glossary).
- **Usage in code:** Used consistently across all modules -- SKILL.md (Phase 5 "Write Contract", contract rules, adversarial separation), all three prompt files, plan.md template, docs/provenance.md, docs/design-decisions.md. No stale terminology found.
- **Drift:** None.

## Module Inventory Verification

### mission-skill (skills/mission/SKILL.md)
- **Exists:** Yes.
- **Uses new terminology:** Yes. "Check" for contract items, "gate" for validator steps, "contract" throughout. No stale "assertion" in contract context. Grep for "check [0-9]" in scrutiny context returns zero hits.
- **Drift:** None.

### scrutiny-prompt (skills/mission/prompts/scrutiny-validator.md)
- **Exists:** Yes.
- **Uses new terminology:** Yes. Gates labeled "Gate 1" through "Gate 14". "Gate sequence" heading. "Contract checks" at gate 10. No stale "check N" for validator steps.
- **Drift:** None.

### usertest-prompt (skills/mission/prompts/usertest-validator.md)
- **Exists:** Yes.
- **Uses new terminology:** Yes. "Contract check" used consistently. Table header uses "Check" column. No stale terminology.
- **Drift:** None.

### adversarial-prompt (skills/mission/prompts/adversarial-reviewer.md)
- **Exists:** Yes.
- **Uses new terminology:** Yes. "Contract checks" used throughout. "Validation Contract" heading. No stale terminology.
- **Drift:** None.

### templates (skills/mission/templates/)
- **Exists:** Yes.
- **Expected files:** contract.md, pr-body.md, plan.md, worker-fix-brief.md. All present.
- **Additional files present:** claude-md.md, coverage-audit.md, design-template.md, worker-brief.md. These are expected (design.md lists "5 template files" and the worker-brief includes the Domain Types section added by this mission's task 1).
- **Uses new terminology:** Yes. contract.md uses "check" in its template lines. plan.md references "contract checks". worker-fix-brief.md references "check" correctly. worker-brief.md includes Domain Types section.
- **Drift:** None.

### glossary (UBIQUITOUS_LANGUAGE.md)
- **Exists:** Yes.
- **Updated definitions:** Check, Gate, and Contract all present with correct definitions. Resolved ambiguities section documents the three-way split (gate/check/assertion).
- **Drift:** None.

### docs (docs/)
- **Exists:** Yes.
- **Expected files:** provenance.md, design-decisions.md, reference_factory_talk.md. All present.
- **Additional files present:** architecture.png, darkfactory-header.jpeg, reference_pocock_talk.md. These are pre-existing and not in scope.
- **Uses new terminology:** Yes. provenance.md uses "checks" for contract items. design-decisions.md uses "gate" for validator steps and "checks" for contract items. reference_factory_talk.md uses "validation contract" and "checks".
- **Drift:** None.

## Summary

No drift found between design.md and the committed files. All three Type Catalog entries (Check, Gate, Contract) are defined in UBIQUITOUS_LANGUAGE.md with matching definitions and used consistently across all modules. No stale "assertion" usage in contract context. No stale "check N" pattern in scrutiny validator steps.
