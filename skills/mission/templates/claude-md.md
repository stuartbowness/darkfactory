| Include | Exclude |
|---------|---------|
| Commands Claude cannot guess | Anything Claude can infer from code |
| Code style rules that differ from defaults | Standard language conventions |
| Testing instructions and preferred runners | Detailed API docs (link instead) |
| Architectural decisions specific to project | File-by-file codebase descriptions |
| Dev environment quirks (required env vars) | Long explanations or tutorials |
| Common gotchas or non-obvious behaviors | Self-evident practices |

For each line, ask: would removing this cause Claude to make mistakes? If not, cut it.

## Template

```markdown
# <project-name>

## Purpose
<One paragraph: what this project does and who it's for>

## Rules
<Specific, actionable coding rules. Include "update ARCH.md when you change architecture." Only rules that differ from language defaults.>

## Commands
<build, test, lint, format commands. Only commands Claude cannot guess from build files.>

## Repo Map
<High-level: what lives where. One line per major directory or module. Not file-by-file.>

## Testing
<Framework, where tests go by change type, any special setup like serial execution or DB requirements.>

## Environment
<Required services, env vars, config files, dev setup quirks.>

## Architecture
See @ARCH.md for architecture decisions.

## Key Terms
See @UBIQUITOUS_LANGUAGE.md for domain terminology.
```

## Example

Adapt section content to the project's language and framework. The example below is illustrative; match the density and specificity to your project.

```markdown
# Invoice Sync

## Purpose
Syncs customer invoices from the billing system into the accounting ledger for operations staff.

## Rules
Keep ledger writes idempotent. Put provider-specific code behind adapters. Update ARCH.md when you change architecture.

## Commands
`make test` runs unit and integration tests. `make lint` runs formatting and static checks.

## Repo Map
`cmd/` holds entrypoints. `core/` holds invoice and ledger rules. `adapters/` holds external service clients.

## Testing
Use unit tests for `core/` changes and integration tests for adapter behavior. Ledger integration tests require `LEDGER_TEST_URL`.

## Environment
Copy `.env.example` to `.env`. Local runs require billing and ledger sandbox credentials.

## Architecture
See @ARCH.md for architecture decisions.

## Key Terms
See @UBIQUITOUS_LANGUAGE.md for domain terminology.
```
