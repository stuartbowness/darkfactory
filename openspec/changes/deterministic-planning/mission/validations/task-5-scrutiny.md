# Task 5 Scrutiny Validation

**Commit:** 303aa20f6638bf2d661be00c1ff2d3769acfa40a
**Title:** [task-5.1] Add CLAUDE.md template and closing task
**Files changed:** `skills/mission/SKILL.md`, `skills/mission/templates/claude-md.md`

## Assertions

- [x] **[output] `skills/mission/templates/claude-md.md` exists.**
  File created with 72 lines. Confirmed present in commit tree.

- [x] **[output] The template contains an include/exclude guidance table.**
  Table present at top of file with two columns. Include column lists: commands Claude cannot guess, code style rules differing from defaults, testing instructions, architectural decisions, dev environment quirks, common gotchas. Exclude column lists: anything inferable from code, standard conventions, detailed API docs, file-by-file descriptions, tutorials, self-evident practices. All six include items and all six exclude items match the contract.

- [x] **[output] The template contains eight sections.**
  Template block contains: Purpose, Rules, Commands, Repo Map, Testing, Environment, Architecture (with `See @ARCH.md`), Key Terms (with `See @UBIQUITOUS_LANGUAGE.md`). That is eight sections. The contract says "@ARCH.md reference" and "@UBIQUITOUS_LANGUAGE.md reference" -- the template names them "Architecture" and "Key Terms" but the content is the reference. Pass.

- [x] **[output] The template contains one example with an "adapt to language" note.**
  The `## Example` section contains one concrete example ("Invoice Sync") preceded by: "Adapt section content to the project's language and framework." This satisfies "adapt to language" note.

- [x] **[output] The Rules section includes "update ARCH.md when you change architecture".**
  Template Rules placeholder says: `Include "update ARCH.md when you change architecture."` Example Rules section says: `Update ARCH.md when you change architecture.` Present in both.

- [x] **[output] SKILL.md Phase 4.4 mandatory closing tasks are ordered correctly.**
  The closing tasks at lines 250-262 are:
  1. Smoke test
  2. Cleanup
  3. Module shape re-check
  4. CLAUDE.md + ARCH.md generation
  5. Documentation
  6. CI configuration (optional)

  This matches the contract order exactly.

- [x] **[output] The CLAUDE.md closing task description specifies: create from template if absent, create ARCH.md from design.md if absent, do not modify existing files.**
  Task 4 text: "If the project root does not have a CLAUDE.md, create one from templates/claude-md.md ... If a CLAUDE.md already exists, do not modify it. Similarly, if no ARCH.md exists, create one from design.md's architecture decisions ... If ARCH.md already exists, do not modify it." All three conditions met.

- [x] **[execution] `grep -c "claude-md.md" skills/mission/SKILL.md` returns at least 1.**
  Returns `1`. Pass.

## Position Verification

The diff shows the CLAUDE.md + ARCH.md generation task was inserted as item 4, between the existing module shape re-check (item 3) and documentation task (renumbered to item 5). The CI configuration task was renumbered from 5 to 6. The insertion position is correct -- it sits after module shape re-check and before documentation, matching the contract ordering.

## Result

**PASS** -- 8/8 assertions satisfied.
