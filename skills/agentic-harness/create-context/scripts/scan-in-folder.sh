#!/usr/bin/env bash
# shell: bash
# Scans .context/session/in/ folder for supported files, including subdirectories
# Outputs JSON list of files for LLM processing

set -e

IN_DIR="${1:-.context/session/in}"

# Security: Skip sensitive file patterns
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

# Check .context/session/in/ exists
if [ ! -d "$IN_DIR" ]; then
    echo "ERROR: Directory not found: $IN_DIR" >&2
    exit 1
fi

# Find supported files
FILES=$(find "$IN_DIR" -type f \( \
    -name "*.md" -o \
    -name "*.txt" -o \
    -name "*.csv" -o \
    -name "*.yaml" -o \
    -name "*.yml" -o \
    -name "*.json" \
    \) 2>/dev/null)

# Filter out sensitive files
FILTERED_FILES=""
for file in $FILES; do
    SKIP=0
    basename_file=$(basename "$file")

    for pattern in "${SKIP_PATTERNS[@]}"; do
        # Simple pattern matching
        if [[ "$basename_file" == "$pattern" ]]; then
            SKIP=1
            echo "SKIPPED (security): $file" >&2
            break
        fi
    done

    if [ $SKIP -eq 0 ]; then
        FILTERED_FILES="$FILTERED_FILES$file"$'\n'
    fi
done

# Count and validate
FILE_COUNT=$(echo -n "$FILTERED_FILES" | grep -c . || true)

if [ "$FILE_COUNT" -eq 0 ]; then
    echo "ERROR: No supported files found in $IN_DIR" >&2
    echo "Supported extensions: .md, .txt, .csv, .yaml, .yml, .json" >&2
    exit 1
fi

# Output as JSON array
echo "{"
echo "  \"count\": $FILE_COUNT,"
echo "  \"files\": ["

FIRST=1
echo -n "$FILTERED_FILES" | while IFS= read -r file; do
    [ -z "$file" ] && continue

    # Get relative path from .context/session/in/
    rel_path="${file#"$IN_DIR"/}"

    # Get file stats
    size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null)
    words=$(wc -w < "$file")

    # Estimate tokens (words / 0.75)
    tokens=$(echo "scale=0; $words / 0.75" | bc)

    if [ $FIRST -eq 1 ]; then
        FIRST=0
    else
        echo ","
    fi

    echo -n "    {"
    echo -n "\"path\": \"$rel_path\", "
    echo -n "\"size\": $size, "
    echo -n "\"words\": $words, "
    echo -n "\"tokens_est\": $tokens"
    echo -n "}"
done

echo ""
echo "  ]"
echo "}"
