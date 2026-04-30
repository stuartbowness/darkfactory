# Issue: Task 2 - Fork grill, language, prd skills (Attempt 1)

## Failed Checks

- Contract assertion 4: `grep "user-invokable: true" skills/grill/SKILL.md skills/prd/SKILL.md skills/language/SKILL.md` returns 0 lines (required: 3 matching lines)

## Observed Symptoms

The three forked skill files have no `user-invokable: true` field in their frontmatter. The source files at ~/Code/dotfiles also lack this field, so the copy preserved that absence.

## Validator Recommendations

Add `user-invokable: true` to the frontmatter of skills/grill/SKILL.md, skills/prd/SKILL.md, and skills/language/SKILL.md.
