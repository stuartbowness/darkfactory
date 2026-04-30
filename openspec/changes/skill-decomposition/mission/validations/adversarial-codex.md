Wrote the adversarial review report to:

[adversarial-codex.md](/Users/stuart/.darkfactory/openspec/changes/skill-decomposition/mission/validations/adversarial-codex.md)

I verified the main blocker by running `darkfactory-dispatch --async --json --prompt test`, which fails because those flags are mutually exclusive. I also reproduced the installer directory-target edge case in `/tmp`.