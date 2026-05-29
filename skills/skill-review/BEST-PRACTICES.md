# Skill Best Practices Reference

Check each dimension below against the skill under review. Skip dimensions marked N/A.

Source: https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices

## Core Principles

### 1. Conciseness
- **Check:** Does every paragraph earn its token cost? Is context included that Claude already knows (what PDFs are, how git works, what TypeScript is)? Could any section be removed without losing skill-specific information?
- **Severity:** WARNING
- **Signals:** Paragraphs explaining general concepts, verbose introductions, restating things Claude already knows

### 2. Degrees of freedom
- **Check:** Does the level of specificity match the task's fragility? Deterministic operations (CLI commands, file formats, migrations) need exact instructions. Heuristic tasks (code review, architecture analysis) need principles and examples, not rigid step-by-step rules.
- **Severity:** WARNING
- **Signals:** Over-specified heuristic tasks (too many rigid rules for judgment calls) or under-specified deterministic tasks (vague instructions for exact operations)

### 3. Model range
- **Check:** Would this skill work for both Haiku (needs more guidance) and Opus (needs less hand-holding)? Are critical steps explicit enough for weaker models without wasting tokens for stronger ones?
- **Severity:** SUGGESTION
- **Signals:** Instructions that assume deep expertise (too sparse for Haiku) or patronizing explanations of basics (too verbose for Opus)

## Structure

### 4. Naming
- **Check:** The `name` field in frontmatter uses only lowercase letters, numbers, and hyphens. Max 64 characters. Does not contain reserved words. Gerund form preferred (e.g., `processing-pdfs` rather than `pdf-processor`).
- **Severity:** CRITICAL if name contains invalid characters or reserved words (prevents discovery). WARNING if non-gerund form.
- **Reserved words:** help, clear, config, compact, login, logout, status, doctor, review, memory, permissions, mcp, listen, vim, cost, init, bug, terminal-setup

### 5. Description
- **Check:** Present in frontmatter. Written in third person (not "I can..." or "You can..."). Max 1024 characters. First sentence says what the skill does. Includes a "Use when" clause with specific trigger words or contexts. Specific enough to distinguish from other skills.
- **Severity:** CRITICAL if missing. WARNING if over 1024 characters, missing "Use when", or uses first/second person.

### 6. Body length
- **Check:** SKILL.md body (after the closing `---` of frontmatter) is under 500 lines. If approaching or exceeding 500 lines, content should be split into companion files.
- **Severity:** WARNING if over 500 lines

### 7. Progressive disclosure
- **Check:** SKILL.md serves as a high-level guide that links to companion files for detailed content. Companion files are loaded on demand, not all referenced at the top. Does the skill use separate files for distinct domains, advanced features, or large reference material?
- **Severity:** WARNING if all content is crammed into SKILL.md when it clearly should be split (e.g., multiple distinct domains, large reference tables, advanced features used rarely)

### 8. Reference depth
- **Check:** File references go one level deep only. SKILL.md links to companion files, but companion files do not link to other companion files. No A -> B -> C chains.
- **Severity:** WARNING

### 9. Table of contents
- **Check:** Files over 100 lines have a table of contents at the top, or clear section headers that serve as one. This ensures Claude can see the full scope even when previewing with partial reads.
- **Severity:** SUGGESTION

## Content

### 10. Time-sensitive information
- **Check:** No date-dependent content ("before August 2025", "as of version 3.2", "recently added"). Content should be valid indefinitely. Old patterns should use a collapsible "Legacy" or "Old patterns" section.
- **Severity:** WARNING
- **Signals:** Date patterns, "as of", "recently", "new in", "deprecated since", month+year combinations

### 11. Consistent terminology
- **Check:** One term per concept, used consistently throughout. No alternating between synonyms for the same thing (e.g., "skill" vs "plugin", "check" vs "validate" vs "verify" for the same operation).
- **Severity:** SUGGESTION
- **Signals:** 3+ different words used for the same operation within the skill

### 12. Concrete examples
- **Check:** Key concepts have input/output pairs, before/after examples, or code snippets. Abstract descriptions without examples are a smell.
- **Severity:** SUGGESTION

### 13. Workflow checklists
- **Check:** Multi-step processes use numbered steps or checkboxes. Complex workflows include copy-paste checklists the agent can track progress against.
- **Severity:** SUGGESTION

### 14. Feedback loops
- **Check:** When the skill instructs the agent to run a command or validator, it specifies what to do with the output. Run-then-check-then-fix patterns are explicit, not implied.
- **Severity:** WARNING if the skill runs validators or commands without specifying error handling

### 15. Conditional workflows
- **Check:** Branching logic uses clear decision trees, if/then structures, or labeled paths. Not ambiguous prose that requires the agent to infer which branch to take.
- **Severity:** SUGGESTION

### 16. Output templates
- **Check:** When the skill produces structured output (reports, artifacts, formatted results), a template or format specification is provided. The level of strictness matches the need (strict for data formats, flexible for analysis).
- **Severity:** WARNING if the skill produces output without specifying format

## Scripts

Skip this entire section if the skill directory contains no `.sh`, `.js`, `.py`, or `.ts` files.

### 17. Error handling
- **Check:** Scripts handle errors explicitly with actionable messages. No bare failures that punt diagnosis to Claude. Errors produce messages that say what went wrong and suggest a fix.
- **Severity:** WARNING

### 18. Magic numbers
- **Check:** All constants in scripts have comments explaining their rationale. No unexplained timeouts, retry counts, or thresholds.
- **Severity:** SUGGESTION

### 19. Utility scripts vs generated code
- **Check:** Deterministic operations (validation, formatting, file manipulation) use pre-written utility scripts rather than asking the agent to generate code each time. Scripts save tokens and improve reliability.
- **Severity:** SUGGESTION

### 20. Dependency declarations
- **Check:** Scripts that use external tools (jq, node, python, curl) state the dependency explicitly. Don't assume packages are installed without mentioning the requirement.
- **Severity:** WARNING

## Anti-patterns

### 21. Path separators
- **Check:** All file paths use forward slashes. No backslash (`\`) path separators anywhere.
- **Severity:** WARNING
- **Signals:** Backslashes in path-like contexts

### 22. Option overload
- **Check:** When offering choices (libraries, approaches, configurations), provide a clear default with an escape hatch. Don't present 4-5 equivalent options and leave the agent to choose without guidance.
- **Severity:** SUGGESTION

### 23. Tool assumptions
- **Check:** If the skill depends on external CLI tools, MCP servers, or packages, it either checks for their presence or documents the requirement. MCP tool references use fully qualified names (`ServerName:tool_name`).
- **Severity:** WARNING
