#!/usr/bin/env bash
# shell: bash
# Scan the session in-folder for supported files, including subdirectories, and
# print a JSON list with a token estimate per file.
#
# Usage: scan-in-folder.sh [IN_DIR]
# Env:   CONTEXT_DIR  context root, default ".context"
#
# Security: a file is skipped when its basename OR any parent directory name
# matches a sensitive pattern, so "secrets/prod.yaml" is skipped as well as
# "prod-secrets.yaml". Skipped paths are reported on stderr.

set -euo pipefail

CONTEXT_DIR="${CONTEXT_DIR:-.context}"
IN_DIR="${1:-$CONTEXT_DIR/session/in}"
IN_DIR="${IN_DIR%/}"

SKIP_PATTERNS=(
    ".env*"
    "*credentials*"
    "*secrets*"
    "*token*"
    "*.key"
    "*.pem"
    "*.crt"
    "*.p12"
    "*.pfx"
)

if [ ! -d "$IN_DIR" ]; then
    echo "ERROR: Directory not found: $IN_DIR" >&2
    exit 1
fi

# True when any path component of $1 (relative to IN_DIR) matches a skip pattern.
is_sensitive() {
    local rel="$1" part pattern
    local IFS='/'
    for part in $rel; do
        for pattern in "${SKIP_PATTERNS[@]}"; do
            # shellcheck disable=SC2053  # glob match against the pattern is intended
            if [[ "$part" == $pattern ]]; then
                return 0
            fi
        done
    done
    return 1
}

json_escape() {
    local s="$1"
    s="${s//\\/\\\\}"
    s="${s//\"/\\\"}"
    printf '%s' "$s"
}

entries=()
while IFS= read -r -d '' file; do
    rel_path="${file#"$IN_DIR"/}"

    if is_sensitive "$rel_path"; then
        echo "SKIPPED (security): $file" >&2
        continue
    fi

    size=$(wc -c < "$file" | tr -d ' ')
    words=$(wc -w < "$file" | tr -d ' ')
    # tokens ~ words / 0.75, written as integer arithmetic to avoid a bc dependency
    tokens=$(( words * 4 / 3 ))

    entries+=("    {\"path\": \"$(json_escape "$rel_path")\", \"size\": $size, \"words\": $words, \"tokens_est\": $tokens}")
done < <(find "$IN_DIR" -type f \( \
    -name '*.md' -o -name '*.txt' -o -name '*.csv' -o \
    -name '*.yaml' -o -name '*.yml' -o -name '*.json' \
    \) -print0 | sort -z)

if [ "${#entries[@]}" -eq 0 ]; then
    echo "ERROR: No supported files found in $IN_DIR" >&2
    echo "Supported extensions: .md, .txt, .csv, .yaml, .yml, .json" >&2
    exit 1
fi

echo "{"
echo "  \"count\": ${#entries[@]},"
echo "  \"files\": ["
last=$(( ${#entries[@]} - 1 ))
for i in "${!entries[@]}"; do
    if [ "$i" -lt "$last" ]; then
        echo "${entries[$i]},"
    else
        echo "${entries[$i]}"
    fi
done
echo "  ]"
echo "}"
