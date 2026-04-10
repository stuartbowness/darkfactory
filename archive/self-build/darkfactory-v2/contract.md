# Validation Contract: darkfactory-v2

## Task 1: Main Mission Skill
- The file darkfactoryV2/.claude/skills/mission/SKILL.md exists and has valid YAML frontmatter with `name: mission` and a description
- The skill documents Phase 0 (enter plan mode and write orchestrator operating manual)
- The skill documents Phase 1 (grill the user for alignment using /grill-me)
- The skill documents Phase 2 (establish shared language using /ubiquitous-language)
- The skill documents Phase 3 (create OpenSpec artifacts using /opsx:propose)
- The skill documents Phase 4 (write validation contract with assertions grouped by task)
- The skill documents Phase 5 (create mission directory with config.yaml, progress.md, and subdirectories)
- The skill documents Phase 6 (present plan for human approval)
- The skill documents Phase 7 (build loop: brief assembly, worker dispatch, scrutiny validation, user-test validation, demo video, fix loop with circuit breaker, progress tracking)
- The skill documents Phase 8 (mission completion: create draft PR with mission summary)
- The skill includes a Monitor bash script for dispatching Codex workers
- The skill includes a Claude subagent fallback for worker dispatch
- The skill includes the full scrutiny validator prompt with check sequence
- The skill includes the full user-test validator prompt with test strategy decision
- The skill includes fix brief format with failure details and commit SHA
- The skill states that workers never see the contract (adversarial separation)
- The skill states that validators never see the proposal or design
- The skill uses terms from UBIQUITOUS_LANGUAGE.md correctly (Mission, Orchestrator, Worker, Validator, Contract, Assertion, Brief, Handoff, Monitor, Engine, Plan mode)
- The skill references all required skills: /grill-me, /ubiquitous-language, /opsx:propose, /review, /code-simplify, /security-review, /test-app, /demo-video, /pr

## Task 2: Mission Resume Skill
- The file darkfactoryV2/.claude/skills/mission-resume/SKILL.md exists and has valid YAML frontmatter with a name and description
- The skill searches for active missions via openspec/changes/*/mission/config.yaml
- The skill handles zero, one, or multiple active missions (reports none, uses one, asks user to pick)
- The skill reads tasks.md, progress.md, config.yaml, contract.md, proposal.md, and design.md
- The skill enters plan mode and writes a reconstructed orchestrator operating manual
- The skill checks git log for orphaned commits matching [task-N] pattern
- The skill asks the human about orphaned commits (count as done or re-run)
- The skill continues the build loop from the next unchecked task
- The skill logs the resume event to progress.md

## Task 3: Mission Status Skill
- The file darkfactoryV2/.claude/skills/mission-status/SKILL.md exists and has valid YAML frontmatter with a name and description
- The skill is read-only (does not create, modify, or delete any files)
- The skill searches for active missions via mission/ directories in OpenSpec changes
- The skill handles zero, one, or multiple active missions
- The skill displays: mission name, tasks completed vs total, current task status, validation results per completed task, and recent activity from progress.md
- The skill shows a formatted progress display (not just raw file dumps)
