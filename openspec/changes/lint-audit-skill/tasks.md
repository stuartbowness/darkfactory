# Tasks: lint-audit-skill

- [x] Task 1: Create rules.json agent-friendly ruleset
  Depends on: none
  Consumes: oxlint rule documentation (external)
  Produces: `skills/lint-audit/rules.json` -- array of rule objects with canonical names, categories, severities, default status (pinned to a specific oxlint version noted in the file), rationale
  Layers: data

- [x] Task 2: Build audit.sh detection and comparison script
  Depends on: Task 1
  Consumes: `skills/lint-audit/rules.json`
  Produces: `skills/lint-audit/audit.sh` -- executable script that takes a project dir (and optional `--oxlint-config`), outputs AuditReport JSON to stdout
  Layers: logic

- [x] Task 3: Write SKILL.md workflow and RULES.md documentation
  Depends on: Task 2
  Consumes: `skills/lint-audit/audit.sh`, `skills/lint-audit/rules.json`
  Produces: `skills/lint-audit/SKILL.md` (agent workflow including config-apply logic: oxlint config creation/update, tsconfig update, package.json lint script update for `--type-aware`), `skills/lint-audit/RULES.md` (human documentation with pinned oxlint version matching rules.json)
  Layers: workflow

- [x] Task 4: Smoke test -- 200-iteration edge case loop
  Depends on: Task 2
  Consumes: `skills/lint-audit/audit.sh`, `skills/lint-audit/rules.json`
  Produces: test harness at `/tmp/lint-audit-test.sh` (temporary), durable summary at `mission/validations/smoke-test-200.md`
  Layers: testing

- [x] Task 5: Cleanup
  Depends on: Task 4
  Consumes: all skill files
  Produces: cleaned codebase (no unused deps, dead code, temp artifacts)
  Layers: maintenance

- [x] Task 6: Update install.sh, README.md, ARCH.md
  Depends on: Task 5
  Consumes: `skills/lint-audit/` directory
  Produces: updated `install.sh` (symlink new skill), updated `README.md` (skill list), updated `ARCH.md` (skill description)
  Layers: documentation

- [x] Task 7: Module shape re-check
  Depends on: Task 6
  Consumes: `design.md` Module Inventory, committed code (including install.sh and docs from Task 6)
  Produces: `mission/validations/module-shape-audit.md`
  Layers: validation
