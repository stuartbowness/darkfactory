# Design Template

Use this template for every `design.md`. Include all six mandatory sections below.

## Architecture Decision

For greenfield missions or missions with explicit architecture changes, evaluate 2-3 architecture candidates. Score each candidate on every dimension from 0-3, then recommend one candidate.

For existing codebase missions without architecture changes, document the current architecture and explain how the planned changes fit. Scored candidates are not required.

| Candidate | Separation of concerns | Testability | Task independence | Module depth | Total | Recommendation |
|-----------|------------------------|-------------|-------------------|--------------|-------|----------------|
| Candidate A | 0-3 | 0-3 | 0-3 | 0-3 | 0-12 | Recommended / Not recommended |
| Candidate B | 0-3 | 0-3 | 0-3 | 0-3 | 0-12 | Recommended / Not recommended |

### Scoring Rubric

#### Separation of Concerns

| Score | Criteria |
|-------|----------|
| 3 | Each responsibility maps to exactly one unit. No unit mixes domain logic with transport, storage, or configuration. |
| 2 | Mostly isolated but 1-2 units mix concerns pragmatically. |
| 1 | Several units mix concerns. Domain logic tangled with transport or storage in more than one place. |
| 0 | No clear separation. Business logic, I/O, and configuration interleaved throughout. |

#### Testability

| Score | Criteria |
|-------|----------|
| 3 | Each module testable in isolation with swappable dependencies via interfaces or dependency injection. |
| 2 | Most modules testable in isolation. One or two require a real external dependency. |
| 1 | Tests require significant setup or multiple running services. Modules tightly coupled to dependencies. |
| 0 | No practical way to test modules in isolation. Testing requires the full system running. |

#### Task Independence

| Score | Criteria |
|-------|----------|
| 3 | Separately buildable or deployable units. A task can modify one unit without touching others. |
| 2 | Explicit interface boundaries within a single build unit with enforced contracts. |
| 1 | Directory/namespace separation only. No enforcement beyond developer discipline. |
| 0 | Flat structure. No meaningful boundaries. Every task touches shared state. |

#### Module Depth

| Score | Criteria |
|-------|----------|
| 3 | Modules expose small public API relative to implementation. Callers don't need to understand internals. |
| 2 | Reasonable interface-to-implementation ratio. One or two modules expose more surface than they hide. |
| 1 | Several modules are thin wrappers. Public API nearly as large as implementation. |
| 0 | No meaningful abstraction. Public surface and implementation effectively the same. |

After the table, include a short rationale for the recommended architecture. If candidates tie on total score, present the tied candidates and their tradeoffs for human selection.

## Module Inventory

List each planned module or major component. Use this table format:

| Module name | Interface width | What it hides | Why this boundary |
|-------------|-----------------|---------------|-------------------|
| `<module>` | Narrow / moderate / broad, with the public entry points named | Implementation details callers do not need to know | Reason this responsibility belongs behind this boundary |

## Type Catalog

List domain types only. Type names must come from `UBIQUITOUS_LANGUAGE.md` when the term exists there. Do not include field-level detail.
Domain types model the problem space; exclude technical helpers such as CLI parsers, test fixtures, and generic containers unless they are public compatibility surfaces.

| Type name | Purpose | Owning module |
|-----------|---------|---------------|
| `<type>` | One-line purpose | `<module>` |

## Error Strategy

Define:
- Error type hierarchy
- Propagation rules
- Which layer throws each error category
- Which layer catches and translates each error category

Keep this at the design level. Do not list every possible error instance unless the specs require it.

## Integration Points

Describe how modules call each other. Use this table format:

| Caller | Callee | Method/interface | Direction |
|--------|--------|------------------|-----------|
| `<module>` | `<module>` | Function, command, event, adapter, or interface name | Inbound, outbound, or internal |

## Test Strategy

Define:
- Test framework and runner
- Where tests live (colocated, separate directory, by module)
- Which modules get unit tests vs integration tests and why
- What needs real external dependencies (database, network) vs mocks/stubs
- Any special setup (serial execution, test database, environment variables)
- Coverage approach (which code paths are critical to cover, which are low-value)

Keep it brief. This section captures decisions that affect every worker, not a full test plan.
