## Context

The worker loop (previous change) dispatches workers and tracks progress but skips validation. This change inserts scrutiny validation between worker completion and task marking. It also adds the fix loop -- the self-healing mechanism that retries failed tasks.

The scrutiny validator is a Claude subagent spawned via the Agent tool. It has clean context (never seen the code before), receives only the commit SHA and contract assertions, and runs a set of checks. It does NOT see the proposal, design, or worker's reasoning.

## Goals / Non-Goals

**Goals:**
- Spawn a scrutiny validator subagent after each worker completes
- Validator runs: build, test, lint, /review against contract, /code-simplify against diff
- Validator runs /security-review for tasks touching auth or sensitive paths
- Validator writes report to mission/validations/task-N-scrutiny.md
- On failure: orchestrator spawns a fix worker with failure details, then re-validates
- Circuit breaker: 3 attempts per task, then ask human
- Re-run BOTH validators after any fix (even though user-test validator doesn't exist yet -- the logic should be in place)

**Non-Goals:**
- User-test validation (change: usertest-validation)
- Demo video (change: completion-phase)

## Decisions

### Scrutiny validator is a Claude subagent via Agent tool
The validator is spawned with a prompt containing: the commit SHA, the contract assertions for this task, and instructions for what checks to run. It uses Read, Bash, and Grep tools to examine the diff and run commands. It writes its report to disk.

**Alternative considered:** Running the checks directly in the orchestrator session. Rejected because the validator should have clean context with no knowledge of the implementation approach. A subagent starts fresh.

### Validator prompt includes the check sequence
The validator is told exactly what to do: (1) git show the commit, (2) run build/test/lint, (3) run /review on the diff, (4) run /code-simplify on the diff, (5) check each contract assertion against the code, (6) write pass/fail per assertion with evidence.

**Alternative considered:** Letting the validator decide what checks to run. Rejected because consistency matters -- every task gets the same scrutiny.

### Fix worker receives failure details but not the contract
When validation fails, the orchestrator spawns a fix worker. The fix worker gets: the original task description, the specific failures from the validator report, and the commit SHA of the broken code. It does NOT get the contract -- fixes are guided by what the validator reported, maintaining adversarial separation.

**Alternative considered:** Giving the fix worker the contract so it can self-validate. Rejected because that defeats the adversarial model.

### Circuit breaker at 3 attempts
After 3 worker attempts on the same task (original + 2 fixes), the orchestrator stops and asks the human. The failures are logged in mission/issues/task-N-v1.md, v2.md, v3.md. This prevents infinite retry loops on tasks that need human judgment.

## Risks / Trade-offs

- **[Validator may be too strict or too lenient]** -> The validator prompt quality determines this. We iterate on the prompt. Starting strict is safer than starting lenient.
- **[Build/test may fail for reasons unrelated to this task]** -> The validator checks the diff, not the entire codebase. Pre-existing failures should be noted but not block.
- **[Fix worker may introduce new bugs]** -> Re-running both validators after a fix catches this. The circuit breaker prevents infinite loops.
