# Adversarial Review B (Claude)

**Base SHA:** 01139a17d0a23e4e15d49cafa9b7286e6dc8261d
**Commits reviewed:** 7 (29bc8ac through 1fb7edf)
**Reviewer:** Claude Opus 4.6

---

## Per-Commit Findings

### 29bc8ac [task-1.1] Add design template, config rules, and architecture scoring gate

No issues with this commit in isolation. Design template has all five sections, scoring rubric has all four dimensions with four levels each, config.yaml has design rules, SKILL.md references the template.

### c152c84 [task-2.1] Update contract template and Phase 5 to use spec references

Clean. Contract template uses `<!-- Specs: capability/N -->` format. Phase 5 language consistently updated. No residual "PRD Requirements" in contract.md or Phase 5.

### ab5d9f9 [task-3.1] Add deterministic anchors to plan template and Phase 4.3

See cross-file findings F-02 and F-03 below.

### 0195f82 [task-4.1] Add Phase 4.3a, task topology, and strategy-aware quality standard

Phase 4.3a is correctly inserted. Topology format is specified. Strategy-aware rules are consistent between SKILL.md and config.yaml. The >2 layers flag correctly applies only for layered/hybrid.

### 303aa20 [task-5.1] Add CLAUDE.md template and closing task

Template has 8 sections. Include/exclude table present. Example with "adapt to language" note present. "Update ARCH.md when you change architecture" rule present. Create-only-if-absent instruction correct. Closing task ordering correct (smoke, cleanup, shape, CLAUDE.md, docs, CI).

### 796084a [task-6.1] Add foundation audit after task 1 in Phase 8

Foundation audit correctly placed in Step 7 sub-step 4a, after both validators pass. Extracts from Produces field. Compares against Module Inventory and Type Catalog. Drift triggers corrective task 1a. No-drift logs "Foundation audit PASSED." Explicitly scoped to task 1 only.

### 1fb7edf [task-10.1] Update documentation for deterministic planning changes

See cross-file findings F-01 and F-04.

---

## Cross-Task Findings

### F-01: CLAUDE.md darkfactory-dispatch section removed without justification [MEDIUM]

**Files:** `CLAUDE.md`
**Commit:** 1fb7edf

The documentation commit removed the `## darkfactory-dispatch` section from the project CLAUDE.md. This section described the `darkfactory-dispatch` tool and was not related to the deterministic planning changes. The tool still exists in the project, is still referenced in SKILL.md's dispatch scripts, and is still documented in UBIQUITOUS_LANGUAGE.md. Removing it silently drops useful documentation.

Additionally, a blank line between the heading and the first paragraph was removed, which is a cosmetic change unrelated to the mission scope.

**Fix:** Restore the `## darkfactory-dispatch` section and the blank line after the heading.

### F-02: Worker brief template does not include Deterministic Anchors section [MEDIUM]

**Files:** `skills/mission/templates/plan.md` (line 31), `skills/mission/templates/worker-brief.md`
**Commit:** ab5d9f9

The plan template says: "Every worker brief references these [Deterministic Anchors]. They survive context compaction." But the worker-brief.md template has no "Deterministic Anchors" section. The brief has an "Architecture" section described as "relevant excerpt from design.md" but no explicit instruction to include the anchors.

This means the claim in the plan is unenforceable. Either the plan promise is aspirational (and should say "may reference" instead of "references"), or the worker-brief.md template needs a Deterministic Anchors section.

**Impact:** If a worker does not receive the anchors, it might use different names for types, modules, or patterns than what the design decided, which is the exact problem the anchors were designed to prevent.

**Fix:** Add a "Deterministic Anchors" section to worker-brief.md, or update the plan template text to accurately describe what happens.

### F-03: "Test strategy" anchor has no source section in the design template [LOW]

**Files:** `skills/mission/SKILL.md` (line 198), `skills/mission/templates/design-template.md`
**Commit:** ab5d9f9

SKILL.md Phase 4.3 says: "Source values from design.md's Architecture Decision, Type Catalog, and Error Strategy sections." The six anchors include "test strategy" and "workspace layout." But the design template has no Testing section. Neither "test strategy" nor "workspace layout" maps to a named section heading.

"Workspace layout" can plausibly be extracted from the Architecture Decision narrative. "Test strategy" has no obvious home -- the Error Strategy section covers errors, not testing approach.

**Impact:** An orchestrator following instructions literally would struggle to find the test strategy source in the three named sections. It would likely improvise, defeating determinism.

**Fix:** Either add a "Test Strategy" section to the design template (making it six mandatory sections, which would require updating config.yaml and all section-count references), or change the anchor source instruction to name the actual section where test strategy information would appear, or acknowledge that test strategy comes from the specs/PRD rather than design.md.

### F-04: README line 86 still says contract traces to PRD requirement [LOW]

**Files:** `README.md` (line 86)
**Commit:** 1fb7edf

Line 84 was correctly added: "Contract assertions trace back to specific spec capability and scenario identifiers for end-to-end traceability." But two lines later, line 86 still says: "Each item traces back to a PRD requirement." These two sentences contradict each other.

**Fix:** Update line 86 to say "Each item traces back to a specific spec scenario" or similar.

### F-05: Phase 4 gate does not include Phase 4.3a approval checkpoint [LOW]

**Files:** `skills/mission/SKILL.md` (lines 270-275)
**Commit:** 0195f82

The Phase 4 gate checklist has items for proposal, specs, design, and tasks approval. Phase 4.3a (task strategy) is a new human-approval step inserted between design and tasks, but the gate does not include a `- [ ] Task strategy human-approved (Phase 4.3a)` item.

In practice this is harmless because the orchestrator would not reach Phase 4.4 without passing through 4.3a. But the gate is the canonical checklist for what must be true before proceeding to Phase 4a. A resuming orchestrator reading the gate could skip 4.3a if it only checks the gate items.

**Fix:** Add `- [ ] Task strategy human-approved (Phase 4.3a)` to the Phase 4 gate.

### F-06: Coverage audit template lacks spec capability/scenario ID column [LOW]

**Files:** `skills/mission/templates/coverage-audit.md`, `skills/mission/SKILL.md` (lines 356-360)
**Commit:** c152c84

Phase 5 instructs: "For each task, use the coverage audit from Phase 4a to find which spec capability/scenario identifiers map to it." But the coverage audit template's "Spec Requirement" column contains WHEN/THEN text, not capability/scenario identifiers (e.g., `add-todo/1`). The orchestrator must derive the ID from the spec file path and scenario position, which is fragile and undocumented.

This is not a blocking issue -- the orchestrator can derive the ID from context. But the traceability chain has an implicit step where a human-readable WHEN/THEN gets translated into a structured capability/N reference.

**Fix:** Add a "Spec ID" column to the coverage audit template (e.g., `add-todo/1`) or document how the orchestrator derives the ID.

### F-07: Phase 4a still uses "PRD requirement" language throughout [INFORMATIONAL]

**Files:** `skills/mission/SKILL.md` (lines 30, 282, 286, 327, 485)

Phase 4a (Spec Coverage Audit) consistently uses "PRD requirement" language: "Verify every PRD requirement maps to at least one task." This is the intended design -- the coverage audit works at the PRD level, while the contract works at the spec level. But the coexistence of two traceability levels (PRD requirement -> task in the audit, spec scenario -> assertion in the contract) is not explicitly documented anywhere. A reader might wonder whether "PRD requirement" references in Phase 4a are stale or intentional.

No fix needed, but a one-line clarification in Phase 5 noting the two-level traceability model would help.

---

## Contract Assertion Verification

### Task 1.1: Design Template and Architecture Scoring

| Assertion | Status | Notes |
|-----------|--------|-------|
| design-template.md exists with five mandatory sections | PASS | |
| Architecture Decision has four-dimension scoring rubric | PASS | |
| Each dimension has criteria for levels 0-3 | PASS | |
| Module Inventory table format correct | PASS | |
| Type Catalog specifies name, purpose, owning module | PASS | |
| config.yaml has rules.design with five sections | PASS | |
| SKILL.md Phase 4.3 references templates/design-template.md | PASS | |
| SKILL.md Phase 4.3 contains five-section gate | PASS | |
| Hard floor (no 0 on any dimension) | PASS | |
| Tied candidates presented to human | PASS | |
| Greenfield vs existing-codebase distinction | PASS | |
| grep design-template.md in SKILL.md >= 1 | PASS | 2 matches |
| grep rules: and design: in config.yaml >= 1 | PASS | |

### Task 2.1: Spec Traceability in Contract

| Assertion | Status | Notes |
|-----------|--------|-------|
| contract.md uses `<!-- Specs: capability/N -->` | PASS | |
| SKILL.md Phase 5 directs tracing to spec capability/scenario | PASS | |
| No "PRD Requirements" in contract template | PASS | |
| grep PRD Requirements returns 0 in contract.md | PASS | |
| grep Specs: returns >= 1 | PASS | |

### Task 3.1: Deterministic Anchors

| Assertion | Status | Notes |
|-----------|--------|-------|
| plan.md has Deterministic Anchors section with six fields | PASS | |
| Each anchor is one-liner pointer with source reference | PASS | |
| SKILL.md Phase 4.3 instructs populating 5 anchors after design | PASS | |
| grep Deterministic Anchors in plan.md >= 1 | PASS | |

### Task 4.1: Task Strategy, Topology, and Quality Standard

| Assertion | Status | Notes |
|-----------|--------|-------|
| Phase 4.3a exists between 4.3 and 4.4 | PASS | |
| Phase 4.3a requires decomposition approach, target count, dependency rationale | PASS | |
| Phase 4.3a requires human approval | PASS | |
| Phase 4.3a populates task strategy anchor | PASS | |
| Phase 4.4 has Task Topology with Depends on, Consumes, Produces, Layers | PASS | |
| Phase 4.4 has fixed markdown format | PASS | |
| >2 layers flag applies only for layered/hybrid | PASS | |
| Task quality standard is strategy-aware | PASS | |
| config.yaml rules.tasks is strategy-aware | PASS | |
| Phase Index includes 4.3a | PASS | |
| grep 4.3a returns >= 2 | PASS | 4 matches |
| grep Depends on: returns >= 1 | PASS | |

### Task 5.1: CLAUDE.md Closing Task and Template

| Assertion | Status | Notes |
|-----------|--------|-------|
| templates/claude-md.md exists with include/exclude table and 8 sections | PASS | |
| Template has one example with "adapt to language" note | PASS | |
| Rules includes "update ARCH.md when you change architecture" | PASS | |
| Closing tasks ordered: smoke, cleanup, shape, CLAUDE.md, docs, CI | PASS | |
| CLAUDE.md task creates only if absent | PASS | |
| grep claude-md.md in SKILL.md >= 1 | PASS | |

### Task 6.1: Foundation Audit

| Assertion | Status | Notes |
|-----------|--------|-------|
| Phase 8 Step 7 has foundation audit after task 1 only | PASS | |
| Audit extracts from Produces field | PASS | |
| Compares against Module Inventory and Type Catalog | PASS | |
| Drift triggers corrective task 1a | PASS | |
| No drift logs "Foundation audit PASSED" | PASS | |
| Does not run for other tasks | PASS | |
| grep foundation audit >= 1 | PASS | 2 matches |

---

## Severity Summary

| Severity | Count | IDs |
|----------|-------|-----|
| HIGH | 0 | -- |
| MEDIUM | 2 | F-01, F-02 |
| LOW | 3 | F-03, F-04, F-05 |
| INFORMATIONAL | 1 | F-07 |

No findings involve a gap in coverage audit traceability (F-06 is low because it's a convenience issue, not a correctness issue).

---

## Shipping Recommendation

**Ship with fixes for F-01 and F-02.**

F-01 (darkfactory-dispatch removal) is an out-of-scope destructive change that should be reverted. F-02 (worker brief missing anchors) is a promise-vs-reality mismatch that could undermine the determinism goals this mission is designed to achieve.

The low-severity findings (F-03, F-04, F-05) are worth addressing but are not blocking. F-03 (test strategy source) and F-05 (Phase 4 gate missing 4.3a) are the most actionable of the three.

All 45 contract assertions pass. The phase flow (4.3 -> 4.3a -> 4.4 -> 4a -> 5) is correctly wired. Terminology is consistent with UBIQUITOUS_LANGUAGE.md. No logic errors in the instruction text.
