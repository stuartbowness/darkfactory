## Context

Darkfactory is a skill-based mission orchestration system. Skills are markdown directories (`skills/<name>/SKILL.md` plus optional reference files) discovered by `install.sh` and invoked via `/name` in Claude Code sessions. The scrutiny validator runs 14 sequential gates against worker output. Worker briefs are assembled from a template that currently hardcodes `/tdd` as the implementation methodology.

This mission adds 3 new skills and modifies the validator, brief template, and design template to support frontend-specific workflows.

## Goals / Non-Goals

**Goals:**
- Add `/frontend`, `/ui-system`, `/react-perf` skills following existing skill conventions
- Extend the scrutiny validator with a self-skipping frontend gate
- Enable the orchestrator to assign different methodologies per task
- Support visual verification via chrome-devtools-mcp during frontend builds
- Zero impact on non-TypeScript missions

**Non-Goals:**
- Visual design (aesthetic direction, layout design)
- Browser-based QA suites (belongs in testkitchen)
- Backend TypeScript patterns (deferred to #144)
- Framework-agnostic support (React only)

## Architecture Decision

This is an existing codebase mission. No architecture change — the new skills follow the established skill pattern (markdown directories, YAML frontmatter, bundled reference files, auto-discovery by install.sh). The validator modification follows the established gate pattern (sequential, self-skipping).

The one architectural addition is the two-axis skill assignment model (`Skill:` + `Constraints:` in task topology), which extends the existing `Layers:` field rather than replacing it.

**Current architecture:** Skills are flat directories in `skills/`. The validator is a single prompt file with numbered gates. The worker brief template has a hardcoded Procedures section. All three patterns are extended, not replaced.

**How changes fit:**
- New skill directories are auto-discovered — no install.sh changes
- New gate is inserted by renumbering — the sequential gate pattern is preserved
- Brief template conditioning uses the existing topology metadata — new fields, same mechanism
- Config flag (`browser_tools`) follows the existing `engines:` pattern in config.yaml

## Module Inventory

| Module name | Interface width | What it hides | Why this boundary |
|---|---|---|---|
| `skills/frontend/` | Narrow: 1 entry point (SKILL.md), 4 reference files | Deep-component rules, workflow phases, stack conventions, code patterns, testing guidance. Workers see the workflow; the principles and examples are progressive disclosure. | The core skill is the single unit a worker interacts with. Reference files are internal to the skill — workers read them via SKILL.md links, not directly. |
| `skills/ui-system/` | Narrow: 1 entry point (SKILL.md) | Verbatim upstream ShadCN/Radix guidelines. Workers get component selection and token discipline without knowing the upstream source. | Standalone constraint skill. Independent of `/frontend` — callable on its own for design system questions. |
| `skills/react-perf/` | Narrow: 1 entry point (SKILL.md) | Verbatim upstream Vercel React best practices. Workers get performance patterns without knowing the upstream source. | Standalone constraint skill. Independent of `/frontend` — callable on its own for performance questions. |
| Gate 7 (frontend depth) | Narrow: 1 gate in the validator sequence | LLM evaluation of React component depth against four rules. The gate reads the diff, applies DEEP-COMPONENTS.md criteria, and produces PASS/FAIL/WARN. | Self-skipping gate follows the established pattern (like gate 9/security). Isolates frontend concerns from the language-agnostic module depth gate (gate 6). |
| Task topology extensions | Narrow: 2 new fields (`Skill:`, `Constraints:`) | Layer-to-methodology mapping rules, constraint validation logic. The orchestrator applies the mapping; workers see only the assembled brief. | Extends existing topology metadata. The mapping rules live in the orchestrator's task generation logic (Phase 4.3a/4.4), not in a separate module. |
| Worker brief conditioning | Narrow: 1 conditional section (Procedures) | Methodology selection logic. The template reads `Skill:` and `Constraints:` and assembles the right references. Workers see a brief with one methodology and zero or more constraints. | Extends the existing template. The conditional is a single if/else on the `Skill:` field. |
| `browser_tools` config | Narrow: 1 boolean flag | Chrome-devtools-mcp lifecycle (availability checks, dispatch pre-flight, human override). The orchestrator manages the flag; workers either have visual verification in their brief or don't. | Follows the existing `engines:` pattern in config.yaml. One flag, two check points (init, dispatch). |

### Frontend module depth

For React component modules in the Module Inventory, describe:
- **Prop interface:** how many semantic concepts the caller must understand
- **State/effects encapsulated:** what the component hides behind its interface
- **Classification:** deep component (owns a workflow) or allowed-shallow exception (design system leaf, route glue, adapter, headless primitive) with reason

## Type Catalog

This mission produces markdown skills, not code with domain types. No domain types are introduced. The terminology is captured in UBIQUITOUS_LANGUAGE.md (deep component, allowed-shallow exception, methodology, constraint, task layer, etc.) rather than in code types.

| Type name | Purpose | Owning module |
|---|---|---|
| (none) | This mission produces markdown files, not code with typed interfaces | -- |

## Error Strategy

This mission modifies markdown files and prompt templates. There are no runtime error types to define.

**Failure modes and handling:**
- **Missing chrome-devtools-mcp at mission-init:** Hard fail with clear error message. Human installs and re-runs.
- **chrome-devtools-mcp unavailable at dispatch:** Block dispatch, log to progress.md, surface to human with options (retry, skip visual verify, abort).
- **Invalid `Skill:` or `Constraints:` values in topology:** Caught during Phase 4.4 human review. The orchestrator validates combinations before dispatch.
- **Gate 7 false positive on allowed-shallow components:** The gate recognizes `*/ui/*` directories and Radix imports as exceptions. Persistent false positives are addressed by updating the gate's exception list.
- **Gate 7 false negatives:** Gate 7 is an LLM gate and may miss violations. Mitigation: the deep-component audit checklist in SKILL.md is the first line of defense; gate 7 is the backstop. Neither is perfect, but both catching the same violation is redundant by design.
- **Upstream content drift:** `/ui-system` and `/react-perf` include verbatim upstream content that may drift from the skills.sh sources over time. Mitigation: diff against upstream periodically. No automated sync — the content is vendored and owned by Layercode.
- **Mid-build chrome-devtools-mcp failure:** If the MCP server crashes after dispatch, the worker's MCP call fails and the worker must handle the error. This is outside the orchestrator's control — the worker reports it in the handoff, and the validator catches incomplete visual verification.
- **Hardcoded symlink count in tests:** The test strategy checks for 20 symlinks (17 + 3). This couples to the current skill count. Mitigation: check for the 3 new symlinks specifically rather than a total count.

## Integration Points

| Caller | Callee | Method/interface | Direction |
|---|---|---|---|
| Orchestrator (Phase 4.3a) | `config.yaml` | Sets `browser_tools: true/false` based on task skills | Internal |
| Orchestrator (Phase 4.4) | `tasks.md` | Writes `Skill:` and `Constraints:` fields per task | Internal |
| Orchestrator (Step 1: Brief) | `worker-brief.md` template | Reads `Skill:` and `Constraints:` fields, conditionally sets Procedures section and appends constraint references | Internal |
| Worker brief | `/frontend` SKILL.md | References skill in Procedures: "Use `/frontend`" | Outbound |
| Worker brief | `/ui-system` SKILL.md | References skill as constraint: "Also follow `/ui-system`" | Outbound |
| Worker brief | `/react-perf` SKILL.md | References skill as constraint: "Also follow `/react-perf`" | Outbound |
| `/frontend` SKILL.md | `DEEP-COMPONENTS.md` | Markdown link: `[DEEP-COMPONENTS.md](DEEP-COMPONENTS.md)` | Internal |
| `/frontend` SKILL.md | `STACK.md` | Markdown link: `[STACK.md](STACK.md)` | Internal |
| `/frontend` SKILL.md | `PATTERNS.md` | Markdown link: `[PATTERNS.md](PATTERNS.md)` | Internal |
| `/frontend` SKILL.md | `TESTING.md` | Markdown link: `[TESTING.md](TESTING.md)` | Internal |
| Scrutiny validator | Gate 1 (test presence) | Gate 1 reads task's `Skill:` field to select rationale text | Internal |
| Scrutiny validator | Gate 7 (frontend depth) | Sequential gate execution after gate 6 | Internal |
| Gate 7 | `DEEP-COMPONENTS.md` criteria | References four rules for evaluation | Outbound |
| `/mission-init` | chrome-devtools-mcp | Reads `browser_tools` from config.yaml; if `true`, checks MCP server is registered via tool listing. Fails fast with install instructions if missing. | Outbound |
| Orchestrator (Step 2: Dispatch) | chrome-devtools-mcp | Soft pre-flight check before `/frontend` task dispatch | Outbound |
| `/frontend` worker | chrome-devtools-mcp | Visual verification during Build phase | Outbound |

## Test Strategy

- **Test framework:** None. This mission produces markdown files, not code. Validation is structural.
- **Where tests live:** No test files. Verification is done via bash commands and manual inspection.
- **Structural validation:**
  - All new skill directories contain SKILL.md with valid YAML frontmatter (`name`, `user-invokable: true`, `description`)
  - `./install.sh` produces 20 symlinks (17 existing + 3 new)
  - `/mission-init` passes with all 12 required skills OK
  - All markdown links in new files resolve to existing files
  - Scrutiny validator gates are numbered 1-15 sequentially with no gaps or duplicates
  - UBIQUITOUS_LANGUAGE.md gate number references match the new sequence
- **Backward compatibility:** Run an existing non-TypeScript mission's scrutiny validator output through the new gate sequence. Gate 7 should SKIP. All other gates should produce identical results.
- **Coverage approach:** Every spec scenario maps to a structural check (file exists, content contains, link resolves, gate number matches). No line coverage — there's no executable code.
