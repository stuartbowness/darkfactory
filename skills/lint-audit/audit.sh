#!/bin/sh
set -eu

if ! command -v jq >/dev/null 2>&1; then
  printf '%s\n' '{"outcome":"error","error":"jq required"}'
  exit 1
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
rules_file="${script_dir}/rules.json"

tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/lint-audit.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

empty_array="${tmp_dir}/empty-array.json"
printf '%s\n' '[]' >"$empty_array"

json_string() {
  jq -Rn --arg value "$1" '$value'
}

emit_report() {
  outcome=$1
  linter=$2
  config_path_json=$3
  oxlint_findings_file=$4
  tsconfig_findings_file=$5
  type_aware=$6
  error_json=$7

  jq -n \
    --arg outcome "$outcome" \
    --arg linter "$linter" \
    --argjson config_path "$config_path_json" \
    --slurpfile oxlint_findings "$oxlint_findings_file" \
    --slurpfile tsconfig_findings "$tsconfig_findings_file" \
    --argjson type_aware "$type_aware" \
    --argjson error "$error_json" '
      {
        outcome: $outcome,
        linter: $linter,
        config_path: $config_path,
        oxlint_findings: $oxlint_findings[0],
        tsconfig_findings: $tsconfig_findings[0],
        type_aware: $type_aware
      }
      + (if $error == null then {} else {error: $error} end)
    '
}

usage_error() {
  emit_report "error" "unknown" "null" "$empty_array" "$empty_array" "false" "$(json_string "$1")"
  exit 1
}

project_dir=${1:-}
shift || true

if [ -z "$project_dir" ]; then
  usage_error "project-dir required"
fi

override_config=
while [ "$#" -gt 0 ]; do
  case "$1" in
    --oxlint-config)
      shift
      [ "$#" -gt 0 ] || usage_error "--oxlint-config requires a path"
      override_config=$1
      shift
      ;;
    *)
      usage_error "unknown argument: $1"
      ;;
  esac
done

if [ ! -d "$project_dir" ]; then
  usage_error "project-dir not found: $project_dir"
fi

project_abs=$(CDPATH= cd -- "$project_dir" && pwd)
package_json="${project_abs}/package.json"

skip_report() {
  emit_report "skip" "$1" "null" "$empty_array" "$empty_array" "false" "null"
  exit 0
}

if [ ! -f "$package_json" ]; then
  skip_report "none"
fi

package_probe="${tmp_dir}/package-probe.json"
if ! jq -e '
  def deps:
    ([.dependencies, .devDependencies, .peerDependencies, .optionalDependencies]
      | map(select(type == "object"))
      | add) // {};
  {
    oxlint: (deps | has("oxlint")),
    eslint: (deps | has("eslint")),
    type_aware_script: ((.scripts.lint // "") | tostring | contains("--type-aware"))
  }
' "$package_json" >"$package_probe" 2>"${tmp_dir}/package-error.txt"; then
  emit_report "error" "unknown" "null" "$empty_array" "$empty_array" "false" \
    "$(json_string "package.json parse failed: $(cat "${tmp_dir}/package-error.txt")")"
  exit 1
fi

has_oxlint=$(jq -r '.oxlint' "$package_probe")
has_eslint=$(jq -r '.eslint' "$package_probe")
type_aware_script=$(jq -r '.type_aware_script' "$package_probe")

if [ "$has_oxlint" != "true" ]; then
  if [ "$has_eslint" = "true" ]; then
    skip_report "eslint"
  fi

  skip_report "none"
fi

strip_json_comments() {
  perl -0777 -pe '
    s{/\*.*?\*/}{}gs;
    s{
      ("(?:[^"\\\\]|\\\\.)*")
      |
      (//[^\n\r]*)
    }{defined($1) ? $1 : ""}gex;
  ' "$1"
}

config_candidates="${tmp_dir}/config-candidates.txt"
: >"$config_candidates"
for candidate in .oxlintrc.json .oxlintrc.jsonc oxlint.config.ts; do
  if [ -f "${project_abs}/${candidate}" ]; then
    printf '%s\n' "$candidate" >>"$config_candidates"
  fi
done

config_path=
config_abs=
config_kind=none
if [ -n "$override_config" ]; then
  case "$override_config" in
    /*) config_abs=$override_config ;;
    *) config_abs="${project_abs}/${override_config}" ;;
  esac
  config_path=$override_config
  case "$config_path" in
    *.json) config_kind=json ;;
    *.jsonc) config_kind=jsonc ;;
    *.ts) config_kind=ts ;;
    *) config_kind=unknown ;;
  esac
else
  config_count=$(wc -l <"$config_candidates" | tr -d ' ')
  if [ "$config_count" -gt 1 ]; then
    tsconfig_findings="${tmp_dir}/tsconfig-findings-requires-selection.json"
    printf '%s\n' '[]' >"$tsconfig_findings"
    emit_report "requires_selection" "oxlint" "null" "$empty_array" "$tsconfig_findings" \
      "$type_aware_script" "$(json_string "multiple oxlint config files found")"
    exit 0
  fi

  if [ "$config_count" -eq 1 ]; then
    config_path=$(sed -n '1p' "$config_candidates")
    config_abs="${project_abs}/${config_path}"
    case "$config_path" in
      *.json) config_kind=json ;;
      *.jsonc) config_kind=jsonc ;;
      *.ts) config_kind=ts ;;
    esac
  fi
fi

config_json="${tmp_dir}/oxlint-config.json"
config_parse_error=
if [ -n "$config_abs" ]; then
  if [ ! -f "$config_abs" ]; then
    config_parse_error="oxlint config not found: $config_path"
  else
    case "$config_kind" in
      json)
        if ! jq '.' "$config_abs" >"$config_json" 2>"${tmp_dir}/config-error.txt"; then
          config_parse_error=$(cat "${tmp_dir}/config-error.txt")
        fi
        ;;
      jsonc)
        if ! strip_json_comments "$config_abs" | jq '.' >"$config_json" 2>"${tmp_dir}/config-error.txt"; then
          config_parse_error=$(cat "${tmp_dir}/config-error.txt")
        fi
        ;;
      ts)
        config_parse_error="oxlint.config.ts detected; rule extraction from TypeScript config is limited"
        ;;
      *)
        config_parse_error="unsupported oxlint config format: $config_path"
        ;;
    esac
  fi
fi

oxlint_findings="${tmp_dir}/oxlint-findings.json"
outcome_override=
if [ -n "$config_parse_error" ]; then
  outcome_override="error"
  jq -n --arg rationale "$config_parse_error" '[
    {
      name: "oxlint-config",
      status: "error",
      category: "code-quality",
      rationale: $rationale
    }
  ]' >"$oxlint_findings"
else
  if [ -n "$config_abs" ]; then
    jq -n --slurpfile rules "$rules_file" --slurpfile config "$config_json" '
      def explicit_status:
        if . == null then null
        elif type == "array" then (.[0] | explicit_status)
        elif type == "string" then
          (ascii_downcase) as $value
          | if $value == "off" or $value == "allow" or $value == "0" then "disabled"
            elif $value == "warn" or $value == "warning" or $value == "error" or $value == "1" or $value == "2" then "enabled"
            else null
            end
        elif type == "number" then if . == 0 then "disabled" else "enabled" end
        elif type == "boolean" then if . then "enabled" else "disabled" end
        elif type == "object" and has("severity") then (.severity | explicit_status)
        else null
        end;

      $rules[0].rules
      | map(
          . as $rule
          | (if ($config[0].rules | type) == "object" then ($config[0].rules[$rule.name] | explicit_status) else null end) as $status
          | {
              name: $rule.name,
              status: (
                if $status == "enabled" or $status == "disabled" then $status
                elif $rule.default_enabled then "enabled"
                else "missing"
                end
              ),
              category: $rule.category,
              severity: $rule.severity,
              rationale: (if (($status == null) and ($rule.default_enabled | not)) then $rule.rationale else "" end)
            }
        )
    ' >"$oxlint_findings"
  else
    jq -n --slurpfile rules "$rules_file" '
      $rules[0].rules
      | map({
          name,
          status: (if .default_enabled then "enabled" else "missing" end),
          category,
          severity,
          rationale: (if .default_enabled then "" else .rationale end)
        })
    ' >"$oxlint_findings"
  fi
fi

type_aware_config=false
if [ -s "$config_json" ]; then
  type_aware_config=$(jq -r '
    [
      .. | objects
      | if has("typeAware") then .typeAware
        elif has("type_aware") then .type_aware
        elif has("type-aware") then ."type-aware"
        else empty
        end
    ]
    | any(. == true or . == "true" or . == "enabled" or . == "on")
  ' "$config_json")
fi

if [ "$type_aware_script" = "true" ] || [ "$type_aware_config" = "true" ]; then
  type_aware=true
else
  type_aware=false
fi

tsconfig_list="${tmp_dir}/tsconfigs.txt"
(
  cd "$project_abs"
  find . \( -path './*/*/*/*' -prune \) -o -type f -name 'tsconfig*.json' -print
) | sed 's#^\./##' | sort >"$tsconfig_list"

tsconfig_items="${tmp_dir}/tsconfig-items.jsonl"
: >"$tsconfig_items"
has_tsc=true
if ! command -v tsc >/dev/null 2>&1; then
  has_tsc=false
fi

setting_rationale() {
  case "$1" in
    strict) printf '%s\n' "Enable strict TypeScript checking so agents cannot lean on weak inferred types." ;;
    noUncheckedIndexedAccess) printf '%s\n' "Require undefined handling for indexed access instead of assuming keys exist." ;;
    erasableSyntaxOnly) printf '%s\n' "Keep TypeScript syntax erasable so runtime behavior stays explicit." ;;
    *) printf '%s\n' "" ;;
  esac
}

while IFS= read -r tsconfig_path; do
  [ -n "$tsconfig_path" ] || continue

  effective_config="${tmp_dir}/effective-$(printf '%s' "$tsconfig_path" | tr '/.' '__').json"
  showconfig_note=
  if [ "$has_tsc" = "true" ]; then
    tsc_ok=false
    if (cd "$project_abs" && tsc --showConfig -p "$tsconfig_path") >"$effective_config" 2>"${tmp_dir}/tsc-showconfig-error.txt"; then
      if jq '.' "$effective_config" >/dev/null 2>&1; then
        tsc_ok=true
      else
        showconfig_note="tsc --showConfig produced non-JSON output; fell back to raw tsconfig"
      fi
    else
      showconfig_note="tsc --showConfig failed; fell back to raw tsconfig: $(cat "${tmp_dir}/tsc-showconfig-error.txt")"
    fi
    if [ "$tsc_ok" = "false" ]; then
      if ! strip_json_comments "${project_abs}/${tsconfig_path}" | jq '.' >"$effective_config" 2>"${tmp_dir}/tsconfig-parse-error.txt"; then
        jq -n --arg path "$tsconfig_path" --arg rationale "$(cat "${tmp_dir}/tsconfig-parse-error.txt")" '
          {
            path: $path,
            settings: [
              {
                name: "tsconfig",
                status: "error",
                category: "type-safety",
                rationale: $rationale
              }
            ]
          }
        ' >>"$tsconfig_items"
        continue
      fi
    fi
  else
    if ! strip_json_comments "${project_abs}/${tsconfig_path}" | jq '.' >"$effective_config" 2>"${tmp_dir}/tsconfig-parse-error.txt"; then
      jq -n --arg path "$tsconfig_path" --arg rationale "$(cat "${tmp_dir}/tsconfig-parse-error.txt")" '
        {
          path: $path,
          settings: [
            {
              name: "tsconfig",
              status: "error",
              category: "type-safety",
              rationale: $rationale
            },
            {
              name: "tsc",
              status: "error",
              category: "type-safety",
              rationale: "tsc not found; effective config and migration scopes unavailable."
            }
          ]
        }
      ' >>"$tsconfig_items"
      continue
    fi
  fi

  settings_items="${tmp_dir}/settings-$(printf '%s' "$tsconfig_path" | tr '/.' '__').jsonl"
  : >"$settings_items"
  for setting in strict noUncheckedIndexedAccess erasableSyntaxOnly; do
    setting_enabled=$(jq -r --arg setting "$setting" 'if type != "object" then false elif (.compilerOptions // {} | type) != "object" then false else .compilerOptions[$setting] == true end' "$effective_config")
    if [ "$setting_enabled" = "true" ]; then
      jq -n --arg name "$setting" '
        {
          name: $name,
          status: "enabled",
          category: "type-safety",
          rationale: ""
        }
      ' >>"$settings_items"
    else
      migration_scope_json=null
      if [ "$has_tsc" = "true" ]; then
        migration_scope=$(
          (cd "$project_abs" && tsc -p "$tsconfig_path" "--${setting}" --noEmit --pretty false 2>&1 | grep -c 'error TS') || true
        )
        case "$migration_scope" in
          ''|*[!0-9]*) migration_scope=0 ;;
        esac
        migration_scope_json=$migration_scope
      fi

      rationale=$(setting_rationale "$setting")
      if [ -n "$showconfig_note" ]; then
        rationale="${rationale} ${showconfig_note}"
      fi

      jq -n \
        --arg name "$setting" \
        --arg rationale "$rationale" \
        --argjson migration_scope "$migration_scope_json" '
          {
            name: $name,
            status: "missing",
            category: "type-safety",
            rationale: $rationale
          }
          + (if $migration_scope == null then {} else {migration_scope: $migration_scope} end)
      ' >>"$settings_items"
    fi
  done

  if [ "$has_tsc" != "true" ]; then
    jq -n '
      {
        name: "tsc",
        status: "error",
        category: "type-safety",
        rationale: "tsc not found; effective config and migration scopes unavailable."
      }
    ' >>"$settings_items"
  fi

  jq -s --arg path "$tsconfig_path" '{path: $path, settings: .}' "$settings_items" >>"$tsconfig_items"
done <"$tsconfig_list"

tsconfig_findings="${tmp_dir}/tsconfig-findings.json"
jq -s '.' "$tsconfig_items" >"$tsconfig_findings"

config_path_json=null
if [ -n "$config_path" ]; then
  config_path_json=$(json_string "$config_path")
fi

emit_report "${outcome_override:-ok}" "oxlint" "$config_path_json" "$oxlint_findings" "$tsconfig_findings" "$type_aware" "null"
