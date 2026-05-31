#!/bin/sh
set -eu

rules_file="${1:-skills/lint-audit/rules.json}"

if ! command -v jq >/dev/null 2>&1; then
  printf '%s\n' "jq is required to validate ${rules_file}" >&2
  exit 1
fi

jq -e '
  def semver: test("^[0-9]+\\.[0-9]+\\.[0-9]+$");
  def canonical_rule_name: test("^[a-z0-9-]+/[a-z0-9-]+$");
  def allowed_category: IN("type-safety", "error-handling", "code-quality", "imports");
  def required_rules: [
    "typescript/no-explicit-any",
    "typescript/no-non-null-assertion",
    "typescript/no-floating-promises",
    "typescript/no-misused-promises",
    "typescript/switch-exhaustiveness-check"
  ];

  (.oxlint_version | type == "string" and semver)
  and (.rules | type == "array" and length >= 8)
  and (
    .rules
    | all(
      (.name | type == "string" and canonical_rule_name)
      and (.category | type == "string" and allowed_category)
      and (.severity | IN("error", "warn"))
      and (.default_enabled | type == "boolean")
      and (.rationale | type == "string" and length > 0)
    )
  )
  and ((.rules | map(.name) | length) == (.rules | map(.name) | unique | length))
  and (
    required_rules as $required
    | (.rules | map(.name)) as $actual
    | all($required[]; . as $rule | $actual | index($rule))
  )
' "$rules_file" >/dev/null
