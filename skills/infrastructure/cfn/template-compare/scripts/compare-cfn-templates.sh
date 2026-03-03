#!/usr/bin/env bash
#
# CloudFormation Template Comparison Script
# 
# Compares a deployed CloudFormation stack with a locally synthesized CDK template
# to detect drift, validate changes, and ensure consistency.
#
# Usage:
#   compare-cfn-templates.sh <deployed-stack-name> <region> <profile> [local-template-path]
#
# Example:
#   compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st
#   compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st cdk.out/my-stack.template.json
#
# Exit codes:
#   0 - Templates match (expected differences only)
#   1 - Critical differences found, deployment not recommended
#   2 - Script error (missing dependencies, AWS error, etc.)
#

set -e
set -o pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DEPLOYED_STACK="${1:?Missing deployed stack name}"
REGION="${2:-eu-west-1}"
PROFILE="${3:-default}"
LOCAL_TEMPLATE="${4}"

# Generate context-aware directory name
TIMESTAMP=$(date +%Y-%m-%d-%H%M%S)
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null | tr '/' '-' | tr '#' '-' || echo "unknown")
PPLAWS_REF=$(grep -E "^export PPLAWS_REFERENCE=" env-local.mk 2>/dev/null | cut -d'=' -f2 | tr -d ' ' || echo "none")

# Working directory for comparison files (timestamped with context)
WORK_DIR="cfn-compare-results/${TIMESTAMP}_deployed-main_local-${CURRENT_BRANCH}_ref-${PPLAWS_REF}"
mkdir -p "$WORK_DIR"

# Preserve artifacts by default
PRESERVE_ARTIFACTS="${PRESERVE_ARTIFACTS:-true}"

# Cleanup on exit
cleanup() {
    if [ "$PRESERVE_ARTIFACTS" != "true" ]; then
        rm -rf "$WORK_DIR"
        log_info "Artifacts cleaned up"
    else
        log_info "Artifacts preserved in: $WORK_DIR"
    fi
}
trap cleanup EXIT

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    local missing=()
    
    command -v aws >/dev/null 2>&1 || missing+=("aws-cli")
    command -v jq >/dev/null 2>&1 || missing+=("jq")
    
    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing required dependencies: ${missing[*]}"
        log_error "Install with: brew install ${missing[*]}"
        exit 2
    fi
}

# Retrieve deployed CloudFormation template
retrieve_deployed_template() {
    log_info "Retrieving deployed template from stack: $DEPLOYED_STACK"
    
    if ! aws cloudformation get-template \
        --stack-name "$DEPLOYED_STACK" \
        --region "$REGION" \
        --profile "$PROFILE" \
        --output json 2>"$WORK_DIR/aws-error.log" \
        | jq -r '.TemplateBody' > "$WORK_DIR/deployed.json"; then
        log_error "Failed to retrieve deployed template"
        cat "$WORK_DIR/aws-error.log" >&2
        exit 2
    fi
    
    local line_count=$(wc -l < "$WORK_DIR/deployed.json")
    log_success "Retrieved deployed template ($line_count lines)"
}

# Locate local template
locate_local_template() {
    if [ -n "$LOCAL_TEMPLATE" ]; then
        if [ ! -f "$LOCAL_TEMPLATE" ]; then
            log_error "Specified local template not found: $LOCAL_TEMPLATE"
            exit 2
        fi
        cp "$LOCAL_TEMPLATE" "$WORK_DIR/local.json"
    else
        # Auto-detect from cdk.out
        log_info "Auto-detecting local template from cdk.out/"
        
        if [ ! -d "cdk.out" ]; then
            log_error "cdk.out directory not found. Run 'make synth' first."
            exit 2
        fi
        
        local template_count=$(find cdk.out -name "*.template.json" -type f | wc -l)
        
        if [ "$template_count" -eq 0 ]; then
            log_error "No CloudFormation templates found in cdk.out/"
            exit 2
        elif [ "$template_count" -gt 1 ]; then
            log_warning "Multiple templates found. Specify which one to compare."
            find cdk.out -name "*.template.json" -type f
            exit 2
        fi
        
        cp cdk.out/*.template.json "$WORK_DIR/local.json"
    fi
    
    local line_count=$(wc -l < "$WORK_DIR/local.json")
    log_success "Located local template ($line_count lines)"
}

# Compare template structure
compare_structure() {
    log_info "Comparing template structure..."
    
    # Top-level keys
    jq -r 'keys | sort[]' "$WORK_DIR/deployed.json" > "$WORK_DIR/deployed-keys.txt"
    jq -r 'keys | sort[]' "$WORK_DIR/local.json" > "$WORK_DIR/local-keys.txt"
    
    if ! diff -q "$WORK_DIR/deployed-keys.txt" "$WORK_DIR/local-keys.txt" >/dev/null; then
        log_warning "Template structure differs (top-level keys)"
        diff "$WORK_DIR/deployed-keys.txt" "$WORK_DIR/local-keys.txt" || true
        return 1
    fi
    
    log_success "Template structure matches"
    return 0
}

# Compare resource counts
compare_resources() {
    log_info "Comparing resources..."
    
    local deployed_count=$(jq -r '.Resources | keys | length' "$WORK_DIR/deployed.json")
    local local_count=$(jq -r '.Resources | keys | length' "$WORK_DIR/local.json")
    
    echo "  Deployed: $deployed_count resources"
    echo "  Local:    $local_count resources"
    
    # Extract resource lists
    jq -r '.Resources | keys | sort[]' "$WORK_DIR/deployed.json" > "$WORK_DIR/deployed-resources.txt"
    jq -r '.Resources | keys | sort[]' "$WORK_DIR/local.json" > "$WORK_DIR/local-resources.txt"
    
    # Find differences
    local added=$(comm -13 "$WORK_DIR/deployed-resources.txt" "$WORK_DIR/local-resources.txt" 2>/dev/null | wc -l)
    local removed=$(comm -23 "$WORK_DIR/deployed-resources.txt" "$WORK_DIR/local-resources.txt" 2>/dev/null | wc -l)
    local common=$(comm -12 "$WORK_DIR/deployed-resources.txt" "$WORK_DIR/local-resources.txt" 2>/dev/null | wc -l)
    
    echo "  Common:   $common resources"
    
    if [ "$added" -gt 0 ]; then
        log_warning "Added resources: $added"
        comm -13 "$WORK_DIR/deployed-resources.txt" "$WORK_DIR/local-resources.txt" 2>/dev/null | head -10
    fi
    
    if [ "$removed" -gt 0 ]; then
        log_warning "Removed resources: $removed"
        comm -23 "$WORK_DIR/deployed-resources.txt" "$WORK_DIR/local-resources.txt" 2>/dev/null | head -10
    fi
    
    if [ "$added" -gt 0 ] || [ "$removed" -gt 0 ]; then
        return 1
    fi
    
    log_success "Resource counts match ($deployed_count resources)"
    return 0
}

# Compare CDK Nag suppressions
compare_cdk_nag() {
    log_info "Comparing CDK Nag suppressions..."
    
    local deployed_suppressions=$(jq -r '[.Resources | to_entries[] | select(.value.Metadata.cdk_nag != null) | .key] | length' "$WORK_DIR/deployed.json")
    local local_suppressions=$(jq -r '[.Resources | to_entries[] | select(.value.Metadata.cdk_nag != null) | .key] | length' "$WORK_DIR/local.json")
    
    echo "  Deployed: $deployed_suppressions resources with suppressions"
    echo "  Local:    $local_suppressions resources with suppressions"
    
    if [ "$deployed_suppressions" -ne "$local_suppressions" ]; then
        log_warning "CDK Nag suppression count differs - review required"
        return 1
    fi
    
    log_success "CDK Nag suppressions consistent"
    return 0
}

# Analyze resource renaming (same resource, different hash suffix)
analyze_renamed_resources() {
    local deployed_res="$1"
    local local_res="$2"
    
    # Extract base name without trailing hash (last 16 chars typically)
    local deployed_base=$(echo "$deployed_res" | sed -E 's/[A-F0-9]{16}$//' | sed -E 's/[a-f0-9]{8}$//')
    local local_base=$(echo "$local_res" | sed -E 's/[A-F0-9]{16}$//' | sed -E 's/[a-f0-9]{8}$//')
    
    # Check if base names are similar (allowing for small variations)
    if [ "$deployed_base" = "$local_base" ]; then
        echo "RENAMED"
        return 0
    fi
    
    # Check if only difference is stack name suffix (e.g., lctmonitoringst vs lctmonitoringstthoroc)
    local deployed_normalized=$(echo "$deployed_res" | sed -E 's/lctmonitoringst[a-z0-9]*/lctmonitoringst/' | sed -E 's/[A-F0-9]{16}$//' | sed -E 's/[a-f0-9]{8}$//')
    local local_normalized=$(echo "$local_res" | sed -E 's/lctmonitoringst[a-z0-9]*/lctmonitoringst/' | sed -E 's/[A-F0-9]{16}$//' | sed -E 's/[a-f0-9]{8}$//')
    
    if [ "$deployed_normalized" = "$local_normalized" ]; then
        echo "RENAMED"
        return 0
    fi
    
    echo "DIFFERENT"
    return 1
}

# Generate markdown comparison report
generate_markdown_report() {
    local report_file="$WORK_DIR/comparison-report.md"
    
    log_info "Generating markdown comparison report..."
    
    # YAML Frontmatter Header
    cat > "$report_file" <<EOF
---
generated: $(date '+%Y-%m-%d %H:%M:%S %Z')
deployed_stack: $DEPLOYED_STACK
region: $REGION
profile: $PROFILE
local_branch: $CURRENT_BRANCH
pplaws_reference: $PPLAWS_REF
---

# CloudFormation Template Comparison Report

## Executive Summary

EOF

    # Resource counts
    deployed_count=$(jq -r '.Resources | keys | length' "$WORK_DIR/deployed.json")
    local_count=$(jq -r '.Resources | keys | length' "$WORK_DIR/local.json")
    
    # Sort files to ensure proper comm operation (comm requires sorted input)
    sort "$WORK_DIR/deployed-resources.txt" > "$WORK_DIR/deployed-resources-sorted.txt"
    sort "$WORK_DIR/local-resources.txt" > "$WORK_DIR/local-resources-sorted.txt"
    
    added=$(comm -13 "$WORK_DIR/deployed-resources-sorted.txt" "$WORK_DIR/local-resources-sorted.txt" | wc -l | tr -d ' ')
    removed=$(comm -23 "$WORK_DIR/deployed-resources-sorted.txt" "$WORK_DIR/local-resources-sorted.txt" | wc -l | tr -d ' ')
    common=$(comm -12 "$WORK_DIR/deployed-resources-sorted.txt" "$WORK_DIR/local-resources-sorted.txt" | wc -l | tr -d ' ')
    
    cat >> "$report_file" <<EOF
| Metric | Deployed | Local | Status |
|--------|----------|-------|--------|
| Total Resources | $deployed_count | $local_count | $([ "$deployed_count" -eq "$local_count" ] && echo "✅" || echo "⚠️") |
| Common Resources | $common | $common | ✅ |
| Added Resources | 0 | $added | $([ "$added" -eq 0 ] && echo "✅" || echo "⚠️") |
| Removed Resources | $removed | 0 | $([ "$removed" -eq 0 ] && echo "✅" || echo "⚠️") |

EOF

    # Structure comparison
    cat >> "$report_file" <<EOF
## Template Structure

EOF

    if diff -q "$WORK_DIR/deployed-keys.txt" "$WORK_DIR/local-keys.txt" >/dev/null 2>&1; then
        echo "✅ **Template structure matches** - Both templates have identical top-level keys" >> "$report_file"
    else
        echo "⚠️ **Template structure differs** - Top-level keys do not match" >> "$report_file"
        echo "" >> "$report_file"
        echo "### Deployed Keys" >> "$report_file"
        echo '```' >> "$report_file"
        cat "$WORK_DIR/deployed-keys.txt" >> "$report_file"
        echo '```' >> "$report_file"
        echo "" >> "$report_file"
        echo "### Local Keys" >> "$report_file"
        echo '```' >> "$report_file"
        cat "$WORK_DIR/local-keys.txt" >> "$report_file"
        echo '```' >> "$report_file"
    fi
    
    echo "" >> "$report_file"
    
    # Added resources
    if [ "$added" -gt 0 ]; then
        # Analyze if resources are renamed vs truly new
        # Build lists of added and removed resources
        comm -13 "$WORK_DIR/deployed-resources-sorted.txt" "$WORK_DIR/local-resources-sorted.txt" > "$WORK_DIR/added-resources.txt"
        comm -23 "$WORK_DIR/deployed-resources-sorted.txt" "$WORK_DIR/local-resources-sorted.txt" > "$WORK_DIR/removed-resources.txt"
        
        # Clear previous analysis files
        rm -f "$WORK_DIR/renamed-resources.txt" "$WORK_DIR/truly-new-resources.txt" "$WORK_DIR/truly-removed-resources.txt"
        touch "$WORK_DIR/renamed-resources.txt" "$WORK_DIR/truly-new-resources.txt" "$WORK_DIR/truly-removed-resources.txt"
        
        # Match renamed resources
        while IFS= read -r local_res; do
            local found_match=0
            local local_type=$(jq -r ".Resources[\"$local_res\"].Type" "$WORK_DIR/local.json")
            
            # Extract base resource name (everything before the hash pattern)
            # For resources like "BeehiveAccessSecurityGrouptolctmonitoringstthorocBeehiveDbSecurityGroup6B95C6C8543226F10A21"
            # We want to match on the functional part, ignoring stack refs and hashes
            local local_base=$(echo "$local_res" | sed -E 's/lctmonitoringst[a-z0-9]*/STACKREF/g' | sed -E 's/[A-F0-9]{4,16}//g' | sed -E 's/[a-f0-9]{8}//g')
            
            while IFS= read -r deployed_res; do
                local deployed_type=$(jq -r ".Resources[\"$deployed_res\"].Type" "$WORK_DIR/deployed.json")
                
                # Only match if types are the same
                if [ "$deployed_type" = "$local_type" ]; then
                    # Extract base from deployed resource
                    local deployed_base=$(echo "$deployed_res" | sed -E 's/lctmonitoringst[a-z0-9]*/STACKREF/g' | sed -E 's/[A-F0-9]{4,16}//g' | sed -E 's/[a-f0-9]{8}//g')
                    
                    if [ "$deployed_base" = "$local_base" ]; then
                        echo "$deployed_res → $local_res ($local_type)" >> "$WORK_DIR/renamed-resources.txt"
                        found_match=1
                        break
                    fi
                fi
            done < "$WORK_DIR/removed-resources.txt"
            
            if [ "$found_match" -eq 0 ]; then
                echo "$local_res" >> "$WORK_DIR/truly-new-resources.txt"
            fi
        done < "$WORK_DIR/added-resources.txt"
        
        # Find truly removed resources (not part of rename pairs)
        while IFS= read -r deployed_res; do
            if ! grep -q "^$deployed_res →" "$WORK_DIR/renamed-resources.txt"; then
                echo "$deployed_res" >> "$WORK_DIR/truly-removed-resources.txt"
            fi
        done < "$WORK_DIR/removed-resources.txt"
        
        # Count renamed vs truly new
        renamed_count=$(wc -l < "$WORK_DIR/renamed-resources.txt" 2>/dev/null | tr -d ' ')
        truly_new_count=$(wc -l < "$WORK_DIR/truly-new-resources.txt" 2>/dev/null | tr -d ' ')
        truly_removed_count=$(wc -l < "$WORK_DIR/truly-removed-resources.txt" 2>/dev/null | tr -d ' ')
        
        if [ "$renamed_count" -gt 0 ]; then
            cat >> "$report_file" <<EOF
## Renamed Resources

The following $renamed_count resource(s) have been renamed (same function, different ID due to naming changes):

EOF
            if [ -f "$WORK_DIR/renamed-resources.txt" ]; then
                cat "$WORK_DIR/renamed-resources.txt" | while read -r line; do
                    echo "- \`$line\`" >> "$report_file"
                done
            fi
            echo "" >> "$report_file"
            echo "> **Note**: These resources will be replaced during deployment. CloudFormation will create the new resource before deleting the old one (if possible)." >> "$report_file"
            echo "" >> "$report_file"
        fi
        
        if [ "$truly_new_count" -gt 0 ]; then
            cat >> "$report_file" <<EOF
## Added Resources (New)

The following $truly_new_count resource(s) are genuinely new and do not exist in deployed:

EOF
            while IFS= read -r resource; do
                resource_type=$(jq -r ".Resources[\"$resource\"].Type" "$WORK_DIR/local.json")
                echo "- **$resource** (\`$resource_type\`)" >> "$report_file"
            done < "$WORK_DIR/truly-new-resources.txt"
            echo "" >> "$report_file"
        fi
    else
        cat >> "$report_file" <<EOF
## Resource Changes

✅ No resources added or removed - templates are equivalent.

EOF
    fi
    
    # Removed resources (only show truly removed, not renamed)
    if [ "$removed" -gt 0 ]; then
        truly_removed_count=$(wc -l < "$WORK_DIR/truly-removed-resources.txt" 2>/dev/null | tr -d ' ')
        
        if [ "$truly_removed_count" -gt 0 ]; then
            cat >> "$report_file" <<EOF
## Removed Resources

⚠️ The following $truly_removed_count resource(s) will be DELETED during deployment:

EOF
            if [ -f "$WORK_DIR/truly-removed-resources.txt" ]; then
                cat "$WORK_DIR/truly-removed-resources.txt" | while read -r resource; do
                    resource_type=$(jq -r ".Resources[\"$resource\"].Type" "$WORK_DIR/deployed.json")
                    echo "- **$resource** (\`$resource_type\`)" >> "$report_file"
                done
            fi
            echo "" >> "$report_file"
            echo "> **Warning**: These resources will be permanently deleted. Ensure data is backed up if needed." >> "$report_file"
            echo "" >> "$report_file"
        fi
    fi
    
    # CDK Nag suppressions
    cat >> "$report_file" <<EOF
## Security Analysis

### CDK Nag Suppressions

EOF
    
    deployed_suppressions=$(jq -r '[.Resources | to_entries[] | select(.value.Metadata.cdk_nag != null) | .key] | length' "$WORK_DIR/deployed.json")
    local_suppressions=$(jq -r '[.Resources | to_entries[] | select(.value.Metadata.cdk_nag != null) | .key] | length' "$WORK_DIR/local.json")
    
    cat >> "$report_file" <<EOF
| Environment | Resources with Suppressions |
|-------------|---------------------------|
| Deployed | $deployed_suppressions |
| Local | $local_suppressions |

EOF

    if [ "$deployed_suppressions" -eq "$local_suppressions" ]; then
        echo "✅ **CDK Nag suppressions are consistent**" >> "$report_file"
    else
        echo "⚠️ **CDK Nag suppression count differs** - Security review required" >> "$report_file"
    fi
    
    echo "" >> "$report_file"
    
    # Detailed SNS Comparison
    cat >> "$report_file" <<EOF
## SNS Topics and Subscriptions

### Topics

EOF

    # Extract and compare SNS Topics with details
    local deployed_topics=$(jq -r '.Resources | to_entries[] | select(.value.Type == "AWS::SNS::Topic") | .key' "$WORK_DIR/deployed.json")
    local local_topics=$(jq -r '.Resources | to_entries[] | select(.value.Type == "AWS::SNS::Topic") | .key' "$WORK_DIR/local.json")
    
    if [ -z "$deployed_topics" ] && [ -z "$local_topics" ]; then
        echo "No SNS Topics found in either template." >> "$report_file"
    else
        echo "#### Deployed Stack Topics" >> "$report_file"
        echo "" >> "$report_file"
        if [ -z "$deployed_topics" ]; then
            echo "No topics found." >> "$report_file"
        else
            echo "$deployed_topics" | while read -r topic; do
                local display_name=$(jq -r ".Resources[\"$topic\"].Properties.DisplayName // \"N/A\"" "$WORK_DIR/deployed.json")
                local topic_name=$(jq -r ".Resources[\"$topic\"].Properties.TopicName // \"N/A\"" "$WORK_DIR/deployed.json")
                echo "- **$topic**" >> "$report_file"
                echo "  - Display Name: \`$display_name\`" >> "$report_file"
                echo "  - Topic Name: \`$topic_name\`" >> "$report_file"
            done
        fi
        
        echo "" >> "$report_file"
        echo "#### Local Template Topics" >> "$report_file"
        echo "" >> "$report_file"
        if [ -z "$local_topics" ]; then
            echo "No topics found." >> "$report_file"
        else
            echo "$local_topics" | while read -r topic; do
                local display_name=$(jq -r ".Resources[\"$topic\"].Properties.DisplayName // \"N/A\"" "$WORK_DIR/local.json")
                local topic_name=$(jq -r ".Resources[\"$topic\"].Properties.TopicName // \"N/A\"" "$WORK_DIR/local.json")
                echo "- **$topic**" >> "$report_file"
                echo "  - Display Name: \`$display_name\`" >> "$report_file"
                echo "  - Topic Name: \`$topic_name\`" >> "$report_file"
            done
        fi
    fi
    
    echo "" >> "$report_file"
    echo "### Subscriptions" >> "$report_file"
    echo "" >> "$report_file"
    
    # Extract and compare SNS Subscriptions with details
    local deployed_subs=$(jq -r '.Resources | to_entries[] | select(.value.Type == "AWS::SNS::Subscription") | .key' "$WORK_DIR/deployed.json")
    local local_subs=$(jq -r '.Resources | to_entries[] | select(.value.Type == "AWS::SNS::Subscription") | .key' "$WORK_DIR/local.json")
    
    if [ -z "$deployed_subs" ] && [ -z "$local_subs" ]; then
        echo "No SNS Subscriptions found in either template." >> "$report_file"
        echo "" >> "$report_file"
        echo "> **Note**: Subscriptions may be managed outside of CloudFormation (e.g., via console, AWS CLI, or created dynamically by application code)." >> "$report_file"
    else
        echo "#### Deployed Stack Subscriptions" >> "$report_file"
        echo "" >> "$report_file"
        if [ -z "$deployed_subs" ]; then
            echo "No subscriptions found in deployed template." >> "$report_file"
        else
            echo "$deployed_subs" | while read -r sub; do
                local protocol=$(jq -r ".Resources[\"$sub\"].Properties.Protocol // \"N/A\"" "$WORK_DIR/deployed.json")
                local endpoint=$(jq -r ".Resources[\"$sub\"].Properties.Endpoint // \"N/A\"" "$WORK_DIR/deployed.json")
                local topic_arn=$(jq -r ".Resources[\"$sub\"].Properties.TopicArn // \"N/A\"" "$WORK_DIR/deployed.json")
                echo "- **$sub**" >> "$report_file"
                echo "  - Protocol: \`$protocol\`" >> "$report_file"
                echo "  - Endpoint: \`$endpoint\`" >> "$report_file"
                echo "  - Topic ARN: \`$topic_arn\`" >> "$report_file"
            done
        fi
        
        echo "" >> "$report_file"
        echo "#### Local Template Subscriptions" >> "$report_file"
        echo "" >> "$report_file"
        if [ -z "$local_subs" ]; then
            echo "No subscriptions found in local template." >> "$report_file"
        else
            echo "$local_subs" | while read -r sub; do
                local protocol=$(jq -r ".Resources[\"$sub\"].Properties.Protocol // \"N/A\"" "$WORK_DIR/local.json")
                local endpoint=$(jq -r ".Resources[\"$sub\"].Properties.Endpoint // \"N/A\"" "$WORK_DIR/local.json")
                local topic_arn=$(jq -r ".Resources[\"$sub\"].Properties.TopicArn // \"N/A\"" "$WORK_DIR/local.json")
                echo "- **$sub**" >> "$report_file"
                echo "  - Protocol: \`$protocol\`" >> "$report_file"
                echo "  - Endpoint: \`$endpoint\`" >> "$report_file"
                echo "  - Topic ARN: \`$topic_arn\`" >> "$report_file"
            done
        fi
    fi
    
    echo "" >> "$report_file"
    
    # Resource type breakdown
    cat >> "$report_file" <<EOF
## Resource Type Breakdown

### Deployed Stack Resource Types

EOF
    
    jq -r '.Resources | to_entries[] | .value.Type' "$WORK_DIR/deployed.json" | sort | uniq -c | sort -rn | while read -r count type; do
        echo "- **$type**: $count resource(s)" >> "$report_file"
    done
    
    echo "" >> "$report_file"
    echo "### Local Template Resource Types" >> "$report_file"
    echo "" >> "$report_file"
    
    jq -r '.Resources | to_entries[] | .value.Type' "$WORK_DIR/local.json" | sort | uniq -c | sort -rn | while read -r count type; do
        echo "- **$type**: $count resource(s)" >> "$report_file"
    done
    
    echo "" >> "$report_file"
    
    # Deployment recommendation
    cat >> "$report_file" <<EOF
## Deployment Recommendation

EOF

    if [ "$added" -eq 0 ] && [ "$removed" -eq 0 ] && [ "$deployed_suppressions" -eq "$local_suppressions" ]; then
        cat >> "$report_file" <<EOF
### ✅ SAFE TO DEPLOY

The templates are functionally equivalent with only expected environmental differences (tags, naming conventions).

**Next Steps**:
1. Review environmental differences if needed
2. Proceed with deployment
3. Monitor deployment progress

EOF
    else
        cat >> "$report_file" <<EOF
### ⚠️ REVIEW REQUIRED

Templates have differences that require careful review before deployment.

**Required Actions**:
1. Review all added and removed resources
2. Verify changes are intentional
3. Assess impact on running services
4. Get stakeholder approval for significant changes
5. Update documentation if needed
6. Re-run comparison after addressing issues

**Impact Assessment**:
- **Resource Changes**: $added added, $removed removed
- **Security Impact**: $([ "$deployed_suppressions" -eq "$local_suppressions" ] && echo "None" || echo "CDK Nag suppressions changed")
- **Risk Level**: $([ "$removed" -gt 0 ] && echo "HIGH (resources will be removed)" || echo "MEDIUM (resources will be added)")

EOF
    fi
    
    # Footer
    cat >> "$report_file" <<EOF
---

## Artifacts

The following files are available in this directory for detailed analysis:

- \`deployed.json\` - Complete CloudFormation template from deployed stack
- \`local.json\` - Complete CloudFormation template from local synthesis
- \`deployed-resources.txt\` - List of deployed resource logical IDs
- \`local-resources.txt\` - List of local resource logical IDs
- \`deployed-keys.txt\` - Deployed template top-level keys
- \`local-keys.txt\` - Local template top-level keys

## Commands for Further Analysis

### Compare specific resources
\`\`\`bash
jq '.Resources["<ResourceLogicalId>"]' deployed.json
jq '.Resources["<ResourceLogicalId>"]' local.json
\`\`\`

### Find resources by type
\`\`\`bash
jq -r '.Resources | to_entries[] | select(.value.Type == "AWS::Lambda::Function") | .key' local.json
\`\`\`

### Compare IAM policies
\`\`\`bash
jq '.Resources | to_entries[] | select(.value.Type == "AWS::IAM::Role")' deployed.json > deployed-iam.json
jq '.Resources | to_entries[] | select(.value.Type == "AWS::IAM::Role")' local.json > local-iam.json
diff deployed-iam.json local-iam.json
\`\`\`

---

**Report Generated by**: cfn-template-compare skill  
**Timestamp**: $(date -u '+%Y-%m-%dT%H:%M:%SZ')
EOF
    
    log_success "Markdown report generated: $report_file"
}

# Generate summary report
generate_report() {
    local exit_code=$1
    
    echo ""
    echo "=================================="
    echo "CloudFormation Comparison Summary"
    echo "=================================="
    echo ""
    echo "Deployed Stack: $DEPLOYED_STACK"
    echo "Region:         $REGION"
    echo "Profile:        $PROFILE"
    echo "Local Branch:   $CURRENT_BRANCH"
    echo "PPLAWS Ref:     $PPLAWS_REF"
    echo ""
    
    if [ "$exit_code" -eq 0 ]; then
        log_success "Templates are functionally equivalent"
        echo ""
        echo "Recommendation: SAFE TO DEPLOY"
        echo ""
        echo "Note: Environmental differences (tags, naming) are expected."
    else
        log_warning "Templates have differences requiring review"
        echo ""
        echo "Recommendation: REVIEW BEFORE DEPLOYING"
        echo ""
        echo "Actions:"
        echo "  1. Review differences in detail"
        echo "  2. Verify changes are intentional"
        echo "  3. Get stakeholder approval for significant changes"
        echo "  4. Re-run comparison after addressing issues"
    fi
    
    echo ""
    echo "Detailed comparison files preserved in: $WORK_DIR"
    echo "  - comparison-report.md: Comprehensive markdown report"
    echo "  - deployed.json: Deployed template from $DEPLOYED_STACK"
    echo "  - local.json: Local template from branch $CURRENT_BRANCH (ref: $PPLAWS_REF)"
    echo "  - *-resources.txt: Resource lists for comparison"
    echo "  - *-keys.txt: Template structure keys"
    echo ""
    echo "View report: cat $WORK_DIR/comparison-report.md"
}

# Main execution
main() {
    log_info "Starting CloudFormation template comparison"
    
    check_dependencies
    retrieve_deployed_template
    locate_local_template
    
    local has_differences=0
    
    compare_structure || has_differences=1
    compare_resources || has_differences=1
    compare_cdk_nag || has_differences=1
    
    # Generate markdown report
    generate_markdown_report
    
    generate_report "$has_differences"
    
    exit "$has_differences"
}

# Run main function
main "$@"
