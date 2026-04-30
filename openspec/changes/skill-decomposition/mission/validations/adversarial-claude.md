# Adversarial Review (Claude)

PR: #110 — skill-decomposition: move skills to `skills/`, add `install.sh`/`migrate.sh`

---

## Critical

### C1 — `install.sh`: `rm -f` on non-symlinks silently deletes real files
**File:** `install.sh:18`

```sh
rm -f "$target"
ln -sf "${skill_dir%/}" "$target"
```

`rm -f` removes the target unconditionally before creating the symlink. If `$target` is a regular file or a real directory (for example, a user's hand-crafted `~/.claude/skills/mission/` directory), it is silently deleted. There is no check that the existing entry is a symlink before removing it. A new user who cloned Darkfactory alongside their own skills directory would lose their work without warning.

**Expected behaviour:** check `[ -L "$target" ] || [ ! -e "$target" ]` before removing; otherwise abort with an error or ask confirmation.

---

### C2 — `install.sh`: glob silently produces no results on empty `skills/` directory
**File:** `install.sh:13-14`

```sh
for skill_dir in "$SKILLS_DIR"/*/; do
    [ -d "$skill_dir" ] || continue
```

In POSIX `sh`, if `$SKILLS_DIR/*/` matches nothing (empty directory) the shell passes the literal string `"$SKILLS_DIR/*/"` as the first iteration value. The `[ -d "$skill_dir" ]` guard does fire — the literal path does not exist, so the loop body is skipped — but the script exits 0 and prints `Done. 0 skills installed.` This is fine, but there is no guard in the `if [ ! -d "$SKILLS_DIR" ]` block at line 8 for the case where the directory exists but contains no subdirectories (e.g. a partial checkout). The script exits successfully with `0 skills installed`, which will leave a user thinking the install succeeded when it did not.

**Impact:** Silent failure during first install on a corrupted or partial clone.

---

### C3 — `mission/SKILL.md` (Phase 8 Step 3): fallback path resets engine for _next_ task but the spec says it does not reset the counter
**File:** `skills/mission/SKILL.md` — Phase 8 Step 3

> "On the next task, revert to the configured engine. This fallback is per-task only."

The fallback re-dispatches using the "If engine is `claude`" path inline, but there is no instruction to reset the engine _variable_ being carried forward. If the orchestrator is tracking the current engine in its plan, it may leave `engine = claude` for the next task if it does not explicitly re-read `config.yaml`. Combined with the note that "this fallback is per-task only", the behavior is ambiguous: the skill says revert, but there is no step that explicitly re-reads `config.yaml` before Step 1 of the next task.

**Risk:** All tasks after a Codex fallback run with the Claude engine, silently ignoring `config.yaml`. This matters because Claude tasks block the orchestrator context while Codex tasks run in background.

---

## Warning

### W1 — `migrate.sh`: `dirname "$0"` is unreliable when called via PATH or symlink
**File:** `migrate.sh:36`

```sh
"$(dirname "$0")/install.sh"
```

When `migrate.sh` is invoked as `~/.darkfactory/migrate.sh` from a shell that has `~/.darkfactory` on `PATH`, `$0` is just `migrate.sh` and `dirname "$0"` is `.`. The install.sh call resolves relative to the current working directory, not to `~/.darkfactory`. If the user runs `migrate.sh` from any other directory it fails with "No such file or directory".

**Fix:** Use `SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"` at the top and reference `$SCRIPT_DIR/install.sh`.

---

### W2 — `mission-init/SKILL.md` Step 1: `readlink` path comparison fails on macOS with relative symlinks
**File:** `skills/mission-init/SKILL.md:~line 45`

```bash
target="$(readlink "$link" 2>/dev/null || true)"
if [ "$target" != "$expected" ]; then
```

`ln -sf` in `install.sh` creates symlinks using the path from `${skill_dir%/}`, which expands to `$HOME/.darkfactory/skills/<name>`. On macOS, if `$HOME` contains a shell expansion (e.g. `/Users/alice`) the comparison works. However, `readlink` returns the literal target stored in the inode. If `install.sh` is ever called with a relative path (e.g. `./install.sh` from inside `~/.darkfactory`), `DARKFACTORY` is still set to `$HOME/.darkfactory` (unexpanded) and `${skill_dir%/}` expands to something like `/Users/alice/.darkfactory/skills/mission`. In practice this is fine, but the SKILL.md says the expected value is `$HOME/.darkfactory/skills/<name>` where `$HOME` is evaluated at check time by the shell running the for-loop. If the system `$HOME` differs from the shell that ran `install.sh` (e.g. under `sudo`), all 12 checks report `[!] wrong target`.

**Risk:** False negatives in `/mission-init` on multi-user or sudo installs.

---

### W3 — `adversarial/SKILL.md`: `--async` flag and background Bash call conflict on `--output`
**File:** `skills/adversarial/SKILL.md:~Step 2`

The dispatch command uses `--async` and `--output "<codex-report-path>"`. The `--async` flag is described in the SKILL.md as returning immediately (the caller waits in Step 4). However, `--output` causes the dispatcher to write the report after the process completes. If the dispatcher's async mode does not wait for the process to finish before returning (which is the definition of async), the output file may not exist when Step 4 reads it. The skill tells the Claude reviewer to "wait for the background Codex dispatch to complete" but never explains _how_ to wait — it only says to call `BashOutput` on the shell task ID. Whether `BashOutput` blocks until the process exits depends on how the Bash tool implements background tasks.

**Risk:** Race condition where Step 4 reads a non-existent or partial codex report file.

---

### W4 — `mission/SKILL.md` Phase 9: base-branch variable is never defined before it is used in the abort path
**File:** `skills/mission/SKILL.md` — Phase 9, Dual Adversarial Review section

```bash
BASE_SHA=$(git merge-base <base-branch> HEAD)
if [ -z "$BASE_SHA" ]; then
  echo "ABORT: could not resolve merge-base for <base-branch>..HEAD"
  exit 1
fi
```

`<base-branch>` is a placeholder. The skill never tells the orchestrator how to determine the base branch. Phase 7 does mention `gh api repos/{owner}/{repo} --jq .default_branch` to check whether HEAD is already on the default branch, but the result of that call is not assigned to a named variable that Phase 9 can reference. An orchestrator following the skill literally would paste the literal string `<base-branch>` into the command.

**Risk:** `git merge-base` receives a literal `<base-branch>` argument, fails, and the ABORT message fires at the start of every Phase 9 adversarial review.

---

### W5 — `mission/SKILL.md` Phase 8 Step 7: `pre-task-sha` is never explicitly captured
**File:** `skills/mission/SKILL.md` — Phase 8 Step 7

> "Run `git diff <pre-task-sha>..HEAD` (where pre-task-sha is the commit before this task's first commit, **recorded when the worker was dispatched**)"

Step 2 (Dispatch Worker) does not include an instruction to record the current HEAD SHA before dispatching. The parenthetical says "recorded when the worker was dispatched", but there is no corresponding instruction in Step 2 to run `git rev-parse HEAD` and store it. An orchestrator following the skill literally would arrive at Step 7 with no pre-task-sha recorded.

**Risk:** The codebase delta generation fails or uses the wrong base commit, giving incorrect `## Codebase Deltas` context to subsequent worker briefs.

---

### W6 — `mission/SKILL.md` Phase 7: branch collision name uses `date +%s` (not POSIX-portable)
**File:** `skills/mission/SKILL.md` — Phase 7

> `git checkout -b mission/<change-name>-$(date +%s)`

`date +%s` (Unix epoch seconds) is not specified by POSIX. On macOS with BSD `date` it works, but the CLAUDE.md project instructions explicitly warn against non-portable `date` flags. If a future skill implementation shells out directly, this would fail on systems where `date +%s` is unsupported. The guidance in CLAUDE.md says to avoid GNU-only flags.

**Risk:** Branch creation fails on non-GNU, non-macOS BSD systems.

---

### W7 — `dispatch/SKILL.md`: `.git` added to `writable_roots` is silently ignored in read-only sandboxes
**File:** `skills/dispatch/SKILL.md:~Step 2`

```
--config 'sandbox_workspace_write.writable_roots=[".git"]'
```

This configuration only applies when `--sandbox workspace-write` is active. The `dispatch` skill is user-facing and generic: users may invoke it for tasks that should not write to `.git`. Unconditionally adding `.git` to writable roots means a task prompt that contains a git command (e.g., "rewrite history", "amend commits") would have unrestricted access to the git object store. There is no documentation in the skill that explains why `.git` needs to be writable or that this is intentional.

**Risk:** User dispatches a task that unintentionally or maliciously corrupts git objects, with no warning about the elevated permission.

---

### W8 — `mission/SKILL.md` Phase 5a Step 2 (circuit breaker): "retry increments but does not reset" is ambiguous at attempt >= 4
**File:** `skills/mission/SKILL.md` — Phase 8 Step 5a

> "Retry: Increment the attempt counter by 1 (e.g., attempt 3 becomes attempt 4). Allow one more fix attempt. If it fails again, the circuit breaker triggers again at the new count."

The check condition is `If attempt >= 3`. After retry at attempt 3 → attempt 4, the next failure has `attempt = 4 >= 3`, so the circuit breaker triggers again immediately, presenting the human with the same three options again. This is correct in spirit (human must acknowledge every failure after the initial three), but it means the circuit breaker fires on every single subsequent failure, potentially flooding the human with repeated prompts on long-running fix loops. There is no cap or escalation path after N repeated circuit-breaker firings.

**Risk:** Annoying UX and potential for a human to blindly retry many times, burning tokens with no progress.

---

## Nit

### N1 — `install.sh`: no shebang comment explains why `sh` not `bash`
**File:** `install.sh:1`

The shebang is `#!/bin/sh`. This is intentional per CLAUDE.md POSIX portability requirements, but there is no comment explaining why. A future contributor might "fix" it to `#!/bin/bash` and add bash-isms, breaking compatibility.

---

### N2 — `migrate.sh`: does not check if `target` is a real directory before `rm`
**File:** `migrate.sh:21`

```sh
if [ -L "$target" ]; then
    rm "$target"
```

This only removes symlinks, which is correct and safe. However it silently skips entries that are regular files or real directories with the old skill name. If a user has a regular directory at `~/.claude/skills/grill-me/` (not a symlink), the migrate script leaves it untouched without reporting it. The new `install.sh` will then fail to create the symlink because the directory exists. A warning for the non-symlink case would help.

---

### N3 — `adversarial/SKILL.md`: mission path detection is fragile — glob `openspec/changes/*/mission/` matches archived missions
**File:** `skills/adversarial/SKILL.md:~Report Paths section`

> "If the repo contains a Darkfactory mission directory matching `openspec/changes/*/mission/`, write reports under that mission"

`openspec/changes/archive/*/mission/` would also match this glob if an archived mission has a `mission/` subdirectory. The skill does not exclude the archive path, so on a repo with a completed archived mission the adversarial reviewer might write its report into the archive instead of the active mission.

---

### N4 — `mission/SKILL.md` Phase 4.2: spec files are generated per-capability but the SKILL does not say how to enumerate capabilities
**File:** `skills/mission/SKILL.md` — Phase 4.2

> "Generate `specs/<capability>/spec.md` files (one per capability listed in the proposal)."

The proposal format is not specified in this skill (it is delegated to the openspec CLI). An orchestrator following only this skill would not know how to extract the capability list from the generated `proposal.md` without additional instruction. The `openspec instructions` CLI call in Phase 4 covers this in practice, but the written instruction is ambiguous.

---

### N5 — `grill/SKILL.md`: one-liner description uses "grill me" but the command is `/grill`
**File:** `skills/grill/SKILL.md:description`

> "Use when user wants to stress-test a plan, get grilled on their design, or mentions 'grill me'."

The old skill was `/grill-me`; this is now `/grill`. The trigger phrase "mentions 'grill me'" might cause the skill router to match the old intent keyword against the new command name. Not a functional bug, but the description would be cleaner saying "mentions 'grill'" to match the actual invocation.

---

### N6 — `mission-init/SKILL.md` Step 3: inconsistent npm package name between init and install instructions
**File:** `skills/mission-init/SKILL.md:~line 90 and line 100`

Step 3 checks for openspec with:
```bash
openspec --version 2>/dev/null || echo "NOT_INSTALLED"
```

The install suggestion is:
```
npm install -g @fission-ai/openspec
```

But the older `darkfactory-init/SKILL.md` (being deleted) used `npm install -g openspec`. README.md still references the openspec CLI without specifying the scoped package name. If the package name changed, the README and any user docs that still say `npm install -g openspec` will install the wrong package.

---

### N7 — `dispatch/SKILL.md`: model name `gpt-5.5` is hardcoded
**File:** `skills/dispatch/SKILL.md:~Step 2` and `skills/adversarial/SKILL.md:~Step 2`

The model `gpt-5.5` appears hardcoded in three dispatch skill templates. If the model is renamed, deprecated, or the user wants to override it, there is no configuration path. The `mission/config.yaml` pattern (used in `mission/SKILL.md`) allows engine selection; the standalone `dispatch` and `adversarial` skills do not offer the same flexibility.

---

**Summary**
