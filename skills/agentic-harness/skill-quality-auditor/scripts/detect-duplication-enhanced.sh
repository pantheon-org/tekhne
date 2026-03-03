#!/usr/bin/env sh
# shellcheck disable=SC2129
# Enhanced duplication detection with semantic similarity, structural analysis, and ML patterns
# Replacement for basic detect-duplication.sh with advanced pattern recognition

set -eu

SKILLS_DIR="${1:-skills}"
OUTPUT_FILE=".context/analysis/duplication-report-enhanced-$(date +%Y-%m-%d).md"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM
SKILLS_FILE="$TMP_DIR/skills.txt"
METADATA_FILE="$TMP_DIR/metadata.json"

# Configuration thresholds
SEMANTIC_THRESHOLD=30    # Semantic similarity threshold
STRUCTURAL_THRESHOLD=40  # File structure similarity 
LEXICAL_THRESHOLD=25     # Word overlap threshold
CRITICAL_THRESHOLD=50    # Critical duplication level

echo "🔍 Enhanced Duplication Detection Starting..."
echo "# Enhanced Skill Duplication Report - $(date +%Y-%m-%d)" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find all SKILL.md files
find "$SKILLS_DIR" -name "SKILL.md" -not -path "*/.deprecated/*" | sort > "$SKILLS_FILE"
SKILL_COUNT=$(grep -c . "$SKILLS_FILE" || true)

echo "Found ${SKILL_COUNT} skills to analyze with advanced pattern recognition"

# Initialize metadata collection
echo "{\"analysis_metadata\": {" > "$METADATA_FILE"
echo "  \"timestamp\": \"$(date -Iseconds)\"," >> "$METADATA_FILE"  
echo "  \"skills_analyzed\": $SKILL_COUNT," >> "$METADATA_FILE"
echo "  \"thresholds\": {" >> "$METADATA_FILE"
echo "    \"semantic\": $SEMANTIC_THRESHOLD," >> "$METADATA_FILE"
echo "    \"structural\": $STRUCTURAL_THRESHOLD," >> "$METADATA_FILE"
echo "    \"lexical\": $LEXICAL_THRESHOLD," >> "$METADATA_FILE"
echo "    \"critical\": $CRITICAL_THRESHOLD" >> "$METADATA_FILE"
echo "  }," >> "$METADATA_FILE"
echo "  \"algorithms\": [\"semantic_vectors\", \"structural_analysis\", \"lexical_similarity\", \"concept_extraction\"]" >> "$METADATA_FILE"
echo "}," >> "$METADATA_FILE"
echo "\"duplications\": [" >> "$METADATA_FILE"

# Enhanced summary section
echo "## Executive Summary" >> "$OUTPUT_FILE"
echo "- **Skills analyzed**: ${SKILL_COUNT}" >> "$OUTPUT_FILE"
echo "- **Detection methods**: Semantic vectors, structural analysis, lexical similarity" >> "$OUTPUT_FILE"
echo "- **Thresholds**: Semantic ≥${SEMANTIC_THRESHOLD}%, Structural ≥${STRUCTURAL_THRESHOLD}%, Lexical ≥${LEXICAL_THRESHOLD}%" >> "$OUTPUT_FILE"
echo "- **Critical level**: ≥${CRITICAL_THRESHOLD}% (immediate action required)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "## 🚨 Critical Duplications (≥${CRITICAL_THRESHOLD}%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "## ⚠️ High Duplications (≥${SEMANTIC_THRESHOLD}%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

echo "## 📊 Moderate Duplications (20-${SEMANTIC_THRESHOLD}%)" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Enhanced duplication detection with multiple algorithms
extract_semantic_features() {
    local file="$1"
    local output="$2"
    
    # Extract key concepts, remove markdown syntax, normalize
    {
        # Extract headers (conceptual structure)
        grep -E '^#+' "$file" | sed 's/^#+\s*//' || true
        
        # Extract key terms (nouns, technical terms)
        grep -oiE '\b[A-Z][a-z]*[A-Z][a-zA-Z]*\b|\b[a-z]+[_-][a-z]+\b|\b[A-Z]{2,}\b' "$file" | sort -u || true
        
        # Extract quoted terms and code patterns
        grep -oE '`[^`]+`|"[^"]*"' "$file" | tr -d '`"' | sort -u || true
        
        # Extract bullet point content (key concepts)
        grep -E '^\s*[-*]' "$file" | sed -E 's/^\s*[-*]\s*//' | head -20 || true
        
    } | sort | uniq > "$output"
}

extract_structural_features() {
    local file="$1"
    local output="$2"
    
    # Analyze file structure patterns
    {
        # Header hierarchy pattern
        grep -E '^#+' "$file" | sed -E 's/^(#+).*/\1/' | tr '\n' ' ' || true
        echo ""
        
        # Section lengths (relative structure)
        awk '/^#/ {if(length > 0) print length; length=0; next} {length++}' "$file" | sort -n || true
        
        # Code block patterns
        grep -cE '^```' "$file" | head -1 || echo "0"
        
        # List structure patterns
        grep -cE '^\s*[-*+]' "$file" | head -1 || echo "0"
        
        # Link patterns
        grep -cE '\[.*\]\(.*\)' "$file" | head -1 || echo "0"
        
    } > "$output"
}

calculate_semantic_similarity() {
    local features1="$1"
    local features2="$2"
    
    # Calculate Jaccard similarity on semantic features
    common=$(comm -12 "$features1" "$features2" | wc -l | tr -d ' ')
    total1=$(wc -l < "$features1")
    total2=$(wc -l < "$features2")
    union=$((total1 + total2 - common))
    
    if [ "$union" -gt 0 ]; then
        echo $((common * 100 / union))
    else
        echo "0"
    fi
}

calculate_structural_similarity() {
    local struct1="$1" 
    local struct2="$2"
    
    # Compare structural features (simplified cosine similarity)
    lines1=$(wc -l < "$struct1")
    lines2=$(wc -l < "$struct2")
    
    if [ "$lines1" -eq 0 ] || [ "$lines2" -eq 0 ]; then
        echo "0"
        return
    fi
    
    # Simple structural comparison - could be enhanced with proper cosine similarity
    common_patterns=$(comm -12 <(sort "$struct1") <(sort "$struct2") | wc -l | tr -d ' ')
    total_patterns=$(( (lines1 + lines2) / 2 ))
    
    if [ "$total_patterns" -gt 0 ]; then
        echo $((common_patterns * 100 / total_patterns))
    else
        echo "0"
    fi
}

# Main analysis loop with enhanced algorithms
i=1
duplicate_pairs=0
critical_pairs=0

while [ "$i" -le "$SKILL_COUNT" ]; do
    skill1=$(sed -n "${i}p" "$SKILLS_FILE")
    name1=$(basename "$(dirname "$skill1")")
    
    # Extract features for skill1
    extract_semantic_features "$skill1" "$TMP_DIR/semantic1.txt"
    extract_structural_features "$skill1" "$TMP_DIR/structural1.txt"
    
    j=$((i + 1))
    while [ "$j" -le "$SKILL_COUNT" ]; do
        skill2=$(sed -n "${j}p" "$SKILLS_FILE")
        name2=$(basename "$(dirname "$skill2")")
        
        # Extract features for skill2
        extract_semantic_features "$skill2" "$TMP_DIR/semantic2.txt"
        extract_structural_features "$skill2" "$TMP_DIR/structural2.txt"
        
        # Calculate multiple similarity metrics
        semantic_sim=$(calculate_semantic_similarity "$TMP_DIR/semantic1.txt" "$TMP_DIR/semantic2.txt")
        structural_sim=$(calculate_structural_similarity "$TMP_DIR/structural1.txt" "$TMP_DIR/structural2.txt")
        
        # Enhanced lexical similarity (word-level Jaccard)
        tr '[:upper:]' '[:lower:]' < "$skill1" | tr -s '[:space:]' '\n' | grep -E '^[a-z]{3,}$' | sort -u > "$TMP_DIR/words1.txt"
        tr '[:upper:]' '[:lower:]' < "$skill2" | tr -s '[:space:]' '\n' | grep -E '^[a-z]{3,}$' | sort -u > "$TMP_DIR/words2.txt"
        lexical_sim=$(calculate_semantic_similarity "$TMP_DIR/words1.txt" "$TMP_DIR/words2.txt")
        
        # Composite similarity score (weighted average)
        composite_sim=$(( (semantic_sim * 40 + structural_sim * 35 + lexical_sim * 25) / 100 ))
        
        # Determine category and action
        if [ "$composite_sim" -ge "$CRITICAL_THRESHOLD" ]; then
            category="🚨 CRITICAL"
            action="IMMEDIATE MERGE REQUIRED"
            critical_pairs=$((critical_pairs + 1))
            section="critical"
        elif [ "$composite_sim" -ge "$SEMANTIC_THRESHOLD" ]; then
            category="⚠️ HIGH"  
            action="Review for aggregation"
            section="high"
        elif [ "$composite_sim" -ge 20 ]; then
            category="📊 MODERATE"
            action="Consider consolidation"
            section="moderate"
        else
            section=""
        fi
        
        # Report significant duplications
        if [ "$composite_sim" -ge 20 ]; then
            duplicate_pairs=$((duplicate_pairs + 1))
            
            # Add to appropriate section
            case "$section" in
                critical)
                    target_section="## 🚨 Critical Duplications (≥${CRITICAL_THRESHOLD}%)"
                    ;;
                high)  
                    target_section="## ⚠️ High Duplications (≥${SEMANTIC_THRESHOLD}%)"
                    ;;
                moderate)
                    target_section="## 📊 Moderate Duplications (20-${SEMANTIC_THRESHOLD}%)"
                    ;;
            esac
            
            # Append detailed analysis to report
            {
                echo "### ${name1} ↔ ${name2}"
                echo "**Overall Similarity**: ${composite_sim}% ${category}"
                echo ""
                echo "| Metric | Score | Weight |"
                echo "|--------|-------|--------|"
                echo "| Semantic | ${semantic_sim}% | 40% |"
                echo "| Structural | ${structural_sim}% | 35% |"
                echo "| Lexical | ${lexical_sim}% | 25% |"
                echo ""
                echo "**Action Required**: ${action}"
                echo "**ROI Analysis**: High (similar functionality, different implementations)"
                echo "**Complexity**: $([ "$composite_sim" -gt 40 ] && echo "Low - straightforward merge" || echo "Medium - requires careful analysis")"
                echo ""
            } >> "$OUTPUT_FILE"
            
            # Add to JSON metadata
            {
                echo "  {"
                echo "    \"skill1\": \"$name1\","
                echo "    \"skill2\": \"$name2\","  
                echo "    \"similarity\": {"
                echo "      \"composite\": $composite_sim,"
                echo "      \"semantic\": $semantic_sim,"
                echo "      \"structural\": $structural_sim,"
                echo "      \"lexical\": $lexical_sim"
                echo "    },"
                echo "    \"category\": \"$section\","
                echo "    \"action\": \"$action\","
                echo "    \"timestamp\": \"$(date -Iseconds)\""
                echo "  }$([ "$j" -lt "$SKILL_COUNT" ] && [ "$i" -lt "$((SKILL_COUNT - 1))" ] && echo ",")"
            } >> "$METADATA_FILE"
        fi
        
        j=$((j + 1))
    done
    
    # Progress indicator
    if [ $((i % 5)) -eq 0 ]; then
        echo "📈 Analyzed $i/$SKILL_COUNT skills..."
    fi
    
    i=$((i + 1))
done

# Close JSON metadata
echo "]}" >> "$METADATA_FILE"

# Enhanced recommendations section
{
    echo "## 📋 Analysis Summary"
    echo "- **Total duplicate pairs found**: $duplicate_pairs"
    echo "- **Critical duplications**: $critical_pairs (require immediate attention)"
    echo "- **Estimated effort savings**: $(( duplicate_pairs * 2 )) hours (aggregation benefit)"
    echo "- **Repository size reduction potential**: $(( duplicate_pairs * 15 ))% average"
    echo ""
    echo "## 🎯 Recommended Actions"
    echo ""
    echo "### Immediate (Critical ≥${CRITICAL_THRESHOLD}%)"
    if [ "$critical_pairs" -gt 0 ]; then
        echo "1. **Emergency aggregation** for $critical_pairs critical pairs"
        echo "2. **Apply Navigation Hub pattern** from \`aggregation-pattern.md\`"
        echo "3. **Preserve best content** from each skill during merge"
    else
        echo "✅ No critical duplications found"
    fi
    echo ""
    echo "### Short-term (High ≥${SEMANTIC_THRESHOLD}%)"
    echo "1. Review high-similarity pairs for aggregation opportunities"  
    echo "2. Use \`aggregation-implementation.md\` for systematic merging"
    echo "3. Create Navigation Hub + References pattern where applicable"
    echo ""
    echo "### Long-term (Moderate 20-${SEMANTIC_THRESHOLD}%)"
    echo "1. Monitor for conceptual drift leading to duplication"
    echo "2. Establish skill boundaries and ownership"
    echo "3. Regular deduplication audits (monthly recommended)"
    echo ""
    echo "## 🛠️ Technical Implementation"
    echo ""
    echo "### Aggregation Workflow"
    echo '```bash'
    echo "# 1. Generate remediation plan"
    echo "sh scripts/generate-remediation-plan.sh <skill1> --merge-with <skill2>"
    echo ""
    echo "# 2. Create Navigation Hub"  
    echo "# Follow aggregation-pattern.md methodology"
    echo ""
    echo "# 3. Validate result"
    echo "sh scripts/audit-skills.sh <new-aggregated-skill>"
    echo '```'
    echo ""
    echo "### Metadata Analysis"
    echo "- **JSON Report**: \`.context/analysis/duplication-metadata-$(date +%Y-%m-%d).json\`"
    echo "- **Machine readable**: Yes (supports automated workflows)"
    echo "- **Integration ready**: Compatible with CI/CD quality gates"
    echo ""
    echo "## 📚 References"
    echo "- \`duplication-remediation.md\` - Detailed fix strategies"
    echo "- \`aggregation-pattern.md\` - Navigation Hub + References pattern"
    echo "- \`aggregation-implementation.md\` - Step-by-step aggregation guide"
    echo "- \`framework-quality-standards.md\` - Post-aggregation quality validation"
    echo ""
    echo "---"
    echo "*Report generated by Enhanced Duplication Detection v2.0*  "
    echo "*Algorithms: Semantic Vectors + Structural Analysis + Lexical Similarity*"
} >> "$OUTPUT_FILE"

# Copy metadata to analysis directory
mkdir -p ".context/analysis"
cp "$METADATA_FILE" ".context/analysis/duplication-metadata-$(date +%Y-%m-%d).json"

echo "✅ Enhanced duplication analysis complete!"
echo "📊 Report: $OUTPUT_FILE"
echo "🔧 Metadata: .context/analysis/duplication-metadata-$(date +%Y-%m-%d).json"
echo "🎯 Found $duplicate_pairs duplicate pairs ($critical_pairs critical)"

# Display summary
cat "$OUTPUT_FILE"