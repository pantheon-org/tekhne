#!/usr/bin/env sh
# batch-audit.sh - Audit multiple skills with failure tracking
# Usage: ./scripts/batch-audit.sh <skill1> <skill2> <skill3> ...

set -e

# Initialize counters
failed_skills=""
success_count=0
failure_count=0
total_count=$#

echo "========================================="
echo "Starting batch audit: $total_count skills"
echo "========================================="
echo ""

# Process each skill
for skill in "$@"; do
  echo "Processing: $skill"
  
  if ./scripts/audit-per-skill.sh "$skill"; then
    success_count=$((success_count + 1))
    echo "  ✅ Success ($success_count/$total_count)"
  else
    failure_count=$((failure_count + 1))
    failed_skills="$failed_skills $skill"
    echo "  ❌ Failed ($failure_count/$total_count)"
    
    # Ask whether to continue after failure
    printf "Continue with remaining skills? (y/N) "
    read -r reply
    case "$reply" in
      [Yy]*) 
        echo "  → Continuing..."
        ;;
      *)
        echo "  → Aborting batch"
        break
        ;;
    esac
  fi
  
  echo ""
done

# Summary
echo "========================================="
echo "Batch complete"
echo "========================================="
echo "Success: $success_count/$total_count"
echo "Failed: $failure_count/$total_count"

if [ -n "$failed_skills" ]; then
  echo ""
  echo "Failed skills:$failed_skills"
  echo ""
  echo "To retry failed skills:"
  echo "  ./scripts/batch-audit.sh$failed_skills"
  exit 1
else
  echo ""
  echo "All skills audited successfully! 🎉"
  exit 0
fi
