#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/.claude/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

line_of() {
  local needle="$1"
  grep -nF "$needle" "$prompt" | head -n 1 | cut -d: -f1 || true
}

check14="$(
  awk '/^14\. \*\*Test quality review:/{in_check=1} /^## Output Format/{in_check=0} in_check {print}' "$prompt"
)"

[[ -n "$check14" ]] || fail "missing check 14"

check13_line="$(line_of '13. **Diff coverage:**')"
check14_line="$(line_of '14. **Test quality review:**')"
output_line="$(line_of '## Output Format')"

[[ -n "$check13_line" ]] || fail "missing check 13"
[[ -n "$check14_line" ]] || fail "missing check 14 line"
[[ -n "$output_line" ]] || fail "missing output format section"
(( check13_line < check14_line )) || fail "check 14 must appear after check 13"
(( check14_line < output_line )) || fail "check 14 must appear before output format"

contains_in_check14() {
  local needle="$1"
  grep -Fq "$needle" <<<"$check14" || fail "check 14 missing: $needle"
}

contains_prompt() {
  local needle="$1"
  grep -Fq "$needle" "$prompt" || fail "prompt missing: $needle"
}

contains_in_check14 'If the diff contains no test files, report `test-quality-review | SKIP | no test files in diff`.'
contains_in_check14 'Run /test-quality-review as a subagent against the full diff, following the same delegation pattern as /review, /simplify, and /security-review.'
contains_in_check14 'Read the skill markdown findings after the subagent completes.'
contains_in_check14 'Findings are grouped by severity (P0, P1, P2, P3) and include file:line, evidence, and remediation.'
contains_in_check14 'Any P0 finding, including tautological assertions, is a FAIL.'
contains_in_check14 '3 or more P1 findings are a FAIL.'
contains_in_check14 'P2 and P3 findings only are WARNINGS; include them in the report but do not fail the task.'
contains_in_check14 'No findings is PASS.'
contains_in_check14 'Report passing output as `test-quality-review | PASS | no findings`.'
contains_in_check14 'Report warning output as `test-quality-review | WARN | 2 P2, 1 P3 findings`.'
contains_in_check14 'Report failing output as `test-quality-review | FAIL | 1 P0 finding (tautological assertion in test_list_open)`.'
contains_in_check14 'Include a summary of findings in the notes column.'

contains_prompt '| Test quality review | PASS/WARN/FAIL/SKIP | ... |'

printf 'PASS: scrutiny validator test quality review instructions are present\n'
