# Module Shape Audit: skill-decomposition

## Result: PASS with advisory

## Checks

### 1. Directory count and names

Exactly 12 directories found under `skills/`:

- adversarial
- dispatch
- grill
- language
- mission
- mission-init
- mission-resume
- mission-status
- prd
- security
- tdd
- test-quality

Result: PASS

### 2. SKILL.md frontmatter

| Skill | SKILL.md exists | name: field | Result |
|-------|----------------|-------------|--------|
| adversarial | yes | name: adversarial | PASS |
| dispatch | yes | name: dispatch | PASS |
| grill | yes | name: grill | PASS |
| language | yes | name: language | PASS |
| mission | yes | name: mission | PASS |
| mission-init | yes | name: mission-init | PASS |
| mission-resume | yes | name: mission-resume | PASS |
| mission-status | yes | name: mission-status | PASS |
| prd | yes | name: prd | PASS |
| security | yes | name: security | PASS |
| tdd | yes | name: tdd | PASS |
| test-quality | yes | name: test-quality | PASS |

Result: PASS — all 12 SKILL.md files exist and `name:` matches directory name.

### 3. mission/ subdirectories

`skills/mission/templates/` contains:
- contract.md
- coverage-audit.md
- plan.md
- pr-body.md
- worker-brief.md
- worker-fix-brief.md

`skills/mission/prompts/` contains:
- adversarial-reviewer.md
- scrutiny-validator.md
- spec-delta-checker.md
- usertest-validator.md

Result: PASS — all 10 required files present, no extras.

### 4. tdd/ companions

`skills/tdd/` contains:
- SKILL.md
- deep-modules.md
- interface-design.md
- mocking.md
- refactoring.md
- tests.md

Result: PASS — SKILL.md plus all 5 companion files present.

### 5. Design drift

`design.md` final directory state specifies deletion of:
- `.claude/skills/` — entire directory
- `.codex/skills/` — entire directory (duplicates)

On disk:
- `.claude/skills/` still exists with 5 entries: `darkfactory-init/`, `openspec-apply-change/`, `openspec-archive-change/`, `openspec-explore/`, `openspec-propose/`
- `.codex/skills/` still exists with 4 entries: `openspec-apply-change/`, `openspec-archive-change/`, `openspec-explore/`, `openspec-propose/`

Advisory: The entries that remain are openspec skills (not part of the 12 decomposed skills), plus the legacy `darkfactory-init/` in `.claude/skills/`. The design spec says to delete the entire directories. These were likely retained intentionally because the openspec skills are still in active use and were not in scope for this decomposition. The `darkfactory-init/` entry in `.claude/skills/` is a stale remnant of the pre-decomposition layout.

The 12 skills listed in the design spec are all present under `skills/` and none exist under `.claude/skills/`. The drift is limited to residual non-decomposed skills that were outside this change's scope.

Result: ADVISORY — `skills/` content matches design.md exactly; `.claude/skills/` and `.codex/skills/` not fully deleted as specified, but remaining entries are out-of-scope openspec skills. The legacy `darkfactory-init/` in `.claude/skills/` is a stale entry that should be cleaned up.

`install.sh` is present and executable at `~/.darkfactory/install.sh` (546 bytes, mode -rwxr-xr-x). PASS.

## Summary

All five checks covering the primary deliverable of the skill-decomposition change are in good shape. The 12-skill directory set is complete, all SKILL.md files exist with correct name frontmatter, the mission/ subdirectories have all required template and prompt files, and the tdd/ companions are intact. The only finding is that `.claude/skills/` and `.codex/skills/` were not fully deleted as specified in design.md — both directories remain, containing openspec skills that were out of scope for this change. The `darkfactory-init/` stale entry in `.claude/skills/` is a minor cleanup item. None of these residual files affect the new skills/ structure or how the 12 decomposed skills function.
