## ADDED Requirements

### Requirement: /frontend skill exists as a Claude Code skill
The system SHALL provide a `/frontend` skill at `skills/frontend/` with YAML frontmatter (`name: frontend`, `user-invokable: true`) and a description containing trigger words for frontend, React, and component development.

#### Scenario: Skill directory structure
- **WHEN** a user lists files in `skills/frontend/`
- **THEN** the directory SHALL contain: `SKILL.md`, `DEEP-COMPONENTS.md`, `STACK.md`, `PATTERNS.md`, `TESTING.md`

#### Scenario: Skill registration
- **WHEN** `./install.sh` is run
- **THEN** a symlink at `~/.claude/skills/frontend` SHALL point to `~/.darkfactory/skills/frontend`

### Requirement: SKILL.md defines the design-implement-test workflow
The `/frontend` skill's SKILL.md SHALL define a prescriptive, worker-facing workflow with four phases executed per component, plus a final audit before commit.

#### Scenario: Workflow phases present
- **WHEN** a worker reads SKILL.md
- **THEN** the following phases SHALL be defined in order: Scan, Contract, Build (with visual verification), Test

#### Scenario: Scan phase
- **WHEN** a worker enters the Scan phase
- **THEN** the skill SHALL instruct the worker to: search existing components in the feature area, identify reusable ShadCN/Radix primitives, check existing hooks to consume, identify test patterns in nearby test files, and write a scan note listing findings

#### Scenario: Contract phase
- **WHEN** a worker enters the Contract phase for a component
- **THEN** the skill SHALL instruct the worker to declare: component name, deep or allowed-shallow classification (with reason if shallow), prop interface with types, and key render states (default, loading, error, empty — only those that apply)

#### Scenario: Build phase with visual verification
- **WHEN** a worker enters the Build phase for a component and `browser_tools: true` in mission config
- **THEN** the skill SHALL instruct the worker to implement the component and verify it visually via chrome-devtools-mcp before proceeding to Test

#### Scenario: Build phase without visual verification
- **WHEN** a worker enters the Build phase for a component and `browser_tools: false` in mission config (human override)
- **THEN** the skill SHALL instruct the worker to implement the component and proceed to Test without visual verification

#### Scenario: Test phase
- **WHEN** a worker enters the Test phase for a component
- **THEN** the skill SHALL instruct the worker to write behavioral tests using Vitest and React Testing Library, testing through rendered output and user interactions

#### Scenario: Per-component loop
- **WHEN** a task produces multiple components
- **THEN** the worker SHALL complete Contract → Build → Test for each component before starting the next

#### Scenario: Deep-component audit before commit
- **WHEN** all components are built and tested
- **THEN** the worker SHALL run a self-check checklist against the four deep-component rules before committing

### Requirement: DEEP-COMPONENTS.md defines the four rules and exceptions
The skill SHALL include a reference document defining four architectural rules for deep components and a list of allowed-shallow exceptions.

#### Scenario: Four rules present
- **WHEN** a worker reads DEEP-COMPONENTS.md
- **THEN** the document SHALL define: (1) Small semantic interfaces, (2) Derive don't sync, (3) Composition over configuration, (4) Collapse coordination — each with a diagnostic question

#### Scenario: Allowed-shallow exceptions
- **WHEN** a worker reads DEEP-COMPONENTS.md
- **THEN** the document SHALL list these exception categories: design system leaves, route/layout glue, adapter components, headless behavior primitives — each with rationale

#### Scenario: Deletion test
- **WHEN** a worker reads DEEP-COMPONENTS.md
- **THEN** the document SHALL include the deletion test: "if you deleted this component, would complexity vanish (pass-through) or reappear across consumers (deep)?"

### Requirement: STACK.md defines the technology reference
The skill SHALL include a reference document specifying the TypeScript React stack and conventions.

#### Scenario: Stack technologies listed
- **WHEN** a worker reads STACK.md
- **THEN** the document SHALL declare: React 19, Vite, TailwindCSS 4, Radix UI / ShadCN, Vitest, React Router

#### Scenario: Import discipline
- **WHEN** a worker reads STACK.md
- **THEN** the document SHALL require direct imports (no barrel files) and specify the `cn()` utility for conditional classes

#### Scenario: File organization
- **WHEN** a worker reads STACK.md
- **THEN** the document SHALL specify: `components/ui/` for design system leaves, `components/<domain>/` for deep components, `hooks/` for stateful logic, `lib/` for pure utilities

### Requirement: PATTERNS.md provides concrete code examples
The skill SHALL include a reference document with real code snippets illustrating each deep-component rule.

#### Scenario: Deep component examples
- **WHEN** a worker reads PATTERNS.md
- **THEN** the document SHALL include at least two examples of deep components with small prop interfaces owning workflows

#### Scenario: Allowed-shallow examples
- **WHEN** a worker reads PATTERNS.md
- **THEN** the document SHALL include at least two examples of allowed-shallow components (design system leaves with compound composition)

#### Scenario: Hook pattern examples
- **WHEN** a worker reads PATTERNS.md
- **THEN** the document SHALL include at least one example of a hook demonstrating the derive-don't-sync rule

#### Scenario: Anti-pattern examples
- **WHEN** a worker reads PATTERNS.md
- **THEN** the document SHALL include at least one anti-pattern with a before/after fix

### Requirement: TESTING.md defines frontend behavioral testing
The skill SHALL include a reference document for component testing that is separate from `/tdd`'s testing philosophy.

#### Scenario: Behavioral testing principles
- **WHEN** a worker reads TESTING.md
- **THEN** the document SHALL instruct: test visible behavior not internals, interact as a user would (render, query by role/label/text, fire events, assert visible outcomes)

#### Scenario: What not to test
- **WHEN** a worker reads TESTING.md
- **THEN** the document SHALL prohibit testing: prop plumbing, hook calls, internal state, class names, snapshot tests for layout

#### Scenario: Hook logic boundary
- **WHEN** a worker reads TESTING.md
- **THEN** the document SHALL state that hook logic is tested in separate `/tdd` tasks, not in component tests
