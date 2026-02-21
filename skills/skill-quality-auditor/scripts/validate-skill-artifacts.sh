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
    *.yaml|*.yml) ;;
    *)
      error "$file is in templates/ but is not a YAML file (.yaml/.yml)."
      return
      ;;
  esac

  if command -v yq >/dev/null 2>&1; then
    if ! yq eval '.' "$file" >/dev/null 2>&1; then
      error "$file is not valid YAML."
    fi
  fi
}

check_schemas_file() {
  file=$1

  case "$file" in
    *.schema.json) ;;
    *)
      error "$file is in schemas/ but does not use the .schema.json extension."
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
    *.sh) ;;
    *)
      error "$file is in scripts/ but is not a shell script (.sh)."
      return
      ;;
  esac

  first_line=$(head -n 1 "$file" 2>/dev/null || true)
  if [ "$first_line" != "#!/usr/bin/env sh" ]; then
    error "$file must start with portable shebang: #!/usr/bin/env sh"
  fi

  if ! sh -n "$file" >/dev/null 2>&1; then
    error "$file failed POSIX shell syntax check (sh -n)."
  fi
}

check_file() {
  file=$1

  case "$file" in
    skills/*/templates/*)
      check_templates_file "$file"
      ;;
    skills/*/schemas/*)
      check_schemas_file "$file"
      ;;
    skills/*/scripts/*)
      check_scripts_file "$file"
      ;;
  esac
}

if [ "$#" -gt 0 ]; then
  for file in "$@"; do
    [ -f "$file" ] || continue
    check_file "$file"
  done
else
  find skills -type f \( -path 'skills/*/templates/*' -o -path 'skills/*/schemas/*' -o -path 'skills/*/scripts/*' \) | while IFS= read -r file; do
    check_file "$file"
  done
fi

if [ "$errors" -ne 0 ]; then
  exit 1
fi
