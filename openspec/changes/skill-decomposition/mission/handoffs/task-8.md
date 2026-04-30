Implemented Task 8.

Commit: `7b1a9b29118157922456b9956e306ba885dba3dc` (`[task-8] create install.sh`)

Changed:
- Added executable [install.sh](/Users/stuart/.darkfactory/install.sh)
- Updated [README.md](/Users/stuart/.darkfactory/README.md) first-time install steps to use `./install.sh`
- Wrote handoff at [task-8.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/handoffs/task-8.md)

Verification:
- `test -f install.sh && test -x install.sh` passed
- First line is `#!/bin/sh`
- Forbidden GNU-only flag grep returned no matches
- `sh -n install.sh` passed
- Ran installer twice in a writable temp HOME; all 12 symlinks resolved correctly both times

Note: running against the real `~/.claude/skills` was blocked by this worker sandbox with `Operation not permitted`, so I verified behavior in a temp HOME instead. The handoff is left uncommitted because it needs to contain the final commit SHA, which cannot be embedded in the same commit without changing that SHA.