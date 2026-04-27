#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
prompt="$repo_root/.claude/skills/mission/prompts/scrutiny-validator.md"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

contains() {
  local needle="$1"
  grep -Fq "$needle" "$prompt" || fail "missing: $needle"
}

line_of() {
  local needle="$1"
  grep -nF "$needle" "$prompt" | head -n 1 | cut -d: -f1 || true
}

prepass_line="$(line_of '## Deterministic Pre-passes')"
check_line="$(line_of '## Check Sequence')"

[[ -n "$prepass_line" ]] || fail "missing deterministic pre-pass section"
[[ -n "$check_line" ]] || fail "missing check sequence section"
(( prepass_line < check_line )) || fail "pre-pass section must appear before check sequence"

contains "Named facts"
contains "Detected language: <Rust | TypeScript/Node | Python | Unknown>"
contains "File sizes: <filename>: <lines>, <filename>: <lines>, ..."
contains "Error handling smells: <count> matches in <lines> changed production lines (<N> per KLOC). Locations: <file:line> <pattern>, ..."
contains "Test runner: <passed> passed, <failed> failed, <skipped> skipped (<pass_rate>% pass rate)"

contains "\`Cargo.toml\` -> Rust"
contains "\`package.json\` -> TypeScript/Node"
contains "\`pyproject.toml\` or \`requirements.txt\` -> Python"

contains "Rust: \`.rs\`"
contains "TypeScript/Node: \`.ts\`, \`.tsx\`, \`.js\`, \`.jsx\`"
contains "Python: \`.py\`"
contains "Rust: files in \`tests/\`"
contains "TypeScript/Node: \`*.test.ts\`, \`*.spec.ts\`, files in \`__tests__/\`"
contains "Python: \`test_*.py\`, \`*_test.py\`, files in \`tests/\`"
contains "\`#[cfg(test)]\` blocks inside production Rust files count toward the line total"

contains "\`.unwrap()\`"
contains "\`panic!()\`"
contains "\`todo!()\`"
contains "\`unimplemented!()\`"
contains "\`.expect(\"\")\`"
contains "\`.unwrap_or()\`"
contains "\`.unwrap_or_default()\`"
contains "\`.unwrap_or_else()\`"
contains "\`.expect(\"non-empty message\")\`"

contains "\`catch {}\`"
contains "\`catch (e) {}\`"
contains "\`as any\`"
contains "\`// @ts-ignore\`"
contains "\`// @ts-expect-error\` without explanation"
contains "\`.catch(() => {})\`"
contains "\`catch\` with a handler body"
contains "\`as unknown\`"

contains "\`except: pass\`"
contains "\`except Exception: pass\`"
contains "\`type: ignore\` without a specific code"
contains "bare \`raise\` without context"
contains "\`except SpecificError:\` with a handler"

contains "\`count / (changed_production_lines / 1000)\`"
contains "Count only added production lines from \`git diff <commit-sha>^ <commit-sha>\`"
contains "Report \`0 per KLOC\` when there are zero changed production lines"

contains "\`test result: ok. N passed; M failed; P ignored\`"
contains "\`Tests: N passed, M failed, P skipped, Q total\`"
contains "Vitest uses the same \`Tests:\` summary shape as Jest"
contains "\`N passed, M failed, P skipped\`"

printf 'PASS: scrutiny validator deterministic pre-pass instructions are present\n'
