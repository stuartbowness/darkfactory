# Worker Brief: Task 6 - Write adversarial skill

## Your Task

Write `skills/adversarial/SKILL.md` from scratch. This is a standalone dual-agent code review skill — independent of the mission system.

**File to create:** `skills/adversarial/SKILL.md`

## Required frontmatter

```yaml
---
name: adversarial
user-invokable: true
description: Run a dual-agent adversarial code review. Dispatches a Codex reviewer in parallel with a Claude reviewer. Both review the same diff independently. Surfaces disagreements after both complete.
---
```

## Required behavior

### Input modes

The skill accepts three input forms:
1. **No argument (default):** Use `git diff HEAD` (staged + unstaged changes in the working tree)
2. **PR number:** `/adversarial 42` — use `gh pr diff 42` to get the diff
3. **File path:** `/adversarial path/to/file.ts` — diff for that specific file only

Determine which mode applies from the argument given.

### Report paths

- If running inside a Darkfactory mission (i.e., `openspec/changes/*/mission/` directory exists in the repo): write reports to `mission/validations/adversarial-codex.md` and `mission/validations/adversarial-claude.md`
- Otherwise: write to a temp location like `/tmp/adversarial-codex-<timestamp>.md` and `/tmp/adversarial-claude.md`, and tell the user the paths

### Step 1: Get the diff

Based on input mode, run one of:
- `git diff HEAD` (no argument)
- `gh pr diff <number>` (PR number given)
- `git diff HEAD -- <path>` (file path given)

If the diff is empty, report "No changes to review" and stop.

### Step 2: Dispatch Codex reviewer (non-blocking)

Dispatch Codex via `darkfactory-dispatch --async` with a reviewer prompt. Use `run_in_background: true` in the Bash tool so it runs in parallel with Step 3.

The prompt for Codex:
```
You are an adversarial code reviewer. Review the following diff for bugs, logic errors, security issues, and missing edge cases. Group findings by severity: critical, warning, nit. Cite file:line for each finding. Write your report to: <codex-report-path>

<diff content>
```

The dispatch command:
```bash
PROMPT_PATH=$(mktemp /tmp/adversarial-prompt.XXXXXX)
cat > "$PROMPT_PATH" << 'PROMPT'
<prompt content above with diff filled in>
PROMPT

darkfactory-dispatch \
  --model gpt-5.5 \
  --effort high \
  --timeout 900 \
  --sandbox workspace-write \
  --prompt-file "$PROMPT_PATH" \
  --output "<codex-report-path>" \
  --cwd "$(pwd)" \
  --json
```

### Step 3: Dispatch Claude reviewer (blocking)

While Codex runs in the background, dispatch Claude via the Agent tool. Pass the same diff and the same reviewer prompt. Claude writes to `adversarial-claude.md`.

### Step 4: Collect results and surface disagreements

After both reviewers complete:
1. Read both reports
2. Find disagreements: findings that appear in one report but not the other, or where severity classifications differ
3. Present to the user:
   - Codex report summary (count by severity: critical/warning/nit)
   - Claude report summary (count by severity)
   - Disagreements section listing items only one reviewer flagged

### Output format presented to user

```
Adversarial review complete.

Codex reviewer: N critical, M warnings, P nits
  Full report: <path>

Claude reviewer: N critical, M warnings, P nits  
  Full report: <path>

Disagreements (flagged by one reviewer only):
- [Codex only] <finding>
- [Claude only] <finding>

Review the full reports and decide what to address.
```

## Acceptance criteria

- `skills/adversarial/SKILL.md` exists and `grep "^name:" skills/adversarial/SKILL.md` returns `name: adversarial`
- `grep -c "gh pr diff" skills/adversarial/SKILL.md` returns at least `1` (PR number input mode documented)
- `grep -c "darkfactory-dispatch" skills/adversarial/SKILL.md` returns at least `1` (codex reviewer via dispatch)
- `grep -c "adversarial-codex.md" skills/adversarial/SKILL.md` returns at least `1` (codex report path documented)
- `grep -icE "critical|warning|nit" skills/adversarial/SKILL.md` returns at least `1` (severity grouping documented)
- Skill dispatches Codex reviewer via `darkfactory-dispatch` before launching Claude reviewer via Agent tool; both receive identical input; disagreements surfaced after both complete

## What Previous Tasks Built

### Task 1: Move mission skills (commit 4f053d0)
- Created: skills/mission/SKILL.md, skills/mission/templates/, skills/mission/prompts/
- Created: skills/mission-resume/SKILL.md, skills/mission-status/SKILL.md

### Task 1a: Update mission skill references (commits 1b951021 + e14fa34)
- Modified: skills/mission/SKILL.md — old names updated, /mission-init call added to Phase 0

### Task 2: Fork grill, prd, language (commits 66500d39 + 2ad413df)
- Created: skills/grill/SKILL.md, skills/prd/SKILL.md, skills/language/SKILL.md

### Task 3: Fork tdd and test-quality (commit 8c12e873)
- Created: skills/tdd/SKILL.md + 5 companions, skills/test-quality/SKILL.md

### Task 4: Fork security (commit c06149050)
- Created: skills/security/SKILL.md

### Task 5: Write mission-init (commit 4a3ed785)
- Created: skills/mission-init/SKILL.md

## Downstream Consumers

- Task C3 (module shape) verifies `skills/adversarial/` exists.

## Procedures

- Write the full SKILL.md following the behavior spec above
- Commit in a single commit with message: `[task-6] write adversarial skill`
- Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-6.md`

## Handoff Report Format

- Task: Task 6 - Write adversarial skill
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <commands and exit codes>
- Concerns: <anything uncertain>
