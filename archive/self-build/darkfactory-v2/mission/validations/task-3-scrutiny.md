# Task 3 Scrutiny Validation: Mission Status Skill

**Commit:** 0af6e36646b6dcdd56ef1bd7598d82b6c003cb83
**File reviewed:** `darkfactoryV2/.claude/skills/mission-status/SKILL.md` (122 lines, new file)

## Assertion Results

### 1. File exists with valid YAML frontmatter (name + description)
**PASS**

The file exists at `darkfactoryV2/.claude/skills/mission-status/SKILL.md`. Lines 1-4 contain valid YAML frontmatter delimited by `---` fences:
```yaml
name: mission-status
description: Check the progress of an active Darkfactory mission. Read-only -- does not modify any files.
```
Both `name` and `description` fields are present and non-empty.

### 2. Skill is read-only (does not create, modify, or delete any files)
**PASS**

The skill explicitly states read-only constraints in three places:
- Line 8: "This skill is strictly **read-only** -- it MUST NOT create, modify, or delete any files."
- Line 120: "**Read-only:** This skill MUST NOT create, modify, or delete any files. Every operation is a read or a glob search."
- Line 121: "**Non-intrusive:** This skill does not interact with the running mission. It only reads files on disk."

The procedure (Steps 1-5) uses only Glob searches and Read operations. No step instructs the agent to write, edit, create, or delete anything.

### 3. Skill searches for active missions via mission/ directories in OpenSpec changes
**PASS**

Step 1 (lines 17-31) instructs the agent to glob for `openspec/changes/*/mission/config.yaml`. This pattern correctly discovers active missions by looking for `mission/` directories inside OpenSpec change directories.

### 4. Skill handles zero, one, or multiple active missions
**PASS**

Step 1 (lines 28-31) explicitly handles all three cases:
- **No missions found:** "Report 'No active missions found.' and stop."
- **One mission found:** "Use it automatically."
- **Multiple missions found:** "List them by change name and ask the user which one to inspect. Wait for their answer before continuing."

### 5. Skill displays: mission name, tasks completed vs total, current task status, validation results per completed task, and recent activity from progress.md
**PASS**

The formatted output template (lines 84-101) includes all required elements:
- **Mission name:** `Mission: <change name>` (line 85)
- **Tasks completed vs total:** `Progress: <completed>/<total> tasks` (line 89)
- **Current task status:** Tasks section shows `[x]`/`[ ]` checkboxes with in-progress annotations (lines 92-97)
- **Validation results per completed task:** Scrutiny, User-test, and Demo results shown per task (lines 93-94, gathered in Step 3)
- **Recent activity from progress.md:** "Recent activity" section with last 5 entries from progress.md (lines 99-100, gathered in Step 2 line 51)

### 6. Skill shows a formatted progress display (not just raw file dumps)
**PASS**

Step 5 (lines 82-117) defines a structured, formatted output with:
- A visual progress bar with precise rendering rules (30 chars wide, `=` fill with `>` leading edge, percentage display)
- Structured task lines with conditional annotations (validation results, in-progress status, next-task marker, fix attempt counts)
- A "Recent activity" section limited to the last 5 entries
- Clear formatting rules for each element rather than dumping raw file contents

## Overall: PASS (6/6 assertions satisfied)

The mission-status skill is a well-structured, read-only reporting skill. It discovers missions via glob patterns, handles edge cases (zero/one/multiple missions, missing files), and presents a formatted progress display with all required information elements.
