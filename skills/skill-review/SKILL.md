---
name: skill-review
user-invokable: true
description: Audit an agent skill directory against Anthropic best practices for skill authoring. Checks naming, description quality, structure, content patterns, progressive disclosure, and common anti-patterns. Use when reviewing a skill for quality, after writing a new skill, or when user mentions "skill review", "best practices", or "audit skill".
---

# Skill Review

Audit a skill directory against the best practices checklist in [BEST-PRACTICES.md](BEST-PRACTICES.md).

## Input

Determine the target skill from `$ARGUMENTS`:

1. **Path** (contains `/`): use the path directly
2. **Bare name** (no `/`): resolve `~/.claude/skills/<name>`. If not found, try `./<name>`
3. **No argument**: look for `SKILL.md` in the current directory

Validate that the resolved path contains a `SKILL.md` file. If not, report the error and stop.

## Step 1: Gather Skill Files

Read all files in the skill directory:

- `SKILL.md` (required)
- Any companion `.md` files
- Any files in subdirectories (`scripts/`, `templates/`, `prompts/`)

Record the file inventory with line counts.

## Step 2: Load Best Practices

Resolve the skill-review skill directory from its symlink and read the checklist:

```bash
SKILL_REVIEW_DIR="$(cd "$(readlink "$HOME/.claude/skills/skill-review")" 2>/dev/null && pwd -P)"
```

Read `$SKILL_REVIEW_DIR/BEST-PRACTICES.md`.

## Step 3: Review

Check every dimension in BEST-PRACTICES.md against the gathered skill files. For each dimension:

1. Determine if it applies (e.g., script checks only apply if the skill has scripts)
2. If it applies, evaluate compliance
3. If non-compliant, record a finding with severity, the specific dimension violated, the file and line affected, and a concrete suggestion for how to fix it

Severities:

- **CRITICAL** -- will break skill discovery or loading (missing frontmatter, reserved name, invalid characters, missing description)
- **WARNING** -- violates a clear best practice that significantly reduces quality (body over 500 lines, no "Use when" trigger, nested file references, inconsistent terminology)
- **SUGGESTION** -- improvement opportunity (add examples, add table of contents, improve conciseness)

## Step 4: Present Findings

```
# Skill Review: <skill-name>

## Summary

<one sentence: clean / N findings across M dimensions>

## Findings

### [CRITICAL] <title>
**Dimension:** <number and name from BEST-PRACTICES.md>
**File:** <file:line>
**Problem:** <what is wrong>
**Suggestion:** <specific fix>

### [WARNING] <title>
...

### [SUGGESTION] <title>
...

## Checklist

[x] 1. Conciseness -- OK
[ ] 4. Naming -- skill name uses underscores instead of hyphens
[x] 5. Description -- OK
...
[-] 17. Error handling -- N/A (no scripts)
```

End with: `N findings: X critical, Y warning, Z suggestion`

If no findings: `No findings. The skill follows all applicable best practices.`

Order findings by severity: CRITICAL first, then WARNING, then SUGGESTION.

$ARGUMENTS
