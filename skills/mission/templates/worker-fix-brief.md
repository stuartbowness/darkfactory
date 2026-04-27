# Fix Brief: Task N - <name> (Attempt <attempt+1>)

## What Failed
<describe the observed symptoms -- error messages, unexpected behavior, test failures. Do NOT copy assertion text from the contract. Describe what went wrong, not what the contract expected.>

## Diagnostic Triage
<paste the triage subagent's output from mission/issues/task-N-v<attempt>-triage.md. If triage was skipped or failed, write "Triage unavailable -- diagnose from the validator output below.">

## Validator Output
<relevant error messages, build/test output, review findings -- symptoms only>

## Commit to Fix
<SHA of the commit that failed validation>

## Procedures
- Use `/tdd` to fix the issues described above
- Do not refactor or change anything unrelated
- Commit as a single commit with message: "[task-N] fix: <description of what was fixed>"
- Write handoff report to openspec/changes/<change-name>/mission/handoffs/task-N-fix-v<attempt+1>.md

## Handoff Report Format
- Task: Task N - <name> (Fix attempt <attempt+1>)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
