#!/usr/bin/env bash
#
# migrate-skills-from-agents.sh
#
# Removes symlinks from skills/ and moves the actual skill directories
# from .agents/skills/ to skills/
#

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="${REPO_ROOT}/skills"
AGENTS_SKILLS_DIR="${REPO_ROOT}/.agents/skills"

echo "==> Starting skill migration from .agents/skills/ to skills/"
echo ""

# Verify directories exist
if [[ ! -d "${SKILLS_DIR}" ]]; then
  echo "Error: skills/ directory not found at ${SKILLS_DIR}"
  exit 1
fi

if [[ ! -d "${AGENTS_SKILLS_DIR}" ]]; then
  echo "Error: .agents/skills/ directory not found at ${AGENTS_SKILLS_DIR}"
  exit 1
fi

# Count symlinks to remove
symlink_count=0
for entry in "${SKILLS_DIR}"/*; do
  if [[ -L "${entry}" ]]; then
    ((symlink_count++))
  fi
done

echo "Found ${symlink_count} symlinks in skills/"
echo ""

# Remove all symlinks from skills/
echo "==> Removing symlinks from skills/"
for entry in "${SKILLS_DIR}"/*; do
  if [[ -L "${entry}" ]]; then
    skill_name="$(basename "${entry}")"
    echo "  Removing symlink: ${skill_name}"
    rm "${entry}"
  fi
done

echo ""
echo "==> Moving skill directories from .agents/skills/ to skills/"

# Move each directory from .agents/skills/ to skills/
moved_count=0
skipped_count=0

for source_dir in "${AGENTS_SKILLS_DIR}"/*; do
  if [[ -d "${source_dir}" ]]; then
    skill_name="$(basename "${source_dir}")"
    target_dir="${SKILLS_DIR}/${skill_name}"
    
    # Check if target already exists (and is not a symlink we just removed)
    if [[ -e "${target_dir}" ]]; then
      echo "  Skipping ${skill_name} (already exists in skills/)"
      ((skipped_count++))
    else
      echo "  Moving ${skill_name}"
      mv "${source_dir}" "${target_dir}"
      ((moved_count++))
    fi
  fi
done

echo ""
echo "==> Migration complete!"
echo "  - Removed ${symlink_count} symlinks"
echo "  - Moved ${moved_count} skill directories"
echo "  - Skipped ${skipped_count} existing directories"
echo ""

# Check if .agents/skills/ is now empty
remaining_count=$(find "${AGENTS_SKILLS_DIR}" -mindepth 1 -maxdepth 1 | wc -l)
if [[ ${remaining_count} -eq 0 ]]; then
  echo "==> .agents/skills/ is now empty"
  echo "    You can safely remove this directory with:"
  echo "    rm -rf .agents/skills/"
else
  echo "==> Warning: .agents/skills/ still contains ${remaining_count} items"
  echo "    Remaining items:"
  find "${AGENTS_SKILLS_DIR}" -mindepth 1 -maxdepth 1 -exec basename {} \;
fi
