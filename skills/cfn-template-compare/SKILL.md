---
name: cfn-template-compare
description: Compares deployed CloudFormation templates with locally synthesized CDK templates to detect drift, validate changes, and ensure consistency before deployment. Use when the user wants to compare CDK output with a deployed stack, check for infrastructure drift, run a pre-deployment validation, audit IAM or security changes, investigate a failing deployment, or perform a 'cdk diff'-style review. Triggered by phrases like 'compare templates', 'check for drift', 'cfn drift', 'stack comparison', 'infrastructure drift detection', 'safe to deploy', or 'what changed in my CDK stack'.
---

# CloudFormation Template Comparison Skill

## Quick Start

```bash
# 1. Retrieve the deployed template
aws cloudformation get-template \
  --stack-name <stack-name> \
  --region <region> \
  --profile <profile> \
  --query TemplateBody \
  --output json > deployed.json

# 2. Synthesize the local CDK template
make synth
cp cdk.out/<stack-name>.template.json local.json

# 3. Compare structure
jq 'keys' deployed.json
jq 'keys' local.json

# 4. Compare resource counts
jq '.Resources | length' deployed.json
jq '.Resources | length' local.json

# 5. Find added/removed resource IDs
diff <(jq -r '.Resources | keys[]' deployed.json | sort) \
     <(jq -r '.Resources | keys[]' local.json | sort)

# 6. Deep diff of a specific resource
diff <(jq '.Resources.<ResourceId>' deployed.json) \
     <(jq '.Resources.<ResourceId>' local.json)
```

## Expected Workflow

### Step 1: Preparation — verify prerequisites

```bash
# Check AWS credentials
aws sts get-caller-identity --profile <profile>
# → If this fails: verify AWS_PROFILE or --profile value before proceeding

# Confirm stack exists
aws cloudformation describe-stacks \
  --stack-name <stack-name> --region <region> --profile <profile> \
  --query 'Stacks[0].StackStatus'
# → If StackNotFoundException: check stack name and region

# Confirm CDK project synthesises cleanly
make synth
# → If synth fails: fix missing env vars in env-local.mk / env.mk before proceeding
```

### Step 2: Retrieval

```bash
# Deployed template
aws cloudformation get-template \
  --stack-name <stack-name> --region <region> --profile <profile> \
  --query TemplateBody --output json > deployed.json
# → Validate: jq '.' deployed.json >/dev/null || echo "ERROR: invalid JSON"

# Local template
cp cdk.out/<stack-name>.template.json local.json
# → Validate: jq '.' local.json >/dev/null || echo "ERROR: invalid JSON"
```

### Step 3: Hierarchical Analysis

```bash
# 1. Structure (top-level keys)
diff <(jq 'keys' deployed.json) <(jq 'keys' local.json)

# 2. Resource count
echo "Deployed: $(jq '.Resources | length' deployed.json)"
echo "Local:    $(jq '.Resources | length' local.json)"

# 3. Added / removed resources
comm -3 \
  <(jq -r '.Resources | keys[]' deployed.json | sort) \
  <(jq -r '.Resources | keys[]' local.json | sort)

# 4. Security — CDK Nag suppressions
diff \
  <(jq '[.Resources[].Metadata."cdk_nag" // empty]' deployed.json) \
  <(jq '[.Resources[].Metadata."cdk_nag" // empty]' local.json)

# 5. IAM roles and policies
diff \
  <(jq '[.Resources | to_entries[] | select(.value.Type | startswith("AWS::IAM"))]' deployed.json) \
  <(jq '[.Resources | to_entries[] | select(.value.Type | startswith("AWS::IAM"))]' local.json)
```

### Step 4: Risk Assessment

Categorise each difference before reporting:

| Category | Examples | Action |
|---|---|---|
| **Expected** | Environmental tags, GitRef, stack-name in resource IDs | Auto-approve |
| **Low risk** | Display names, cosmetic metadata | Note and approve |
| **Medium risk** | Alarm thresholds, EventBridge schedules, Lambda config | Review and approve |
| **High risk** | IAM policies, encryption settings | Require explicit sign-off |
| **Critical** | CDK Nag suppressions, public access flags, resource removal | Block — get InfoSec/stakeholder approval |

### Step 5: Save Artifacts

```bash
# Timestamped directory for audit trail
BRANCH=$(git rev-parse --abbrev-ref HEAD)
DIR="cfn-compare-results/$(date +%Y-%m-%d-%H%M%S)_deployed-main_local-${BRANCH}"
mkdir -p "$DIR"

cp deployed.json "$DIR/"
cp local.json    "$DIR/"
jq -r '.Resources | keys[]' deployed.json | sort > "$DIR/deployed-resources.txt"
jq -r '.Resources | keys[]' local.json    | sort > "$DIR/local-resources.txt"

# Write report
cat > "$DIR/comparison-report.md" <<EOF
# CloudFormation Template Comparison

## Summary
- Deployed: <stack-name> ($(jq '.Resources|length' deployed.json) resources)
- Local:    <stack-name> ($(jq '.Resources|length' local.json) resources)
- Status: ✅ Safe to deploy | ⚠️ Review required | ❌ Critical issues

## Differences
<!-- Populate from Step 3 output -->

## Recommendations
<!-- List required actions -->

## Deployment Decision
<!-- Approve | Reject | Conditional — reasoning -->
EOF
```

## Error Recovery

| Error | Cause | Fix |
|---|---|---|
| `Stack not found` | Wrong name/region | Verify `--stack-name`, `--region`, `--profile` |
| `CDK synth failed` | Missing env var | Check `env-local.mk` and `env.mk` |
| `jq: parse error` | Invalid JSON from CLI | Use `--output json` and `--query TemplateBody` |
| `Diff > 5000 lines` | Template too large | Switch to hierarchical comparison (Step 3) instead of line diff |

## Required Tools

- `aws` CLI — configured with appropriate profile
- `jq` — JSON query and transformation
- `make` — CDK synthesis via `make synth`
- `bash` — shell scripting
- `diff` / `comm` — comparison utilities

## Common Scenarios

- **Clean deployment**: identical resource counts, only expected environmental differences → approve
- **Drift detected**: deployed threshold differs from local → revert console change or update CDK
- **New CDK Nag suppression**: requires documented justification and InfoSec approval
- **Resource removal**: block deployment, data-loss risk — review with stakeholders first

## References

- Automated script: `scripts/compare-cfn-templates.sh`
- Real-world examples: `references/compare-cfn-templates.md`
- CI/CD integration: `.gitlab-ci.yml` `validate-template` stage
