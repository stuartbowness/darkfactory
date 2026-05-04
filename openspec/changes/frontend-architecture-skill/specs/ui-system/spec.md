## ADDED Requirements

### Requirement: /ui-system skill exists as a standalone Claude Code skill
The system SHALL provide a `/ui-system` skill at `skills/ui-system/` with YAML frontmatter (`name: ui-system`, `user-invokable: true`) and a description containing trigger words for ShadCN, Radix, component library, design system, and Tailwind tokens.

#### Scenario: Skill registration
- **WHEN** `./install.sh` is run
- **THEN** a symlink at `~/.claude/skills/ui-system` SHALL point to `~/.darkfactory/skills/ui-system`

#### Scenario: Independent invocation
- **WHEN** a user invokes `/ui-system` outside of a mission
- **THEN** the skill SHALL provide component selection and token discipline guidance without requiring `/frontend` to be active

### Requirement: /ui-system includes verbatim upstream ShadCN guidelines
The skill SHALL include the upstream ShadCN skill content from skills.sh verbatim, covering component selection, token discipline, composition patterns, and accessibility defaults.

#### Scenario: Component selection guidance
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL instruct: always search existing ShadCN components before writing custom UI

#### Scenario: Semantic token discipline
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL require semantic color tokens only (never raw values like `bg-blue-500`), no manual `dark:` overrides, and `className` for layout only

#### Scenario: Composition patterns
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL cover: `cn()` for conditional classes, `gap-*` over `space-x/y-*`, compound component adaptation, `data-slot` conventions

#### Scenario: Accessibility defaults
- **WHEN** a worker reads the skill
- **THEN** the skill SHALL require: Dialog/Sheet/Drawer always need a Title (use `sr-only` if hidden), Avatar always needs AvatarFallback

### Requirement: /ui-system is a valid Constraint for /frontend tasks
The skill SHALL be usable as a Constraint in task topology metadata alongside `Skill: /frontend`.

#### Scenario: Valid constraint pairing
- **WHEN** a task has `Skill: /frontend` and `Constraints: /ui-system`
- **THEN** the worker brief SHALL reference both `/frontend` in Procedures and `/ui-system` as additional guidance

#### Scenario: Invalid constraint pairing
- **WHEN** a task has `Skill: /tdd` and `Constraints: /ui-system`
- **THEN** the orchestrator SHALL flag this as an invalid combination during task review
