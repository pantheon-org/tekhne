#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: zip-skill.sh <full-path-to-skill-dir> [output-dir]" >&2
  exit 1
fi

SKILL_DIR="$1"
OUT_DIR="${2:-dist/skills}"

if [ ! -d "$SKILL_DIR" ]; then
  echo "Not a directory: $SKILL_DIR" >&2
  exit 1
fi

SKILL_DIR="$(cd "$SKILL_DIR" && pwd)"

if [ ! -f "$SKILL_DIR/SKILL.md" ]; then
  echo "No SKILL.md found in $SKILL_DIR — is this a skill directory?" >&2
  exit 1
fi

SKILL_NAME="$(basename "$SKILL_DIR")"
mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

# -L dereferences symlinks, copying link targets as real files/dirs so the
# zip contains no symlink entries.
cp -RL "$SKILL_DIR" "$WORK_DIR/$SKILL_NAME"

# Drop dotfiles/dotfolders (audits, tessl plugin metadata, .git, etc.) and
# CHANGELOG.md — none of these belong in the uploaded skill bundle.
find "$WORK_DIR/$SKILL_NAME" -mindepth 1 -name '.*' -prune -exec rm -rf {} +
find "$WORK_DIR/$SKILL_NAME" -name 'CHANGELOG.md' -exec rm -f {} +

REMAINING_LINKS="$(find "$WORK_DIR/$SKILL_NAME" -type l)"
if [ -n "$REMAINING_LINKS" ]; then
  echo "Warning: unresolved symlinks remain after dereferencing:" >&2
  echo "$REMAINING_LINKS" >&2
  exit 1
fi

ZIP_PATH="$OUT_DIR/$SKILL_NAME.zip"
rm -f "$ZIP_PATH"

(
  cd "$WORK_DIR"
  zip -r -X "$ZIP_PATH" "$SKILL_NAME"
)

echo "$ZIP_PATH"
