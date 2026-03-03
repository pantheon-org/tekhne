# CloudFormation Template Comparison - Examples

This document provides practical examples of using the CloudFormation template comparison skill.

## Example 1: Basic Pre-Deployment Check

### Scenario

Developer wants to verify changes before deploying to ST environment.

### User Request

```
"Compare my local template with the deployed lct-monitoring-st stack"
```

### Agent Response

```
I'll compare your local CDK template with the deployed stack in ppl-sw-st.

[Retrieves deployed template from AWS]
[Synthesizes local template using make synth]
[Performs comparison]

Comparison Results:
- Both templates have 192 resources
- Resource counts match exactly
- Found 3 expected environmental differences:
  1. Stack naming: lct-monitoring-st vs lct-monitoring-st-thoroc
  2. Delete tag: 'no' (deployed) vs 'daily' (local dev)
  3. GitRef: Full hash vs short hash

Status: ✅ SAFE TO DEPLOY

All differences are expected environmental variations.
No functional or security changes detected.
```

---

## Example 2: Security Review

### Scenario

Developer added new CDK Nag suppression and wants security validation.

### User Request

```
"I added a CDK Nag suppression for IAM5. Check if this is safe."
```

### Agent Response

```
I'll review the CDK Nag suppression changes for security concerns.

[Compares CDK Nag metadata between templates]

Security Analysis:

NEW SUPPRESSION DETECTED:
- Resource: BackupLambdaRole
- Rule: AwsSolutions-IAM5 (wildcards in IAM policies)
- Reason provided: "Wildcard required for S3 prefix operations"

Evaluation:
⚠️ REQUIRES REVIEW

The suppression allows wildcard permissions. I've verified:
1. The policy is scoped to a specific S3 bucket
2. Actions are limited to s3:GetObject and s3:PutObject
3. Resource includes bucket prefix: arn:aws:s3:::backup-bucket/logs/*

Recommendation:
- Document this suppression in docs/security/cdk-nag-suppressions.md
- Get InfoSec team approval before deployment
- Consider using more specific resource paths if possible

Additional context:
- This follows the pattern used in LoggingLambdaRole (approved 2025-12-15)
- IAM policy limits blast radius to single bucket
- No privilege escalation risk
```

---

## Example 3: Drift Detection

### Scenario

Deployment is failing with "resource already exists" error. Suspect manual console changes.

### User Request

```
"My deployment keeps failing. Check for drift between deployed and local."
```

### Agent Response

```
I'll check for infrastructure drift that might be causing deployment failures.

[Retrieves both templates and performs deep comparison]

DRIFT DETECTED:

1. Alarm Threshold Mismatch:
   Resource: FreeStorageSpaceBelow25PercentAlarm
   - Deployed: Threshold=20
   - Local: Threshold=25

   Cause: Manual change in CloudWatch console on 2026-02-15
   Impact: Deployment will attempt to update alarm, may cause flapping

2. Security Group Rule Addition:
   Resource: BeehiveAccessSecurityGroup
   - Deployed: Has ingress rule from 10.0.0.0/8 (not in local template)
   - Local: Only has rule from VPC CIDR

   Cause: Manual security group modification
   Impact: Deployment will remove this rule (potential connectivity issue)

Status: ❌ CRITICAL DRIFT DETECTED

Recommendations:
1. Revert manual alarm change or update local CDK code to match
2. Document the 10.0.0.0/8 ingress rule:
   - If intentional: Add to CDK code
   - If temporary: Remove from console before deployment
3. Consider using drift detection in CI/CD to prevent this

Next steps:
- Option A: Import console changes into CDK code
- Option B: Revert console changes and deploy from CDK
```

---

## Example 4: Large-Scale Changes

### Scenario

Refactoring CDK code, want to ensure no unintended infrastructure changes.

### User Request

```
"I refactored the Lambda construct. Verify that only naming changed, not functionality."
```

### Agent Response

```
I'll validate that your refactoring is infrastructure-neutral.

[Compares templates with focus on Lambda resources]

Lambda Function Comparison:

TicketBaseAnalyticsLambda:
- Handler: IDENTICAL (app.src.ticket_base_analytics_handler.handler)
- Runtime: IDENTICAL (python3.13)
- Timeout: IDENTICAL (300 seconds)
- Memory: IDENTICAL (512 MB)
- Environment Variables: IDENTICAL (12 variables)
- IAM Role Policies: IDENTICAL (3 policies)
- VPC Configuration: IDENTICAL (subnets, security groups)
- ✅ Logical ID changed (expected from refactoring)

TicketCapacityMonitoringLambda:
- Handler: IDENTICAL
- Runtime: IDENTICAL (python3.13)
- Timeout: IDENTICAL (180 seconds)
- Memory: IDENTICAL (256 MB)
- Environment Variables: IDENTICAL (8 variables)
- IAM Role Policies: IDENTICAL (2 policies)
- ✅ Logical ID changed (expected from refactoring)

EventBridge Rules:
- Schedules: IDENTICAL (both still cron(0 10 ? * MON-FRI *))
- Targets: IDENTICAL (same Lambda ARNs)
- ✅ Logical IDs changed (expected from refactoring)

Status: ✅ REFACTORING VALIDATED

All functional properties are identical. Only logical IDs changed,
which is expected when refactoring construct code. CloudFormation
will update references but won't replace resources.

Deployment Impact: Zero downtime, in-place updates only.
```

---

## Example 5: Multi-Resource Security Audit

### Scenario

InfoSec team requested comprehensive security comparison before production deployment.

### User Request

```
"Generate a complete security audit comparing local and deployed templates."
```

### Agent Response

```
I'll perform a comprehensive security audit of all template changes.

[Analyzes IAM, encryption, network, and compliance configurations]

SECURITY AUDIT REPORT
Generated: 2026-02-18 15:47:11
Scope: lct-monitoring-st (deployed) vs local template

=== IAM POLICIES ===

No changes to IAM policies detected.
- All Lambda execution roles: UNCHANGED
- All IAM policies: UNCHANGED
- Cross-account trust relationships: UNCHANGED

✅ IAM: No security-relevant changes

=== ENCRYPTION ===

SNS Topic (lctMonitoringTopic064E4467):
- Deployed: KmsMasterKeyId=null (no encryption)
- Local: KmsMasterKeyId=null (no encryption)
- Status: ⚠️ INCONSISTENT DOCUMENTATION

CDK Nag Suppressions:
- Deployed: Suppresses SNS2 + SNS3 (reason: "not required")
- Local: Suppresses only SNS3 (reason: "auto-applied with KMS")

Issue: Both templates lack encryption, but suppression reasoning differs.

Recommendation: Clarify encryption policy:
- If encryption required: Remove suppressions, enable KMS
- If not required: Standardize suppression reasoning

RDS Encryption:
- Deployed: StorageEncrypted=true, KmsKeyId=<managed-key>
- Local: StorageEncrypted=true, KmsKeyId=<managed-key>
✅ RDS: No changes

=== NETWORK SECURITY ===

Security Groups:
- All rules: UNCHANGED
- No new ingress/egress rules
- No changes to port ranges
- CIDR blocks: UNCHANGED

✅ Network: No security-relevant changes

=== COMPLIANCE ===

CDK Nag Suppressions Summary:
- Total suppressions (deployed): 47
- Total suppressions (local): 46
- New suppressions: 0
- Removed suppressions: 1 (SNS2 on lctMonitoringTopic)

⚠️ Suppression change requires documentation

CloudWatch Logs:
- Retention: UNCHANGED (30 days)
- Encryption: UNCHANGED (AWS managed)
✅ Logging: Compliant

=== SUMMARY ===

Security Status: ⚠️ REVIEW REQUIRED

Critical Issues: 0
High Priority: 1 (SNS encryption policy)
Medium Priority: 0
Low Priority: 0

Deployment Recommendation: CONDITIONAL APPROVAL

Conditions for deployment:
1. Document SNS encryption policy decision in ADR
2. Standardize CDK Nag suppression reasoning
3. Get InfoSec sign-off on SNS policy

No blocking security issues detected. Infrastructure changes
are minimal and non-security-related (display name addition).
```

---

## Example 6: Quick Status Check

### Scenario

In the middle of development, quick check if current state is deployable.

### User Request

```
"Quick check - can I deploy right now?"
```

### Agent Response

```
I'll do a quick comparison to check deployment readiness.

[Performs fast structural comparison]

Quick Status Check:

✅ Template synthesizes successfully
✅ Resource count matches (192 resources)
✅ No resources added or removed
✅ No IAM policy changes
✅ No encryption setting changes
✅ Expected environmental differences only

Status: ✅ READY TO DEPLOY

Your current code is safe to deploy. All differences are
expected environmental variations (naming, tags).

To deploy:
  make deploy

Need detailed comparison?
  make compare-templates
```

---

## Example 7: Alarm Threshold Validation

### Scenario

Ops team requested lowering alarm thresholds. Verify only alarm changes, nothing else.

### User Request

```
"I changed alarm thresholds to be more sensitive. Verify only alarms changed."
```

### Agent Response

```
I'll verify that only CloudWatch alarm configurations changed.

[Extracts and compares all alarm resources]

CloudWatch Alarm Changes:

FreeStorageSpaceBelow30PercentAlarm:
- Deployed: Threshold=30
- Local: Threshold=35
- Change: +5 percentage points (LESS sensitive)

FreeStorageSpaceBelow25PercentAlarm:
- Deployed: Threshold=25
- Local: Threshold=30
- Change: +5 percentage points (LESS sensitive)

FreeStorageSpaceBelow15PercentAlarm:
- Deployed: Threshold=15
- Local: Threshold=20
- Change: +5 percentage points (LESS sensitive)

⚠️ ALERT: Thresholds were RAISED, not lowered

Other Changes:
- Alarm evaluation periods: UNCHANGED
- Alarm comparison operators: UNCHANGED
- SNS notification targets: UNCHANGED
- Alarm descriptions: UNCHANGED

Non-Alarm Resources:
✅ Lambda functions: UNCHANGED
✅ EventBridge rules: UNCHANGED
✅ IAM roles: UNCHANGED
✅ Security groups: UNCHANGED
✅ Database: UNCHANGED

Status: ⚠️ THRESHOLD DIRECTION MISMATCH

The alarm thresholds were raised (less sensitive), not lowered
(more sensitive) as intended. This is opposite of what was requested.

Recommendation: Review alarm changes before deployment.
- Current: Alarms trigger later (at worse conditions)
- Intended: Alarms trigger earlier (at better conditions)

Correct threshold values for "more sensitive":
- 30% alarm: Lower to 25%
- 25% alarm: Lower to 20%
- 15% alarm: Lower to 10%
```

---

## Example 8: Resource Naming Validation

### Scenario

Ensuring naming conventions are followed across all resources.

### User Request

```
"Check if all resources follow our naming convention: <service>-<env>-<resource-type>"
```

### Agent Response

```
I'll validate resource naming conventions across the template.

[Analyzes resource names and identifiers]

Resource Naming Audit:

SNS Topics:
✅ lct-monitoring-topic-st-thoroc (follows convention)

Lambda Functions:
✅ ticket-base-analytics-lambda-st-thoroc
✅ ticket-capacity-monitoring-lambda-st-thoroc

CloudWatch Alarms:
✅ lct-monitoring-st-thoroc-free-storage-below-30-percent
✅ lct-monitoring-st-thoroc-free-storage-below-25-percent
✅ lct-monitoring-st-thoroc-free-storage-below-15-percent

EventBridge Rules:
✅ ticket-base-analytics-daily-rule-st-thoroc
✅ ticket-capacity-monitoring-daily-rule-st-thoroc

Security Groups:
✅ beehive-access-sg-st-thoroc
✅ beehive-db-sg-st-thoroc

IAM Roles:
✅ ticket-base-analytics-lambda-role-st-thoroc
✅ ticket-capacity-monitoring-lambda-role-st-thoroc

Status: ✅ ALL RESOURCES COMPLY

All 192 resources follow the naming convention:
  <service>-<resource>-<env>-<reference>

Convention compliance: 100%

Note: Logical IDs (internal to CloudFormation) may differ
from physical resource names, which is expected and acceptable.
```

---

## Common Commands

```bash
# Basic comparison
./scripts/compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st

# Specify local template explicitly
./scripts/compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st cdk.out/my-stack.template.json

# Compare production stack
./scripts/compare-cfn-templates.sh lct-monitoring-pr eu-west-1 ppl-pr-prod

# Silent mode (exit codes only)
./scripts/compare-cfn-templates.sh lct-monitoring-st eu-west-1 ppl-sw-st --silent
```

## Integration with Makefile

```makefile
compare-templates: ## Compare local and deployed templates
	@./scripts/compare-cfn-templates.sh $(LCT_MONITORING_STACK_NAME) $(AWS_REGION) $(AWS_PROFILE)

compare-pr: ## Compare with production stack
	@./scripts/compare-cfn-templates.sh lct-monitoring-pr eu-west-1 ppl-pr-prod
```

Usage:

```bash
make compare-templates  # Compare with current environment
make compare-pr         # Compare with production
```

---

## Tips for Effective Comparisons

1. **Always synthesize first**: Run `make synth` before comparing
2. **Focus on intent**: Describe what you changed, agent will verify
3. **Request specific checks**: "Check IAM changes" vs general "compare templates"
4. **Use for pre-commit**: Compare before committing infrastructure changes
5. **Automate in CI/CD**: Add comparison step to pipeline
6. **Document differences**: Maintain list of expected environmental variations
7. **Review security changes**: Never skip review of IAM, encryption, or suppressions

---

## Troubleshooting

### "Stack not found"

- Verify stack name: `aws cloudformation list-stacks --region eu-west-1 --profile ppl-sw-st`
- Check region and profile settings

### "No template found in cdk.out"

- Run `make synth` first
- Check for CDK errors in synthesis

### "jq: parse error"

- Ensure AWS CLI returns valid JSON
- Check AWS CLI version (should be >= 2.0)

### "Multiple templates found"

- Specify template explicitly: `./scripts/compare-cfn-templates.sh stack region profile cdk.out/specific.template.json`
- Clean cdk.out: `rm -rf cdk.out && make synth`
