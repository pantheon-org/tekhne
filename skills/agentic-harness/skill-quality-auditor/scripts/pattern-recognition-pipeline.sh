#!/usr/bin/env sh
# Comprehensive Pattern Recognition Pipeline
# Runs all advanced pattern recognition algorithms and generates unified report

set -eu

SKILLS_DIR="${1:-skills}"
OUTPUT_DIR=".context/analysis"
FINAL_REPORT="$OUTPUT_DIR/comprehensive-pattern-analysis-$(date +%Y-%m-%d).md"
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

echo "🚀 Comprehensive Pattern Recognition Pipeline Starting..."

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Initialize final report
cat > "$FINAL_REPORT" << 'EOF'
# Comprehensive Pattern Recognition Analysis

## Executive Summary

This report combines **three advanced pattern recognition algorithms** to provide comprehensive quality and duplication analysis:

1. **Enhanced Duplication Detection** - Multi-metric similarity analysis
2. **Semantic Similarity Engine** - NLP-inspired content analysis  
3. **ML Quality Prediction** - Machine learning-based quality assessment

---

EOF

echo "📊 Running Enhanced Duplication Detection..."
if [ -x "./scripts/detect-duplication-enhanced.sh" ]; then
    # Run enhanced duplication detection
    ./scripts/detect-duplication-enhanced.sh "$SKILLS_DIR" > "$TMP_DIR/duplication-report.md" 2>/dev/null || {
        echo "Warning: Enhanced duplication detection failed, using basic version"
        [ -x "./scripts/detect-duplication.sh" ] && \
            ./scripts/detect-duplication.sh "$SKILLS_DIR" > "$TMP_DIR/duplication-report.md"
    }
else
    echo "Warning: Enhanced duplication detection script not found"
    printf '## Duplication Analysis\n\n❌ Enhanced duplication script not available\n' > "$TMP_DIR/duplication-report.md"
fi

echo "🧠 Running Semantic Similarity Analysis..."
if [ -x "./scripts/semantic-analysis.sh" ]; then
    ./scripts/semantic-analysis.sh "$SKILLS_DIR" > "$TMP_DIR/semantic-report.md" 2>/dev/null || {
        echo "Warning: Semantic analysis failed"
        printf '## Semantic Analysis\n\n❌ Semantic analysis script failed\n' > "$TMP_DIR/semantic-report.md"
    }
else
    echo "Warning: Semantic analysis script not found"
    printf '## Semantic Analysis\n\n❌ Semantic analysis script not available\n' > "$TMP_DIR/semantic-report.md"
fi

echo "🤖 Running ML Quality Prediction..."
if [ -x "./scripts/ml-pattern-detection.sh" ]; then
    ./scripts/ml-pattern-detection.sh "$SKILLS_DIR" > "$TMP_DIR/ml-report.md" 2>/dev/null || {
        echo "Warning: ML pattern detection failed"
        printf '## ML Quality Analysis\n\n❌ ML pattern detection script failed\n' > "$TMP_DIR/ml-report.md"
    }
else
    echo "Warning: ML pattern detection script not found"
    printf '## ML Quality Analysis\n\n❌ ML pattern detection script not available\n' > "$TMP_DIR/ml-report.md"
fi

echo "📋 Consolidating Analysis Results..."

# Extract key metrics from each analysis
DUPLICATION_PAIRS=$(grep -o 'Found [0-9]* duplicate pairs' "$TMP_DIR/duplication-report.md" 2>/dev/null | grep -o '[0-9]*' || echo "0")
CRITICAL_PAIRS=$(grep -o '([0-9]* critical)' "$TMP_DIR/duplication-report.md" 2>/dev/null | grep -o '[0-9]*' || echo "0")
SEMANTIC_PAIRS=$(grep -o '[0-9]* semantically similar pairs' "$TMP_DIR/semantic-report.md" 2>/dev/null | grep -o '[0-9]*' || echo "0")
AVG_QUALITY=$(grep -o 'Average predicted quality.*: [0-9]*%' "$TMP_DIR/ml-report.md" 2>/dev/null | grep -o '[0-9]*' || echo "0")
HIGH_QUALITY=$(grep -o '[0-9]* skills.*≥85%' "$TMP_DIR/ml-report.md" 2>/dev/null | grep -o '^[0-9]*' || echo "0")

# Count total skills analyzed
SKILL_COUNT=$(find "$SKILLS_DIR" -name "SKILL.md" -not -path "*/.deprecated/*" | wc -l)

# Calculate composite health score
if [ "$SKILL_COUNT" -gt 0 ]; then
    # Duplication penalty: -2 points per critical pair, -1 per moderate pair
    DUPLICATION_PENALTY=$(( (CRITICAL_PAIRS * 2) + ((DUPLICATION_PAIRS - CRITICAL_PAIRS) * 1) ))
    
    # Semantic clustering bonus: +1 point for good organization (low similarity)
    SEMANTIC_BONUS=$([ "$SEMANTIC_PAIRS" -lt "$((SKILL_COUNT / 4))" ] && echo "5" || echo "0")
    
    # Quality foundation: Use ML average as base
    COMPOSITE_SCORE=$(( AVG_QUALITY - DUPLICATION_PENALTY + SEMANTIC_BONUS ))
    COMPOSITE_SCORE=$([ "$COMPOSITE_SCORE" -lt 0 ] && echo "0" || echo "$COMPOSITE_SCORE")
    COMPOSITE_SCORE=$([ "$COMPOSITE_SCORE" -gt 100 ] && echo "100" || echo "$COMPOSITE_SCORE")
else
    COMPOSITE_SCORE=0
fi

# Determine overall repository grade
if [ "$COMPOSITE_SCORE" -ge 90 ]; then
    REPO_GRADE="A+ (Exceptional)"
    GRADE_COLOR="🟢"
elif [ "$COMPOSITE_SCORE" -ge 80 ]; then
    REPO_GRADE="A (Excellent)" 
    GRADE_COLOR="🟢"
elif [ "$COMPOSITE_SCORE" -ge 70 ]; then
    REPO_GRADE="B (Good)"
    GRADE_COLOR="🟡"
elif [ "$COMPOSITE_SCORE" -ge 60 ]; then
    REPO_GRADE="C (Fair)"
    GRADE_COLOR="🟠"
else
    REPO_GRADE="D (Needs Work)"
    GRADE_COLOR="🔴"
fi

# Add consolidated summary to final report
# shellcheck disable=SC2129
cat >> "$FINAL_REPORT" << EOF
## 📈 Consolidated Metrics

### Repository Health Score: ${GRADE_COLOR} ${COMPOSITE_SCORE}% - ${REPO_GRADE}

| Metric | Value | Impact |
|--------|-------|--------|
| **Total Skills** | $SKILL_COUNT | Repository size |
| **Duplicate Pairs** | $DUPLICATION_PAIRS ($CRITICAL_PAIRS critical) | -${DUPLICATION_PENALTY} points |
| **Semantic Similarity** | $SEMANTIC_PAIRS pairs | Organization quality |  
| **Average ML Quality** | ${AVG_QUALITY}% | Foundation score |
| **High-Quality Skills** | $HIGH_QUALITY | Quality distribution |

### Key Insights

$(
    if [ "$CRITICAL_PAIRS" -gt 0 ]; then
        echo "🚨 **Critical Issue**: $CRITICAL_PAIRS skill pairs need immediate aggregation"
    elif [ "$DUPLICATION_PAIRS" -gt 5 ]; then
        echo "⚠️ **Moderate Issue**: $DUPLICATION_PAIRS duplicate pairs found - consider consolidation"
    else
        echo "✅ **Good Organization**: Low duplication detected"
    fi
)

$(
    if [ "$AVG_QUALITY" -ge 80 ]; then
        echo "🌟 **Quality Excellence**: ${AVG_QUALITY}% average quality indicates mature skill repository"
    elif [ "$AVG_QUALITY" -ge 70 ]; then
        echo "📈 **Good Quality**: ${AVG_QUALITY}% average with room for improvement"
    else
        echo "🔧 **Quality Focus Needed**: ${AVG_QUALITY}% average requires attention"
    fi
)

$(
    semantic_ratio=$((SEMANTIC_PAIRS * 100 / SKILL_COUNT))
    if [ "$semantic_ratio" -lt 20 ]; then
        echo "🎯 **Well-Organized**: Low semantic overlap indicates clear skill boundaries"
    else
        echo "🔍 **Organization Review**: High semantic overlap suggests consolidation opportunities"
    fi
)

---

EOF

# Append individual analysis reports with section headers
{
    echo "## 🔍 Enhanced Duplication Analysis"
    echo ""
    tail -n +2 "$TMP_DIR/duplication-report.md" 2>/dev/null || echo "Analysis not available"
    echo ""
    echo "---"
    echo ""
    echo "## 🧠 Semantic Similarity Analysis"
    echo ""
    tail -n +2 "$TMP_DIR/semantic-report.md" 2>/dev/null || echo "Analysis not available"
    echo ""
    echo "---"
    echo ""
    echo "## 🤖 ML Quality Prediction Analysis"  
    echo ""
    tail -n +2 "$TMP_DIR/ml-report.md" 2>/dev/null || echo "Analysis not available"
    echo ""
} >> "$FINAL_REPORT"

# Add integrated recommendations
cat >> "$FINAL_REPORT" << EOF
---

## 🎯 Integrated Action Plan

### Immediate Actions (Next 7 Days)
$(
    if [ "$CRITICAL_PAIRS" -gt 0 ]; then
        echo "1. 🚨 **Emergency Aggregation**: Address $CRITICAL_PAIRS critical duplicate pairs"
        echo "2. 🔧 **Apply Navigation Hub Pattern**: Use aggregation-pattern.md methodology"  
        echo "3. 📊 **Quality Validation**: Re-run analysis post-aggregation"
    else
        echo "1. ✅ **Maintain Quality**: Continue current practices"
        echo "2. 📈 **Optimization**: Focus on skills scoring <${AVG_QUALITY}%"
        echo "3. 🔍 **Monitoring**: Set up regular pattern analysis (monthly)"
    fi
)

### Short-term Goals (Next 30 Days)
1. 📚 **Documentation Enhancement**: Improve skills scoring <70% using ML recommendations
2. 🧠 **Semantic Organization**: Address moderate similarity pairs for better boundaries
3. 🔄 **Process Integration**: Incorporate pattern analysis into skill review workflow

### Long-term Strategy (Next Quarter)
1. 🤖 **ML Enhancement**: Implement real machine learning on historical audit data
2. 📊 **Continuous Monitoring**: Automated quality drift detection
3. 🎯 **Predictive Quality**: ML-guided skill creation and improvement

## 🛠️ Implementation Commands

\`\`\`bash
# Run complete analysis pipeline
./scripts/pattern-recognition-pipeline.sh skills/

# Address critical duplications
./scripts/generate-remediation-plan.sh --critical-only

# ML-guided improvement for low-scoring skills  
./scripts/auto-improve-quality.sh --ml-threshold=70

# Schedule regular monitoring
echo "0 9 * * Mon ./scripts/pattern-recognition-pipeline.sh skills/" >> cron.txt
\`\`\`

## 📚 References

- **Enhanced Duplication**: \`detect-duplication-enhanced.sh\` - Multi-metric similarity analysis
- **Semantic Analysis**: \`semantic-analysis.sh\` - NLP-inspired content analysis
- **ML Quality**: \`ml-pattern-detection.sh\` - Machine learning quality prediction
- **Remediation**: \`aggregation-pattern.md\` - Systematic skill consolidation methodology
- **Implementation**: \`duplication-remediation.md\` - Step-by-step improvement guide

---

*Comprehensive Pattern Recognition Pipeline v1.0*  
*Generated: $(date)*  
*Algorithms: Enhanced Duplication + Semantic Similarity + ML Quality Prediction*
EOF

echo "✅ Comprehensive Pattern Recognition Analysis Complete!"
echo ""
echo "📊 **Repository Health**: ${GRADE_COLOR} ${COMPOSITE_SCORE}% - ${REPO_GRADE}"
echo "🔍 **Duplicate Pairs**: $DUPLICATION_PAIRS ($CRITICAL_PAIRS critical)"  
echo "🧠 **Semantic Pairs**: $SEMANTIC_PAIRS"
echo "🤖 **Average Quality**: ${AVG_QUALITY}%"
echo ""
echo "📋 **Full Report**: $FINAL_REPORT"
echo "📁 **Analysis Files**: $OUTPUT_DIR/"
echo ""

# Display key recommendations
echo "🎯 **Next Steps**:"
if [ "$CRITICAL_PAIRS" -gt 0 ]; then
    echo "   1. 🚨 Address $CRITICAL_PAIRS critical duplicate pairs immediately"
    echo "   2. 🔧 Use aggregation-pattern.md for systematic consolidation"
elif [ "$DUPLICATION_PAIRS" -gt 3 ]; then
    echo "   1. ⚠️ Review $DUPLICATION_PAIRS duplicate pairs for consolidation"  
    echo "   2. 📊 Focus on skills scoring <${AVG_QUALITY}% for quality improvement"
else
    echo "   1. ✅ Repository is well-organized, continue monitoring"
    echo "   2. 📈 Optimize ${HIGH_QUALITY}/$SKILL_COUNT high-quality skills as templates"
fi

echo ""
echo "Run \`cat $FINAL_REPORT | head -50\` to see detailed analysis"