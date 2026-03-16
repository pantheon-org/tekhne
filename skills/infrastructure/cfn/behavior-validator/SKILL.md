---
name: cfn-behavior-validator
description: "Creates test stacks, analyzes CloudFormation events, and compares actual vs documented update behavior to validate whether resource property changes trigger replacement or in-place updates. Use when: a user wants to test if a CFN property change causes resource replacement; when investigating stack update behavior or \"Update requires\" documentation accuracy; when validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; when questioning UpdateRequiresReplacement behavior for immutable properties; when empirical evidence is needed before an architectural decision involving CDK or CloudFormation stack updates."
---

# CloudFormation Resource Update Behavior Validator

## Purpose

Empirically validate how CloudFormation handles specific resource property changes by deploying a controlled test stack, making a targeted change, and observing actual CFN events — then deciding whether workarounds are justified.

## Workflow

### 1. Research

- Find the resource's CloudFormation reference page; note **"Update requires"** for the target property.
- Search GitHub (AWS CDK repo), Stack Overflow, and AWS re:Post for community reports of discrepancies.
- State a hypothesis: _"Docs say Replacement — does CFN actually replace the resource?"_

### 2. Design Minimal Test Stack

- Use a non-production, disposable environment.
- Isolate the single property under test; remove unrelated resources.
- Define observable success criteria (e.g. DELETE + CREATE events for the resource type).

```typescript
// Example: minimal CDK stack parameterised via context
export class BehaviorTestStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    // Add only the resource under test, driven by this.node.tryGetContext(...)
  }
}
```

### 3. Execute Test

```bash
# 1. Deploy initial state
cdk deploy --require-approval never

# 2. Record resource ARNs / IDs, confirm any required manual steps (e.g. email confirmation)

# 3. Make the single property change, then redeploy
cdk deploy --require-approval never

# 4. Inspect CFN events — stop and debug if deployment fails before proceeding
aws cloudformation describe-stack-events \
  --stack-name <stack-name> \
  --query 'StackEvents[?ResourceType==`<ResourceType>`].[Timestamp,ResourceStatus,ResourceStatusReason]' \
  --output table
```

**Validation gates:**
- If initial deployment fails → stop and fix before making any changes.
- If events show unexpected behavior → document immediately and abort further changes.
- If behavior is ambiguous → repeat the test to confirm repeatability.

### 4. Document Findings & Decide

```markdown
## CloudFormation Behavior Test Results
- **Date / Region / CDK Version:**
- **Resource Type & Property Changed:**
- **AWS Docs Say:** "Update requires: ..."
- **What Actually Happened:** [UPDATE_IN_PLACE | REPLACEMENT | NO-OP | error]
- **CFN Events:** [paste relevant rows]
- **Matches Docs:** Yes / No
- **Workaround Needed:** Yes / No — Reasoning: ...
- **Code Changes:** [commit/PR link]
```

Update the code: implement or remove the workaround and add a comment citing this test.

## Related Skills

- `cfn-template-compare` — Compare deployed vs local templates
- `aws-cdk` — General AWS CDK development
- `terraform-validator` — Similar testing for Terraform

## Anti-Patterns

### NEVER assume UPDATE_REQUIRES_REPLACEMENT from documentation without testing

- **WHY**: AWS documentation for resource property behavior is sometimes incorrect or lags behind API changes; only testing against an actual stack confirms actual replacement behavior.
- **BAD**: Trust the CloudFormation docs that say a property change is "No interruption" and skip testing.
- **GOOD**: Create a test stack and apply the change to observe actual behavior via stack events.

### NEVER test behavior-changing updates on production stacks

- **WHY**: Replacement validation creates and deletes resources; testing on production risks unintended downtime or data loss.
- **BAD**: Test property change behavior directly on a prod stack to "save time".
- **GOOD**: Use a dedicated test stack in a non-production account with realistic (but non-sensitive) configuration.

### NEVER ignore `UPDATE_ROLLBACK_FAILED` stack status

- **WHY**: A stack in this state cannot be updated or deleted without manual intervention; document the recovery procedure whenever behavior testing triggers a rollback.
- **BAD**: Treat `UPDATE_ROLLBACK_FAILED` as a transient error and retry.
- **GOOD**: Use `continue-update-rollback` with skipped resources, or manually resolve the failed resource before retrying.

### NEVER compare template drift without accounting for CDK synthesizer metadata

- **WHY**: CDK-synthesized templates include `aws:cdk:path` metadata and synthesizer version fields that differ between environments but are not functional differences; filter these before comparing.
- **BAD**: Flag every metadata field difference as drift.
- **GOOD**: Normalize templates by stripping CDK-specific metadata keys before comparison.

## References

| Script | Location | Purpose |
|---|---|---|
| `watch-cfn-events.sh` | `./scripts/watch-cfn-events.sh` | Stream CFN events in real-time during deployment |
| `compare-resources.sh` | `./scripts/compare-resources.sh` | Diff resource properties before and after deployment |

See `EXAMPLES.md` in this skill directory for a full walkthrough of an SNS email subscription endpoint change test.
- [CloudFormation Update Behaviors](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html)
- [CloudFormation Resource Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-template-resource-type-ref.html)
- [AWS CDK Best Practices](https://docs.aws.amazon.com/cdk/v2/guide/best-practices.html)
