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

check13="$(
  awk '/^13\. \*\*Diff coverage:/{in_check=1} /^## Output Format/{in_check=0} in_check {print}' "$prompt"
)"

[[ -n "$check13" ]] || fail "missing check 13"

check12_line="$(line_of '12. **Silent skip detection:**')"
check13_line="$(line_of '13. **Diff coverage:**')"
output_line="$(line_of '## Output Format')"

[[ -n "$check12_line" ]] || fail "missing check 12"
[[ -n "$check13_line" ]] || fail "missing check 13 line"
[[ -n "$output_line" ]] || fail "missing output format section"
(( check12_line < check13_line )) || fail "check 13 must appear after check 12"
(( check13_line < output_line )) || fail "check 13 must appear before output format"

contains_in_check13() {
  local needle="$1"
  grep -Fq "$needle" <<<"$check13" || fail "check 13 missing: $needle"
}

contains_prompt() {
  local needle="$1"
  grep -Fq "$needle" "$prompt" || fail "prompt missing: $needle"
}

contains_in_check13 'Use the `Detected language` named fact from the deterministic pre-pass to select the coverage tool.'
contains_in_check13 'Run coverage for the primary language only. In multi-language projects, use the language with the most production source files in the diff; do not run two coverage tools.'
contains_in_check13 'Rust: run `cargo tarpaulin --out json` first; if that is unavailable or fails because the tool is missing, use `cargo llvm-cov --json` as fallback.'
contains_in_check13 'TypeScript/Node: run `npx jest --coverage --json` or `npx c8 --reporter json`.'
contains_in_check13 'Python: run `pytest --cov --cov-report json`.'
contains_in_check13 'Run `git diff <commit-sha>^ <commit-sha> --name-only` to get changed files.'
contains_in_check13 'Filter the coverage JSON to those changed files, then count only added lines from `git diff <commit-sha>^ <commit-sha>`.'
contains_in_check13 'This is diff coverage: line coverage for new code only, not whole-file or whole-project coverage.'
contains_in_check13 'If the diff contains no production source files, report `diff-coverage | SKIP | no production source files in diff`.'
contains_in_check13 '80% or higher: PASS.'
contains_in_check13 '70-79%: WARN, include it in the report, and do not fail the task.'
contains_in_check13 'Below 70%: FAIL and trigger the fix loop.'
contains_in_check13 'If the coverage tool is not installed, first report `diff-coverage | SKIP | <tool> not installed` as a WARNING, then attempt auto-install as a fallback.'
contains_in_check13 'If auto-install succeeds, retry the coverage run.'
contains_in_check13 'If auto-install fails, keep the SKIP result.'
contains_in_check13 'For Rust, attempt `cargo install cargo-tarpaulin` before retrying tarpaulin.'
contains_in_check13 'Report success as `diff-coverage | PASS | 85% coverage on changed lines`.'
contains_in_check13 'Report warnings as `diff-coverage | WARN | 74% coverage on changed lines (target: 80%)`.'
contains_in_check13 'Report failures as `diff-coverage | FAIL | 55% coverage on changed lines (minimum: 70%)`.'
contains_in_check13 'Include details listing under-covered files with added-line ranges that lack coverage.'

contains_prompt '| Diff coverage | PASS/WARN/FAIL/SKIP | ... |'

printf 'PASS: scrutiny validator diff coverage instructions are present\n'
