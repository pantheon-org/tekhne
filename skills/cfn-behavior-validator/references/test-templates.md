# Test Templates

## Scenario Template

```markdown
## Test Information
- Resource Type:
- Property Changed:
- Expected Behavior (docs):
- Environment:

## Execution
1. Baseline deploy
2. Change property
3. Redeploy
4. Capture events

## Results
- Observed Behavior:
- Matches docs: Yes/No
- Workaround needed: Yes/No
```

## Event Capture Command Template

```bash
aws cloudformation describe-stack-events \
  --stack-name <stack-name> \
  --query 'StackEvents[].[Timestamp,LogicalResourceId,ResourceType,ResourceStatus,ResourceStatusReason]' \
  --output table
```

## Decision Record Snippet

```markdown
Decision: [Keep workaround | Remove workaround]
Reason: [one paragraph]
Evidence: [event output location]
```
