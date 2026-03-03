#!/usr/bin/env sh
# check-publication-readiness.sh
# Validates a Tessl skill meets all requirements for public registry publishing

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Usage function
usage() {
    echo "Usage: $0 <skill-path>"
    echo "Example: $0 skills/infrastructure/terraform-validator"
    exit 1
}

# Validation result tracking
CHECKS_PASSED=0
CHECKS_FAILED=0
WARNINGS=0

# Helper functions
log_pass() {
    echo "${GREEN}✓${NC} $1"
    CHECKS_PASSED=$((CHECKS_PASSED + 1))
}

log_fail() {
    echo "${RED}✗${NC} $1"
    CHECKS_FAILED=$((CHECKS_FAILED + 1))
}

log_warn() {
    echo "${YELLOW}⚠${NC} $1"
    WARNINGS=$((WARNINGS + 1))
}

log_info() {
    echo "${BLUE}ℹ${NC} $1"
}

# Check if skill path provided
if [ $# -ne 1 ]; then
    usage
fi

SKILL_PATH="$1"

# Validate skill path exists
if [ ! -d "$SKILL_PATH" ]; then
    echo "${RED}Error: Skill directory not found: $SKILL_PATH${NC}"
    exit 1
fi

# Extract skill name from path
SKILL_NAME=$(basename "$SKILL_PATH")
SKILL_DOMAIN=$(basename "$(dirname "$SKILL_PATH")")

echo "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo "${BLUE}║  Tessl Public Publication Readiness Check                 ║${NC}"
echo "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Skill: ${BLUE}${SKILL_DOMAIN}/${SKILL_NAME}${NC}"
echo "Path:  ${SKILL_PATH}"
echo ""

# Check 1: SKILL.md exists
echo "${BLUE}[1/8]${NC} Checking SKILL.md existence..."
if [ -f "$SKILL_PATH/SKILL.md" ]; then
    log_pass "SKILL.md exists"
else
    log_fail "SKILL.md not found"
fi

# Check 2: tile.json comprehensive validation
echo ""
echo "${BLUE}[2/8]${NC} Checking tile.json (comprehensive validation)..."
if [ -f "$SKILL_PATH/tile.json" ]; then
    log_pass "tile.json exists"
    
    if command -v jq >/dev/null 2>&1; then
        # Validate JSON syntax
        if jq empty "$SKILL_PATH/tile.json" 2>/dev/null; then
            log_pass "tile.json has valid JSON syntax"
        else
            log_fail "tile.json has invalid JSON syntax"
            echo ""
            echo "${BLUE}[2/8]${NC} Skipping further tile.json checks due to syntax errors"
            echo ""
            echo "${BLUE}[3/8]${NC} Checking evaluation scenarios..."
        fi
        
        # REQUIRED FIELD: private
        PRIVATE_VALUE=$(jq -r '.private // "null"' "$SKILL_PATH/tile.json")
        if [ "$PRIVATE_VALUE" = "false" ]; then
            log_pass "tile.json has private: false (required for public publishing)"
        elif [ "$PRIVATE_VALUE" = "true" ]; then
            log_fail "tile.json has private: true (MUST be false for public publishing)"
        else
            log_fail "tile.json missing 'private' field (MUST be explicitly set to false)"
        fi
        
        # REQUIRED FIELD: name
        NAME_VALUE=$(jq -r '.name // "null"' "$SKILL_PATH/tile.json")
        if [ "$NAME_VALUE" != "null" ] && [ "$NAME_VALUE" != "" ]; then
            # Validate name format: workspace/tile-name
            if echo "$NAME_VALUE" | grep -qE '^[a-z0-9-]+/[a-z0-9-]+$'; then
                log_pass "tile.json has valid 'name' field: $NAME_VALUE"
            else
                log_fail "tile.json 'name' field has invalid format: $NAME_VALUE (must be workspace/tile-name, lowercase, kebab-case)"
            fi
        else
            log_fail "tile.json missing 'name' field (REQUIRED: workspace/tile-name)"
        fi
        
        # REQUIRED FIELD: version
        VERSION_VALUE=$(jq -r '.version // "null"' "$SKILL_PATH/tile.json")
        if [ "$VERSION_VALUE" != "null" ] && [ "$VERSION_VALUE" != "" ]; then
            # Validate semantic versioning: x.y.z
            if echo "$VERSION_VALUE" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
                log_pass "tile.json has valid 'version' field: $VERSION_VALUE (semantic versioning)"
            else
                log_fail "tile.json 'version' field invalid: $VERSION_VALUE (must be x.y.z format, e.g., 1.0.0)"
            fi
        else
            log_fail "tile.json missing 'version' field (REQUIRED: semantic versioning x.y.z)"
        fi
        
        # REQUIRED FIELD: summary
        SUMMARY_VALUE=$(jq -r '.summary // "null"' "$SKILL_PATH/tile.json")
        if [ "$SUMMARY_VALUE" != "null" ] && [ "$SUMMARY_VALUE" != "" ]; then
            SUMMARY_LENGTH=${#SUMMARY_VALUE}
            if [ "$SUMMARY_LENGTH" -ge 100 ] && [ "$SUMMARY_LENGTH" -le 500 ]; then
                log_pass "tile.json has descriptive 'summary' field ($SUMMARY_LENGTH chars, recommended 150-300)"
            elif [ "$SUMMARY_LENGTH" -lt 100 ]; then
                log_warn "tile.json 'summary' is short ($SUMMARY_LENGTH chars, recommended 150-300 for discoverability)"
            else
                log_warn "tile.json 'summary' is long ($SUMMARY_LENGTH chars, recommended 150-300)"
            fi
            
            # Check for generic summaries
            if echo "$SUMMARY_VALUE" | grep -qiE '(useful|helpful|good|nice|simple)'; then
                log_warn "tile.json 'summary' may be too generic (avoid words like 'useful', 'helpful', 'simple')"
            fi
        else
            log_fail "tile.json missing 'summary' field (REQUIRED: descriptive summary with use cases)"
        fi
        
        # REQUIRED FIELD: skills
        SKILLS_COUNT=$(jq -r '.skills // {} | length' "$SKILL_PATH/tile.json")
        if [ "$SKILLS_COUNT" -gt 0 ]; then
            log_pass "tile.json has 'skills' object with $SKILLS_COUNT skill(s)"
            
            # Validate each skill path exists
            jq -r '.skills | to_entries[] | "\(.key)=\(.value.path)"' "$SKILL_PATH/tile.json" 2>/dev/null | while IFS='=' read -r skill_id skill_path; do
                FULL_SKILL_PATH="$SKILL_PATH/$skill_path"
                if [ -f "$FULL_SKILL_PATH" ]; then
                    log_pass "Skill '$skill_id' path exists: $skill_path"
                    
                    # Check SKILL.md has YAML frontmatter
                    if grep -q '^---$' "$FULL_SKILL_PATH" 2>/dev/null; then
                        log_pass "Skill '$skill_id' has YAML frontmatter"
                    else
                        log_warn "Skill '$skill_id' missing YAML frontmatter (required: name, description)"
                    fi
                else
                    log_fail "Skill '$skill_id' path does not exist: $skill_path"
                fi
            done
        else
            log_fail "tile.json missing 'skills' object or empty (REQUIRED: at least one skill)"
        fi
        
        # OPTIONAL ROOT-LEVEL: files (11% usage)
        FILES_COUNT=$(jq -r '.files // [] | length' "$SKILL_PATH/tile.json")
        if [ "$FILES_COUNT" -gt 0 ]; then
            log_pass "tile.json has 'files' array with $FILES_COUNT file(s) to bundle"
            
            # Validate each file exists
            jq -r '.files[]' "$SKILL_PATH/tile.json" 2>/dev/null | while IFS= read -r file_path; do
                FULL_FILE_PATH="$SKILL_PATH/$file_path"
                if [ -f "$FULL_FILE_PATH" ]; then
                    log_pass "File exists: $file_path"
                else
                    log_warn "File does not exist: $file_path"
                fi
            done
        fi
        
        # ANTI-PATTERN CHECK: keywords array (deprecated, 2% usage)
        KEYWORDS_COUNT=$(jq -r '.keywords // [] | length' "$SKILL_PATH/tile.json")
        if [ "$KEYWORDS_COUNT" -gt 0 ]; then
            log_warn "tile.json has deprecated 'keywords' array (ANTI-PATTERN: embed keywords in summary instead)"
            log_info "Example: 'Keywords: term1, term2, term3' at end of summary field"
        fi
        
        # OPTIONAL SKILL-LEVEL: references/resources (45% usage for references)
        jq -r '.skills | to_entries[] | "\(.key)=\(.value.references // [] | length)=\(.value.resources // [] | length)"' "$SKILL_PATH/tile.json" 2>/dev/null | while IFS='=' read -r skill_id ref_count res_count; do
            if [ "$ref_count" -gt 0 ]; then
                log_pass "Skill '$skill_id' has $ref_count reference(s)"
            fi
            if [ "$res_count" -gt 0 ]; then
                log_pass "Skill '$skill_id' has $res_count resource(s)"
            fi
        done
        
        # OPTIONAL: docs
        DOCS_VALUE=$(jq -r '.docs // "null"' "$SKILL_PATH/tile.json")
        if [ "$DOCS_VALUE" != "null" ] && [ "$DOCS_VALUE" != "" ]; then
            DOCS_PATH="$SKILL_PATH/$DOCS_VALUE"
            if [ -f "$DOCS_PATH" ]; then
                log_pass "tile.json has 'docs' field pointing to existing file: $DOCS_VALUE"
            else
                log_warn "tile.json 'docs' field points to non-existent file: $DOCS_VALUE"
            fi
        fi
    else
        log_fail "jq not installed (required for tile.json validation)"
        log_info "Install jq: brew install jq (macOS) or apt-get install jq (Linux)"
    fi
else
    log_fail "tile.json not found (run 'tessl skill import $SKILL_PATH' first)"
fi

# Check 3: Evaluation scenarios
echo ""
echo "${BLUE}[3/8]${NC} Checking evaluation scenarios..."
if [ -d "$SKILL_PATH/evaluation-scenarios" ]; then
    SCENARIO_COUNT=$(find "$SKILL_PATH/evaluation-scenarios" -name "*.md" -type f | wc -l | tr -d ' ')
    if [ "$SCENARIO_COUNT" -ge 5 ]; then
        log_pass "Found $SCENARIO_COUNT evaluation scenarios (≥5 required)"
    else
        log_fail "Found only $SCENARIO_COUNT evaluation scenarios (minimum 5 required)"
    fi
else
    log_fail "evaluation-scenarios/ directory not found (required for public publishing)"
fi

# Check 4: Quality audit results
echo ""
echo "${BLUE}[4/8]${NC} Checking quality audit results..."
AUDIT_PATH=".context/audits/${SKILL_DOMAIN}/${SKILL_NAME}/latest"
if [ -d "$AUDIT_PATH" ]; then
    if [ -f "$AUDIT_PATH/analysis.md" ]; then
        # Extract total score from analysis.md
        if grep -q "Total Score" "$AUDIT_PATH/analysis.md"; then
            TOTAL_SCORE=$(grep "Total Score" "$AUDIT_PATH/analysis.md" | grep -o '[0-9]\+/120' | cut -d'/' -f1)
            if [ "$TOTAL_SCORE" -ge 108 ]; then
                log_pass "Quality audit score: ${TOTAL_SCORE}/120 (A-grade, ≥108 required)"
            else
                log_fail "Quality audit score: ${TOTAL_SCORE}/120 (below A-grade threshold of 108/120)"
                log_info "Review remediation plan: ${AUDIT_PATH}/remediation-plan.md"
            fi
        else
            log_warn "Could not extract score from analysis.md"
        fi
    else
        log_fail "Quality audit analysis.md not found"
    fi
else
    log_fail "No quality audit results found in ${AUDIT_PATH}"
    log_info "Run: sh skills/agentic-harness/skill-quality-auditor/scripts/evaluate.sh ${SKILL_DOMAIN}/${SKILL_NAME} --json --store"
fi

# Check 5: Agent-agnostic compliance (basic scan)
echo ""
echo "${BLUE}[5/8]${NC} Checking agent-agnostic compliance..."
if [ -f "$SKILL_PATH/SKILL.md" ]; then
    VIOLATIONS=""
    
    # Check for common agent-specific references
    if grep -iq "claude code" "$SKILL_PATH/SKILL.md"; then
        VIOLATIONS="${VIOLATIONS}\n  - Found 'Claude Code' references"
    fi
    if grep -iq "cursor" "$SKILL_PATH/SKILL.md"; then
        VIOLATIONS="${VIOLATIONS}\n  - Found 'Cursor' references"
    fi
    if grep -iq "vs code" "$SKILL_PATH/SKILL.md"; then
        VIOLATIONS="${VIOLATIONS}\n  - Found 'VS Code' references"
    fi
    if grep -iq "opencode" "$SKILL_PATH/SKILL.md" && [ "$SKILL_NAME" != "opencode" ]; then
        VIOLATIONS="${VIOLATIONS}\n  - Found 'OpenCode' references"
    fi
    
    if [ -z "$VIOLATIONS" ]; then
        log_pass "No obvious agent-specific references found"
    else
        log_fail "Found potential agent-specific violations:${VIOLATIONS}"
        log_info "Manual review required for cross-platform compatibility"
    fi
else
    log_warn "Cannot check agent-agnostic compliance without SKILL.md"
fi

# Check 6: Frontmatter validation
echo ""
echo "${BLUE}[6/8]${NC} Checking SKILL.md frontmatter..."
if [ -f "$SKILL_PATH/SKILL.md" ]; then
    if head -n 1 "$SKILL_PATH/SKILL.md" | grep -q '^---$'; then
        log_pass "YAML frontmatter detected"
        
        # Check for name and description
        if grep -q "^name:" "$SKILL_PATH/SKILL.md"; then
            log_pass "Frontmatter has 'name' field"
        else
            log_fail "Frontmatter missing 'name' field"
        fi
        
        if grep -q "^description:" "$SKILL_PATH/SKILL.md"; then
            log_pass "Frontmatter has 'description' field"
        else
            log_fail "Frontmatter missing 'description' field"
        fi
    else
        log_fail "SKILL.md missing YAML frontmatter"
    fi
else
    log_warn "Cannot validate frontmatter without SKILL.md"
fi

# Check 7: Required sections
echo ""
echo "${BLUE}[7/8]${NC} Checking required SKILL.md sections..."
if [ -f "$SKILL_PATH/SKILL.md" ]; then
    SECTIONS_FOUND=0
    SECTIONS_REQUIRED=6
    
    if grep -q "## When to Use" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: When to Use"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: When to Use"
    fi
    
    if grep -q "## Mindset" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: Mindset"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: Mindset"
    fi
    
    if grep -q "## Workflow" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: Workflow"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: Workflow"
    fi
    
    if grep -q "## Anti-Patterns" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: Anti-Patterns"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: Anti-Patterns"
    fi
    
    if grep -q "## Success Metrics" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: Success Metrics"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: Success Metrics"
    fi
    
    if grep -q "## Quick Reference" "$SKILL_PATH/SKILL.md"; then
        log_pass "Section: Quick Reference"
        SECTIONS_FOUND=$((SECTIONS_FOUND + 1))
    else
        log_warn "Missing section: Quick Reference"
    fi
    
    if [ "$SECTIONS_FOUND" -ge "$SECTIONS_REQUIRED" ]; then
        log_pass "All required sections present ($SECTIONS_FOUND/$SECTIONS_REQUIRED)"
    else
        log_warn "Some recommended sections missing ($SECTIONS_FOUND/$SECTIONS_REQUIRED found)"
    fi
else
    log_warn "Cannot check sections without SKILL.md"
fi

# Check 8: Tessl review score (if available)
echo ""
echo "${BLUE}[8/8]${NC} Tessl review recommendation..."
log_info "Run 'tessl skill review $SKILL_PATH' to check Tessl quality score"
log_info "If score < 90%, run 'tessl skill review $SKILL_PATH --optimize'"
log_info "Target: ≥90% for public publishing"

# Final summary
echo ""
echo "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo "${BLUE}║  Summary                                                   ║${NC}"
echo "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Checks passed:  ${GREEN}${CHECKS_PASSED}${NC}"
echo "Checks failed:  ${RED}${CHECKS_FAILED}${NC}"
echo "Warnings:       ${YELLOW}${WARNINGS}${NC}"
echo ""

if [ "$CHECKS_FAILED" -eq 0 ]; then
    echo "${GREEN}✓ Skill appears ready for public publishing!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Run: tessl skill review $SKILL_PATH"
    echo "  2. If score < 90%, run: tessl skill review $SKILL_PATH --optimize"
    echo "  3. Publish: tessl skill publish $SKILL_PATH --public"
    echo ""
    exit 0
else
    echo "${RED}✗ Skill NOT ready for public publishing${NC}"
    echo ""
    echo "Fix the failed checks above before attempting publication."
    echo ""
    exit 1
fi
