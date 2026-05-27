---
name: artifact-review
user-invokable: true
description: Dispatch a Codex review of a planning artifact (language glossary, PRD, spec, design, tasks, contract, or any structured document). Use when user wants a second opinion on an artifact before approving it, or when the mission skill needs a review gate before human Q&A.
---

# Artifact Review

Dispatch a planning artifact to Codex for review against six dimensions: ambiguity, terminology consistency, contradictions with prior artifacts, missing edge cases, coverage gaps, and feasibility. Returns structured findings with severity levels.

## Startup Config

Resolve the Darkfactory repo root from the skill symlink and read `default_model` from its `config.yaml`. Fall back to `gpt-5.5`.

```bash
DARKFACTORY_ROOT="$(cd "$(readlink "$HOME/.claude/skills/artifact-review")/../.." 2>/dev/null && pwd -P)"
DEFAULT_MODEL=$(awk -F: '/^[[:space:]]*default_model[[:space:]]*:/ { gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit }' "$DARKFACTORY_ROOT/config.yaml" 2>/dev/null)
: "${DEFAULT_MODEL:=gpt-5.5}"
```

## Input

Accept instructions in either mode:

- Inline: `/artifact-review Review the proposal.md I just wrote`
- No argument: ask the user `What artifact should I review?` and wait.

The instructions should tell you which artifact to review. They may also name prior artifacts to cross-reference, or the user may leave that to you.

## Step 1: Gather Context

Read the artifact the user wants reviewed. Then gather optional context:

1. **UBIQUITOUS_LANGUAGE.md** — read from the repo root if it exists. If absent, use "Not established."
2. **Prior artifacts** — if the user named specific prior artifacts, read those. If not, auto-discover from the current change directory. First, find the active change: look for a single directory under `openspec/changes/` (if multiple exist, ask the user which change to scope to). Then check the artifact chain in dependency order within that directory:
   - `UBIQUITOUS_LANGUAGE.md` (repo root)
   - PRD (read from the GitHub issue URL in the mission plan if available, or ask)
   - `openspec/changes/<change-name>/proposal.md`
   - `openspec/changes/<change-name>/specs/`
   - `openspec/changes/<change-name>/design.md`
   - `openspec/changes/<change-name>/tasks.md`
   - `openspec/changes/<change-name>/coverage-audit.md`
   - `openspec/changes/<change-name>/contract.md`

   Include artifacts that come **before** the artifact being reviewed in the chain. Do not include the artifact itself or artifacts that come after it. If none exist, use "None — this is the first artifact."

3. **Artifact type** — infer from the file name or content:
   - `UBIQUITOUS_LANGUAGE.md` → "UBIQUITOUS_LANGUAGE.md"
   - GitHub issue PRD → "PRD"
   - `proposal.md` → "proposal.md"
   - `specs/<capability>/spec.md` → "spec: <capability>"
   - `design.md` → "design.md"
   - `tasks.md` → "tasks.md"
   - `coverage-audit.md` → "coverage-audit.md"
   - `contract.md` → "contract.md"
   - Anything else → use the file name or a short description

## Step 2: Assemble Prompt

Read the prompt template from [prompt.md](prompt.md). Fill in the four variable slots:

- `{artifact_type}` — from Step 1
- `{artifact_content}` — full text of the artifact
- `{prior_artifacts}` — concatenated prior artifacts with `## <name>` headers between them
- `{ubiquitous_language}` — content of UBIQUITOUS_LANGUAGE.md or "Not established."

Write the assembled prompt to a temp file:

```bash
PROMPT_PATH=$(mktemp /tmp/artifact-review.XXXXXX)
# Write assembled prompt to $PROMPT_PATH
```

## Step 3: Dispatch

```bash
REPORT_PATH=$(mktemp /tmp/artifact-review-report.XXXXXX)

DISPATCH_RESULT=$(darkfactory-dispatch \
  --model "$DEFAULT_MODEL" \
  --effort high \
  --timeout 600 \
  --sandbox read-only \
  --prompt-file "$PROMPT_PATH" \
  --output "$REPORT_PATH" \
  --cwd "$(pwd)" \
  --json 2>/dev/null)

rm -f "$PROMPT_PATH"

DISPATCH_STATUS=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.status')
DISPATCH_ERROR=$(printf '%s\n' "$DISPATCH_RESULT" | jq -r '.error.message // empty')
```

## Step 4: Present Findings

If `$DISPATCH_STATUS` is `completed` and `$REPORT_PATH` exists, read the report and present the findings to the user.

If `$DISPATCH_STATUS` is `failed`, `timed_out`, or empty:

```text
Artifact review failed.

Status: <DISPATCH_STATUS>
Error: <DISPATCH_ERROR or "no error message">

The review did not complete. You can retry with /artifact-review or review the artifact manually.
```

Always clean up `$REPORT_PATH` after you have read and presented its contents:

```bash
rm -f "$REPORT_PATH"
```
