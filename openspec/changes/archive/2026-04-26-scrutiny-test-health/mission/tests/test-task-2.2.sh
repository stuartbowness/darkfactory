#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/.claude/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

check6="$(awk '/^6\. \*\*Module depth review:/{in_check=1} /^7\. \*\*Code standards check:/{in_check=0} in_check {print}' "$prompt")"

contains_in_check6() {
  local needle="$1"
  grep -Fq "$needle" <<<"$check6" || fail "check 6 missing: $needle"
}

contains_in_check6 'Before evaluating module depth, read the `File sizes` named fact from the deterministic pre-pass.'
contains_in_check6 'If any touched production source file in `File sizes` exceeds 900 lines, name that file and its line count in the module depth evaluation.'
contains_in_check6 'Cross-reference `design.md` for concern boundaries for that module.'
contains_in_check6 'If `design.md` specifies concern boundaries that the implementation did not follow, flag Module depth as FAIL.'
contains_in_check6 'If `design.md` says this module is intentionally one file because of a specific reason, accept that file size as PASS.'
contains_in_check6 'If `design.md` does not address concern boundaries for that module, flag Module depth as FAIL for a design gap.'
contains_in_check6 'Exclude standalone test files from the 900-line check: files in `tests/` directories, `*.test.ts`, `*.spec.ts`, `test_*.py`, and `*_test.py`.'
contains_in_check6 'Rust `#[cfg(test)]` blocks inside production files count toward the 900-line threshold.'

printf 'PASS: scrutiny validator module depth file size instructions are present\n'
