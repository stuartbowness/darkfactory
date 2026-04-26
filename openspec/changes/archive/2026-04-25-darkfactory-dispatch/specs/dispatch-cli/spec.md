## ADDED Requirements

### Requirement: CLI argument interface
The dispatcher SHALL accept these flags for dispatch configuration.

#### Scenario: Core dispatch flags
- **WHEN** the user runs `darkfactory-dispatch`
- **THEN** the following flags are available: `--model`, `--effort`, `--timeout`, `--grace`, `--sandbox`, `--config` (repeatable), `--prompt`, `--prompt-file`, `--output`, `--cwd`, `--log`, `--json`, `--async`

#### Scenario: Prompt sources are mutually exclusive
- **WHEN** both `--prompt` and `--prompt-file` are provided
- **THEN** the dispatcher exits with an error

#### Scenario: Prompt from file
- **WHEN** `--prompt-file PATH` is provided
- **THEN** the dispatcher reads the prompt from the file and passes it to Codex

### Requirement: Effort flag
The `--effort` flag SHALL map to both result metadata and the Codex reasoning config.

#### Scenario: Effort passed to Codex
- **WHEN** `--effort high` is provided
- **THEN** the dispatcher passes `-c model_reasoning_effort="high"` to `codex exec`
- **AND** records `effort: "high"` in the dispatch result metadata

### Requirement: Config pass-through
The `--config` flag SHALL pass key=value pairs directly to Codex's `-c` flag.

#### Scenario: Config forwarded to Codex
- **WHEN** `--config sandbox_workspace_write.writable_roots=[".git"]` is provided
- **THEN** the dispatcher passes `-c sandbox_workspace_write.writable_roots=[".git"]` to `codex exec`

### Requirement: Sandbox flag
The `--sandbox` flag SHALL control the Codex sandbox mode.

#### Scenario: Sandbox mode
- **WHEN** `--sandbox workspace-write` is provided
- **THEN** the dispatcher passes `--full-auto --sandbox workspace-write` to `codex exec`

### Requirement: Output path pass-through
The `--output` flag SHALL be passed to Codex's `-o` flag, and the result SHALL report whether the file exists.

#### Scenario: Output file written by child
- **WHEN** `--output /path/to/handoff.md` is provided
- **THEN** the dispatcher passes `-o /path/to/handoff.md` to `codex exec`
- **AND** the dispatch result includes `output: "/path/to/handoff.md"` and `output_exists: true` if the file exists after the child exits

### Requirement: Dispatch ID generation
Each dispatch SHALL receive a unique ULID identifier.

#### Scenario: Dispatch ID in result
- **WHEN** any dispatch runs
- **THEN** the dispatch result includes a `dispatch_id` field containing a ULID

### Requirement: Subcommands for lifecycle management
The dispatcher SHALL support `status`, `result`, and `wait` subcommands.

#### Scenario: Subcommand routing
- **WHEN** `darkfactory-dispatch status <id>` is called
- **THEN** the status subcommand executes (not a dispatch)
- **AND** similarly for `result <id>` and `wait <id>`

### Requirement: Static binary
The dispatcher SHALL compile to a single static binary with no runtime dependencies.

#### Scenario: Installation
- **WHEN** `make install` is run in `tools/darkfactory-dispatch/`
- **THEN** a release binary is compiled and copied to `~/.local/bin/darkfactory-dispatch`
