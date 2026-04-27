#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
skill="$repo_root/.claude/skills/darkfactory-init/SKILL.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

line_of() {
  local needle="$1"
  grep -nF "$needle" "$skill" | head -n 1 | cut -d: -f1 || true
}

contains_skill() {
  local needle="$1"
  grep -Fq "$needle" "$skill" || fail "skill missing: $needle"
}

step2="$(
  awk '/^## Step 2: Check supporting skills/{in_step=1} /^## Step 3: Initialize OpenSpec/{in_step=0} in_step {print}' "$skill"
)"

step4="$(
  awk '/^## Step 4: Verify CLI dependencies/{in_step=1} /^## Step 5: Report status/{in_step=0} in_step {print}' "$skill"
)"

step5="$(
  awk '/^## Step 5: Report status/{in_step=1} in_step {print}' "$skill"
)"

[[ -n "$step2" ]] || fail "missing step 2"
[[ -n "$step4" ]] || fail "missing step 4"
[[ -n "$step5" ]] || fail "missing step 5"

contains_in_step2() {
  local needle="$1"
  grep -Fq "$needle" <<<"$step2" || fail "step 2 missing: $needle"
}

contains_in_step4() {
  local needle="$1"
  grep -Fq "$needle" <<<"$step4" || fail "step 4 missing: $needle"
}

contains_in_step5() {
  local needle="$1"
  grep -Fq "$needle" <<<"$step5" || fail "step 5 missing: $needle"
}

step2_line="$(line_of '## Step 2: Check supporting skills')"
step4_line="$(line_of '## Step 4: Verify CLI dependencies')"
coverage_line="$(line_of '### Coverage tool verification')"
step5_line="$(line_of '## Step 5: Report status')"

[[ -n "$step2_line" ]] || fail "missing step 2 heading"
[[ -n "$step4_line" ]] || fail "missing step 4 heading"
[[ -n "$coverage_line" ]] || fail "missing coverage tool verification heading"
[[ -n "$step5_line" ]] || fail "missing step 5 heading"
(( step4_line < coverage_line )) || fail "coverage verification must be inside step 4"
(( coverage_line < step5_line )) || fail "coverage verification must appear before step 5"

contains_in_step2 '~/.claude/skills/test-quality-review/SKILL.md'
contains_in_step2 'ln -sf ~/.darkfactory-skills/test-quality-review ~/.claude/skills/test-quality-review'

contains_in_step4 'Detect the project language from build files at the project root:'
contains_in_step4 '`Cargo.toml` -> Rust'
contains_in_step4 '`package.json` -> TypeScript/Node'
contains_in_step4 '`pyproject.toml` or `requirements.txt` -> Python'
contains_in_step4 'cargo tarpaulin --version 2>/dev/null || echo "NOT_INSTALLED"'
contains_in_step4 'cargo install cargo-tarpaulin'
contains_in_step4 '`c8`, `@jest/coverage`, `istanbul`, or `jest`'
contains_in_step4 'npm install -D c8'
contains_in_step4 'pip show pytest-cov 2>/dev/null || echo "NOT_INSTALLED"'
contains_in_step4 'pip install pytest-cov'
contains_in_step4 'No recognized language detected -- coverage tool check skipped.'
contains_in_step4 'command --version 2>/dev/null || echo "NOT_INSTALLED"'

contains_in_step5 '[x] /test-quality-review'
contains_in_step5 'Coverage tools:'
contains_in_step5 '[x] cargo-tarpaulin (Rust detected)'
contains_in_step5 '[ ] pytest-cov (Python detected, install: pip install pytest-cov)'
contains_in_step5 '[~] No recognized language detected -- skipped'

contains_skill 'If anything required is missing, list the install commands at the bottom.'

printf 'PASS: darkfactory-init coverage tooling and test-quality-review checks are present\n'
