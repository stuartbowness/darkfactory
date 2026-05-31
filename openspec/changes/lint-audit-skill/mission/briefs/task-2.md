# Worker Brief: Task 2 - Build audit.sh detection and comparison script

## Your Task

Create `skills/lint-audit/audit.sh` -- an executable shell script that audits a TypeScript project's oxlint config and tsconfig strictness against the agent-friendly ruleset in `rules.json`.

**Interface:** `audit.sh <project-dir> [--oxlint-config <path>]`
**Output:** JSON on stdout following the AuditReport schema (see Domain Types below).

The script must handle these scenarios:

**Linter detection (from package.json):**
- oxlint in deps → proceed with audit, outcome `ok`
- eslint only (exact package name match) → outcome `skip`
- both oxlint and eslint → audit oxlint, outcome `ok`
- neither → outcome `skip`
- no package.json → outcome `skip`

**Oxlint config detection:**
- `.oxlintrc.json` → read and parse rules
- `.oxlintrc.jsonc` → strip comments, then parse
- `oxlint.config.ts` → detect but note auto-apply is limited
- Multiple formats → outcome `requires_selection` (unless `--oxlint-config` flag is passed)
- No config → report defaults from rules.json

**Rule comparison:**
- For each rule in rules.json, check if it's in the project's oxlint config
- Status: `enabled` (in config as error/warn, or default_enabled), `missing` (not configured, not default), `disabled` (explicitly off/allow)
- Include the rationale from rules.json for missing rules

**tsconfig detection:**
- Find all `tsconfig*.json` files up to depth 3
- For each, resolve effective config via `tsc --showConfig -p <file>` (fall back to raw file if tsc fails)
- Check for `strict`, `noUncheckedIndexedAccess`, `erasableSyntaxOnly`
- For missing settings, run migration scope count: `tsc -p <file> --<setting> --noEmit --pretty false 2>&1 | grep -c 'error TS'`

**Type-aware detection:**
- Check package.json lint script for `--type-aware`
- Check oxlint config for type-aware mode setting
- Report as boolean

**Error handling:**
- jq is a hard prerequisite -- exit 1 with `{"outcome": "error", "error": "jq required"}` if missing
- Missing tsc → report as finding with status `error`, continue with partial results
- Unparseable config → finding with status `error` and the parse error
- tsc --showConfig failure → fall back to raw file read with a note

**Portability:** Must work on macOS (BSD coreutils). No GNU-only flags (`grep -P`, `sed -i`, `readlink -f`).

## Project Context

Building a `/lint-audit` darkfactory skill. audit.sh is the testable core -- all detection and comparison logic lives here. SKILL.md (Task 3) orchestrates the user interaction and calls this script.

## Architecture

audit.sh reads `rules.json` (from the same directory as the script) to get the recommended ruleset. It outputs a single JSON object to stdout. The SKILL.md agent parses this JSON and presents findings to the user.

## Key Terms

- **AuditReport** -- The full JSON output: outcome, linter type, config path, oxlint findings, tsconfig findings, type_aware status
- **AuditFinding** -- A single finding: rule name, status (enabled/missing/disabled/error), category, rationale, migration_scope (optional)
- **Migration scope** -- Error count from `tsc -p <tsconfig> --strict --noEmit --pretty false 2>&1 | grep -c 'error TS'`

## Domain Types

- AuditReport: `{outcome: "ok"|"skip"|"error"|"requires_selection", linter: string, config_path: string|null, oxlint_findings: AuditFinding[], tsconfig_findings: [{path: string, settings: AuditFinding[]}], type_aware: boolean, error?: string}`
- AuditFinding: `{name: string, status: "enabled"|"missing"|"disabled"|"error", category: string, rationale: string, migration_scope?: number}`

## What Previous Tasks Built

### Task 1: rules.json agent-friendly ruleset
- Created: `skills/lint-audit/rules.json` (14 rules, pinned to oxlint 1.67.0)
- Created: `skills/lint-audit/tests/validate-rules-json.sh` (schema validation test)
- Schema: each rule has `name`, `category`, `severity`, `default_enabled`, `rationale`
- Categories: type-safety, error-handling, code-quality, imports

## Downstream Consumers

- Task 3 (SKILL.md) calls `bash audit.sh <project-dir>` and parses the JSON output. Minimum integration test: create a temp dir with a package.json containing oxlint, run audit.sh, verify JSON output has `"outcome": "ok"`.
- Task 4 (smoke test) runs audit.sh against 200 mock project directories.

## Procedures

**Before writing any code**, invoke the `/tdd` skill. This is mandatory -- do not skip it, do not treat it as optional guidance.

- Create `skills/lint-audit/audit.sh` with the detection and comparison logic
- Make it executable: `chmod +x skills/lint-audit/audit.sh`
- Write tests that cover the key scenarios (oxlint detected, eslint skip, no config, tsconfig strict missing)
- Commit all changes in a single commit with message: "[task-2] Build audit.sh detection and comparison script"
- Write your handoff report to openspec/changes/lint-audit-skill/mission/handoffs/task-2.md

## Handoff Report Format
- Task: Task 2 - Build audit.sh detection and comparison script
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
