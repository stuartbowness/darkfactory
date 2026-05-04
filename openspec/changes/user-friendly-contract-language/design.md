## Architecture Decision

This mission modifies an existing codebase without architecture changes. No scored candidates required.

**Current architecture:** Darkfactory is a set of Claude Code skills (markdown files in `skills/`), prompt templates (`skills/mission/prompts/`), artifact templates (`skills/mission/templates/`), and documentation (`docs/`, `README.md`, `UBIQUITOUS_LANGUAGE.md`). There is no application code, no build system, and no runtime. The "modules" are markdown files consumed by LLM agents.

**How this change fits:** The rename is a find-and-replace operation across markdown files, guided by context-dependent rules (contract items become "checks", validator steps become "gates", code test references stay "assertions"). The architecture is unchanged -- the same files serve the same roles, with different vocabulary.

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `mission-skill` | Broad -- skills/mission/SKILL.md is the orchestrator's operating manual (~800 lines) | Phase sequencing, contract rules, validation logic, fix loop mechanics | The orchestrator reads this file to run a mission end-to-end; workers and validators never see it |
| `scrutiny-prompt` | Narrow -- one prompt file consumed by the scrutiny validator subagent | Gate sequencing, pass/fail criteria, report format | Validators are adversarially separated from the orchestrator; the prompt is their only interface |
| `usertest-prompt` | Narrow -- one prompt file consumed by the user-test validator subagent | Test execution strategy, result format | Same adversarial separation as scrutiny |
| `adversarial-prompt` | Narrow -- one prompt file consumed by adversarial reviewers | Review concerns, commit-walking strategy, report format | Independent review; sees contract but not design |
| `templates` | Narrow -- 5 template files consumed by the orchestrator during artifact generation | Format and structure of contracts, PR bodies, plans, worker briefs, fix briefs | Templates are filled in by the orchestrator; workers see the filled result, not the template |
| `glossary` | Narrow -- UBIQUITOUS_LANGUAGE.md is a single reference file | Term definitions, relationships, resolved ambiguities | Consumed by all agents (orchestrator, workers, validators) for consistent vocabulary |
| `docs` | Narrow -- 3 documentation files for human readers | Design rationale, provenance, reference material | Read by developers, not consumed by LLM agents at runtime |

## Type Catalog

This mission has no domain types in the traditional sense (no code, no runtime types). The "types" are terminology entries in the glossary.

| Type name | Purpose | Owning module |
|-----------|---------|---------------|
| `Check` | A single verifiable statement in a contract | `glossary` |
| `Gate` | A numbered step in the scrutiny validator's sequence | `glossary` |
| `Contract` | A set of checks grouped by task, consumed by validators | `glossary` |

## Error Strategy

No runtime errors. The only failure mode is an incomplete or inconsistent rename:

- **Missed occurrence:** A file still says "assertion" where it should say "check," or "check N" where it should say "gate N." Detected by post-task grep verification.
- **Over-eager rename:** A file says "check" where it should still say "assertion" (code test context). Detected by reading preserved files (test-quality, tdd) and verifying they are unchanged.
- **Inconsistent phrasing:** Awkward constructions like "check each check" introduced by mechanical replacement. Detected by read-through of each modified file.

Mitigation for all three: each task includes a grep-based verification step, and the module shape re-check closing task does a full sweep.

## Integration Points

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| `mission-skill` | `templates` | Orchestrator reads templates and fills in values | Internal |
| `mission-skill` | `scrutiny-prompt` | Orchestrator pastes contract checks into prompt, spawns validator subagent | Internal |
| `mission-skill` | `usertest-prompt` | Orchestrator pastes contract checks into prompt, spawns validator subagent | Internal |
| `mission-skill` | `adversarial-prompt` | Orchestrator pastes full contract into prompt, spawns reviewer | Internal |
| `mission-skill` | `glossary` | Orchestrator and workers read glossary for consistent terminology | Internal |
| `docs` | `glossary` | Docs reference glossary terms | Internal |

## Test Strategy

- **Framework:** No test framework. Darkfactory has no application code and no test suite.
- **Verification method:** Grep-based verification after each task. After all changes:
  1. `grep -rn "assertion" <in-scope files>` should return only code-test-context hits (tautological assertion, test assertions, assertion quality, logical assertion per test)
  2. `grep -rn "check [0-9]" <in-scope files>` should return zero hits in scrutiny validator prompt (all should be "gate N")
  3. Read-through of each modified file to verify natural phrasing
- **Coverage approach:** Every in-scope file is verified by grep. Each task's brief includes the specific files to modify and the grep to run afterward.
- **What is NOT tested:** Archived files and in-flight changes are not scanned (they should be unchanged). The prd skill's general English use of "assertions" is not modified.
