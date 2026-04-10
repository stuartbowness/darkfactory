## 1. User-Test Validator

- [x] 1.1 Replace the Step 6 placeholder in the /mission skill with user-test validation: spawn a Claude subagent that receives the commit SHA and contract assertions, reads the diff to decide API tests vs browser tests vs both, runs HTTP requests and/or agent-browser checks against each assertion, and writes a report to mission/validations/task-N-usertest.md -- on failure, the existing fix loop handles retries with both validators re-running
