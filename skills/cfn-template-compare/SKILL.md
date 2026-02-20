---
name: cfn-template-compare
description: Compare deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment.
version: 1.0.0
category: infrastructure
tags:
  - cloudformation
  - cdk
  - aws
  - infrastructure
  - devops
  - drift-detection
  - validation
author: LCT Monitoring Team
created: 2026-02-18
---

# CloudFormation Template Comparison Skill

## Overview

This skill provides a systematic approach to comparing deployed CloudFormation stacks with locally synthesized CDK templates. Use it to detect infrastructure drift, validate changes before deployment, and ensure consistency across environments.

## When to Use This Skill

Invoke this skill when:

- **Pre-deployment validation**: Before pushing infrastructure changes to AWS
- **Drift detection**: Identifying differences between deployed and local state
- **Security audits**: Reviewing changes to IAM policies, encryption, or CDK Nag suppressions
- **Debugging deployments**: Investigating why deployments fail or behave unexpectedly
- **Change review**: Understanding the impact of CDK code modifications
- **Compliance checks**: Ensuring infrastructure adheres to organizational policies

## Skill Capabilities

### 1. Template Retrieval

- Download deployed CloudFormation template from AWS
- Locate and copy synthesized CDK template from local build

### 2. Structural Analysis

- Compare top-level template structure (Parameters, Resources, Outputs)
- Count resources in each template
- Identify added, removed, or modified resources

### 3. Resource Comparison

- Compare specific resource properties
- Analyze tags, naming conventions, and metadata
- Detect changes in critical configurations

### 4. Security Review

- Compare CDK Nag suppressions
- Analyze IAM policy changes
- Review encryption settings and security groups
- Identify public access changes

### 5. Operational Impact

- Compare CloudWatch alarm thresholds
- Analyze EventBridge schedules
- Review Lambda configurations
- Assess database and storage settings

## Usage Patterns

### Pattern 1: Quick Pre-Deployment Check

```
User: "Compare my local template with the deployed ST stack"
Agent:
1. Retrieves deployed template from ppl-sw-st
2. Synthesizes local CDK template
3. Performs quick resource count and structural comparison
4. Reports: "192 resources in both, 3 expected differences (tags), ready to deploy"
```

### Pattern 2: Security-Focused Review

```
User: "Check if my IAM changes are safe to deploy"
Agent:
1. Retrieves both templates
2. Extracts all IAM roles and policies
3. Compares permission boundaries
4. Identifies new CDK Nag suppressions
5. Reports security-relevant changes with risk assessment
```

### Pattern 3: Drift Investigation

```
User: "Why is my deployment failing? Check for drift"
Agent:
1. Retrieves deployed template
2. Synthesizes local template
3. Performs deep diff on all resources
4. Identifies resources with property mismatches
5. Reports likely causes (e.g., manual console changes, environment variables)
```

### Pattern 4: Full Audit

```
User: "Generate a complete comparison report for InfoSec review"
Agent:
1. Retrieves both templates
2. Compares all aspects (structure, resources, security, operations)
3. Categorizes differences (expected, concerning, critical)
4. Generates markdown report with risk assessment
5. Provides deployment recommendation
```

### Pattern 5: Generate Report for Stakeholder Review

```
User: "Generate a comparison report for the deployment review meeting"
Agent:
1. Retrieves both templates
2. Performs comprehensive comparison
3. Generates markdown report with:
   - Resource changes summary
   - Security analysis
   - Risk assessment
   - Deployment recommendation
4. Saves report to timestamped directory
5. Report can be shared with stakeholders or attached to tickets
```

## Expected Workflow

### Step 1: Preparation

- Verify AWS CLI is configured with correct profile
- Ensure CDK project is buildable (`make synth` succeeds)
- Confirm target stack name and region

### Step 2: Retrieval

```bash
# Deployed template
aws cloudformation get-template --stack-name <stack> --region <region> --profile <profile>

# Local template
make synth
# Output: cdk.out/<stack-name>.template.json
```

### Step 3: Analysis

Perform hierarchical comparison:

1. **Structure**: Keys, parameter count, output count
2. **Resources**: Count, additions, removals
3. **Properties**: Configurations, tags, metadata
4. **Security**: IAM, encryption, suppressions
5. **Operations**: Alarms, schedules, handlers

### Step 4: Risk Assessment

Categorize each difference:

- **Expected**: Environmental tags, stack naming, GitRef
- **Low Risk**: Cosmetic (display names), resource IDs
- **Medium Risk**: Alarm thresholds, schedule changes
- **High Risk**: IAM policies, encryption settings
- **Critical**: Security suppressions, public access

### Step 5: Reporting

Generate structured output:

- **Summary**: Overall assessment (safe to deploy / needs review)
- **Differences**: Categorized list with descriptions
- **Recommendations**: Actions required before deployment
- **Risks**: Potential impacts of changes

## Integration Points

### CI/CD Pipeline

```yaml
# .gitlab-ci.yml
validate-template:
  stage: validate
  script:
    - make synth
    - ./scripts/compare-cfn-templates.sh $STACK_NAME $REGION $PROFILE
  only:
    - merge_requests
```

### Makefile Target

```makefile
compare-templates: ## Compare local and deployed templates
	@./scripts/compare-cfn-templates.sh $(STACK_NAME) $(AWS_REGION) $(AWS_PROFILE)
```

### Pre-commit Hook

```bash
#!/bin/bash
# .husky/pre-push
make synth
./scripts/compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st --silent
```

## Key Outputs

### Artifact Organization

Comparison artifacts are preserved in timestamped directories for historical tracking:

**Directory Structure**:

```
cfn-compare-results/
  {date}_{time}_deployed-{branch}_local-{branch}_ref-{pplaws_reference}/
    comparison-report.md      # Comprehensive markdown report
    deployed.json             # Deployed CloudFormation template
    deployed-resources.txt    # List of deployed resource IDs
    deployed-keys.txt         # Deployed template structure
    local.json                # Local synthesized template
    local-resources.txt       # List of local resource IDs
    local-keys.txt           # Local template structure
```

**Naming Convention**:

- **Date/Time**: ISO format (YYYY-MM-DD-HHMMSS) for chronological sorting
- **Deployed Branch**: Assumed to be "main" (production baseline)
- **Local Branch**: Current git branch being compared
- **PPLAWS_REFERENCE**: Environment identifier from env-local.mk

**Example**:

```
2026-02-18-153045_deployed-main_local-feat-CC-990-clean-subscription-fix_ref-thoroc/
```

**Markdown Report** (`comparison-report.md`):

The comparison report provides a comprehensive, human-readable analysis including:

- Executive summary with resource counts
- Template structure comparison
- Added/removed resources with types
- Security analysis (CDK Nag suppressions)
- Resource type breakdown
- Deployment recommendation with risk assessment
- Commands for further analysis

This structure enables:

- Historical tracking of infrastructure changes
- Easy identification of which branch/environment was compared
- Chronological sorting of comparison runs
- Comparison between different branches/environments
- Shareable reports for stakeholder review

### Comparison Report Structure

```markdown
# CloudFormation Template Comparison

## Summary

- Deployed: <stack-name> (<resource-count> resources)
- Local: <stack-name> (<resource-count> resources)
- Status: ✅ Safe to deploy | ⚠️ Review required | ❌ Critical issues

## Differences

### Expected (Environmental)

- Stack naming: <deployed> vs <local>
- Tags: Delete, GitRef, Stack
- Resource IDs: Include stack name in hash

### Functional Changes

- [Resource]: [Property] changed from [old] to [new]
  - Impact: [High|Medium|Low]
  - Category: [Security|Operational|Cosmetic]

### Security Changes

- IAM Policy: [policy-name]
  - Added permissions: [list]
  - Removed permissions: [list]
  - Risk: [Assessment]

## Recommendations

1. [Action required]
2. [Action required]

## Deployment Decision

[Approve|Reject|Conditional] - [Reasoning]
```

## Common Scenarios

### Scenario 1: Clean Deployment

**Input**: Local changes to add SNS DisplayName  
**Output**: "1 property change (cosmetic), safe to deploy"  
**Action**: Approve deployment

### Scenario 2: Drift Detected

**Input**: Manual console change to alarm threshold  
**Output**: "Deployed has threshold=20, local has threshold=30, manual change detected"  
**Action**: Revert console change or update CDK code

### Scenario 3: Security Concern

**Input**: New CDK Nag suppression added  
**Output**: "New suppression for AwsSolutions-IAM5, requires security review"  
**Action**: Document justification, get InfoSec approval

### Scenario 4: Breaking Change

**Input**: Resource removal  
**Output**: "Critical: RDS instance will be deleted, data loss risk"  
**Action**: Block deployment, review with stakeholders

## Error Handling

### Stack Not Found

```
Error: Stack 'lct-monitoring-st' not found in region eu-west-1
Solution: Verify stack name, region, and AWS profile
```

### Synthesis Failure

```
Error: CDK synth failed - missing environment variable
Solution: Check env-local.mk and env.mk configuration
```

### jq Parse Error

```
Error: jq: parse error
Solution: Ensure AWS CLI output is valid JSON, use jq -r '.TemplateBody'
```

### Large Diff Output

```
Warning: Diff output exceeds 5000 lines
Solution: Use hierarchical comparison instead of line-by-line diff
```

## Best Practices

1. **Always Compare Before Deploying**: Make it part of your workflow
2. **Automate in CI/CD**: Catch issues before they reach production
3. **Document Expected Differences**: Maintain a list of environmental variations
4. **Focus on Security**: Never auto-approve IAM, encryption, or suppression changes
5. **Use Scripts**: Automate repetitive comparison tasks
6. **Version Control**: Store comparison results for audit trail
7. **Test Locally**: Synthesize with production-like environment variables

## Related Skills

- **terraform-validator**: Similar functionality for Terraform users
- **gitlab-ci-validator**: Pipeline validation and testing
- **code-reviewer**: General code review including infrastructure

## Resources

- **Scripts**: `scripts/compare-cfn-templates.sh` - Automated comparison tool
- **Examples**: `references/compare-cfn-templates.md` - Real-world usage examples

## Dependencies

### Required Tools

- AWS CLI (configured with appropriate profiles)
- jq (JSON query tool)
- CDK CLI (npx cdk)
- bash (shell scripting)

### Optional Tools

- diff (enhanced with colordiff)
- yq (YAML query, if using YAML templates)
- cfn-lint (template validation)
- cfn_nag (security analysis)

## Success Criteria

This skill is successful when:

- ✅ Detects all functional differences between templates
- ✅ Identifies security-relevant changes
- ✅ Provides clear deployment recommendation
- ✅ Reduces deployment failures by catching issues early
- ✅ Completes comparison in < 30 seconds for typical stacks

## Limitations

- Does not detect runtime configuration drift (use AWS Config)
- Cannot predict deployment failures unrelated to template changes
- Requires valid AWS credentials and network access
- Large templates (>1000 resources) may require performance tuning
- Does not validate against AWS service limits

## Future Enhancements

1. **CloudFormation Change Sets**: Use native AWS change preview
2. **Visual Diff**: Generate HTML report with color-coded changes
3. **Cost Estimation**: Calculate cost impact of changes
4. **Rollback Planning**: Suggest rollback strategy for risky changes
5. **Historical Tracking**: Compare against previous deployed versions
6. **Multi-Region**: Compare across multiple region deployments

---

## Quick Start

```bash
# 1. Load the skill (OpenCode will do this automatically when invoked)
# 2. Run comparison
User: "Compare my local CDK template with deployed lct-monitoring-st stack"

# Agent will:
# - Retrieve deployed template
# - Synthesize local template
# - Perform comprehensive comparison
# - Report findings with recommendations
```

## Example Invocations

```
"Compare templates before I deploy"
"Check if my infrastructure changes are safe"
"What's different between my local CDK and deployed stack?"
"Audit my CloudFormation changes for security issues"
"Why is my CDK deployment failing? Check for drift"
"Generate a template comparison report for the ST environment"
```

---

**Version History**

- 1.0.0 (2026-02-18): Initial skill creation based on LCT Monitoring session
