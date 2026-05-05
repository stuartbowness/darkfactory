# Worker Brief: Task N - <name>

## Your Task
<task description from tasks.md, expanded with acceptance criteria>

## Project Context
<relevant excerpt from proposal.md -- what is being built and why>

## Architecture
<relevant excerpt from design.md -- how this task fits into the system>

## Key Terms (from UBIQUITOUS_LANGUAGE.md)
<any domain terms the worker needs to understand>

## Domain Types
Use the exact Type Catalog names listed here when modeling domain concepts. Include entries relevant to this task from both the Produces and Consumes topology fields.

<For each relevant Type Catalog entry, include:
- TypeName: one-line purpose and owning module

If no Type Catalog entries are relevant, write:
None for this task.
>

## What Previous Tasks Built
<For task 1, this section is empty: "This is the first task. No prior code has been committed for this mission."

For task 2+, include a codebase delta summary generated from `git diff` after each previous task passed validation. List ONLY observable facts about the code -- never include worker reasoning, handoff content, or concerns.

For each completed task, include:
- Files created or modified (paths only)
- Public function/endpoint signatures added (name, params, return type)
- Key type definitions or interfaces introduced
- Patterns used (e.g., "repository pattern via src/repositories/userRepo.ts")

Example:
### Task 1: User service
- Created: src/services/users.ts, src/models/user.ts, src/repositories/userRepo.ts
- Endpoints: POST /api/users (body: {email: string, name: string}) → 201 {id, email, name}
- Types: User {id: string, email: string, name: string, createdAt: Date}
- Pattern: repository pattern via src/repositories/userRepo.ts

### Task 2: Auth middleware
- Created: src/middleware/requireAuth.ts, src/services/auth.ts
- Exports: requireAuth (Express middleware, attaches req.user: User)
- Endpoints: POST /api/auth/login (body: {email, password}) → 200 {token: string}
>

## Downstream Consumers
<Which later tasks in tasks.md will call this code? For each, name one minimum integration test that proves the call path works. Leave empty if no later task consumes this one.>

## Procedures

**Before writing any code**, invoke the `<task Skill: value, or /tdd if no Skill: field>` skill. This is mandatory — do not skip it, do not treat it as optional guidance. The skill contains the workflow you must follow for this task. Invoke it now, read the full instructions it returns, and follow its phases in order.

<If task has Constraints, one line per constraint skill, e.g.:>
Also invoke `/<constraint skill name>` and follow its rules alongside the primary methodology.

<If task Skill: is /frontend and browser_tools is false, include this line:>
Browser tools are disabled for this mission. Skip the chrome-devtools-mcp visual verification step in the /frontend workflow.

- Commit all changes (code + tests) in a single commit with message: "[task-N] <description>"
- Write your handoff report to openspec/changes/<change-name>/mission/handoffs/task-N.md

## Handoff Report Format
- Task: Task N - <name>
- Commit SHA: <the full SHA>
- Approach: <brief explanation of what you did and why>
- Commands run: <any commands and their exit codes>
- Concerns: <anything you're uncertain about>
