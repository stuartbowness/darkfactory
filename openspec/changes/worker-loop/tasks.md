## 1. Brief Assembly

- [x] 1.1 Add build phase to the /mission skill that reads the next unchecked task from tasks.md, reads proposal.md and design.md, and writes a self-contained brief to mission/briefs/task-N.md (excluding contract.md content)

## 2. Worker Dispatch

- [x] 2.1 Add Monitor-based worker dispatch to the /mission skill that kicks off Codex via `codex exec` in the background, watches for the handoff file, and handles completion, failure, and timeout -- with Claude subagent as fallback when config says `worker: claude`

## 3. Progress Tracking

- [x] 3.1 Add the post-worker loop to the /mission skill that reads the handoff report, extracts the commit SHA, marks the task complete in tasks.md, logs to progress.md, and iterates to the next unchecked task until all tasks are done
