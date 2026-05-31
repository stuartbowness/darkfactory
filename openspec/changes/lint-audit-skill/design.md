# Design: lint-audit skill

## Architecture Decision

This is a new skill added to an existing codebase. The darkfactory skill system is well-established: SKILL.md for workflow instructions, optional reference files, optional scripts. No architecture change needed.

The key decision is how much logic lives in the SKILL.md (agent instructions) vs in `audit.sh` (executable script). The skill needs to be testable via a 200-iteration loop, which requires executable logic.

| Candidate | Separation of concerns | Testability | Task independence | Module depth | Total | Recommendation |
|-----------|------------------------|-------------|-------------------|--------------|-------|----------------|
| A: SKILL.md only (pure instructions) | 2 — skill reads configs and reasons about rules, but detection + comparison + apply are all interleaved | 1 — can only test by running the full agent, not programmatically | 3 — single file, no dependencies | 1 — entire workflow is surface area | 7 | Not recommended |
| B: SKILL.md + audit.sh (detection/comparison script) + RULES.md (ruleset reference) | 3 — detection and comparison in script, workflow orchestration in SKILL.md, ruleset data in RULES.md | 3 — audit.sh is independently testable with mock project dirs | 3 — three files, each independently modifiable | 3 — audit.sh has a narrow interface (project dir in, JSON findings out), hides all parsing | 12 | Recommended |

**Rationale:** Candidate B separates concerns cleanly. The audit.sh script handles detection and comparison (the testable logic). SKILL.md orchestrates the workflow and user interaction (the agent-specific part). RULES.md is pure data (the ruleset). The 200-iteration test loop runs against audit.sh directly, not the full agent.

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `audit.sh` | Narrow: `audit.sh <project-dir> [--oxlint-config <path>]`, outputs JSON findings to stdout. Optional `--oxlint-config` resolves ambiguity when multiple config formats exist. | Config file discovery, parsing, rule comparison against rules.json defaults, tsconfig effective config resolution, migration scope counting | All detection and comparison logic is behind a single script call. The SKILL.md reads JSON output and presents it. Workers and validators never need to understand the parsing internals. |
| `SKILL.md` | Narrow: single slash command `/lint-audit` | User interaction flow, apply mode orchestration, file modification logic | Agent workflow instructions are separate from detection logic. Changes to the user flow don't affect what rules are checked. |
| `rules.json` | Narrow: read-only JSON data file, array of rule objects | The recommended ruleset with canonical names, categories, severities, default status, and rationale | Machine-parseable data separate from human documentation. audit.sh reads this directly with jq. Adding a new rule means adding an entry to this array. |
| `RULES.md` | Narrow: read-only human documentation | Explanation of each rule category and why it matters for agents | Human context for the ruleset. Not parsed by audit.sh. |

## Type Catalog

| Type name | Purpose | Owning module |
|-----------|---------|---------------|
| `AuditFinding` | A single finding: rule name, status (`enabled`, `missing`, `disabled`, `error`, `skipped`), category, rationale, migration_scope (optional error count) | `audit.sh` (JSON output) |
| `AuditReport` | The full audit result: outcome (`ok`, `skip`, `error`, `requires_selection`), linter type, config path, oxlint findings, tsconfig findings (per file), type_aware status | `audit.sh` (JSON output) |

These are not TypeScript types -- they're the JSON structure that audit.sh outputs and SKILL.md consumes. Defined here so workers and validators agree on the contract.

**Outcome values:**
- `ok` -- audit completed, findings are populated
- `skip` -- ESLint-only project or no package.json, no findings
- `error` -- jq or other hard dependency missing, error message in report
- `requires_selection` -- multiple oxlint config formats found, SKILL.md must ask user and re-run with `--oxlint-config`

## Error Strategy

- **audit.sh errors:** Script exits non-zero with a JSON error object `{"error": "<message>"}` on stdout. SKILL.md checks exit code and presents the error to the user. No silent failures.
- **Missing jq:** jq is a hard prerequisite. If missing, audit.sh exits 1 with `{"outcome": "error", "error": "jq is required but not installed"}`. No partial results without jq.
- **Missing oxlint or tsc:** If `oxlint` or `tsc` are not installed, audit.sh reports them as findings with status `error`. The skill continues with partial results (e.g., can still check config files exist even if the tool is missing).
- **Unreadable configs:** If a config file exists but can't be parsed (malformed JSON, permissions), audit.sh reports a finding with status `[!]` and the parse error message. It does not skip the file silently.
- **tsc --showConfig failure:** If `tsc --showConfig` fails (e.g., tsconfig has errors), audit.sh falls back to reading the raw file and notes "effective config unavailable, reading file directly."

## Integration Points

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| `SKILL.md` (agent) | `audit.sh` | `bash audit.sh <project-dir>` via Bash tool | Internal |
| `audit.sh` | `rules.json` | Reads file to get recommended ruleset (array of rule objects) | Internal |
| `audit.sh` | `tsc` | `tsc --showConfig -p <tsconfig>` and `tsc -p <tsconfig> --strict --noEmit --pretty false` | Outbound |
| `audit.sh` | `jq` | JSON parsing of package.json, oxlintrc, tsc output | Outbound |
| `SKILL.md` (agent) | `.oxlintrc.json` | Creates or updates via Write/Edit tools during apply | Outbound |
| `SKILL.md` (agent) | `tsconfig.json` | Updates via Edit tool during apply | Outbound |
| `SKILL.md` (agent) | `package.json` | Reads lint script to check for `--type-aware`; updates lint script to add `--type-aware` during apply | Inbound + Outbound |

## Test Strategy

- **Framework:** Bash test harness (same pattern as ts-quality.sh testing). No test framework dependency.
- **Test location:** `/tmp/lint-audit-tests/` (temporary, created during test runs)
- **What gets tested:** `audit.sh` is the unit under test. Each test creates a mock project directory with specific package.json, tsconfig.json, and optional oxlint config, runs `audit.sh`, and verifies the JSON output matches expectations.
- **External dependencies:** `jq` (required, available everywhere). `tsc` for effective config resolution and migration scope -- tests that exercise these paths need TypeScript installed. Tests that exercise detection/comparison only do not need tsc.
- **Test categories:**
  - Detection: oxlint present/absent, ESLint present/absent, both, neither, no package.json
  - Config format: `.oxlintrc.json`, `.oxlintrc.jsonc`, `oxlint.config.ts`, multiple formats, no config
  - Rules: all enabled, all missing, partially configured, explicitly disabled
  - tsconfig: strict on/off, inherited via extends, multiple files, named variants
  - Monorepo: root + subproject configs, different strictness per subproject
  - Edge cases: malformed JSON, empty config, missing compilerOptions
- **Coverage:** All scenarios from the three spec files map to at least one test case. The 200-iteration loop covers these categories plus adversarial edge cases.
