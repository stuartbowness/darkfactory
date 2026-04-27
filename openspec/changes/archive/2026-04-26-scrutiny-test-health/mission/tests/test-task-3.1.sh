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

check12="$(
  awk '/^12\. \*\*Silent skip detection:/{in_check=1} /^## Output Format/{in_check=0} in_check {print}' "$prompt"
)"

[[ -n "$check12" ]] || fail "missing check 12"

check11_line="$(line_of '11. **Behavioral conformance:**')"
check12_line="$(line_of '12. **Silent skip detection:**')"
output_line="$(line_of '## Output Format')"

[[ -n "$check11_line" ]] || fail "missing check 11"
[[ -n "$check12_line" ]] || fail "missing check 12 line"
[[ -n "$output_line" ]] || fail "missing output format section"
(( check11_line < check12_line )) || fail "check 12 must appear after check 11"
(( check12_line < output_line )) || fail "check 12 must appear before output format"

contains_in_check12() {
  local needle="$1"
  grep -Fq "$needle" <<<"$check12" || fail "check 12 missing: $needle"
}

contains_prompt() {
  local needle="$1"
  grep -Fq "$needle" "$prompt" || fail "prompt missing: $needle"
}

contains_in_check12 'Use the `Detected language` named fact from the deterministic pre-pass to choose the primary language-specific scan, and also inspect file extensions in the diff so mixed-language projects are covered.'
contains_in_check12 'Test file detection uses the same language-specific patterns as the file size count exclusions: Rust files in `tests/` or files containing `#[test]`; TypeScript/Node `*.test.ts`, `*.spec.ts`, or files in `__tests__/`; Python `test_*.py`, `*_test.py`, or files in `tests/`.'
contains_in_check12 'If the diff contains no test files, report `silent-skip-detection | SKIP | no test files in diff`.'
contains_in_check12 'Any silent skip found is a FAIL because silent skips give false confidence by hiding missing coverage.'

contains_in_check12 'Flag `else { return; }` or `else { return }` inside functions annotated with `#[test]` or `#[tokio::test]`.'
contains_in_check12 'Flag `if env::var("X").is_err() { return; }` or similar top-of-test conditional returns based on environment variables.'
contains_in_check12 'Do not flag Rust test functions marked with `#[ignore]`.'

contains_in_check12 'Flag `if (!process.env.X) return` inside `it()`, `test()`, or `describe()` blocks.'
contains_in_check12 'Flag `if (!process.env.X) { return; }` inside `it()`, `test()`, or `describe()` blocks.'
contains_in_check12 'Do not flag `it.skip()`, `describe.skip()`, or `@jest.skip`.'

contains_in_check12 'Flag `if not os.environ.get("X"): return` inside `test_` functions.'
contains_in_check12 'Flag bare `return` at the top of `test_` functions when it is conditional on environment variables.'
contains_in_check12 'Do not flag `@pytest.mark.skip` or `pytest.skip("reason")` with an explicit message.'

contains_in_check12 'Report failures as `silent-skip-detection | FAIL | N silent skips found`.'
contains_in_check12 'For each instance, list the file path, line number, test function name, and this remediation guidance: "Either fail explicitly with `panic!("requires X")` or mark as `#[ignore]` / `@skip` so the test runner reports it as skipped, not passed."'

contains_prompt '| Silent skip detection | PASS/FAIL/SKIP | ... |'

printf 'PASS: scrutiny validator silent skip detection instructions are present\n'
