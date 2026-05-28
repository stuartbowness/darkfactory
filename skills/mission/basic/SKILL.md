---
name: mission-basic
description: Lightweight single-task build-validate-fix loop. No planning ceremony, no multi-agent orchestration, no human gates. Retains TDD, contract thinking, and fix loops from the full mission.
---

# Basic Mission Mode

Single-session, single-task, fully autonomous. You are the builder, validator, and fixer.

## Rules

1. **Contract before code.** Write the mini-contract before any implementation. Do not revise it to match what you built.
2. **Re-run all gates after every fix.** Not just the failed one. Regressions hide in passing gates.
3. **Circuit breaker at 3.** After 3 failed fix attempts, stop. Commit whatever you have and report.
4. **No human interaction.** Do not ask the user questions. Work from the instruction alone.
5. **Single commit.** All changes go in one commit at the end, after validation passes (or circuit breaker triggers).

---

## Phase 0: Orient

### Parse Input

Extract from the args string:
- `--no-tdd` flag: if present, skip TDD workflow in Phase 1
- `--instruction <path>`: if present, read that file as the task description
- Remaining text (after removing flags): use as inline task description

Priority for task description:
1. `--instruction <path>` (read file)
2. Inline text from args
3. `instruction.md` in current working directory (DeepSWE convention)
4. If none found: ask the user (only exception to Rule 4)

### Explore Codebase

1. Detect the project language and tooling. Check for: `package.json`, `Cargo.toml`, `pyproject.toml`, `go.mod`, `Makefile`, `CLAUDE.md`.
2. Identify the commands for: build, test, lint, format. Store these for Phase 2.
3. Read 3-5 source files most relevant to the task (use grep/find to locate them from keywords in the instruction).
4. Understand existing test patterns: where tests live, what framework is used, how to run a single test file.

### Generate Mini-Contract

Write 3-8 observable behavior checks derived from the instruction. Print them. Format:

```
## Contract
1. [execution] <project test command> passes with 0 failures
2. [execution] <specific behavioral check from instruction>
3. [output] <specific output or file content to verify>
...
```

Rules for the contract:
- Check 1 is always: the project's existing test suite still passes (no regressions).
- Remaining checks: one per concrete requirement in the instruction. Each "should", "must", or expected outcome becomes a check.
- Prefer `[execution]` (run a command) over `[output]` (check file contents) over `[inspection]` (read code).
- Cap at 8 checks. If the instruction has more than 8 distinct requirements, pick the most important.
- Do not include "the code compiles" as a contract check (that is validation gate 1, not a contract item).

---

## Phase 1: Build

### With TDD (default)

Invoke the `/tdd` skill. Follow its workflow, but self-answer the planning questions:

- "What interface changes are needed?" -- Answer from the instruction and your codebase exploration.
- "Which behaviors to test?" -- Answer from the mini-contract checks 2-8.
- "User approval on the plan?" -- Approve it yourself. Do not ask the user.

Then follow TDD's tracer bullet loop: one test, one implementation, repeat. The mini-contract checks map to behaviors to test.

### Without TDD (`--no-tdd`)

Implement directly:
1. Write the feature/fix based on the instruction.
2. After implementation is complete, write tests for the key behaviors.
3. Run tests to verify they pass.

### Either path

Do not commit yet. Staging is fine. Committing happens in Phase 4 after validation passes.

---

## Phase 2: Validate

Run these 5 gates in order. Stop at the first FAIL (no point running later gates if build is broken).

### Gate 1: Build

Run the project's build command. Must exit 0.
- TypeScript: `npm run build` or `tsc --noEmit`
- Rust: `cargo build`
- Go: `go build ./...`
- Python: skip (no compile step) or `python -m py_compile <changed files>`

If no build command exists, SKIP this gate.

### Gate 2: Tests

Run the project's test command. Must exit 0.
- Parse output for pass/fail/skip counts.
- If any test fails, this gate is FAIL. Note which tests failed and their error output.

### Gate 3: Lint

Run the project's lint command.
- TypeScript: `eslint .` or `npm run lint`
- Rust: `cargo clippy -- -D warnings`
- Go: `golangci-lint run` or `go vet ./...`
- Python: `ruff check .` or `flake8`

Errors = FAIL. If no lint command exists, SKIP.

### Gate 4: Contract Checks

Walk through each mini-contract check:
- `[execution]` checks: run the command, verify expected behavior.
- `[output]` checks: read the file or output, verify expected content.
- `[inspection]` checks: read the code, verify the property holds.

Mark each check PASS or FAIL. If any FAIL, this gate is FAIL.

### Gate 5: Behavioral Spot-Check

Pick the 1-2 most important contract checks that are `[execution]` type. Actually run them independently (not as part of the test suite). This catches cases where tests pass but the actual runtime behavior is wrong (e.g., test mocks hiding a real failure).

If no `[execution]` checks exist, SKIP this gate.

### Result

- **All gates PASS (or SKIP):** Proceed to Phase 4 (Finalize).
- **Any gate FAIL:** Proceed to Phase 3 (Fix Loop).

---

## Phase 3: Fix Loop

Track attempt number. The initial build (Phase 1) is attempt 1. Each fix iteration increments it.

### On Failure

1. **Diagnose.** Read the error output from the failed gate. Identify the root cause: which file, which line, what went wrong.
2. **Fix.** Apply the minimal change to address the root cause. Do not refactor unrelated code.
3. **Re-validate.** Go back to Phase 2 and run ALL 5 gates (not just the failed one).

### Circuit Breaker

After attempt 3 fails validation:
- Stop the fix loop.
- Proceed to Phase 4 with status PARTIAL.
- The commit will contain whatever progress was made.

---

## Phase 4: Finalize

### Commit

Create a single commit with all changes:
```
git add -A && git commit -m "[basic] <short description>"
```

If nothing to commit (no changes made), skip the commit.

### Report

Print a summary:

```
## Result: PASS | PARTIAL

**Task:** <one-line description>
**Files changed:** <list>
**Tests:** <N passed, M failed, P skipped>
**Contract:** <N/M checks passed>
**Fix attempts:** <count, 0 if first try passed>
```

If PARTIAL, also list which contract checks still fail and the last error for each.
