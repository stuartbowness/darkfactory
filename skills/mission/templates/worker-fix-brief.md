# Fix Brief: Task N - <name> (Attempt <attempt+1>)

## What Failed
<describe the observed symptoms -- error messages, unexpected behavior, test failures. Do NOT copy check text from the contract. Describe what went wrong, not what the contract expected.>

## Diagnostic Triage
<paste the triage subagent's output from mission/issues/task-N-v<attempt>-triage.md. If triage was skipped or failed, write "Triage unavailable -- diagnose from the validator output below.">

## Validator Output
<relevant error messages, build/test output, review findings -- symptoms only>

## Commit to Fix
<SHA of the commit that failed validation>

## Procedures

**Before writing any code**, invoke the `<task Skill: value, or /tdd if no Skill: field>` skill. This is mandatory — do not skip it, do not treat it as optional guidance. The skill contains the workflow you must follow for this fix. Invoke it now, read the full instructions it returns, and follow its phases in order.

<If task has Constraints, one line per constraint skill, e.g.:>
Also invoke `/<constraint skill name>` and follow its rules alongside the primary methodology.

<If task Skill: is /frontend and browser_tools is false, include this line:>
Browser tools are disabled for this mission. Skip the chrome-devtools-mcp visual verification step in the /frontend workflow.

- Do not refactor or change anything unrelated
- Commit as a single commit with message: "[task-N] fix: <description of what was fixed>"
- Write handoff report to openspec/changes/<change-name>/mission/handoffs/task-N-fix-v<attempt+1>.md

## Handoff Report Format
- Task: Task N - <name> (Fix attempt <attempt+1>)
- Commit SHA: <the full SHA>
- Approach: <what you fixed and why>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
