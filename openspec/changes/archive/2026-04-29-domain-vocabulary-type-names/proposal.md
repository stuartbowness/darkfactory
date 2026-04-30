## Why

Workers can implement domain concepts using reasonable aliases instead of the exact names from the Type Catalog in design.md. The Type Catalog tells the orchestrator what to name things, but the worker brief doesn't carry those names forward. The foundation audit catches task 1 drift, but nothing checks type names for later tasks. The relationship between UBIQUITOUS_LANGUAGE.md (domain concepts) and the Type Catalog (code type names) is implicit.

## What Changes

- Worker brief template gains a "Domain Types" section listing exact Type Catalog names for the task, with a one-line instruction to use them.
- Design template's Type Catalog section gains a one-sentence clarification of what "domain types only" means versus helper types.
- SKILL.md's module shape re-check closing task gains a sentence requiring Type Catalog name verification.
- UBIQUITOUS_LANGUAGE.md's Type Catalog definition is updated to state its relationship to the glossary. A "Domain type" term is added. A relationship entry documents the full type naming chain.
- README.md is updated to reflect the changes.

## Capabilities

### New Capabilities
- `brief-domain-types`: Worker brief template carries exact Type Catalog names so workers use correct domain type names.
- `type-name-audit`: Module shape re-check closing task verifies all Type Catalog names exist in code with matching names.
- `domain-type-boundary`: Design template and ubiquitous language clarify which types are domain types versus helper types.

### Modified Capabilities

(none -- no existing spec-level behavior changes)

## Impact

- `skills/mission/templates/worker-brief.md` -- new section added
- `skills/mission/templates/design-template.md` -- one sentence added to Type Catalog section
- `skills/mission/SKILL.md` -- module shape re-check instructions extended
- `UBIQUITOUS_LANGUAGE.md` -- definition updated, term added, relationship added, dialogue updated
- `README.md` -- updated to reflect changes
