## 1. Scrutiny Validator

- [x] 1.1 Add scrutiny validator dispatch to the /mission skill: after reading a worker handoff, spawn a Claude subagent that receives the commit SHA and contract assertions, runs build/test/lint + /review + /code-simplify (+ /security-review for sensitive tasks), checks each assertion, and writes a report to mission/validations/task-N-scrutiny.md

## 2. Fix Loop

- [x] 2.1 Add the fix loop to the /mission skill: on scrutiny failure, write failure details to mission/issues/, assemble a fix brief (failure context + commit SHA, no contract), dispatch a fix worker, then re-run both validators -- with circuit breaker after 3 attempts that asks the human to retry, skip, or abort
