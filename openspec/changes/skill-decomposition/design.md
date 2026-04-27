# Design: skill-decomposition

## Final directory state

```
~/.darkfactory/
  skills/
    adversarial/SKILL.md        # New — standalone dual-agent review
    dispatch/SKILL.md           # New — user-facing darkfactory-dispatch wrapper
    grill/SKILL.md              # Forked from grill-me (name change only)
    language/SKILL.md           # Forked from ubiquitous-language (name change only)
    mission/
      SKILL.md                  # Moved from .claude/skills/mission/ (unchanged)
      templates/                # Moved unchanged
      prompts/                  # Moved, scrutiny-validator.md gets 1 ref update
    mission-init/SKILL.md       # Renamed from darkfactory-init + rewritten
    mission-resume/SKILL.md     # Moved from .claude/skills/mission-resume/ (unchanged)
    mission-status/SKILL.md     # Moved from .claude/skills/mission-status/ (unchanged)
    prd/SKILL.md                # Forked from write-a-prd (name change only)
    security/SKILL.md           # Forked from security-review (strip toyo sections)
    tdd/
      SKILL.md                # Forked from tdd (name change only)
      deep-modules.md         # Forked unchanged
      interface-design.md     # Forked unchanged
      mocking.md              # Forked unchanged
      refactoring.md          # Forked unchanged
      tests.md                # Forked unchanged
    test-quality/SKILL.md     # Forked from test-quality-review (name change only)
  install.sh                  # New — symlinks skills/ to ~/.claude/skills/
  tools/darkfactory-dispatch/ # Unchanged
  docs/                       # Unchanged
  openspec/                   # Unchanged
  README.md                   # Rewritten: new install, new skill names, owned skills
  UBIQUITOUS_LANGUAGE.md      # Minor: "separate skill" → "owned skill" for test-quality
```

**Deleted:**
- `.claude/skills/` — entire directory
- `.codex/skills/` — entire directory (duplicates)

## Key decisions

**skills/ at repo root, not .claude/skills/**
Install.sh manages one symlink per skill from `~/.claude/skills/<name>` → `~/.darkfactory/skills/<name>`. This separates what's owned from how Claude Code discovers it. The `.claude/` directory inside the repo becomes empty and can be removed.

**Naming convention**
Forked and new skills use bare names (no prefix): `grill`, `language`, `prd`, `tdd`, `test-quality`, `security`, `adversarial`, `dispatch`. The three mission utility skills keep their `mission-` prefix (`mission-resume`, `mission-status`, `mission-init`) since that prefix is part of their established identity and changing it would break existing workflows.

**Fork, don't symlink external skills**
External symlinks (to `~/.darkfactory-skills/`) meant we couldn't improve skills without touching a second repo and they were silently out of date. Forking makes this repo self-contained. The external `layercodedev/skills` repo dependency is eliminated entirely.

**test-quality is a name-only fork**
The `test-quality-review` skill has no toyo-specific content to strip. The only change is the frontmatter name field. Content is preserved exactly.

**security strips toyo sections only**
The `security-review` skill has toyo-specific sections (references to toyo's review conventions and severity levels). These are removed. The general security review logic is preserved.

**adversarial is written fresh, not forked**
The existing adversarial reviewer logic lives in `mission/prompts/adversarial-reviewer.md` (used only by mission Phase 9). The new `/adversarial` standalone skill embeds its own reviewer instructions. Mission Phase 9 continues to use `prompts/adversarial-reviewer.md` unchanged — the mission-orchestration spec prohibits behavior changes to mission.

**dispatch is a thin skill**
`/dispatch` wraps `darkfactory-dispatch` with a human-readable interface. It handles prompt file creation, JSON result parsing, and error surfacing. It does not add retry logic or state management — those are handled by the binary.

**mission-init checks symlinks, not source locations**
Old `darkfactory-init` checked two possible install locations (`.claude/skills/` inside the repo and `~/.darkfactory-skills/`). After this refactor, there is exactly one install path: `install.sh`. The new `/mission-init` verifies that `~/.claude/skills/<name>` is a symlink resolving to `~/.darkfactory/skills/<name>` for each of the 12 skills. If broken or missing, it points to `install.sh`.

## Module concerns

Each skill is a self-contained module. Its public interface is the frontmatter (name, description, user-invokable). Its hidden implementation is the SKILL.md body. No skill file imports from another at the filesystem level.

The mission skill is intentionally monolithic (one SKILL.md, ~825 lines). Per spec, it is not decomposed into sub-skills. Its companion files (templates/, prompts/) are read on demand and remain nested under `skills/mission/`.

## Reference updates required

Two files need content changes beyond a move or name change:

| File | Change |
|------|--------|
| `skills/mission/SKILL.md` | `/grill-me` → `/grill`, `/write-a-prd` → `/prd`, `/ubiquitous-language` → `/language`, `/darkfactory-init` → `/mission-init` (all occurrences) |
| `skills/mission/prompts/scrutiny-validator.md` | `/test-quality-review` → `/test-quality` (check 14) |

The SKILL.md ref update was added during the coverage audit to implement PRD user story 10. All four old skill names appear in the SKILL.md body where the mission orchestrator invokes sub-skills. After Task 1a, those references resolve to the new bare names installed via `install.sh`.

## install.sh design

```sh
#!/bin/sh
DARKFACTORY="$HOME/.darkfactory"
SKILLS_DIR="$DARKFACTORY/skills"
CLAUDE_SKILLS="$HOME/.claude/skills"

mkdir -p "$CLAUDE_SKILLS"

for skill_dir in "$SKILLS_DIR"/*/; do
    name=$(basename "$skill_dir")
    target="$CLAUDE_SKILLS/$name"
    ln -sf "$skill_dir" "$target"
    echo "linked: $target -> $skill_dir"
done

echo "Done. $(ls "$SKILLS_DIR" | wc -l | tr -d ' ') skills installed."
```

- `#!/bin/sh` not `#!/bin/bash` — POSIX portable
- `ln -sf` — creates or overwrites; idempotent
- Loops over `skills/*/` — adding a new skill directory automatically gets picked up on next run without editing the script
- Does not touch any existing `~/.claude/skills/` entry that isn't a Darkfactory skill

## README rewrite scope

Three sections change:

1. **Setup / First-time install** — replace the block of 4 manual `ln -sf` commands with:
   ```bash
   git clone https://github.com/layercodedev/darkfactory.git ~/.darkfactory
   ~/.darkfactory/install.sh
   ```

2. **Skills table** — replace the 4-row table (mission, mission-resume, mission-status, darkfactory-init) with the full 12-skill table.

3. **Required external skills** — this table listed 6 skills from `layercodedev/skills`. After the refactor, all are owned. Replace the section with a brief note that `/review` and `/simplify` are built-in Claude Code skills (always available), and `/test-app` and `/demo-video` are optional project-specific skills the user writes themselves.

## Task order

Tasks decompose naturally into these groups:

1. **Move existing skills** — mission (+ templates/ prompts/), mission-resume, mission-status
2. **Fork external skills** — grill, language, prd, tdd (+ companions), test-quality (+ scrutiny-validator.md ref update), security
3. **Rewrite init**
4. **Write new skills** — adversarial, dispatch
5. **Create install.sh**
6. **Update README.md and UBIQUITOUS_LANGUAGE.md**
7. **Closing tasks** — smoke test, cleanup, module shape re-check, docs

Groups 1–2 are the bulk of the file work and are low-risk (move/copy with name changes). Groups 3–4 are the only net-new writing. Groups 5–6 are targeted edits. Each group maps cleanly to one or two tasks.
