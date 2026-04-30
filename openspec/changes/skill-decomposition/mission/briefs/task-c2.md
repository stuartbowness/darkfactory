# Worker Brief: Task C2 - Cleanup

## Your Task

Three cleanup operations:

1. Remove `.claude/skills/` and `.codex/skills/` from git tracking (and from disk)
2. Update four stale skill name references in `skills/mission/prompts/scrutiny-validator.md`
3. Update four stale skill name references in `skills/mission/templates/plan.md`

## Repo root: /Users/stuart/.darkfactory

## Operation 1: Remove old skill directories from git

These directories contain old skill files that were either moved to `skills/` or replaced. They are dead code.

```bash
git rm -r .claude/skills/
git rm -r .codex/skills/
```

This removes them from git tracking AND from disk. The `.claude/commands/` directory (at `.claude/commands/opsx/`) is NOT affected — only `.claude/skills/`.

Verify afterward:
- `git ls-files .claude/skills/` → no output
- `git ls-files .codex/skills/` → no output
- `.claude/commands/` directory still exists with its files

## Operation 2: Update scrutiny-validator.md

File: `skills/mission/prompts/scrutiny-validator.md`

**Change A — line 180:** Update the security skill invocation:
- Old: `run /security-review. Otherwise "N/A."`
- New: `run /security. Otherwise "N/A."`

**Change B — line 246:** Update the test quality skip report string:
- Old: `report \`test-quality-review | SKIP | no test files in diff\``
- New: `report \`test-quality | SKIP | no test files in diff\``

**Change C — line 248:** Update the security-review reference in the delegation pattern example:
- Old: `following the same delegation pattern as /review, /simplify, and /security-review.`
- New: `following the same delegation pattern as /review, /simplify, and /security.`

**Change D — line 257:** Update the report format example strings:
- Old: `` `test-quality-review | PASS | no findings`. Report warning output as `test-quality-review | WARN | 2 P2, 1 P3 findings`. Report failing output as `test-quality-review | FAIL | 1 P0 finding (tautological assertion in test_list_open)` ``
- New: `` `test-quality | PASS | no findings`. Report warning output as `test-quality | WARN | 2 P2, 1 P3 findings`. Report failing output as `test-quality | FAIL | 1 P0 finding (tautological assertion in test_list_open)` ``

## Operation 3: Update plan.md

File: `skills/mission/templates/plan.md`

Replace the "Skills to Use" section entries (lines 31-37):
- Old: `- /grill-me -- alignment interview (planning)`
- New: `- /grill -- alignment interview (planning)`

- Old: `- /ubiquitous-language -- shared language (planning)`
- New: `- /language -- shared language (planning)`

- Old: `- /write-a-prd -- structured PRD from grill conversation (planning)`
- New: `- /prd -- structured PRD from grill conversation (planning)`

- Old: `- /security-review -- security check (scrutiny validation, sensitive tasks only)`
- New: `- /security -- security check (scrutiny validation, sensitive tasks only)`

## Acceptance Criteria

- `git ls-files .claude/skills/` → no output
- `git ls-files .codex/skills/` → no output
- `grep -cE "grill-me|write-a-prd|ubiquitous-language|darkfactory-init|test-quality-review|security-review" skills/mission/prompts/scrutiny-validator.md` → 0
- `grep -cE "grill-me|write-a-prd|ubiquitous-language|darkfactory-init|test-quality-review|security-review" skills/mission/templates/plan.md` → 0
- `.claude/commands/opsx/*.md` files still exist (git ls-files .claude/commands/ → output)

## Procedures

1. Run the git rm commands
2. Make the text replacements in scrutiny-validator.md
3. Make the text replacements in plan.md
4. Commit everything in a single commit: `[task-c2] cleanup old skill refs and remove dead skill directories`
5. Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-c2.md`

## Handoff Report Format

- Task: Task C2 - Cleanup
- Commit SHA: <the full SHA>
- Commands run: <any commands and exit codes>
- Concerns: <anything uncertain>
