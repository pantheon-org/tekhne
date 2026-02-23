#!/usr/bin/env sh
# Check basic consistency across skill directories.

set -eu

SKILLS_DIR="${1:-skills}"
ISSUES=0

for skill_dir in "$SKILLS_DIR"/*; do
  [ -d "$skill_dir" ] || continue
  skill_name=$(basename "$skill_dir")

  if [ ! -f "$skill_dir/SKILL.md" ]; then
    echo "MISSING_SKILL: $skill_name/SKILL.md"
    ISSUES=$((ISSUES + 1))
    continue
  fi

  if ! grep -q '^---$' "$skill_dir/SKILL.md"; then
    echo "NO_FRONTMATTER: $skill_name/SKILL.md"
    ISSUES=$((ISSUES + 1))
  fi

  if [ -d "$skill_dir/scripts" ]; then
    for script in "$skill_dir"/scripts/*.sh; do
      [ -f "$script" ] || continue
      first_line=$(sed -n '1p' "$script")
      if [ "$first_line" != "#!/usr/bin/env sh" ]; then
        echo "BAD_SHEBANG: $script"
        ISSUES=$((ISSUES + 1))
      fi
    done
  fi
done

echo "Consistency check complete: $ISSUES issue(s) found"
exit "$ISSUES"
