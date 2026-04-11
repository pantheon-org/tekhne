#!/usr/bin/env bash
# shell: bash
# Validates .context/session/ctx/manifest.yaml schema
# Note: Manifest lives in .context/session/ctx/ (actionable snapshot), not .context/session/in/ (immutable bootstrap)

set -e

MANIFEST_FILE="${1:-.context/session/ctx/manifest.yaml}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

error() {
    echo -e "${RED}✗ ERROR:${NC} $1" >&2
    exit 1
}

warn() {
    echo -e "${YELLOW}⚠ WARNING:${NC} $1" >&2
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Check file exists
[ -f "$MANIFEST_FILE" ] || error "Manifest file not found: $MANIFEST_FILE"

success "Manifest file exists: $MANIFEST_FILE"

# Check YAML syntax (basic - just try to parse it)
if ! grep -q "^project:" "$MANIFEST_FILE"; then
    error "Missing required field: 'project'"
fi
success "Field 'project' present"

if ! grep -q "^created:" "$MANIFEST_FILE"; then
    error "Missing required field: 'created'"
fi
success "Field 'created' present"

if ! grep -q "^source_folder:" "$MANIFEST_FILE"; then
    warn "Missing field: 'source_folder' (recommended)"
fi

if ! grep -q "^sources:" "$MANIFEST_FILE"; then
    error "Missing required field: 'sources'"
fi
success "Field 'sources' present"

# Check priority sections exist
if ! grep -q "^  high:" "$MANIFEST_FILE"; then
    error "Missing sources.high section"
fi
success "Section 'sources.high' present"

if ! grep -q "^  medium:" "$MANIFEST_FILE"; then
    error "Missing sources.medium section"
fi
success "Section 'sources.medium' present"

if ! grep -q "^  low:" "$MANIFEST_FILE"; then
    error "Missing sources.low section"
fi
success "Section 'sources.low' present"

# Validate file paths exist
echo ""
echo "Validating file paths..."

# Manifest is in .context/session/ctx/, but files are in .context/session/in/ (source_folder)
SOURCE_FOLDER=$(grep "^source_folder:" "$MANIFEST_FILE" | sed 's/source_folder:[[:space:]]*//' || echo ".context/session/in/")
SOURCE_FOLDER="${SOURCE_FOLDER%/}"  # Remove trailing slash

# If source_folder not specified, assume .context/session/in
if [ -z "$SOURCE_FOLDER" ]; then
    SOURCE_FOLDER=".context/session/in"
fi

MISSING_FILES=0

# Extract file paths and check existence in source folder
while IFS= read -r line; do
    if [[ $line =~ ^[[:space:]]*-[[:space:]]+file:[[:space:]]+(.+)$ ]]; then
        filepath="${BASH_REMATCH[1]}"
        fullpath="$SOURCE_FOLDER/$filepath"

        if [ ! -f "$fullpath" ]; then
            warn "File not found in $SOURCE_FOLDER/: $filepath"
            MISSING_FILES=$((MISSING_FILES + 1))
        else
            success "File exists: $SOURCE_FOLDER/$filepath"
        fi
    fi
done < "$MANIFEST_FILE"

# Check descriptions are present
echo ""
echo "Validating descriptions..."

DESC_COUNT=$(grep -c "^[[:space:]]*desc:" "$MANIFEST_FILE" || true)
FILE_COUNT=$(grep -c "^[[:space:]]*- file:" "$MANIFEST_FILE" || true)

if [ "$DESC_COUNT" -ne "$FILE_COUNT" ]; then
    error "Mismatch: $FILE_COUNT files but $DESC_COUNT descriptions (should be equal)"
fi
success "All $FILE_COUNT files have descriptions"

# Check for empty descriptions
EMPTY_DESC=$(grep -c "^[[:space:]]*desc:[[:space:]]*$" "$MANIFEST_FILE" || true)
if [ "$EMPTY_DESC" -gt 0 ]; then
    error "Found $EMPTY_DESC empty description(s)"
fi
success "No empty descriptions"

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ "$MISSING_FILES" -eq 0 ]; then
    echo -e "${GREEN}✓ Validation passed${NC}"
    echo "  Files: $FILE_COUNT"
    echo "  Descriptions: $DESC_COUNT"
    exit 0
else
    echo -e "${YELLOW}⚠ Validation passed with warnings${NC}"
    echo "  Files: $FILE_COUNT"
    echo "  Missing: $MISSING_FILES"
    exit 0
fi
