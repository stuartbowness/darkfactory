# Task 5 User-Test Validation

**Commit:** 4a3ed785a86f7b0871169861ca059a9083627d43  
**Task:** Write mission-init skill  
**Date:** 2026-04-26  
**Result:** PASS

## Assertions

### 1. File exists and name is correct
- `test -f skills/mission-init/SKILL.md` → PASS
- `grep "^name:" skills/mission-init/SKILL.md` → `name: mission-init` (line 2) → PASS

### 2. Forbidden skill names absent
- `grep -cE "grill-me|darkfactory-init|test-quality-review" skills/mission-init/SKILL.md` → `0` → PASS

### 3. install.sh referenced
- `grep -c "install.sh" skills/mission-init/SKILL.md` → `1` (line 59: `~/.darkfactory/install.sh`) → PASS

### 4. Tool references present
- `grep -cE "darkfactory-dispatch|codex|openspec|jq" skills/mission-init/SKILL.md` → `18` (>= 1) → PASS

### 5. Skill lists 12 symlinks and runs openspec init correctly
- Line 16 lists exactly 12 skill names: `mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch` → PASS
- `openspec init --tools claude,codex --force` appears on line 97 → PASS

## Summary

All 5 contract assertions pass. The skill file is well-formed: it names the correct 12 skills, references `install.sh` for repair instructions, references all required tools (`darkfactory-dispatch`, `codex`, `openspec`, `jq`), and includes the exact `openspec init --tools claude,codex --force` invocation for first-time config setup.
