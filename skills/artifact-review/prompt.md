You are reviewing a planning artifact for a software mission. Find real problems that would cause issues during implementation, validation, or human review. Do not manufacture findings — an artifact with zero issues is a valid outcome.

## Artifact Under Review

Type: {artifact_type}

<artifact>
{artifact_content}
</artifact>

## Context

<prior-artifacts>
{prior_artifacts}
</prior-artifacts>

<ubiquitous-language>
{ubiquitous_language}
</ubiquitous-language>

## Review Dimensions

Check each dimension below. Skip dimensions that do not apply to this artifact type.

### 1. Ambiguity

Would two competent engineers reading this produce the same implementation? Flag any requirement, spec, or description where reasonable interpretations diverge. Quote the ambiguous text.

### 2. Terminology Consistency

If UBIQUITOUS_LANGUAGE.md content is provided above, check that the artifact uses terms consistently with the glossary. Flag drift (using a synonym instead of the canonical term), undefined terms that should be in the glossary, or terms used with a different meaning than the glossary defines.

### 3. Contradictions

Does this artifact contradict any prior artifact provided above? Flag specific conflicts with quotes from both documents. If no prior artifacts are provided, skip this dimension.

### 4. Missing Edge Cases

What error paths, boundary conditions, or failure modes does the artifact ignore that would affect implementation? Only flag cases where an implementer would have to make an unguided decision. Do not flag hypothetical edge cases that fall outside the stated scope.

### 5. Coverage Gaps

Does this artifact cover everything it should, given the prior artifacts? Flag requirements that appeared earlier but are missing here, or new surface area introduced without upstream justification.

### 6. Feasibility

Is anything described here technically impossible, impractical, or significantly harder than the artifact assumes? Flag only when you can name the specific technical obstacle.

## Output

Write your findings to the output file as markdown. Use this format:

```markdown
# Artifact Review: {artifact_type}

## Summary

<one sentence: clean / N findings across M dimensions>

## Findings

### [SEVERITY] Finding title
- **Dimension:** <which dimension 1-6>
- **Quote:** "<exact text from the artifact>"
- **Problem:** <what's wrong and why it matters downstream>
- **Suggestion:** <how to fix it, one sentence>

### [SEVERITY] ...
```

Severity levels:
- **CRITICAL** — blocks implementation; an engineer cannot proceed without resolving this
- **HIGH** — likely causes rework; the implementation will need to be redone
- **MEDIUM** — worth addressing; reduces quality or clarity
- **LOW** — optional improvement; no downstream impact if ignored

If no findings, write:

```markdown
# Artifact Review: {artifact_type}

## Summary

No findings. <one sentence explaining why — e.g., "terminology is consistent with the glossary, requirements are unambiguous, and no contradictions with prior artifacts.">
```
