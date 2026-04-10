## 1. Demo Video Step

- [x] 1.1 Add a demo video step to the /mission skill between user-test validation and task marking: the orchestrator decides if the task is frontend-facing, spawns a Claude subagent to run /demo-video, saves to mission/validations/task-N-demo.mp4 -- failure is logged but does not block progress

## 2. Mission Completion

- [x] 2.1 Add a completion phase to the /mission skill after the build loop: create a draft PR via /pr with a mission summary (tasks, validation results, demo links, issues, attempt counts) and update the plan status to mission complete
