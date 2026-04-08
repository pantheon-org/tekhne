#!/usr/bin/env sh
set -eu

errors=0

error() {
  printf 'ERROR: %s\n' "$1" >&2
  errors=1
}

check_templates_file() {
  file=$1
  case "$file" in
    */.gitkeep) return ;;
    *.yaml|*.yml)
      if command -v yq >/dev/null 2>&1; then
        if grep -qE '\{\{|\{%|\[[a-z_]+\]' "$file" 2>/dev/null; then
          :
        elif ! yq eval '.' "$file" >/dev/null 2>&1; then
          error "$file is not valid YAML."
        fi
      fi
      ;;
  esac
}

check_schemas_file() {
  file=$1

  case "$file" in
    *.schema.json) ;;
    *)
      error "$file is in assets/schemas/ but does not use the .schema.json extension."
      return
      ;;
  esac

  if command -v jq >/dev/null 2>&1; then
    if ! jq -e . "$file" >/dev/null 2>&1; then
      error "$file is not valid JSON."
      return
    fi
  fi

  schema_pattern="\"\\\$schema\"[[:space:]]*:[[:space:]]*\"https?://json-schema.org/"
  if ! grep -Eq "$schema_pattern" "$file"; then
    error "$file does not declare a JSON Schema \"\$schema\" URL from json-schema.org."
  fi
}

check_scripts_file() {
  file=$1

  case "$file" in
    */__pycache__/*|*/.gitkeep) return ;;
  esac

  case $file in
    *.sh) check_shell_script "$file" ;;
    *.py) check_python_script "$file" ;;
    *.ts) check_ts_script "$file" ;;
    *.js) check_js_script "$file" ;;
    *)
      error "$file is in scripts/ but is not a recognised script type (.sh, .py, .ts, .js)."
      ;;
  esac
}

check_shell_script() {
  file=$1
  first_line=$(head -n 1 "$file" 2>/dev/null || true)
  if [ "$first_line" = "#!/usr/bin/env sh" ]; then
    :
  elif [ "$first_line" = "#!/usr/bin/env bash" ]; then
    if ! grep -q '^# shell: bash' "$file" 2>/dev/null; then
      error "$file must start with portable shebang: #!/usr/bin/env sh (or add '# shell: bash' to allow bash)"
    fi
  else
    error "$file must start with portable shebang: #!/usr/bin/env sh (or add '# shell: bash' to allow bash)"
  fi

  if [ "$first_line" = "#!/usr/bin/env sh" ]; then
    if ! sh -n "$file" >/dev/null 2>&1; then
      error "$file failed POSIX shell syntax check (sh -n)."
    fi
  fi
}

check_python_script() {
  file=$1
  first_line=$(head -n 1 "$file" 2>/dev/null || true)
  case "$first_line" in
    "#!/usr/bin/env python3"|"#!/usr/bin/env python") ;;
    *)
      error "$file must start with shebang: #!/usr/bin/env python3"
      return
      ;;
  esac

  if command -v python3 >/dev/null 2>&1; then
    if ! python3 -m py_compile "$file" 2>/dev/null; then
      error "$file failed Python syntax check (py_compile)."
    fi
  fi
}

check_ts_script() {
  file=$1
  first_line=$(head -n 1 "$file" 2>/dev/null || true)
  case "$first_line" in
    "#!/usr/bin/env bun"|"#!/usr/bin/env -S bun"|"#!/usr/bin/env -S bun run") ;;
    *)
      error "$file must start with shebang: #!/usr/bin/env bun"
      return
      ;;
  esac
}

check_js_script() {
  file=$1
  first_line=$(head -n 1 "$file" 2>/dev/null || true)
  case "$first_line" in
    "#!/usr/bin/env node"|"#!/usr/bin/env bun") ;;
    *)
      error "$file must start with shebang: #!/usr/bin/env node (or #!/usr/bin/env bun)"
      return
      ;;
  esac
}

check_file() {
  file=$1

  case "$file" in
    skills/*/assets/templates/*)
      check_templates_file "$file"
      ;;
    skills/*/assets/schemas/*)
      check_schemas_file "$file"
      ;;
    skills/*/scripts/*)
      check_scripts_file "$file"
      ;;
  esac
}

check_skill_dir() {
  skill_dir=$1
  skill_name=$(basename "$skill_dir")

  for entry in "$skill_dir"/*/; do
    [ -d "$entry" ] || continue
    dir_name=$(basename "$entry")
    case "$dir_name" in
      scripts|references|assets|evals|schemas|templates) ;;
      *)
        error "$skill_dir: non-standard directory '$dir_name' (allowed: scripts/, references/, assets/, evals/, schemas/, templates/)"
        ;;
    esac
  done

  # Validate assets/ subdirectory names and contents
  if [ -d "$skill_dir/assets" ]; then
    for entry in "$skill_dir/assets"/*/; do
      [ -d "$entry" ] || continue
      assets_dir_name=$(basename "$entry")
      case "$assets_dir_name" in
        templates|schemas|requirements|examples) ;;
        *)
          error "$skill_dir: non-standard assets/ subdirectory '$assets_dir_name' (allowed: templates/, schemas/, requirements/, examples/)"
          ;;
      esac
    done

    for f in "$skill_dir/assets/"*.yaml "$skill_dir/assets/"*.yml; do
      [ -f "$f" ] || continue
      error "$f: YAML files must be placed in assets/templates/, not directly in assets/"
    done
  fi

  skill_md="$skill_dir/SKILL.md"
  [ -f "$skill_md" ] || return

  line_count=$(wc -l < "$skill_md")
  if [ "$line_count" -gt 500 ]; then
    error "$skill_md: $line_count lines exceeds 500-line limit — move detail to references/"
  fi

  name_val=$(grep '^name:' "$skill_md" | head -1 | sed 's/^name:[[:space:]]*//' | tr -d '"'"'" | tr -d ' ')
  if [ -n "$name_val" ] && [ "$name_val" != "$skill_name" ]; then
    error "$skill_md: frontmatter name '$name_val' does not match directory name '$skill_name'"
  fi

  # shellcheck disable=SC2016
  stripped=$(sed '/^```/,/^```/d' "$skill_md")
  if printf '%s\n' "$stripped" | grep -qE '\.\./'; then
    error "$skill_md: contains '../' path reference outside code blocks (skills must be self-contained)"
  fi
}

if [ "$#" -gt 0 ]; then
  for file in "$@"; do
    [ -f "$file" ] || continue
    check_file "$file"
    case "$file" in
      skills/*/*/SKILL.md)
        skill_dir=$(dirname "$file")
        check_skill_dir "$skill_dir"
        ;;
    esac
  done
else
  tmp_files=$(mktemp)
  trap 'rm -f "$tmp_files"' EXIT INT TERM
  find skills -type f \( -path 'skills/*/assets/templates/*' -o -path 'skills/*/assets/schemas/*' -o -path 'skills/*/scripts/*' \) > "$tmp_files"
  while IFS= read -r file; do
    check_file "$file"
  done < "$tmp_files"

  find skills -mindepth 2 -maxdepth 2 -type d > "$tmp_files"
  while IFS= read -r skill_dir; do
    [ -f "$skill_dir/SKILL.md" ] || continue
    check_skill_dir "$skill_dir"
  done < "$tmp_files"
fi

if [ "$errors" -ne 0 ]; then
  exit 1
fi
