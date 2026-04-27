# Worker Brief: Task 1.1 - Language detection and deterministic pre-passes

## Your Task

Add a "Deterministic Pre-passes" section to the scrutiny validator prompt at `.claude/skills/mission/prompts/scrutiny-validator.md`. This section runs before the existing 11-check sequence and produces named facts that later checks reference.

The pre-pass section must include three sub-steps:

### 1. Language detection
Detect the project's primary language from build files at the repo root:
- `Cargo.toml` → Rust
- `package.json` → TypeScript/Node
- `pyproject.toml` or `requirements.txt` → Python

Store the detected language as a fact (e.g., "Detected language: Rust").

### 2. File size count
Count lines on every source file the task touched (from `git show <commit-sha> --stat`). Source file extensions:
- Rust: `.rs`
- TypeScript/Node: `.ts`, `.tsx`, `.js`, `.jsx`
- Python: `.py`

Exclude standalone test files:
- Rust: files in `tests/` directory
- TypeScript: `*.test.ts`, `*.spec.ts`, files in `__tests__/`
- Python: `test_*.py`, `*_test.py`, files in `tests/`

But do NOT exclude `#[cfg(test)]` blocks inside production Rust files -- those count toward the line total.

Store results as: "File sizes: <filename>: <lines>, <filename>: <lines>, ..."

### 3. Error handling smell grep
Grep changed production lines (from the diff, excluding test files) for language-specific anti-patterns:

**Rust patterns to flag:**
- `.unwrap()`, `panic!()`, `todo!()`, `unimplemented!()`, `.expect("")` (empty string)

**Rust patterns to NOT flag:**
- `.unwrap_or()`, `.unwrap_or_default()`, `.unwrap_or_else()`, `.expect("non-empty message")`

**TypeScript patterns to flag:**
- `catch {}` (empty), `catch (e) {}` (unused variable), `as any`, `// @ts-ignore`, `// @ts-expect-error` without explanation, `.catch(() => {})` (arrow that does nothing)

**TypeScript patterns to NOT flag:**
- `catch` with a handler body, `as unknown`

**Python patterns to flag:**
- `except: pass`, `except Exception: pass`, `type: ignore` without a specific code, bare `raise` without context

**Python patterns to NOT flag:**
- `except SpecificError:` with a handler

Count the matches and compute per-KLOC density: `count / (changed_production_lines / 1000)`.

Store results as: "Error handling smells: <count> matches in <lines> changed production lines (<N> per KLOC). Locations: <file:line> <pattern>, ..."

### 4. Test runner output parsing
After the test command runs (existing check 3), parse its summary line:
- Rust: `test result: ok. N passed; M failed; P ignored`
- Jest: `Tests: N passed, M failed, P skipped, Q total`
- Vitest: similar to Jest format
- Pytest: `N passed, M failed, P skipped`

Store results as: "Test runner: <passed> passed, <failed> failed, <skipped> skipped (<pass_rate>% pass rate)"

### Structure requirement
The pre-pass section must go BEFORE the existing "## Check Sequence" heading. Results must be stored as named facts that later checks can reference by name, not inlined into individual check descriptions. The existing 11 checks remain unchanged in this task -- later tasks will modify them to reference these facts.

## Project Context
Darkfactory missions produce code that passes the scrutiny validator but scores lower on test quality than human-iterated code. The ironbench benchmark showed -2 on test quality, -1 on architecture, -1 on robustness, and -1 on maintainability. This mission adds test health checks to close those gaps. This task builds the foundation that all subsequent checks depend on.

## Architecture
The scrutiny validator is a single markdown prompt file at `.claude/skills/mission/prompts/scrutiny-validator.md`. It runs as a Claude subagent during Phase 8 of every mission. The design calls for a "Deterministic Pre-passes" section that gathers data via shell commands before the LLM check sequence begins. This keeps the prompt organized: gather data first, evaluate second.

## Key Terms
- **Scrutiny validator**: The first validator type that runs build, test, lint, code review, code simplify, and test health checks against the contract.
- **Error handling smell**: A language-specific anti-pattern in error handling code counted per KLOC of changed code.
- **File size pre-pass**: A deterministic line count on touched files injected into the module depth review.
- **Self-gating check**: A scrutiny check that naturally skips itself when the diff contains no relevant files.

## What Previous Tasks Built
This is the first task. No prior code has been committed for this mission.

## Downstream Consumers
- Task 2.1 references test runner output parsing results
- Task 2.2 references file size count results
- Task 2.3 references error handling smell results
- Task 3.1 uses language detection for grep pattern selection
- Task 3.2 uses language detection for coverage tool selection

Minimum integration test: after this task, the pre-pass section exists and a subsequent check can reference a named fact from it.

## Procedures
- Use `/tdd` to implement this task
- Commit all changes (code + tests) in a single commit with message: "[task-1.1] Add language detection and deterministic pre-passes to scrutiny validator"
- Write your handoff report to openspec/changes/scrutiny-test-health/mission/handoffs/task-1.1.md

## Handoff Report Format
- Task: Task 1.1 - Language detection and deterministic pre-passes
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
