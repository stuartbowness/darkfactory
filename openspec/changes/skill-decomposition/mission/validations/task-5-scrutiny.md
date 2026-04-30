# Task 5 Scrutiny Validation

**Task:** Write mission-init skill
**Commit:** 4a3ed785a86f7b0871169861ca059a9083627d43
**File:** `skills/mission-init/SKILL.md`
**Result:** PASS

---

## Checks Run

### 1. Test Presence
EXEMPT — new `.md` skill file, no testable code.

### 2–9. N/A for .md files.

### 10–11. Contract Assertions

**Assertion 1:** File exists.

```
test -f skills/mission-init/SKILL.md && echo PASS || echo FAIL
→ PASS
```

**Assertion 2:** Old names (`grill-me`, `darkfactory-init`, `test-quality-review`) not referenced.

```
grep -cE "grill-me|darkfactory-init|test-quality-review" skills/mission-init/SKILL.md
→ 0
```

PASS.

**Assertion 3:** `install.sh` appears at least once.

```
grep -c "install.sh" skills/mission-init/SKILL.md
→ 1
```

Line 59: `~/.darkfactory/install.sh`. PASS.

**Assertion 4:** CLI tool references (`darkfactory-dispatch`, `codex`, `openspec`, `jq`) present.

```
grep -cE "darkfactory-dispatch|codex|openspec|jq" skills/mission-init/SKILL.md
→ 18
```

PASS.

**Assertion 5 (inspection):** Skill must verify 12 symlinks and run `openspec init --tools claude,codex --force` when `openspec/config.yaml` absent.

- **12 skills listed** on line 16: `mission mission-resume mission-status mission-init grill prd language tdd test-quality security adversarial dispatch` — all 12 present.
- **Symlink check** (lines 21–23): verifies `$HOME/.claude/skills/<name>` is a symlink whose `readlink` equals `$HOME/.darkfactory/skills/<name>`. Correct.
- **`openspec init --tools claude,codex --force`** appears on line 97, run only when `openspec/config.yaml` is absent and `openspec` CLI is installed. Correct.

PASS.

### 12–14. SKIP.

---

## Summary

| Assertion | Result |
|-----------|--------|
| 1. `SKILL.md` exists, `name: mission-init` | PASS |
| 2. Old names not referenced | PASS (count = 0) |
| 3. `install.sh` suggested | PASS (count = 1) |
| 4. CLI tool checks present | PASS (count = 18) |
| 5. 12 symlinks checked, `openspec init --tools claude,codex --force` present | PASS |

**Overall: PASS**
