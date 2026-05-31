# Worker Brief: Task 1 - Create rules.json agent-friendly ruleset

## Your Task

Create `skills/lint-audit/rules.json` -- a machine-parseable JSON file defining the recommended agent-friendly oxlint ruleset. This is the data layer that `audit.sh` (Task 2) will read to compare against a project's oxlint config.

The file must contain:
- A top-level `"oxlint_version"` field with the semver version the defaults were researched against
- A `"rules"` array with at least 8 rule entries
- Each rule entry must have: `name` (oxlint canonical format like `typescript/no-explicit-any`), `category`, `severity` (`"error"` or `"warn"`), `default_enabled` (boolean), `rationale` (one-line string)

Required rules (minimum -- add more if appropriate for agent enforcement):
- `typescript/no-explicit-any`
- `typescript/no-non-null-assertion`
- `typescript/no-floating-promises`
- `typescript/no-misused-promises`
- `typescript/switch-exhaustiveness-check`

Research the current oxlint docs to verify canonical rule names and which rules are enabled by default. Use oxlint's naming convention (`typescript/rule-name`, `eslint/rule-name`, etc.), not the `@typescript-eslint/*` prefix.

Categories to organize by: type-safety, error-handling, code-quality, imports.

## Project Context

Building a `/lint-audit` darkfactory skill that audits TypeScript projects' oxlint and tsconfig configs against agent-friendly rules. This rules.json is the source of truth for what rules the skill recommends.

## Architecture

Three-module design: `rules.json` (data, this task) → `audit.sh` (logic, Task 2) → `SKILL.md` (workflow, Task 3). This task produces pure data that the other modules consume.

## Key Terms

- **Agent-friendly ruleset** -- The recommended set of oxlint rules and tsconfig settings that mechanically enforce patterns agents tend to violate: type safety bypasses, silent failures, error suppression, dead code
- **Mechanical enforcement** -- Binary pass/fail rules enforced by tooling rather than advisory instructions

## Domain Types

- AuditFinding: rule name, status, category, rationale -- rules.json defines the canonical rule data that becomes findings in audit.sh output

## What Previous Tasks Built

This is the first task. No prior code has been committed for this mission.

## Downstream Consumers

- Task 2 (audit.sh) reads rules.json with jq to compare against the project's oxlint config. Minimum integration test: `jq '.rules | length' skills/lint-audit/rules.json` returns a number >= 8.
- Task 3 (RULES.md) references the pinned oxlint version from rules.json.

## Procedures

**Before writing any code**, invoke the `/tdd` skill. This is mandatory -- do not skip it, do not treat it as optional guidance.

- Create the directory `skills/lint-audit/` if it does not exist
- Create `skills/lint-audit/rules.json` with the ruleset
- Write at least one test that validates the JSON schema (valid JSON, required fields present, rule names in canonical format)
- Commit all changes in a single commit with message: "[task-1] Create rules.json agent-friendly ruleset"
- Write your handoff report to openspec/changes/lint-audit-skill/mission/handoffs/task-1.md

## Handoff Report Format
- Task: Task 1 - Create rules.json agent-friendly ruleset
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
