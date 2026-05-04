## ADDED Requirements

### Requirement: /react-perf skill exists as a standalone Claude Code skill
The system SHALL provide a `/react-perf` skill at `skills/react-perf/` with YAML frontmatter (`name: react-perf`, `user-invokable: true`) and a description containing trigger words for React performance, waterfalls, bundle size, re-renders, and optimization.

#### Scenario: Skill registration
- **WHEN** `./install.sh` is run
- **THEN** a symlink at `~/.claude/skills/react-perf` SHALL point to `~/.darkfactory/skills/react-perf`

#### Scenario: Independent invocation
- **WHEN** a user invokes `/react-perf` outside of a mission
- **THEN** the skill SHALL provide performance guidance without requiring `/frontend` to be active

### Requirement: /react-perf includes verbatim upstream Vercel React best practices
The skill SHALL include the upstream Vercel React best practices content from skills.sh verbatim, covering waterfall elimination, bundle size, server-side patterns, re-render discipline, rendering patterns, and JS performance.

#### Scenario: Waterfall elimination
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL cover: check cheap sync conditions before awaiting, use Promise.all for independent ops, start promises early and await late, use Suspense to stream

#### Scenario: Bundle size discipline
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL cover: direct imports (no barrel files), dynamic imports for heavy components, load analytics after hydration, preload on hover/focus

#### Scenario: Re-render discipline
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL cover: don't subscribe to state only used in callbacks, derive state during render not effects, use functional setState for stable callbacks, never define components inside components, use startTransition for non-urgent updates

#### Scenario: React 19 patterns
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL cover React 19 specific patterns including transitions and Suspense boundaries

### Requirement: /react-perf is a valid Constraint for /frontend tasks
The skill SHALL be usable as a Constraint in task topology metadata alongside `Skill: /frontend`.

#### Scenario: Valid constraint pairing
- **WHEN** a task has `Skill: /frontend` and `Constraints: /react-perf`
- **THEN** the worker brief SHALL reference both `/frontend` in Procedures and `/react-perf` as additional guidance

#### Scenario: Invalid constraint pairing
- **WHEN** a task has `Skill: /tdd` and `Constraints: /react-perf`
- **THEN** the orchestrator SHALL flag this as an invalid combination during task review
