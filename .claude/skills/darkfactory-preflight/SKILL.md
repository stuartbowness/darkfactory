---
name: darkfactory-preflight
description: Verify project is ready for a Darkfactory mission. Checks git, GitHub remote, OpenSpec, and Codex. Auto-fixes what it can, warns about the rest.
---

# Darkfactory Pre-flight Check

Verify this project is ready for a `/mission`. Auto-fix what's missing, warn about the rest.

## Checks

1. **Git repo:** Run `git status`. If not a git repo, run `git init`.

2. **GitHub remote:** Run `git remote -v`. If no remote exists, warn: "No GitHub remote found. The mission will run but cannot create a draft PR at the end. Add a remote with `git remote add origin <url>` if you want a PR."

3. **OpenSpec:** Check if `openspec/config.yaml` exists. If not, run `openspec init --tools claude,codex`. If the `openspec` CLI is not installed, stop and tell the user: "OpenSpec is required. Install with `npm install -g openspec`."

4. **Codex CLI:** Run `codex --version`. If not found, warn: "Codex CLI not installed. Workers will use Claude subagents instead. Install Codex with `npm install -g @openai/codex` for the default worker engine."

## Output

Report a one-line summary per check:

```
Pre-flight: git [ok] | remote [ok] | openspec [ok] | codex [ok]
```

Or with issues:

```
Pre-flight: git [ok] | remote [MISSING - no draft PR] | openspec [initialized] | codex [ok]
```

If any hard stop (OpenSpec CLI missing), report the error and stop. Do not return "ready" status.

If all checks pass or only have warnings, report "Pre-flight complete. Ready for mission."
