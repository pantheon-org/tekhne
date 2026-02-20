#!/bin/bash
# compare-evidence.sh - Compare baseline and changed evidence
#
# Usage: ./compare-evidence.sh <baseline-dir> <changed-dir> <output-report>
#
# Example:
#   ./compare-evidence.sh ./docs/change-fix/baseline ./docs/change-fix/changed ./docs/change-fix/comparison.md

set -e

BASELINE_DIR="${1:-./docs/change-fix/baseline}"
CHANGED_DIR="${2:-./docs/change-fix/changed}"
OUTPUT_REPORT="${3:-./docs/change-fix/comparison-report.md}"

if [ ! -d "$BASELINE_DIR" ]; then
  echo "Error: Baseline directory not found: $BASELINE_DIR"
  exit 1
fi

if [ ! -d "$CHANGED_DIR" ]; then
  echo "Error: Changed directory not found: $CHANGED_DIR"
  exit 1
fi

echo "=== Evidence Comparison ==="
echo "Baseline: $BASELINE_DIR"
echo "Changed: $CHANGED_DIR"
echo "Output: $OUTPUT_REPORT"
echo ""

# Create output directory
mkdir -p "$(dirname "$OUTPUT_REPORT")"

# Start report
cat > "$OUTPUT_REPORT" << 'EOF'
# Evidence Comparison Report

**Generated**: $(date '+%Y-%m-%d %H:%M:%S')

---

## Summary

EOF

# Compare file lists
echo "## File Comparison" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"
echo "### Baseline Files" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"
echo "\`\`\`" >> "$OUTPUT_REPORT"
ls -lh "$BASELINE_DIR" | tail -n +2 | awk '{print $9 " (" $5 ")"}' >> "$OUTPUT_REPORT"
echo "\`\`\`" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"

echo "### Changed Files" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"
echo "\`\`\`" >> "$OUTPUT_REPORT"
ls -lh "$CHANGED_DIR" | tail -n +2 | awk '{print $9 " (" $5 ")"}' >> "$OUTPUT_REPORT"
echo "\`\`\`" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"

# Compare JSON files if they exist
echo "## Data Comparison" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"

for json_file in "$BASELINE_DIR"/*.json; do
  if [ -f "$json_file" ]; then
    filename=$(basename "$json_file")
    changed_file="$CHANGED_DIR/$filename"
    
    if [ -f "$changed_file" ]; then
      echo "### $filename" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      echo "**Baseline**:" >> "$OUTPUT_REPORT"
      echo "\`\`\`json" >> "$OUTPUT_REPORT"
      cat "$json_file" | python3 -m json.tool 2>/dev/null || cat "$json_file" >> "$OUTPUT_REPORT"
      echo "\`\`\`" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      
      echo "**Changed**:" >> "$OUTPUT_REPORT"
      echo "\`\`\`json" >> "$OUTPUT_REPORT"
      cat "$changed_file" | python3 -m json.tool 2>/dev/null || cat "$changed_file" >> "$OUTPUT_REPORT"
      echo "\`\`\`" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      
      # Attempt diff
      if command -v jq &> /dev/null; then
        baseline_sorted=$(cat "$json_file" | jq -S 2>/dev/null)
        changed_sorted=$(cat "$changed_file" | jq -S 2>/dev/null)
        
        if [ "$baseline_sorted" != "$changed_sorted" ]; then
          echo "**Differences**:" >> "$OUTPUT_REPORT"
          echo "\`\`\`diff" >> "$OUTPUT_REPORT"
          diff <(echo "$baseline_sorted") <(echo "$changed_sorted") || true >> "$OUTPUT_REPORT"
          echo "\`\`\`" >> "$OUTPUT_REPORT"
        else
          echo "**Result**: ✅ Identical" >> "$OUTPUT_REPORT"
        fi
      fi
      echo "" >> "$OUTPUT_REPORT"
    else
      echo "### $filename" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      echo "⚠️ **Warning**: File exists in baseline but not in changed directory" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
    fi
  fi
done

# Screenshot comparison section
echo "## Screenshot Comparison" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"

for screenshot in "$BASELINE_DIR"/*.png; do
  if [ -f "$screenshot" ]; then
    filename=$(basename "$screenshot")
    changed_screenshot="$CHANGED_DIR/$filename"
    
    if [ -f "$changed_screenshot" ]; then
      echo "### $filename" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      echo "| Baseline | Changed |" >> "$OUTPUT_REPORT"
      echo "|----------|---------|" >> "$OUTPUT_REPORT"
      echo "| ![Baseline]($BASELINE_DIR/$filename) | ![Changed]($CHANGED_DIR/$filename) |" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      
      # If ImageMagick is available, compute diff
      if command -v compare &> /dev/null; then
        diff_image="$CHANGED_DIR/diff-$filename"
        compare -metric AE -fuzz 5% "$screenshot" "$changed_screenshot" "$diff_image" 2>/dev/null || true
        
        if [ -f "$diff_image" ]; then
          echo "**Visual Diff**:" >> "$OUTPUT_REPORT"
          echo "" >> "$OUTPUT_REPORT"
          echo "![Diff]($diff_image)" >> "$OUTPUT_REPORT"
          echo "" >> "$OUTPUT_REPORT"
        fi
      else
        echo "*Note: Install ImageMagick for automatic visual diff generation*" >> "$OUTPUT_REPORT"
        echo "" >> "$OUTPUT_REPORT"
      fi
    else
      echo "### $filename" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
      echo "⚠️ **Warning**: Screenshot exists in baseline but not in changed directory" >> "$OUTPUT_REPORT"
      echo "" >> "$OUTPUT_REPORT"
    fi
  fi
done

# Completion message
echo "---" >> "$OUTPUT_REPORT"
echo "" >> "$OUTPUT_REPORT"
echo "**Report generated by**: ui-debug-workflow skill" >> "$OUTPUT_REPORT"
echo "**Baseline directory**: \`$BASELINE_DIR\`" >> "$OUTPUT_REPORT"
echo "**Changed directory**: \`$CHANGED_DIR\`" >> "$OUTPUT_REPORT"

echo ""
echo "✅ Comparison report generated: $OUTPUT_REPORT"
echo ""
echo "Next steps:"
echo "  1. Review the comparison report"
echo "  2. Analyze differences"
echo "  3. Update the comprehensive debug report with findings"
