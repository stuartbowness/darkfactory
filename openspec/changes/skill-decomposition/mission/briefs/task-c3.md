# Worker Brief: Task C3 - Module Shape Re-check

## Your Task

Audit the final `skills/` directory against the design spec. Write a validation report.

## Repo root: /Users/stuart/.darkfactory

## What to verify

### 1. Directory count and names

Run `ls skills/` and confirm exactly 12 directories exist:
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

Any extra or missing directories is a FAIL.

### 2. SKILL.md frontmatter for each skill

For each of the 12 skill directories, verify `skills/<name>/SKILL.md` exists and the first line after the opening `---` contains `name: <name>` matching the directory name.

Example check:
```bash
head -5 skills/adversarial/SKILL.md
```

### 3. mission/ subdirectory completeness

`skills/mission/` must have both `templates/` and `prompts/` subdirectories with these exact files:

templates/:
- contract.md
- coverage-audit.md
- plan.md
- pr-body.md
- worker-brief.md
- worker-fix-brief.md

prompts/:
- adversarial-reviewer.md
- scrutiny-validator.md
- spec-delta-checker.md
- usertest-validator.md

### 4. tdd/ companion files

`skills/tdd/` must have SKILL.md plus all 5 companion files:
- deep-modules.md
- interface-design.md
- mocking.md
- refactoring.md
- tests.md

### 5. No drift from design.md

Read `openspec/changes/skill-decomposition/design.md`. Verify the `skills/` structure matches the "Final directory state" section. Flag any file that is present in design.md but missing on disk, or present on disk but absent from design.md.

## Output

Write the audit report to: `openspec/changes/skill-decomposition/mission/validations/module-shape-audit.md`

Format:
```markdown
# Module Shape Audit: skill-decomposition

## Result: PASS | FAIL

## Checks

### 1. Directory count and names
<list of 12 dirs found — PASS or list of discrepancies — FAIL>

### 2. SKILL.md frontmatter
| Skill | SKILL.md exists | name: field | Result |
|-------|----------------|-------------|--------|
| adversarial | yes | name: adversarial | PASS |
| ... | ... | ... | ... |

### 3. mission/ subdirectories
<list of files in templates/ and prompts/ — PASS or missing files — FAIL>

### 4. tdd/ companions
<list of files found — PASS or missing — FAIL>

### 5. Design drift
<PASS if matches design.md, or list of discrepancies>

## Summary
<one paragraph>
```

## Procedures

1. Run the checks (no code changes needed — this is audit only)
2. Write the report to the path above
3. Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-c3.md`

Do NOT commit — this task produces only an audit report file (untracked), not a code commit.

## Handoff Report Format

- Task: Task C3 - Module Shape Re-check
- Overall result: PASS | FAIL
- Findings: <any discrepancies>
- Concerns: <anything uncertain>
