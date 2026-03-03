#!/usr/bin/env sh
# Semantic Similarity Analyzer with Vector-based Pattern Recognition
# Advanced NLP-inspired techniques for skill content analysis

set -eu

SKILLS_DIR="${1:-skills}"
OUTPUT_FILE=".context/analysis/semantic-analysis-$(date +%Y-%m-%d).md"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

echo "🧠 Advanced Semantic Analysis Starting..."

# Create analysis directory
mkdir -p ".context/analysis"

# Configuration
VECTOR_DIMENSIONS=100        # Simulated vector dimensions
MIN_SEMANTIC_SCORE=25       # Minimum score for reporting
TOPIC_THRESHOLD=0.6         # Topic clustering threshold

echo "# Semantic Similarity Analysis - $(date +%Y-%m-%d)" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find all SKILL.md files
find "$SKILLS_DIR" -name "SKILL.md" -not -path "*/.deprecated/*" | sort > "$TMP_DIR/skills.txt"
SKILL_COUNT=$(wc -l < "$TMP_DIR/skills.txt")

echo "📊 Analyzing $SKILL_COUNT skills with semantic vectors..."

echo "## Analysis Summary" >> "$OUTPUT_FILE"
echo "- **Skills analyzed**: $SKILL_COUNT" >> "$OUTPUT_FILE"
echo "- **Vector dimensions**: $VECTOR_DIMENSIONS (simulated)" >> "$OUTPUT_FILE"
echo "- **Minimum threshold**: $MIN_SEMANTIC_SCORE%" >> "$OUTPUT_FILE"
echo "- **Algorithms**: TF-IDF simulation, concept extraction, topic modeling" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Advanced semantic feature extraction
extract_semantic_vectors() {
    local skill_file="$1"
    local skill_name="$2"
    local output_dir="$3"
    
    # Extract different semantic layers
    
    # 1. Concept extraction (technical terms, patterns)
    {
        # Technical patterns and acronyms
        grep -oiE '\b[A-Z]{2,}[a-z]*\b|\b[A-Z][a-z]*[A-Z][a-zA-Z]*\b' "$skill_file"
        
        # Code/tool references  
        grep -oE '`[^`]+`' "$skill_file" | tr -d '`'
        
        # Framework and tool names
        grep -oiE '\b(docker|kubernetes|aws|terraform|ansible|jenkins|git|npm|yarn|pnpm|webpack|vite|rollup)\b' "$skill_file"
        
    } | tr '[:upper:]' '[:lower:]' | sort | uniq -c | sort -rn > "$output_dir/concepts.txt"
    
    # 2. Domain vocabulary extraction
    {
        # Remove markdown syntax and extract meaningful terms
        sed -E 's/\*\*//g; s/\*//g; s/`//g; s/\[([^\]]*)\]\([^)]*\)/\1/g' "$skill_file" |
        tr '[:upper:]' '[:lower:]' |
        grep -oE '\b[a-z]{4,}\b' |
        grep -vE '^(this|that|will|with|have|been|from|they|were|when|what|where|your|then|more|some|like|into|time|work)$' |
        sort | uniq -c | sort -rn | head -50
        
    } > "$output_dir/vocabulary.txt"
    
    # 3. Structural patterns (headers, lists, code blocks)
    {
        echo "HEADERS:"
        grep -E '^#+' "$skill_file" | sed 's/^#+\s*//' | head -10
        echo "LISTS:"
        grep -E '^\s*[-*+]' "$skill_file" | sed -E 's/^\s*[-*+]\s*//' | head -10
        echo "CODE_BLOCKS:"
        grep -c '^```' "$skill_file" || echo "0"
        
    } > "$output_dir/structure.txt"
    
    # 4. Intent/purpose classification
    {
        # Look for action words and intent signals
        grep -oiE '\b(create|build|implement|configure|setup|install|deploy|test|validate|analyze|optimize|debug)\b' "$skill_file" |
        tr '[:upper:]' '[:lower:]' | sort | uniq -c | sort -rn
        
    } > "$output_dir/intents.txt"
}

# Topic modeling simulation  
extract_topics() {
    local skill_file="$1"
    local output="$2"
    
    # Simplified topic extraction based on content patterns
    {
        # Infrastructure/DevOps patterns
        if grep -qi 'deploy\|infrastructure\|cloud\|server\|docker\|kubernetes' "$skill_file"; then
            echo "infrastructure:$(grep -ci 'deploy\|infrastructure\|cloud\|server\|docker\|kubernetes' "$skill_file")"
        fi
        
        # Development/Coding patterns  
        if grep -qi 'code\|function\|class\|api\|library\|framework' "$skill_file"; then
            echo "development:$(grep -ci 'code\|function\|class\|api\|library\|framework' "$skill_file")"
        fi
        
        # Testing patterns
        if grep -qi 'test\|spec\|mock\|assert\|coverage' "$skill_file"; then
            echo "testing:$(grep -ci 'test\|spec\|mock\|assert\|coverage' "$skill_file")"
        fi
        
        # Documentation patterns
        if grep -qi 'document\|readme\|guide\|tutorial\|example' "$skill_file"; then
            echo "documentation:$(grep -ci 'document\|readme\|guide\|tutorial\|example' "$skill_file")"
        fi
        
        # Quality/Analysis patterns
        if grep -qi 'quality\|lint\|format\|audit\|analyze\|review' "$skill_file"; then
            echo "quality:$(grep -ci 'quality\|lint\|format\|audit\|analyze\|review' "$skill_file")"
        fi
        
        # Security patterns
        if grep -qi 'security\|auth\|token\|encrypt\|secure\|vulnerability' "$skill_file"; then
            echo "security:$(grep -ci 'security\|auth\|token\|encrypt\|secure\|vulnerability' "$skill_file")"
        fi
        
    } | sort -t: -k2 -rn > "$output"
}

# Calculate semantic similarity using multiple metrics
calculate_advanced_similarity() {
    local vectors1="$1"
    local vectors2="$2"
    local name1="$3"
    local name2="$4"
    local output_file="$5"
    
    # 1. Concept overlap (weighted by frequency)
    concept_sim=$(awk '
        NR==FNR { a[$2] = $1; next }
        $2 in a { 
            common += ($1 < a[$2] ? $1 : a[$2])
            total1 += a[$2]
            total2 += $1
        }
        { total2 += $1 }
        END { 
            for (word in a) total1 += a[word]
            union = total1 + total2 - common
            print (union > 0 ? int(common * 100 / union) : 0)
        }' "$vectors1/concepts.txt" "$vectors2/concepts.txt" 2>/dev/null || echo "0")
    
    # 2. Vocabulary overlap (TF-IDF simulation)
    vocab_sim=$(awk '
        NR==FNR { a[$2] = $1; next }
        $2 in a {
            # Weighted overlap using frequency scores
            score = ($1 * a[$2]) / (($1 + a[$2]) / 2)
            total_score += score
            terms++
        }
        END { print (terms > 0 ? int(total_score / terms) : 0) }
    ' "$vectors1/vocabulary.txt" "$vectors2/vocabulary.txt" 2>/dev/null || echo "0")
    
    # 3. Topic similarity
    topic_sim=$(
        topics1=$(mktemp)
        topics2=$(mktemp)
        trap 'rm -f "$topics1" "$topics2"' EXIT
        
        awk -F: '{print $1}' "$vectors1/../topics.txt" 2>/dev/null | sort > "$topics1"
        awk -F: '{print $1}' "$vectors2/../topics.txt" 2>/dev/null | sort > "$topics2"
        
        common=$(comm -12 "$topics1" "$topics2" | wc -l)
        total1=$(wc -l < "$topics1")
        total2=$(wc -l < "$topics2")
        union=$((total1 + total2 - common))
        
        echo $([ "$union" -gt 0 ] && echo $((common * 100 / union)) || echo "0")
    )
    
    # 4. Intent similarity  
    intent_sim=$(
        intents1=$(awk '{print $2}' "$vectors1/intents.txt" 2>/dev/null | sort)
        intents2=$(awk '{print $2}' "$vectors2/intents.txt" 2>/dev/null | sort)
        
        common=$(echo -e "$intents1\n$intents2" | sort | uniq -d | wc -l)
        total=$(echo -e "$intents1\n$intents2" | sort | uniq | wc -l)
        
        echo $([ "$total" -gt 0 ] && echo $((common * 100 / total)) || echo "0")
    )
    
    # Composite semantic score (weighted average)
    composite=$(( (concept_sim * 30 + vocab_sim * 25 + topic_sim * 25 + intent_sim * 20) / 100 ))
    
    # Return detailed scores
    cat << EOF >> "$output_file"
### Semantic Analysis: $name1 ↔ $name2

**Composite Similarity**: ${composite}%

| Component | Score | Weight | Description |
|-----------|-------|--------|-------------|
| Concept Overlap | ${concept_sim}% | 30% | Technical terms, patterns |
| Vocabulary | ${vocab_sim}% | 25% | Domain-specific language |
| Topics | ${topic_sim}% | 25% | Subject matter alignment |  
| Intent | ${intent_sim}% | 20% | Purpose and action similarity |

**Semantic Category**: $(
    if [ "$composite" -ge 60 ]; then echo "🔴 High Semantic Overlap"
    elif [ "$composite" -ge 35 ]; then echo "🟡 Moderate Semantic Similarity"  
    elif [ "$composite" -ge 20 ]; then echo "🟢 Low Semantic Overlap"
    else echo "⚪ Minimal Semantic Connection"
    fi
)

**Recommended Action**: $(
    if [ "$composite" -ge 50 ]; then echo "Consider skill aggregation"
    elif [ "$composite" -ge 30 ]; then echo "Review for conceptual boundaries"
    else echo "Skills are semantically distinct"
    fi
)

---

EOF
    
    echo "$composite"  # Return composite score for caller
}

# Main analysis loop
echo "## 🔍 Detailed Semantic Analysis" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

skill_pairs=0
high_similarity_pairs=0
total_comparisons=0

i=1
while [ "$i" -le "$SKILL_COUNT" ]; do
    skill1=$(sed -n "${i}p" "$TMP_DIR/skills.txt")
    name1=$(basename "$(dirname "$skill1")")
    
    # Create vector directory for skill1
    mkdir -p "$TMP_DIR/vectors/$name1"
    
    echo "📊 Extracting semantic vectors for $name1..."
    extract_semantic_vectors "$skill1" "$name1" "$TMP_DIR/vectors/$name1"
    extract_topics "$skill1" "$TMP_DIR/vectors/$name1/../topics.txt"
    
    j=$((i + 1))
    while [ "$j" -le "$SKILL_COUNT" ]; do
        skill2=$(sed -n "${j}p" "$TMP_DIR/skills.txt")
        name2=$(basename "$(dirname "$skill2")")
        
        # Create vector directory for skill2  
        mkdir -p "$TMP_DIR/vectors/$name2"
        
        # Extract vectors for skill2 (if not already done)
        if [ ! -f "$TMP_DIR/vectors/$name2/concepts.txt" ]; then
            extract_semantic_vectors "$skill2" "$name2" "$TMP_DIR/vectors/$name2"  
            extract_topics "$skill2" "$TMP_DIR/vectors/$name2/../topics.txt"
        fi
        
        # Calculate advanced semantic similarity
        similarity=$(calculate_advanced_similarity \
            "$TMP_DIR/vectors/$name1" \
            "$TMP_DIR/vectors/$name2" \
            "$name1" "$name2" "$OUTPUT_FILE")
            
        total_comparisons=$((total_comparisons + 1))
        
        if [ "$similarity" -ge "$MIN_SEMANTIC_SCORE" ]; then
            skill_pairs=$((skill_pairs + 1))
            
            if [ "$similarity" -ge 45 ]; then
                high_similarity_pairs=$((high_similarity_pairs + 1))
            fi
        fi
        
        j=$((j + 1))
    done
    
    # Progress indicator
    if [ $((i % 3)) -eq 0 ]; then
        echo "🧠 Semantic analysis: $i/$SKILL_COUNT skills processed..."
    fi
    
    i=$((i + 1))
done

# Final summary and recommendations
{
    echo "## 📈 Semantic Analysis Results"
    echo ""
    echo "### Key Metrics"
    echo "- **Total comparisons**: $total_comparisons" 
    echo "- **Semantically similar pairs**: $skill_pairs (≥${MIN_SEMANTIC_SCORE}%)"
    echo "- **High similarity pairs**: $high_similarity_pairs (≥45%)"
    echo "- **Unique semantic clusters**: ~$((SKILL_COUNT - skill_pairs / 2))"
    echo ""
    echo "### Semantic Quality Insights"
    similarity_ratio=$((skill_pairs * 100 / total_comparisons))
    echo "- **Repository diversity**: $([ "$similarity_ratio" -lt 15 ] && echo "High (low semantic overlap)" || echo "Moderate (some concept clustering)")"
    echo "- **Conceptual boundaries**: $([ "$high_similarity_pairs" -lt 3 ] && echo "Well-defined" || echo "Needs improvement")"  
    echo "- **Knowledge organization**: $([ "$skill_pairs" -lt 5 ] && echo "Excellent separation" || echo "Consider aggregation")"
    echo ""
    echo "### 🎯 Strategic Recommendations"
    echo ""
    if [ "$high_similarity_pairs" -gt 2 ]; then
        echo "**Immediate Actions:**"
        echo "1. 🔥 Aggregate high-similarity skills (≥45%) to reduce redundancy"  
        echo "2. 🔧 Apply Navigation Hub pattern from aggregation-pattern.md"
        echo "3. 🎯 Establish clearer skill boundaries and ownership"
        echo ""
    fi
    echo "**Quality Improvements:**"
    echo "1. 📊 Monitor semantic drift in future skill additions"
    echo "2. 🧠 Use semantic analysis for skill placement decisions" 
    echo "3. 🔍 Regular semantic audits (quarterly recommended)"
    echo ""
    echo "### 🛠️ Implementation Guide"
    echo ""
    echo '```bash'
    echo "# Run semantic analysis"
    echo "./scripts/semantic-analysis.sh skills/"
    echo ""
    echo "# Generate aggregation plan for high-similarity pairs"
    echo "./scripts/generate-remediation-plan.sh --semantic-threshold=45"
    echo ""
    echo "# Validate post-aggregation semantic quality"
    echo "./scripts/audit-skills.sh --semantic-check"
    echo '```'
    echo ""
    echo "---"
    echo "*Advanced Semantic Analysis v1.0*  "
    echo "*Algorithms: Multi-layer concept extraction, TF-IDF simulation, topic modeling*"
    echo "*Next evolution: Actual ML embeddings, transformer-based similarity*"
} >> "$OUTPUT_FILE"

echo "✅ Advanced semantic analysis complete!"
echo "📊 Report: $OUTPUT_FILE" 
echo "🧠 Found $skill_pairs semantically similar pairs out of $total_comparisons comparisons"
echo "🔥 High similarity pairs: $high_similarity_pairs"

# Display key results
tail -30 "$OUTPUT_FILE"