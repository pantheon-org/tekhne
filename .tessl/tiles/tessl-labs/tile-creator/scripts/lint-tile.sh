#!/bin/bash
# Validate a tessl tile and display results
# Usage: lint-tile.sh <tile-path>

set -e

TILE_PATH="${1:?Usage: lint-tile.sh <tile-path>}"

if [ ! -d "$TILE_PATH" ]; then
    echo "Error: Directory not found: $TILE_PATH"
    exit 1
fi

if [ ! -f "$TILE_PATH/tile.json" ]; then
    echo "Error: No tile.json found in $TILE_PATH"
    exit 1
fi

echo "Validating tile: $TILE_PATH"
echo "================================"

# Run tessl tile lint and capture output
LINT_OUTPUT=$(tessl tile lint "$TILE_PATH" 2>&1) || true
LINT_EXIT=${PIPESTATUS[0]:-$?}

echo "$LINT_OUTPUT"
echo ""

# Categorize results
ERRORS=$(echo "$LINT_OUTPUT" | grep -ci "error" || true)
WARNINGS=$(echo "$LINT_OUTPUT" | grep -ci "warning" || true)

if [ "$LINT_EXIT" -ne 0 ]; then
    echo "RESULT: $ERRORS error(s), $WARNINGS warning(s)"
    echo "Fix errors. Warnings about source code references (src/, lib/) are typically expected in documentation tiles."
    exit 1
else
    echo "Tile is valid! ($WARNINGS warning(s) -- review if unexpected)"
fi

# Show tile structure
echo ""
echo "Tile structure:"
echo "---------------"
if command -v tree &> /dev/null; then
    tree "$TILE_PATH"
else
    find "$TILE_PATH" -type f | sort
fi
