# CloudFormation Behavior Validator - Testing Guide

## Quick Start

### Testing SNS Email Subscription Updates

This guide walks through testing whether CloudFormation properly handles SNS email subscription endpoint changes.

## Prerequisites

- AWS CLI configured with appropriate credentials
- Access to non-production AWS environment
- Two email addresses you can access for testing
- CloudFormation stack deployed with SNS email subscription

## Step-by-Step Testing Procedure

### 1. Capture Initial State

Before making any changes, capture the current resource state:

```bash
.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh \
  <your-stack-name> \
  <logical-resource-id>
```

**Example:**

```bash
.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh \
  MyAppStack-Dev \
  AlertEmailSubscription
```

This creates a snapshot in `.context/cfn-resource-snapshots/`

### 2. Start Event Monitoring

In a separate terminal, start watching CloudFormation events:

```bash
.opencode/skills/cfn-behavior-validator/scripts/watch-cfn-events.sh \
  <your-stack-name> \
  5
```

**Example:**

```bash
.opencode/skills/cfn-behavior-validator/scripts/watch-cfn-events.sh \
  MyAppStack-Dev \
  5
```

This will display events in real-time as they occur.

### 3. Make the Change

Update the email address in your configuration using your project's method:

**Option A: CDK Context Parameter**

```bash
cdk deploy --context alertEmail="new-email@example.com"
```

**Option B: Environment Variable**

```bash
export ALERT_EMAIL="new-email@example.com"
cdk deploy
```

**Option C: Update Configuration File**
Edit your stack configuration file (e.g., `cdk.json`, `.env`, or stack properties):

```json
{
  "context": {
    "alertEmail": "new-email@example.com"
  }
}
```

Then deploy using your project's deployment method:

```bash
cdk deploy
# OR
make deploy
# OR
npm run deploy
```

This creates a snapshot in `.context/cfn-resource-snapshots/`

### 2. Start Event Monitoring

In a separate terminal, start watching CloudFormation events:

```bash
.opencode/skills/cfn-behavior-validator/scripts/watch-cfn-events.sh \
  LctMonitoringStack-ST \
  5
```

This will display events in real-time as they occur.

### 3. Make the Change

Update the email address in your configuration:

**Option A: Environment variable**

```bash
export PROCESS_EMAIL="new-email@example.com"
make deploy
```

**Option B: Update env file**
Edit `.env` or relevant configuration file:

```
PROCESS_EMAIL=new-email@example.com
```

Then deploy:

```bash
make deploy
```

### 4. Observe CloudFormation Behavior

Watch the event monitor terminal. Look for:

✅ **Expected (Replacement):**

```
DELETE_IN_PROGRESS    <YourSubscriptionLogicalId>    AWS::SNS::Subscription
DELETE_COMPLETE       <YourSubscriptionLogicalId>    AWS::SNS::Subscription
CREATE_IN_PROGRESS    <YourSubscriptionLogicalId>    AWS::SNS::Subscription
CREATE_COMPLETE       <YourSubscriptionLogicalId>    AWS::SNS::Subscription
```

❌ **Unexpected (No Change):**

```
UPDATE_COMPLETE       <YourSubscriptionLogicalId>    AWS::SNS::Subscription
```

or no events at all for the subscription.

### 5. Verify Resource Changes

After deployment completes, capture the new state:

```bash
.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh \
  <your-stack-name> \
  <logical-resource-id>
```

**Example:**

```bash
.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh \
  MyAppStack-Dev \
  AlertEmailSubscription
```

DELETE_IN_PROGRESS ProcessAlertEmailSubscription AWS::SNS::Subscription
DELETE_COMPLETE ProcessAlertEmailSubscription AWS::SNS::Subscription
CREATE_IN_PROGRESS ProcessAlertEmailSubscription AWS::SNS::Subscription
CREATE_COMPLETE ProcessAlertEmailSubscription AWS::SNS::Subscription

```

❌ **Unexpected (No Change):**

```

UPDATE_COMPLETE ProcessAlertEmailSubscription AWS::SNS::Subscription

````

or no events at all for the subscription.

### 5. Verify Resource Changes

After deployment completes, capture the new state:

```bash
.opencode/skills/cfn-behavior-validator/scripts/compare-resources.sh \
  LctMonitoringStack-ST \
  ProcessAlertEmailSubscription
````

The script will automatically compare with the previous snapshot and show:

- Whether Physical Resource ID changed (indicates replacement)
- Timestamp differences
- Full property diff

### 6. Verify Functional Behavior

**Check email:**

- New email address should receive confirmation request
- Click confirmation link
- Trigger an alert to verify emails arrive at new address

**Check AWS Console:**

- Navigate to SNS → Topics → Select the topic
- Check subscriptions list
- Verify old email is gone and new email is present

### 7. Document Results

Create a test results document:

```bash
mkdir -p .context/cfn-test-results

cat > .context/cfn-test-results/sns-subscription-endpoint-test-$(date +%Y%m%d).md << 'EOF'
# SNS Subscription Endpoint Change Test Results

## Test Details
- Date: $(date +%Y-%m-%d)
- Stack: <your-stack-name>
- Resource: <logical-resource-id>
- Change: Email endpoint from old@example.com to new@example.com

## CloudFormation Behavior
[Describe what happened - replacement/update/no-op]

## Physical Resource ID
- Before: [paste from snapshot]
- After: [paste from snapshot]
- Changed: Yes/No

## Functional Verification
- Confirmation email received: Yes/No
- Old email still receives alerts: Yes/No
- New email receives alerts after confirmation: Yes/No

## Conclusion
[Does the hash pattern provide value or is CloudFormation behavior sufficient?]

## Decision
[Keep hash pattern / Remove hash pattern / Not applicable]

## Artifacts
- Before snapshot: [filename]
- After snapshot: [filename]
- CloudFormation events: [paste key events]
EOF
```

## Interpreting Results

### Scenario A: CloudFormation Replaced the Subscription

**Observations:**

- Physical Resource ID changed
- DELETE and CREATE events in CloudFormation
- New confirmation email sent

**Conclusion:** CloudFormation correctly handles endpoint changes via replacement. **Hash pattern is NOT needed.**

**Action:** Remove hash from subscription IDs to simplify code.

### Scenario B: CloudFormation Updated In-Place

**Observations:**

- Physical Resource ID stayed the same
- UPDATE event (or no event) in CloudFormation
- No new confirmation email

**Conclusion:** CloudFormation doesn't properly handle email changes. **Hash pattern IS needed.**

**Action:** Fix hash implementation to use correct email per subscription.

### Scenario C: CloudFormation Did Nothing

**Observations:**

- No events for the subscription resource
- Physical Resource ID unchanged
- No confirmation email

**Conclusion:** CloudFormation failed to detect the change. **Hash pattern IS needed.**

**Action:** Fix hash implementation to use correct email per subscription.

## Troubleshooting

### "Stack not found" error

- Verify stack name is correct (check CloudFormation console)
- Ensure you're in the correct AWS region (check AWS_REGION or --region flag)
- Check AWS CLI credentials (`aws sts get-caller-identity`)

### Events not appearing

- Increase watch interval (try 10 seconds)
- Check if deployment actually started (`aws cloudformation describe-stacks --stack-name <name>`)
- Verify IAM permissions to describe stack events

### Can't see resource details

- Resource might not exist yet (first deployment)
- Check logical resource ID spelling (case-sensitive)
- Verify resource type is correct in CloudFormation template

## Cleanup

After testing, clean up snapshots:

```bash
rm -rf .context/cfn-resource-snapshots/
rm -rf .context/cfn-test-results/
```

Or keep them for reference and version control them.

## Related Documentation

- [SKILL.md](../SKILL.md) - Full skill documentation
- [AWS CloudFormation Update Behaviors](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-updating-stacks-update-behaviors.html)
- [AWS::SNS::Subscription Reference](https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/aws-resource-sns-subscription.html)
