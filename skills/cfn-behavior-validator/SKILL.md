---
name: cfn-behavior-validator
description: Validate CloudFormation resource update behaviors through repeatable experiments before introducing workarounds; use when update semantics are unclear, replacement behavior is disputed, or architecture decisions depend on real deployment evidence.
---

# CloudFormation Behavior Validator

Navigation hub for testing how CloudFormation handles resource property changes.

## When to Use

- You are unsure whether a property change updates in place or replaces a resource.
- A workaround exists and you need proof before keeping/removing it.
- Documentation and observed behavior appear inconsistent.

## Workflow

1. Review official resource docs and expected update behavior.
2. Create a minimal reproducible stack for one property change.
3. Deploy baseline, then deploy changed version.
4. Observe stack events and verify runtime outcomes.
5. Document decision and apply/remove workaround accordingly.

## Quick Commands

```bash
# Baseline deploy
cdk deploy --require-approval never
```

```bash
# Redeploy after single-property change
cdk deploy --require-approval never
```

```bash
# Inspect recent stack events
aws cloudformation describe-stack-events --stack-name <stack-name> --max-items 30
```

```bash
# Filter for a specific resource type
aws cloudformation describe-stack-events --stack-name <stack-name> \
  --query 'StackEvents[?ResourceType==`AWS::SNS::Subscription`].[Timestamp,ResourceStatus,LogicalResourceId]'
```

## Anti-Patterns

### NEVER test behavior in production stacks

WHY: experiments can trigger destructive replacements.
BAD: validating replacement semantics in live customer stack. GOOD: use isolated disposable environment.

### NEVER change multiple properties in one validation run

WHY: multi-variable changes make causality unclear.
BAD: change endpoint, topic policy, and tags simultaneously. GOOD: change one property per experiment.

### NEVER keep undocumented workarounds

WHY: future maintainers cannot assess why complexity exists.
BAD: hash-based ID workaround with no evidence trail. GOOD: record findings and rationale in decision notes.

### NEVER trust docs alone when behavior is disputed

WHY: implementation edge cases can differ from expectations.
BAD: assume docs are always sufficient. GOOD: verify with controlled deployment evidence.

## Quick Reference

| Topic | Reference |
| --- | --- |
| End-to-end validation workflow | [references/validation-workflow.md](references/validation-workflow.md) |
| Test templates and output format | [references/test-templates.md](references/test-templates.md) |

## References

- [CloudFormation Update Behaviors](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html)
- [CloudFormation Resource Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-template-resource-type-ref.html)
