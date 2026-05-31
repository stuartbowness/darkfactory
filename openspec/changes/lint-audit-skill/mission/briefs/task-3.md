# Worker Brief: Task 3 - Write SKILL.md workflow and RULES.md documentation

## Your Task

Create two files:
1. `skills/lint-audit/SKILL.md` -- Agent workflow instructions for the `/lint-audit` skill
2. `skills/lint-audit/RULES.md` -- Human-readable documentation of the agent-friendly ruleset

### SKILL.md Requirements

The SKILL.md must have this frontmatter:
```yaml
---
name: lint-audit
user-invokable: true
description: Audit a TypeScript project's oxlint config and tsconfig strictness against an agent-friendly ruleset. Reports gaps and offers to apply fixes. Use when setting up a new project, auditing an existing project's linting config, or before a mission to ensure mechanical enforcement is in place.
---
```

The workflow must:
1. Run `bash <skill-dir>/audit.sh <project-dir>` and parse the JSON output
2. Handle each outcome: `ok` (present report), `skip` (explain why), `error` (show error), `requires_selection` (ask user which config, re-run with `--oxlint-config`)
3. Present the combined report using status markers (`[x]` enabled, `[ ]` missing, `[!]` disabled) grouped by:
   - Oxlint Rules (grouped by category from the findings)
   - TypeScript Compiler (per tsconfig file)
   - Type-Aware Linting (enabled/missing)
4. After the report, offer three apply modes: "apply all", "skip all", "review per-item"
5. For apply-all: apply every missing recommendation without further prompts
6. For per-item: present each missing item individually with accept/deny. For tsconfig strictness settings, show migration scope before asking.
7. Apply logic:
   - Create `.oxlintrc.json` if no config exists, with accepted rules
   - Update existing `.oxlintrc.json` by adding rules (don't remove existing)
   - For `.oxlintrc.jsonc` and `oxlint.config.ts`: print the rule JSON for manual addition, do NOT edit the file
   - Update tsconfig.json files to add accepted strictness settings to compilerOptions
   - Update package.json lint script to add `--type-aware` if accepted (find the lint script, append the flag)

### RULES.md Requirements

Human-readable documentation explaining:
- What the agent-friendly ruleset is and why it exists (reference the Armin/Christina friction talk)
- Each rule category (type-safety, error-handling, code-quality, imports) with explanation of why agents violate these patterns
- The pinned oxlint version (must match rules.json: 1.67.0)
- How to add new rules (edit rules.json)

## Project Context

This is the user-facing layer of the `/lint-audit` skill. The agent reads SKILL.md when the user invokes `/lint-audit`, follows the workflow, calls audit.sh, and presents results.

## Architecture

SKILL.md orchestrates the user interaction. audit.sh (Task 2) does all detection and comparison. rules.json (Task 1) defines the ruleset. RULES.md is reference documentation for humans.

## Key Terms

- **Lint audit** -- A darkfactory skill that audits a TypeScript project's oxlint config and tsconfig strictness
- **Agent-friendly ruleset** -- The recommended set of oxlint rules and tsconfig settings
- **Migration scope** -- Error count before enabling a strictness setting

## Domain Types

None for this task (SKILL.md is markdown instructions, not code with types).

## What Previous Tasks Built

### Task 1: rules.json agent-friendly ruleset
- Created: `skills/lint-audit/rules.json` (14 rules, pinned to oxlint 1.67.0)
- Schema: each rule has `name`, `category`, `severity`, `default_enabled`, `rationale`

### Task 2: audit.sh detection and comparison script
- Created: `skills/lint-audit/audit.sh` (419 lines, executable)
- Interface: `audit.sh <project-dir> [--oxlint-config <path>]` → JSON on stdout
- Output schema: `{outcome, linter, config_path, oxlint_findings[], tsconfig_findings[], type_aware}`
- Each oxlint finding: `{name, status, category, rationale, severity}`
- Each tsconfig finding: `{path, settings: [{name, status, rationale, migration_scope?}]}`

## Downstream Consumers

None -- this is the last feature task. Closing tasks follow.

## Procedures

**Before writing any code**, invoke the `/tdd` skill. This is mandatory -- do not skip it, do not treat it as optional guidance.

- Create `skills/lint-audit/SKILL.md` with the workflow
- Create `skills/lint-audit/RULES.md` with the documentation
- Commit all changes in a single commit with message: "[task-3] Write SKILL.md workflow and RULES.md documentation"
- Write your handoff report to openspec/changes/lint-audit-skill/mission/handoffs/task-3.md

## Handoff Report Format
- Task: Task 3 - Write SKILL.md workflow and RULES.md documentation
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
