# Agent-Friendly Lint Rules

This ruleset defines the mechanical checks that make TypeScript projects safer for coding agents to edit.

The goal is to catch patterns agents often produce when they are moving quickly: weak types, hidden errors, stale code, and imports that only fail at runtime. It exists because of the Armin/Christina friction talk: agents are fast, but they create friction when the project relies on human review to catch routine mistakes. These rules move that friction into tools that fail loudly.

The oxlint defaults are pinned to oxlint `1.67.0`. Keep this version in sync with `rules.json`.

## Source of Truth

`rules.json` is the machine-readable source of truth. It contains:

- `oxlint_version`
- `rules[].name`
- `rules[].category`
- `rules[].severity`
- `rules[].default_enabled`
- `rules[].rationale`

`RULES.md` explains the intent for humans. `audit.sh` reads `rules.json`, not this file.

## Type-Safety

Type-safety rules prevent agents from escaping the type system.

Agents commonly use `any`, non-null assertions, or unsafe calls when they cannot immediately prove a value's shape. That makes the code look complete while moving the risk to runtime. These rules force the agent to model the value honestly or handle the uncertain case.

Rules in this category:

- `typescript/no-explicit-any`
- `typescript/no-non-null-assertion`
- `typescript/no-unsafe-assignment`
- `typescript/no-unsafe-call`
- `typescript/no-unsafe-member-access`

Related TypeScript compiler settings:

- `strict`
- `noUncheckedIndexedAccess`
- `erasableSyntaxOnly`

## Error-Handling

Error-handling rules prevent agents from dropping failures.

Agents often leave promises floating, put async functions where a plain callback was expected, or create empty blocks while reshaping code. These patterns hide failed work from the caller. The code may pass a quick smoke test and still fail under real data.

Rules in this category:

- `typescript/no-floating-promises`
- `typescript/no-misused-promises`
- `eslint/no-empty`

## Code-Quality

Code-quality rules catch unfinished edits and impossible branches.

Agents tend to leave behind unused variables, unused expressions, and constant conditions during iterative edits. These are not style issues. They are signs that a branch, call, or assignment was deleted without finishing the surrounding behavior.

Rules in this category:

- `typescript/switch-exhaustiveness-check`
- `eslint/no-unused-vars`
- `eslint/no-unused-expressions`
- `eslint/no-constant-condition`

## Imports

Import rules prevent module graph problems that TypeScript may not catch early enough.

Agents frequently move code between files and update imports by pattern matching. That can create circular imports or named imports that do not exist. These failures are expensive because they may appear only after bundling, test execution, or a specific runtime path.

Rules in this category:

- `import/no-cycle`
- `import/named`

## Type-Aware Linting

Several rules need type information. The audit recommends `--type-aware` when oxlint is installed but type-aware linting is not active.

Without type-aware linting, rules like unsafe assignment and floating promises cannot see enough of the program to enforce the intended safety boundary.

## Adding New Rules

Add new recommendations by editing `skills/lint-audit/rules.json`.

For each rule:

- Use the canonical oxlint rule name.
- Pick one category: `type-safety`, `error-handling`, `code-quality`, or `imports`.
- Set `severity` to `error` unless there is a clear reason to warn only.
- Set `default_enabled` based on oxlint `1.67.0`.
- Write a one-line rationale that explains why the rule matters for agents.

After changing `rules.json`, update this document if the new rule adds a new category or changes the intent of an existing category.
