---
name: cfn-behavior-validator
description: "Creates test stacks, analyzes CloudFormation events, compares actual vs documented update behavior, then confirms whether a resource property change triggers replacement rather than an in-place update. Use when: testing if a CFN property change causes resource replacement; investigating stack update behavior, \"Update requires\" documentation accuracy; validating whether a workaround (e.g. hash-based logical IDs) is actually necessary; questioning UpdateRequiresReplacement behavior for immutable properties; needing empirical evidence before an architectural decision involving CDK/CloudFormation stack updates."
---

# CloudFormation Resource Update Behavior Validator

## Purpose

Empirically validate how CloudFormation handles specific resource property changes by deploying a controlled test stack, making a targeted change, and observing actual CFN events — then deciding whether workarounds are justified.

## Mindset

Documentation is a hypothesis, not a fact: the "Update requires" value is maintained by hand and drifts from actual resource-provider behavior, and it never encodes interaction effects between properties on the same resource. Treat every rating as unverified until real CFN events back it up — never carry a workaround, or its removal, into production on a documentation read alone. A change CloudFormation applies in-place can still be disruptive underneath: a Lambda VPC change is rated "some interruption" but always forces ENI churn that shows up only in the `ELASTIC_NETWORK_INTERFACE` events, never on the Lambda resource's own status line.

**Decision framework:**
1. **State the hypothesis before touching a stack** — a test with no falsifiable prediction is not a test.
2. **Isolate the property.** Every other variable must be held constant, or the result cannot be attributed to the one change under test.
3. **Watch the events, not the exit code.** A successful `cdk deploy` does not tell you whether the result was an in-place update or a replacement; always read the `ResourceStatus` transitions.
4. **Repeat before trusting an ambiguous result.** A handful of resource types are genuinely non-deterministic across runs; one clean result is a data point, not proof — a classic gotcha is treating a single successful in-place update as confirmation when the resource actually replaces on every third deploy.

**When to use this skill:**
- A CDK/CFN property change's actual update behavior needs verification before shipping.
- A workaround (hash-suffixed logical IDs, `UpdateReplacePolicy` overrides, forced replacement) is proposed and its necessity is unconfirmed.
- The team is deciding whether "Update requires: Replacement" for a property is still accurate on the current CloudFormation resource-provider version.

**When NOT to use this skill:**
- The property's replacement behavior is already confirmed by a recent, repeatable test result documented for this exact resource type and CFN provider version — re-run only if the provider version changed.
- The change is trivial and reversible with no state to lose (e.g. adding a tag), where the cost of being wrong does not justify a dedicated test stack.
- Testing would touch a stateful production resource that cannot be safely replaced — route that decision through change management instead of an ad hoc validation stack.

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

- **WHY:** AWS documentation for resource property behavior is sometimes incorrect or lags behind API changes; only testing against an actual stack confirms actual replacement behavior.
- **BAD**: Trust the CloudFormation docs that say a property change is "No interruption" and skip testing.
- **GOOD**: Create a test stack and apply the change to observe actual behavior via stack events.

### NEVER test behavior-changing updates on production stacks

- **WHY:** Replacement validation creates and deletes resources; testing on production risks unintended downtime or data loss.
- **BAD**: Test property change behavior directly on a prod stack to "save time".
- **GOOD**: Use a dedicated test stack in a non-production account with realistic (but non-sensitive) configuration.

### NEVER ignore `UPDATE_ROLLBACK_FAILED` stack status

- **WHY:** A stack in this state cannot be updated or deleted without manual intervention; document the recovery procedure whenever behavior testing triggers a rollback.
- **BAD**: Treat `UPDATE_ROLLBACK_FAILED` as a transient error and retry.
- **GOOD**: Resolve the failed resource, then skip it explicitly so the rollback can complete:

```bash
aws cloudformation continue-update-rollback \
  --stack-name <stack-name> \
  --resources-to-skip <LogicalResourceId>
```

### NEVER compare template drift without accounting for CDK synthesizer metadata

- **WHY:** CDK-synthesized templates include `aws:cdk:path` metadata and synthesizer version fields that differ between environments but are not functional differences; filter these before comparing.
- **BAD**: Flag every metadata field difference as drift.
- **GOOD**: Normalize templates by stripping CDK-specific metadata keys before comparison:

```bash
jq 'del(.Metadata."aws:cdk:path", .Metadata."aws:asset:path")' template.json > template.normalized.json
```

### NEVER run a replacement test on a stateful resource without `DeletionPolicy: Retain`

- **WHY:** replacement deletes the original resource once the new one is `CREATE_COMPLETE`; for RDS, DynamoDB, EFS, or any resource holding real data, that deletion is permanent. Set `DeletionPolicy: Retain` (and `UpdateReplacePolicy: Retain`) before the change, never after.
- **BAD**: Trigger the property change on a stateful test resource with default deletion behavior.
- **GOOD**:

```yaml
MyTestTable:
  Type: AWS::DynamoDB::Table
  DeletionPolicy: Retain
  UpdateReplacePolicy: Retain
  Properties:
    # property under test goes here
```

### NEVER treat a "No interruption" rating as proof of zero operational impact

- **WHY:** the interruption rating describes the _stack update_, not the infrastructure underneath. A Lambda VPC change is rated "some interruption" but always forces ENI churn that briefly leaves the function uninvokable — visible only as separate `ENI` events, never on the Lambda resource's own status line.
- **BAD**: Conclude "no interruption" means safe to deploy at any time.
- **GOOD**: Check the full event stream for auxiliary resources (ENIs, DNS records, target-group registrations) that carry impact the top-level rating hides.

## References

| Script | Location | Purpose |
|---|---|---|
| `watch-cfn-events.sh` | `./scripts/watch-cfn-events.sh` | Stream CFN events in real-time during deployment |
| `compare-resources.sh` | `./scripts/compare-resources.sh` | Diff resource properties before and after deployment |

See `EXAMPLES.md` in this skill directory for a full walkthrough of an SNS email subscription endpoint change test.
- [CloudFormation Update Behaviors](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html)
- [CloudFormation Resource Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-template-resource-type-ref.html)
- [AWS CDK Best Practices](https://docs.aws.amazon.com/cdk/v2/guide/best-practices.html)
