# Worker Brief: Task 6.1 - Update darkfactory-init with coverage tool checks

## Your Task

Update `.claude/skills/darkfactory-init/SKILL.md` to add coverage tool verification and the new /test-quality-review skill to the dependency checks.

### Changes to Step 2 (Supporting skills)

Add `/test-quality-review` to the list of required supporting skills:
- `~/.claude/skills/test-quality-review/SKILL.md`

Add it to the install command suggestions if missing, and add it to the Step 5 status report.

### Changes to Step 4 (CLI dependencies)

Add a new subsection after the existing CLI tool checks for coverage tool verification. This subsection should:

1. **Detect the project language from build files:**
   - `Cargo.toml` at the project root → Rust
   - `package.json` at the project root → TypeScript/Node
   - `pyproject.toml` or `requirements.txt` at the project root → Python

2. **Check for the corresponding coverage tool:**
   - Rust: Run `cargo tarpaulin --version 2>/dev/null || echo "NOT_INSTALLED"`. If missing, suggest `cargo install cargo-tarpaulin`.
   - TypeScript/Node: Check `package.json` for `c8`, `@jest/coverage`, `istanbul`, or `jest` (which has built-in coverage) in deps or devDeps. If no coverage capability found, suggest `npm install -D c8`.
   - Python: Run `pip show pytest-cov 2>/dev/null || echo "NOT_INSTALLED"`. If missing, suggest `pip install pytest-cov`.

3. **Skip if no recognized language:** If no Cargo.toml, package.json, pyproject.toml, or requirements.txt exists, report: "No recognized language detected -- coverage tool check skipped."

4. **Use the same exit-0 pattern** as existing CLI checks for parallel execution safety: `command --version 2>/dev/null || echo "NOT_INSTALLED"`.

### Changes to Step 5 (Status report)

Add a "Coverage tools" section to the status report:

```
Coverage tools:
  [x] cargo-tarpaulin (Rust detected)
```

or:

```
Coverage tools:
  [ ] pytest-cov (Python detected, install: pip install pytest-cov)
```

or:

```
Coverage tools:
  [~] No recognized language detected -- skipped
```

## Project Context
The scrutiny validator's diff coverage check (check 13) requires language-specific coverage tools. darkfactory-init verifying these upfront prevents coverage checks from failing at validation time due to missing tooling.

## Architecture
darkfactory-init is a skill at `.claude/skills/darkfactory-init/SKILL.md`. It has 5 steps: core skills check, supporting skills check, OpenSpec init, CLI dependencies, and status report. This task adds to steps 2, 4, and 5.

## What Previous Tasks Built

### Tasks 1.1-5.1: Scrutiny validator prompt with 14 checks
- Check 13 (diff coverage) uses cargo tarpaulin, jest --coverage/c8, or pytest --cov
- Check 14 invokes /test-quality-review skill

## Downstream Consumers
No later tasks depend on this. This is the last feature task.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-6.1] Add coverage tool checks and test-quality-review skill to darkfactory-init"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-6.1.md

## Handoff Report Format
- Task: Task 6.1 - Update darkfactory-init with coverage tool checks
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
