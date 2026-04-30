# Worker Brief: Task 8 - Create install.sh

## Your Task

Write `install.sh` at the repo root (`/Users/stuart/.darkfactory/install.sh`). This script symlinks all skills from `skills/` to `~/.claude/skills/`.

## Required content

```sh
#!/bin/sh
DARKFACTORY="$HOME/.darkfactory"
SKILLS_DIR="$DARKFACTORY/skills"
CLAUDE_SKILLS="$HOME/.claude/skills"

mkdir -p "$CLAUDE_SKILLS"

count=0
for skill_dir in "$SKILLS_DIR"/*/; do
    name=$(basename "$skill_dir")
    target="$CLAUDE_SKILLS/$name"
    ln -sf "$skill_dir" "$target"
    echo "linked: $target -> $skill_dir"
    count=$((count + 1))
done

echo "Done. $count skills installed."
```

After writing the file, make it executable:
```bash
chmod +x install.sh
```

**Key requirements:**
- First line must be `#!/bin/sh` (not `#!/bin/bash`)
- Uses `ln -sf` — creates or overwrites, idempotent
- Loops over `skills/*/` dynamically — no hardcoded skill list
- Prints one line per skill linked
- Prints a final count summary line
- NO GNU-only flags: no `sed -i`, `date -d`, `stat -c`, `mktemp --suffix`, `readlink -f`, `grep -P`
- Script is POSIX-portable (must work on both macOS/BSD and Debian/Ubuntu/GNU)

## Acceptance criteria

- `test -f install.sh && test -x install.sh` exits 0
- First line of `install.sh` is `#!/bin/sh`
- After running `install.sh`, all 12 symlinks exist:
  - `~/.claude/skills/mission` → `~/.darkfactory/skills/mission`
  - `~/.claude/skills/mission-resume` → `~/.darkfactory/skills/mission-resume`
  - `~/.claude/skills/mission-status` → `~/.darkfactory/skills/mission-status`
  - `~/.claude/skills/mission-init` → `~/.darkfactory/skills/mission-init`
  - `~/.claude/skills/grill` → `~/.darkfactory/skills/grill`
  - `~/.claude/skills/prd` → `~/.darkfactory/skills/prd`
  - `~/.claude/skills/language` → `~/.darkfactory/skills/language`
  - `~/.claude/skills/tdd` → `~/.darkfactory/skills/tdd`
  - `~/.claude/skills/test-quality` → `~/.darkfactory/skills/test-quality`
  - `~/.claude/skills/security` → `~/.darkfactory/skills/security`
  - `~/.claude/skills/adversarial` → `~/.darkfactory/skills/adversarial`
  - `~/.claude/skills/dispatch` → `~/.darkfactory/skills/dispatch`
- Running `install.sh` a second time exits 0 with all 12 symlinks still resolving correctly (idempotency)
- `grep -E "sed -i|date -d|stat -c|mktemp --suffix|readlink -f|grep -P" install.sh` returns no output
- Script output includes one line per skill linked and a final summary line with total count

## What Previous Tasks Built

### Tasks 1-7: All 12 skill directories now exist under `skills/`
- skills/mission/ (SKILL.md + templates/ + prompts/)
- skills/mission-resume/SKILL.md
- skills/mission-status/SKILL.md
- skills/mission-init/SKILL.md
- skills/grill/SKILL.md
- skills/prd/SKILL.md
- skills/language/SKILL.md
- skills/tdd/ (SKILL.md + 5 companions)
- skills/test-quality/SKILL.md
- skills/security/SKILL.md
- skills/adversarial/SKILL.md
- skills/dispatch/SKILL.md

## Procedures

- Write install.sh at the repo root
- Make it executable with `chmod +x install.sh`
- Run it once to verify symlinks are created
- Run it a second time to verify idempotency
- Commit in a single commit with message: `[task-8] create install.sh`
- Write handoff to `openspec/changes/skill-decomposition/mission/handoffs/task-8.md`

## Handoff Report Format

- Task: Task 8 - Create install.sh
- Commit SHA: <the full SHA>
- Approach: <brief explanation>
- Commands run: <any commands and their exit codes>
- Concerns: <anything uncertain>
