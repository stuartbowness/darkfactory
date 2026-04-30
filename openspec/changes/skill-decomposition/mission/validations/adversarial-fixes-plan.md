# Adversarial Review Fixes Plan
# PR #110 — skill-decomposition

Source: adversarial-claude.md review (2026-04-27)

---

## Fixes

### F1 — install.sh: exit 1 when no skills found
**Finding:** C2
**File:** install.sh
After the loop, if `count` is 0 exit with an error:
```sh
if [ "$count" -eq 0 ]; then
    echo "Error: no skills found in $SKILLS_DIR" >&2
    exit 1
fi
```

---

### F2 — mission/SKILL.md: reset engine to config.yaml after fallback
**Finding:** C3
**File:** skills/mission/SKILL.md — Phase 8 Step 3
Add explicit instruction: after a Codex→Claude fallback, before starting the next task re-read `engine` from `config.yaml` to reset it to the configured value.

---

### F3 — migrate.sh: fix dirname $0 unreliability
**Finding:** W1
**File:** migrate.sh
Replace `$(dirname "$0")` with:
```sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
"$SCRIPT_DIR/install.sh"
```

---

### F4 — adversarial/SKILL.md: explicit wait step using darkfactory-dispatch wait
**Finding:** W3
**File:** skills/adversarial/SKILL.md — Step 4
Replace "wait for the background Codex dispatch to complete" with explicit instruction to run `darkfactory-dispatch wait <dispatch-id>` before reading the Codex report file.

---

### F5 — mission/SKILL.md: resolve base-branch from config.yaml in Phase 9
**Finding:** W4
**File:** skills/mission/SKILL.md — Phase 9
Before the `git merge-base` command, add: "Read `base_branch` from `openspec/changes/<change-name>/mission/config.yaml` and use it in place of `<base-branch>`."

---

### F6 — mission/SKILL.md: record pre-task SHA before dispatching
**Finding:** W5
**File:** skills/mission/SKILL.md — Phase 8 Step 2
Add before the dispatch command: "Record the current HEAD SHA: `git rev-parse HEAD`. This is the pre-task SHA used in Step 7."

---

### F7 — mission/SKILL.md: replace date +%s with portable alternative
**Finding:** W6
**File:** skills/mission/SKILL.md — Phase 7
Replace `$(date +%s)` with `$(date -u +%Y%m%d%H%M%S)`. Works on macOS BSD date and GNU date.

---

### F8 — migrate.sh: warn on non-symlink entries
**Finding:** N2
**File:** migrate.sh
After the `[ -L "$target" ]` branch, add:
```sh
elif [ -e "$target" ]; then
    echo "warning: $target exists and is not a symlink — skipping"
fi
```

---

### F9 — adversarial/SKILL.md: exclude archive path from mission detection
**Finding:** N3
**File:** skills/adversarial/SKILL.md — Report Paths section
When globbing for `openspec/changes/*/mission/`, skip any match whose path contains `openspec/changes/archive/`.

---

### F10 — grill/SKILL.md: fix trigger phrase in description
**Finding:** N5
**File:** skills/grill/SKILL.md
Change "mentions 'grill me'" to "mentions 'grill'" in the description field.

---

### F11 — README.md: use correct openspec package name
**Finding:** N6
**File:** README.md
Replace any reference to `npm install -g openspec` with `npm install -g @fission-ai/openspec`.

---

### F12 — Global config.yaml + model override for dispatch and adversarial
**Finding:** N7
**Files:** skills/dispatch/SKILL.md, skills/adversarial/SKILL.md, new file: config.yaml
Create `~/.darkfactory/config.yaml` (or `config.yaml` at repo root) with a `default_model` key.
Both `dispatch` and `adversarial` skills should read this file at startup and use `default_model` if present, falling back to `gpt-5.5` if absent.
Document the config file in README.md.

---

## Also fix (known pre-existing issue)

### F13 — adversarial/SKILL.md: remove --json from --async dispatch
**Finding:** Issue #112 (filed separately)
**File:** skills/adversarial/SKILL.md — Step 2
Remove `--json` from the dispatch command. Use `--async` only, capture the `dispatch_id` from the `async_started` JSON, and use `darkfactory-dispatch wait <id>` in Step 4.

---

## Skipped

| Finding | Reason |
|---------|--------|
| C1 — install.sh rm -f on non-symlinks | Target is always a symlink in practice |
| W2 — readlink sudo/HOME mismatch | sudo not used for Claude Code |
| W7 — .git in writable roots | Codex needs git write access to commit |
| W8 — circuit breaker no escalation cap | Human oversight on every failure is intentional |
| N1 — no shebang comment | Covered by CLAUDE.md |
| N4 — capability enumeration | Deliberate firewall — CLI owns this |
