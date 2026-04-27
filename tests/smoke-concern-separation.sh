#!/bin/sh
set -eu

script_dir=$(CDPATH= cd "$(dirname "$0")" && pwd)
repo_root=$(CDPATH= cd "$script_dir/.." && pwd)
validator_prompt="$repo_root/skills/mission/prompts/scrutiny-validator.md"
tmp_base=${TMPDIR:-/tmp}
tmp_root=$(mktemp -d "$tmp_base/scrutiny-smoke.XXXXXX")

cleanup() {
  rm -rf "$tmp_root"
}

trap cleanup EXIT HUP INT TERM

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  if [ "${2:-}" ] && [ -f "$2" ]; then
    printf '%s\n' '--- claude output tail ---' >&2
    tail -n 80 "$2" >&2
    printf '%s\n' '--- end output tail ---' >&2
  fi
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

write_source_file() {
  line_count=$1
  source_file=$2

  {
    printf 'function main() { return 42; }\n'
    printf 'module.exports = { main };\n'
    line=3
    while [ "$line" -le "$line_count" ]; do
      printf '// padding line %04d\n' "$line"
      line=$((line + 1))
    done
  } > "$source_file"

  actual_count=$(wc -l < "$source_file" | tr -d ' ')
  [ "$actual_count" = "$line_count" ] || fail "fixture source has $actual_count lines, expected $line_count"
}

create_fixture_repo() {
  case_name=$1
  line_count=$2
  fixture_dir="$tmp_root/$case_name"

  mkdir -p "$fixture_dir/src" "$fixture_dir/__tests__"
  (
    cd "$fixture_dir"
    git init -q
    git config user.name 'Scrutiny Smoke'
    git config user.email 'scrutiny-smoke@example.invalid'

    cat > package.json <<'JSON'
{"scripts":{"build":"true","lint":"true","test":"node --test"},"devDependencies":{}}
JSON

    cat > design.md <<'DESIGN'
# Design

This fixture intentionally leaves implementation module boundaries unspecified.
DESIGN

    write_source_file "$line_count" src/index.js

    cat > __tests__/index.test.js <<'TEST'
const assert = require('node:assert/strict');
const test = require('node:test');
const { main } = require('../src/index.js');

test('main returns fixture value', () => {
  assert.equal(main(), 42);
});
TEST

    git add .
    git commit -q -m "fixture for $case_name"
    git rev-parse HEAD
  )
}

fill_prompt() {
  commit_sha=$1
  output_file=$2

  awk -v sha="$commit_sha" '
    $0 == "SHA: <commit-sha>" {
      print "SHA: " sha
      next
    }
    $0 == "<paste the assertions from contract.md for task N>" {
      print "- [ ] [inspection] file exists"
      next
    }
    { print }
  ' "$validator_prompt" > "$output_file"
}

run_validator() {
  case_name=$1
  line_count=$2
  output_file=$3

  commit_sha=$(create_fixture_repo "$case_name" "$line_count")
  fixture_dir="$tmp_root/$case_name"
  prompt_file="$tmp_root/$case_name-prompt.md"
  fill_prompt "$commit_sha" "$prompt_file"
  prompt_text=$(cat "$prompt_file")

  (
    cd "$fixture_dir"
    claude -p --output-format text --permission-mode bypassPermissions "$prompt_text"
  ) > "$output_file" 2>&1 || fail "$case_name: claude -p failed" "$output_file"
}

assert_contains() {
  case_name=$1
  needle=$2
  output_file=$3

  grep -Fq "$needle" "$output_file" || fail "$case_name: expected output to contain '$needle'" "$output_file"
}

assert_contains_ci() {
  case_name=$1
  needle=$2
  output_file=$3

  grep -iq "$needle" "$output_file" || fail "$case_name: expected output to contain '$needle' case-insensitively" "$output_file"
}

assert_module_depth_fail() {
  case_name=$1
  output_file=$2

  grep -Eiq 'module[[:space:]_-]*depth.*FAIL|FAIL.*module[[:space:]_-]*depth' "$output_file" \
    || fail "$case_name: expected Module depth result to be FAIL" "$output_file"
}

require_command git
require_command node
require_command npm
require_command claude
[ -f "$validator_prompt" ] || fail "missing scrutiny validator prompt: $validator_prompt"

warning_output="$tmp_root/warning-output.txt"
fail_output="$tmp_root/fail-output.txt"

run_validator "fixture-warning-650-lines" 650 "$warning_output"
assert_contains_ci "Fixture Test A" "warn" "$warning_output"

run_validator "fixture-fail-1050-lines" 1050 "$fail_output"
assert_module_depth_fail "Fixture Test B" "$fail_output"

printf '%s\n' 'PASS: scrutiny validator graduated file size thresholds fire for 650-line WARN and 1050-line FAIL fixtures'
