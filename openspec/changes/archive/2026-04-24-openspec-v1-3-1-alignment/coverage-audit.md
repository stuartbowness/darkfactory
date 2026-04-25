# Coverage Audit: openspec-v1-3-1-alignment

PRD: https://github.com/layercodedev/darkfactory/issues/80

## Coverage Table

| # | PRD Requirement (User Story) | Status | Task(s) | Notes |
|---|------------------------------|--------|---------|-------|
| 1 | CLI version checks complete reliably (no parallel cancellation) | COVERED | 1.1 | Safe Bash pattern note |
| 2 | openspec init cleans up legacy files automatically (--force) | COVERED | 1.1 | --force flag addition |
| 3 | generatedBy metadata reflects v1.3.1 | COVERED | 2.1 | openspec update --force |
| 4 | Skill content matches v1.3.1 instructions | COVERED | 2.1 | openspec update --force |
| 5 | README /opsx:archive says "Bundled with Darkfactory" | COVERED | 3.1 | Single line change |
| 6 | .codex/skills/ copies match .claude/skills/ | COVERED | 2.1 | Copy after regeneration |

## PRD Solution Items

| # | Solution Item | Status | Task(s) |
|---|---------------|--------|---------|
| S1 | Fix parallel Bash cancellation | COVERED | 1.1 |
| S2 | Add --force flag to openspec init | COVERED | 1.1 |
| S3 | Regenerate openspec-* skills from v1.3.1 | COVERED | 2.1 |
| S4 | Update README skill table | COVERED | 3.1 |

## Gaps

None. All 6 user stories and 4 solution items are covered by tasks 1.1, 2.1, and 3.1.

## Deviations

None. All tasks implement requirements as specified in the PRD.

## Closing Tasks Coverage

| Task | Purpose |
|------|---------|
| 4.1 Smoke test | Verifies requirements 3, 4 (generatedBy metadata) and requirements 1, 2 (init skill content) |
| 4.2 Cleanup | Catches any stale references that slipped through |
