#!/usr/bin/env sh
# Tessl Registry Compliance Checker
# Validates skills for agent-agnostic compatibility and performance metrics

set -e

SKILL_NAME="${1:-}"
SKILLS_DIR="skills"
VERBOSE="${2:-}"

if [ -z "$SKILL_NAME" ]; then
    echo "Usage: $0 <skill-name> [--verbose]"
    echo "Example: $0 bdd-testing --verbose"
    exit 1
fi

SKILL_PATH="$SKILLS_DIR/$SKILL_NAME"

if [ ! -d "$SKILL_PATH" ]; then
    echo "❌ Error: Skill directory '$SKILL_PATH' not found"
    exit 1
fi

echo "🔍 Checking Tessl compliance for: $SKILL_NAME"
echo ""

# Initialize results
AGENT_AGNOSTIC_PASS=true
PERFORMANCE_METRICS_PASS=true
CROSS_PLATFORM_PASS=true

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_verbose() {
    if [ "$VERBOSE" = "--verbose" ]; then
        echo "  $1"
    fi
}

# 1. Agent-Agnostic Validation
echo "${BLUE}1. Agent-Agnostic Validation${NC}"
echo "================================"

# Check for agent-specific terms in content (excluding clear negative examples and compliance docs)
agent_specific_terms=""
temp_file=$(mktemp)
find "$SKILL_PATH" -name "*.md" > "$temp_file"
while read -r file; do
    # Skip tessl compliance documentation files (they teach about agent-agnostic patterns using bad examples)
    if echo "$file" | grep -q "tessl-compliance"; then
        continue
    fi
    
    # Check if file contains agent names
    if grep -qi "claude\|cursor\|openai\|copilot\|gemini\|chatgpt" "$file"; then
        # Check if it's not just negative examples (has agent names outside BAD examples)
        if grep -i "claude\|cursor\|openai\|copilot\|gemini\|chatgpt" "$file" | grep -v "❌.*BAD\|BAD:" | head -1 | grep -q .; then
            agent_specific_terms="$agent_specific_terms $file"
        fi
    fi
done < "$temp_file"
rm "$temp_file"

if [ -n "$agent_specific_terms" ]; then
    echo "${RED}❌ FAIL: Agent-specific references found${NC}"
    AGENT_AGNOSTIC_PASS=false
    for file in $agent_specific_terms; do
        echo "  📄 $file"
        if [ "$VERBOSE" = "--verbose" ]; then
            grep -n -i "claude\|cursor\|openai\|copilot\|gemini\|chatgpt" "$file" | head -3 || true
        fi
    done
else
    echo "${GREEN}✅ PASS: No agent-specific references found${NC}"
fi

# Check allowed-tools for agent-specific tools
if [ -f "$SKILL_PATH/SKILL.md" ]; then
    agent_tools=$(grep -E "^allowed-tools:" "$SKILL_PATH/SKILL.md" | grep -E "(claude|cursor|openai|copilot)-" || true)
    if [ -n "$agent_tools" ]; then
        echo "${RED}❌ FAIL: Agent-specific tools in allowed-tools${NC}"
        AGENT_AGNOSTIC_PASS=false
        log_verbose "$agent_tools"
    else
        echo "${GREEN}✅ PASS: No agent-specific tools in allowed-tools${NC}"
    fi
fi

echo ""

# 2. Performance Metrics Integration  
echo "${BLUE}2. Performance Metrics Integration${NC}"
echo "==================================="

# Look for success metrics sections
metrics_sections=$(find "$SKILL_PATH" -name "*.md" -exec grep -l -i "success metrics\|expected outcomes\|effectiveness\|performance" {} \; 2>/dev/null || true)

if [ -z "$metrics_sections" ]; then
    echo "${RED}❌ FAIL: No performance metrics sections found${NC}"
    PERFORMANCE_METRICS_PASS=false
else
    echo "${GREEN}✅ PASS: Performance metrics sections found${NC}"
    log_verbose "Files with metrics: $metrics_sections"
fi

# Check for quantified claims (percentages, numbers, etc)
quantified_claims=$(find "$SKILL_PATH" -name "*.md" -exec grep -l -E "[0-9]+(%|x|×|times|\s(seconds|minutes|hours)|reduction|improvement|faster|slower)" {} \; 2>/dev/null || true)

if [ -z "$quantified_claims" ]; then
    echo "${RED}❌ FAIL: No quantified performance claims found${NC}"
    PERFORMANCE_METRICS_PASS=false
else
    echo "${GREEN}✅ PASS: Quantified performance claims found${NC}"
    log_verbose "Files with quantified claims: $quantified_claims"
fi

echo ""

# 3. Cross-Platform Compatibility
echo "${BLUE}3. Cross-Platform Compatibility${NC}" 
echo "================================="

# Check for platform-specific paths
platform_paths=$(find "$SKILL_PATH" -name "*.md" -exec grep -l -E "/usr/local|C:\\\\|/home/[^/]+|~/\." {} \; 2>/dev/null || true)

if [ -n "$platform_paths" ]; then
    echo "${YELLOW}⚠️  WARNING: Platform-specific paths found${NC}"
    log_verbose "Files with platform paths: $platform_paths"
    # Don't fail, just warn
else
    echo "${GREEN}✅ PASS: No hard-coded platform paths${NC}"
fi

# Check for Windows/Unix specific commands
platform_commands=$(find "$SKILL_PATH" -name "*.md" -exec grep -l -E "brew install|apt install|yum install|choco install" {} \; 2>/dev/null || true)

if [ -n "$platform_commands" ]; then
    # Check if alternatives are provided
    alternatives_provided=true
    for file in $platform_commands; do
        if ! grep -q -E "(macOS|Ubuntu|Windows|Linux)" "$file"; then
            alternatives_provided=false
            break
        fi
    done
    
    if [ "$alternatives_provided" = true ]; then
        echo "${GREEN}✅ PASS: Platform-specific commands with alternatives${NC}"
    else
        echo "${YELLOW}⚠️  WARNING: Platform-specific commands without alternatives${NC}"
        log_verbose "Consider providing cross-platform installation options"
    fi
else
    echo "${GREEN}✅ PASS: No platform-specific package commands${NC}"
fi

echo ""

# Summary
echo "${BLUE}📊 Tessl Compliance Summary${NC}"
echo "============================="

overall_pass=true

if [ "$AGENT_AGNOSTIC_PASS" = true ]; then
    echo "${GREEN}✅ Agent-Agnostic: PASS${NC}"
else
    echo "${RED}❌ Agent-Agnostic: FAIL${NC}"
    overall_pass=false
fi

if [ "$PERFORMANCE_METRICS_PASS" = true ]; then
    echo "${GREEN}✅ Performance Metrics: PASS${NC}"
else
    echo "${RED}❌ Performance Metrics: FAIL${NC}"  
    overall_pass=false
fi

if [ "$CROSS_PLATFORM_PASS" = true ]; then
    echo "${GREEN}✅ Cross-Platform: PASS${NC}"
else
    echo "${RED}❌ Cross-Platform: FAIL${NC}"
    overall_pass=false
fi

echo ""

if [ "$overall_pass" = true ]; then
    echo "${GREEN}🎉 OVERALL: TESSL COMPLIANT${NC}"
    echo "This skill is ready for Tessl registry submission"
    exit 0
else
    echo "${RED}❌ OVERALL: NOT TESSL COMPLIANT${NC}"
    echo "Fix the issues above before submitting to Tessl registry"
    exit 1
fi