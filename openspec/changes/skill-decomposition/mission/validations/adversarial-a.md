Wrote the adversarial report to:

[adversarial-a.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/validations/adversarial-a.md)

Key findings: 2 HIGH, 3 MEDIUM, 1 LOW. Main blockers are the invalid `/adversarial` dispatcher command using mutually exclusive `--async` and `--json`, plus `install.sh` producing symlinks that `/mission-init` can reject. I also reproduced the installer directory-upgrade failure with a temp `HOME`.