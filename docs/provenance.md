# Provenance

How Darkfactory relates to its two original sources: Factory.ai's multi-agent mission
architecture and Matt Pocock's AI development skills. Reviewed against the current
implementation (SKILL.md) as of 2026-04-12.

---

## Factory.ai

Source: [docs/reference_factory_talk.md](reference_factory_talk.md)

### Faithfully implemented

| Concept | Factory description | Darkfactory implementation |
|---------|--------------------|----------------------------|
| Three-role architecture | Orchestrator, worker, validator as distinct roles | Orchestrator is the /mission session. Workers are dispatched subagents. Validators are separate subagents with isolated prompts. |
| Validation contract | "Defines done before coding starts, can be hundreds of checks" | contract.md (Phase 5), written after specs/tasks but before the build loop. Workers never see it. |
| Adversarial separation | Workers don't see the contract, validators don't see the plan | Stricter than Factory: workers never see contract.md, validators never see proposal.md or design.md. |
| Serial execution | One worker at a time to avoid conflicts | Rule 1: "One worker at a time. Never dispatch two workers in parallel." |
| Structured handoffs | Workers produce reports with what was done, commands, exit codes, issues | Handoff template: commit SHA, approach, commands run, concerns. |
| Two validator types | Scrutiny validator (tests, lint, review) and user-testing validator (spawns app, interacts) | Scrutiny (Phase 8 Step 4): build/test/lint/review/standards/simplify/security/checks/behavioral conformance. User-test (Phase 8 Step 6): HTTP tests or browser. |
| Multi-model strategy | Different LLMs for different roles | config.yaml: workers default to Codex (GPT-5.4), validators to Claude, adversarial reviewers use one of each. |
| Read-only parallelization | Only parallelize read-only operations | Only parallel dispatch is the two adversarial reviewers in Phase 9, which are read-only. |

### Implemented differently

| Concept | Factory description | Darkfactory difference |
|---------|--------------------|-----------------------|
| User-test validator | "Computer use" QA engine that spawns the app and interacts | Uses HTTP tests and browser automation (/test-app). Same concept, less sophisticated execution. |

### Not implemented

| Concept | Factory description | Why not |
|---------|--------------------|---------| 
| Negotiation pattern | Agents coordinating over shared resources | Darkfactory is purely orchestrator-mediated. No inter-agent coordination. Serial execution eliminates the need for resource negotiation. |

---

## Matt Pocock

Source: [docs/reference_pocock_talk.md](reference_pocock_talk.md)

### Faithfully implemented

| Concept | Pocock description | Darkfactory implementation |
|---------|-------------------|-----------------------------|
| Adversarial alignment interview | 40-100 questions to reach shared understanding before any work | Phase 1: /grill. Adds a safeguard Pocock didn't describe: warns if user tries to skip. |
| Ubiquitous language | DDD-inspired glossary with canonical terms, aliases to avoid, relationships | Phase 2: /language. Glossary threaded through briefs, documentation, and all phases. |
| TDD as speed limit | Force small steps -- write test, make pass, refactor. Prevents AI from outracing its headlights. | Worker briefs mandate /tdd. Scrutiny validator enforces with automatic FAIL if diff has zero new tests (test presence gate). |
| Deep modules | Wrap related code in deep modules. Large functionality behind simple interfaces. Test at the interface. | Phase 4.3 (design) requires module depth analysis: describe the interface, what it hides, why that boundary. Flags shallow modules. |

### Implemented differently

| Concept | Pocock description | Darkfactory difference |
|---------|-------------------|-----------------------|
| Gray boxes | Treat deep modules as gray boxes. Design the interface, delegate internals to AI. Human selectively reviews internals for critical paths. | Automated via adversarial separation: validators review code without seeing design intent. Human reviews only the final adversarial reports and draft PR. More automated than Pocock's manual approach. |
| Strategic vs tactical | Include module changes and interface designs in PRDs. Human thinks strategically, AI programs tactically. | Replaced /prd-to-plan with a sequential artifact pipeline (proposal -> specs -> design -> tasks) with human gates at each step. Same goal, different mechanism. |

---

## What Darkfactory adds

Concepts in the current implementation that neither source described:

| Addition | Purpose |
|----------|---------|
| Spec coverage audit with deviation classification | Maps every PRD requirement to tasks. Classifies deviations as interpretation, substitution, or removal with escalating approval. |
| Contract cross-referencing to PRD | Checks trace to specific PRD requirements. Deviated items are marked. |
| Fix loop with circuit breaker | On validation failure, spawns a fix worker and re-runs both validators. Caps at 3 attempts before escalating to human. |
| Spec delta check | Post-build verification that every COVERED item in the coverage audit is actually present in the built code. |
| Final gates | Whole-repo build/test/lint/format after the last task catches regressions from late tasks. |
| Behavioral conformance | Scrutiny validator attempts to verify observable behavior by execution, not just code reading. |
| Dual adversarial review | Two independent engines (Codex + Claude) review the full diff in parallel at mission end. |
| File-backed state and resume | All decisions persisted to disk. Missions survive session death and can be resumed. |
| Mandatory closing tasks | Cleanup and documentation tasks after all feature tasks. |
| Demo video capture | Records frontend functionality after validation passes, for human review. |
