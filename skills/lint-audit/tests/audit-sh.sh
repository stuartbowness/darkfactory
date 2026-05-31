#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
audit_sh=$(CDPATH= cd -- "${script_dir}/.." && pwd)/audit.sh

tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/lint-audit-test.XXXXXX")
trap 'rm -rf "$tmp_root"' EXIT HUP INT TERM

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

assert_jq() {
  file=$1
  filter=$2
  message=$3

  jq -e "$filter" "$file" >/dev/null || fail "$message"
}

write_package_json() {
  project=$1
  package_name=$2

  mkdir -p "$project"
  jq -n --arg package_name "$package_name" '
    {
      scripts: {},
      devDependencies: {
        ($package_name): "1.0.0"
      }
    }
  ' >"${project}/package.json"
}

write_package_json_with_lint_script() {
  project=$1
  package_name=$2
  lint_script=$3

  mkdir -p "$project"
  jq -n --arg package_name "$package_name" --arg lint_script "$lint_script" '
    {
      scripts: {
        lint: $lint_script
      },
      devDependencies: {
        ($package_name): "1.0.0"
      }
    }
  ' >"${project}/package.json"
}

run_audit() {
  project=$1
  output=$2

  "$audit_sh" "$project" >"$output"
}

test_oxlint_project_without_config_reports_defaults() {
  project="${tmp_root}/oxlint-no-config"
  output="${tmp_root}/oxlint-no-config.json"
  write_package_json "$project" oxlint

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "ok"' "oxlint project should audit"
  assert_jq "$output" '.linter == "oxlint"' "oxlint project should report oxlint linter"
  assert_jq "$output" '.config_path == null' "missing oxlint config should report null config path"
  assert_jq "$output" '
    any(.oxlint_findings[];
      .name == "typescript/no-floating-promises"
      and .status == "enabled"
    )
  ' "default-enabled rule should be enabled without config"
  assert_jq "$output" '
    any(.oxlint_findings[];
      .name == "typescript/no-explicit-any"
      and .status == "missing"
      and (.rationale | length > 0)
    )
  ' "non-default rule should be missing without config and include rationale"
}

test_eslint_only_project_skips() {
  project="${tmp_root}/eslint-only"
  output="${tmp_root}/eslint-only.json"
  write_package_json "$project" eslint

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "skip"' "eslint-only project should skip"
  assert_jq "$output" '.linter == "eslint"' "eslint-only project should report eslint linter"
}

test_project_without_package_json_skips() {
  project="${tmp_root}/no-package"
  output="${tmp_root}/no-package.json"
  mkdir -p "$project"

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "skip"' "project without package.json should skip"
  assert_jq "$output" '.linter == "none"' "project without package.json should report no linter"
}

test_json_config_reports_enabled_and_disabled_rules() {
  project="${tmp_root}/json-config"
  output="${tmp_root}/json-config.json"
  write_package_json_with_lint_script "$project" oxlint "oxlint --type-aware"
  jq -n '{
    rules: {
      "typescript/no-explicit-any": "error",
      "typescript/no-non-null-assertion": "off"
    }
  }' >"${project}/.oxlintrc.json"

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "ok"' "json config project should audit"
  assert_jq "$output" '.config_path == ".oxlintrc.json"' "json config path should be reported"
  assert_jq "$output" '.type_aware == true' "lint script --type-aware should be detected"
  assert_jq "$output" '
    any(.oxlint_findings[];
      .name == "typescript/no-explicit-any"
      and .status == "enabled"
    )
  ' "configured error rule should be enabled"
  assert_jq "$output" '
    any(.oxlint_findings[];
      .name == "typescript/no-non-null-assertion"
      and .status == "disabled"
    )
  ' "configured off rule should be disabled"
}

test_tsconfig_missing_strict_reports_migration_scope() {
  project="${tmp_root}/tsconfig-strict"
  output="${tmp_root}/tsconfig-strict.json"
  bin_dir="${tmp_root}/bin"
  mkdir -p "$bin_dir"
  write_package_json "$project" oxlint
  jq -n '{
    compilerOptions: {
      strict: false,
      noUncheckedIndexedAccess: true
    },
    files: ["index.ts"]
  }' >"${project}/tsconfig.json"
  printf '%s\n' 'const value = 1;' >"${project}/index.ts"
  cat >"${bin_dir}/tsc" <<'EOF'
#!/bin/sh
case " $* " in
  *" --showConfig "*)
    printf '%s\n' '{"compilerOptions":{"strict":false,"noUncheckedIndexedAccess":true},"files":["./index.ts"]}'
    exit 0
    ;;
  *" --strict "*)
    printf '%s\n' 'index.ts(1,1): error TS7006: Parameter implicitly has an any type.'
    printf '%s\n' 'index.ts(2,1): error TS2322: Type number is not assignable to string.'
    exit 2
    ;;
  *)
    exit 0
    ;;
esac
EOF
  chmod +x "${bin_dir}/tsc"

  PATH="${bin_dir}:$PATH" run_audit "$project" "$output"

  assert_jq "$output" '
    any(.tsconfig_findings[];
      .path == "tsconfig.json"
      and any(.settings[];
        .name == "strict"
        and .status == "missing"
        and .migration_scope == 2
      )
    )
  ' "missing strict should include migration scope from tsc errors"
  assert_jq "$output" '
    any(.tsconfig_findings[];
      .path == "tsconfig.json"
      and any(.settings[];
        .name == "noUncheckedIndexedAccess"
        and .status == "enabled"
      )
    )
  ' "enabled tsconfig setting should be reported"
}

test_jsonc_config_parses_correctly() {
  project="${tmp_root}/jsonc-config"
  output="${tmp_root}/jsonc-config.json"
  write_package_json "$project" oxlint
  cat >"${project}/.oxlintrc.jsonc" <<'JSONC'
{
  // Enable strict rules
  /* Block comment */
  "rules": {
    "typescript/no-explicit-any": "error"
  },
  "description": "path with // slashes"
}
JSONC

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "ok"' "jsonc config should audit successfully"
  assert_jq "$output" '.config_path == ".oxlintrc.jsonc"' "jsonc config path should be reported"
  assert_jq "$output" '
    any(.oxlint_findings[];
      .name == "typescript/no-explicit-any"
      and .status == "enabled"
    )
  ' "jsonc config rule should be read correctly"
}

test_requires_selection_with_multiple_configs() {
  project="${tmp_root}/multi-config"
  output="${tmp_root}/multi-config.json"
  write_package_json "$project" oxlint
  printf '%s\n' '{"rules":{}}' >"${project}/.oxlintrc.json"
  printf '%s\n' 'export default {}' >"${project}/oxlint.config.ts"

  run_audit "$project" "$output"

  assert_jq "$output" '.outcome == "requires_selection"' "multiple configs should require selection"
}

test_oxlint_config_override() {
  project="${tmp_root}/config-override"
  output="${tmp_root}/config-override.json"
  write_package_json "$project" oxlint
  printf '%s\n' '{"rules":{}}' >"${project}/.oxlintrc.json"
  printf '%s\n' 'export default {}' >"${project}/oxlint.config.ts"

  "$audit_sh" "$project" --oxlint-config "${project}/.oxlintrc.json" >"$output"

  assert_jq "$output" '.outcome == "ok"' "override should resolve to ok"
  assert_jq "$output" '(.config_path | endswith(".oxlintrc.json"))' "override config path should be reported"
}

test_missing_config_override_returns_error() {
  project="${tmp_root}/missing-override"
  output="${tmp_root}/missing-override.json"
  write_package_json "$project" oxlint

  "$audit_sh" "$project" --oxlint-config "${project}/nonexistent.json" >"$output"

  assert_jq "$output" '.outcome == "error"' "missing config override should return error outcome"
}

test_oxlint_project_without_config_reports_defaults
test_eslint_only_project_skips
test_project_without_package_json_skips
test_json_config_reports_enabled_and_disabled_rules
test_tsconfig_missing_strict_reports_migration_scope
test_jsonc_config_parses_correctly
test_requires_selection_with_multiple_configs
test_oxlint_config_override
test_missing_config_override_returns_error

printf 'audit-sh tests passed\n'
