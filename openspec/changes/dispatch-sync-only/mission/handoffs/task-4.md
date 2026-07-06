Done.

Committed the docs-only change:

`070c683fcdef2fd52c1a8f6dcc93835723e75229`  
`[task-4] Update dispatch README and CLAUDE.md for synchronous-only`

Updated:
- [README.md](/home/ironbank/.darkfactory/tools/darkfactory-dispatch/README.md)
- [CLAUDE.md](/home/ironbank/.darkfactory/tools/darkfactory-dispatch/CLAUDE.md)

Wrote the uncommitted handoff to [task-4.md](/home/ironbank/.darkfactory/openspec/changes/dispatch-sync-only/mission/handoffs/task-4.md).

Checks run:
- `cargo run --quiet -- --help`: source-built CLI has no `--async` and no `status`/`wait`/`result`.
- `rg` checks for removed async/store/status terms.
- `git diff --check`.
- `git show --stat --name-only HEAD`.

Concern: the installed `darkfactory-dispatch` on PATH still appears stale and showed the old async CLI. I did not reinstall it because this task was docs-only.