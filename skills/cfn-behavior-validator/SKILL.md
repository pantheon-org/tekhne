# CloudFormation Resource Update Behavior Validator

## Purpose

Validate how CloudFormation handles specific resource property changes by testing in a controlled environment. This skill helps determine whether workarounds (like hash patterns in resource IDs) are actually necessary or if CloudFormation's native update behavior is sufficient.

## When to Use This Skill

Use this skill when:

- Questioning whether a workaround pattern is necessary
- CloudFormation update behavior is unclear from documentation
- Need empirical evidence for architectural decisions
- Investigating why a resource property change didn't trigger expected behavior
- Evaluating whether defensive coding patterns are justified

## Common Use Cases

1. **SNS Email Subscription Endpoint Changes**
   - Question: Does changing the email endpoint trigger subscription replacement?
   - Workaround: Hash-based logical ID to force replacement

2. **Lambda Environment Variable Updates**
   - Question: Do env var changes trigger function updates or require replacement?

3. **Security Group Rule Changes**
   - Question: Are rules updated in-place or does group get replaced?

4. **IAM Role Policy Changes**
   - Question: Are inline policies updated without replacing the role?

## Skill Workflow

### Phase 1: Research

1. **Review AWS CloudFormation documentation**
   - Find the resource reference page
   - Check "Update requires" for each property
   - Document what AWS says should happen

2. **Search for community experiences**
   - GitHub issues in AWS CDK repository
   - Stack Overflow questions
   - AWS re:Post forums
   - Blog posts and articles

3. **Create hypothesis**
   - What should happen according to docs?
   - What might actually happen based on community reports?
   - What workaround exists (if any)?

### Phase 2: Design Test

1. **Identify test environment**
   - Use non-production environment
   - Ensure environment is disposable/resettable
   - Consider cost implications of test resources

2. **Create minimal reproduction**
   - Smallest possible stack that demonstrates behavior
   - Remove unrelated resources
   - Focus on single property change

3. **Define success criteria**
   - What observable behavior indicates correct handling?
   - What indicates incorrect handling requiring workaround?
   - How will you measure the outcome?

### Phase 3: Execute Test

1. **Initial deployment**

   ```bash
   # Deploy stack with initial configuration
   cdk deploy --require-approval never
   ```

2. **Verify initial state**
   - Confirm resource exists as expected
   - Document resource ARNs/IDs
   - Take screenshots if relevant

3. **Make the change**
   - Modify only the target property
   - Document the exact change made
   - Commit or note the change for repeatability

4. **Redeploy and observe**

   ```bash
   # Deploy with change
   cdk deploy --require-approval never

   # Watch CloudFormation events
   aws cloudformation describe-stack-events \
     --stack-name <stack-name> \
     --max-items 20
   ```

5. **Verify outcome**
   - Check CloudFormation events for update type
   - Verify resource was updated/replaced correctly
   - Confirm functionality works as expected
   - Note any manual steps required

### Phase 4: Document Findings

1. **Record test results**
   - What CloudFormation did (update/replace/no-op)
   - Whether it matched documentation
   - Any errors or issues encountered
   - Time taken for updates

2. **Make decision**
   - Is workaround necessary?
   - Document reasoning
   - Create decision record

3. **Update code**
   - Implement or remove workaround based on results
   - Add code comments explaining decision
   - Reference test results in comments

## Example: SNS Email Subscription Test

### Research Phase

**Documentation Says:**

- `AWS::SNS::Subscription` Endpoint property
- Update requires: `Replacement`
- Should delete old subscription and create new one

**Hypothesis:**
CloudFormation should automatically handle email changes by replacing the subscription resource.

### Test Design

```typescript
// minimal-sns-test-stack.ts
export class MinimalSnsTestStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const topic = new sns.Topic(this, 'TestTopic', {
      displayName: 'Email Subscription Test',
    });

    new sns.Subscription(this, 'TestSubscription', {
      topic,
      protocol: sns.SubscriptionProtocol.EMAIL,
      endpoint: this.node.tryGetContext('testEmail') || 'initial@example.com',
    });
  }
}
```

**Test Steps:**

1. Deploy with `testEmail=initial@example.com`
2. Confirm subscription via email
3. Change to `testEmail=updated@example.com`
4. Redeploy and observe

**Success Criteria:**

- CloudFormation shows "DELETE" event for old subscription
- CloudFormation shows "CREATE" event for new subscription
- New confirmation email sent to updated address
- After confirmation, emails arrive at new address

### Execution

```bash
# Initial deployment
cdk deploy -c testEmail=initial@example.com

# Confirm subscription (check email and click link)

# Change email and redeploy
cdk deploy -c testEmail=updated@example.com

# Watch events
aws cloudformation describe-stack-events \
  --stack-name MinimalSnsTestStack \
  --query 'StackEvents[?ResourceType==`AWS::SNS::Subscription`].[Timestamp,ResourceStatus,ResourceStatusReason]' \
  --output table
```

### Documentation Template

```markdown
# CloudFormation Behavior Test Results

## Test Information

- **Date:** YYYY-MM-DD
- **Resource Type:** AWS::SNS::Subscription
- **Property Changed:** Endpoint
- **AWS Region:** us-east-1
- **CDK Version:** X.X.X

## Documentation Review

- **AWS Docs Say:** "Update requires: Replacement"
- **Expected Behavior:** Delete old subscription, create new one

## Test Results

- **What Happened:** [Description of actual behavior]
- **CloudFormation Events:** [Relevant events from stack]
- **Matches Documentation:** Yes/No
- **Manual Steps Required:** [Any manual intervention needed]

## Decision

- **Workaround Needed:** Yes/No
- **Reasoning:** [Explanation of decision]
- **Code Changes:** [Link to commit/PR if applicable]

## References

- CloudFormation Events Log: [Link or attachment]
- AWS Console Screenshots: [If applicable]
```

## Scripts

### CloudFormation Event Watcher

Location: `.opencode/skills/cfn-behavior-validator/scripts/watch-cfn-events.sh`

Watches CloudFormation events in real-time during deployments to observe update behavior.

### Resource Comparison Tool

Location: `.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh`

Compares resource properties before and after deployment to verify changes.

## Tips for Effective Testing

1. **Use Unique Stack Names**
   - Append timestamp or identifier to avoid conflicts
   - Makes it easy to identify test stacks for cleanup

2. **Enable CloudTrail**
   - Provides detailed API call logs
   - Helps understand what CloudFormation actually did

3. **Test Multiple Times**
   - Behavior might be inconsistent
   - Confirm results are repeatable

4. **Document Everything**
   - Screenshots of console
   - Copy/paste of relevant logs
   - Timestamp of when tests ran

5. **Clean Up Test Resources**
   - Delete test stacks after completion
   - Avoid accumulating costs

## Common Pitfalls

1. **Testing in Production**
   - Never test CloudFormation behavior in production
   - Always use development/test environments

2. **Incomplete State Changes**
   - Ensure resources reach desired state before testing next change
   - Example: Confirm email subscription before changing endpoint

3. **Overlooking Dependencies**
   - Some updates depend on other resources
   - Test in isolation when possible

4. **Ignoring IAM Permissions**
   - Ensure test environment has proper permissions
   - Permission issues can mask actual CloudFormation behavior

## Related Skills

- `cfn-template-compare` - Compare deployed vs local templates
- `aws-cdk` - General AWS CDK development
- `terraform-validator` - Similar testing for Terraform

## References

- [AWS CloudFormation Update Behaviors](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html)
- [AWS CloudFormation Resource Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-template-resource-type-ref.html)
- [AWS CDK Best Practices](https://docs.aws.amazon.com/cdk/v2/guide/best-practices.html)
