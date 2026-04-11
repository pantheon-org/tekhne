# Scenario 03: Execute DynamoDB Table Class Change Validation

## User Prompt

Your cost optimization team wants to change a DynamoDB table from STANDARD to STANDARD_INFREQUENT_ACCESS table class to reduce costs. However, they're concerned this might cause table replacement and data loss. The AWS documentation needs to be validated through actual testing.

You have a test environment with a simple DynamoDB table already defined in CDK. Your task is to document the exact command sequence and event inspection process needed to validate this change safely.

Create a bash script `execute-validation.sh` that contains the complete command sequence for:
1. Deploying the initial state (with STANDARD table class)
2. Recording the table ARN/name after initial deployment
3. Making the change to STANDARD_INFREQUENT_ACCESS
4. Redeploying with the change
5. Inspecting CloudFormation events to determine what happened
6. Extracting the specific events related to the DynamoDB table

Also create a file `event-analysis.md` that explains:
- What specific CloudFormation events indicate replacement
- What events indicate in-place update
- How to extract and interpret the event data
- What to do if the deployment fails

The script should include comments explaining each step and what to look for. Use placeholder stack names like `<test-stack-name>` where appropriate.

## Expected Behavior

1. Include a `cdk deploy` command for the initial state in `execute-validation.sh`
2. Use `--require-approval never` flag on cdk deploy commands
3. Include a step to record or note the table ARN/name after initial deployment
4. Show making the property change and redeploying
5. Include the `aws cloudformation describe-stack-events` command in the script or analysis
6. Use a `--query` filter with `ResourceType` to filter for `AWS::DynamoDB::Table` events
7. Extract at least 2 of: Timestamp, ResourceStatus, ResourceStatusReason from events
8. Explain that DELETE + CREATE events indicate replacement in `event-analysis.md`
9. Explain that UPDATE_IN_PROGRESS/UPDATE_COMPLETE indicate in-place update
10. Mention stopping or debugging if the deployment fails

## Success Criteria

- **Initial deployment command**: execute-validation.sh includes 'cdk deploy' for initial state
- **No approval flag**: execute-validation.sh uses '--require-approval never' flag on cdk deploy commands
- **Resource ARN recording**: execute-validation.sh includes step to record or note the table ARN/name after initial deployment
- **Redeploy after change**: execute-validation.sh shows making the property change and redeploying
- **describe-stack-events command**: execute-validation.sh or event-analysis.md includes 'aws cloudformation describe-stack-events' command
- **Resource type filter**: The describe-stack-events command uses --query with ResourceType filter (e.g., ResourceType==`AWS::DynamoDB::Table`)
- **Event field extraction**: The --query extracts at least 2 of: Timestamp, ResourceStatus, ResourceStatusReason
- **Replacement indicators**: event-analysis.md explains that DELETE + CREATE events indicate replacement
- **Update indicators**: event-analysis.md explains that UPDATE_IN_PROGRESS/UPDATE_COMPLETE indicate in-place update
- **Failure handling**: event-analysis.md or execute-validation.sh mentions stopping/debugging if deployment fails

## Failure Conditions

- No `cdk deploy` command for initial state
- `--require-approval never` flag is absent from deploy commands
- No step to record the table ARN or name after initial deployment
- No redeployment after the property change
- `aws cloudformation describe-stack-events` is not used anywhere
- Events are not filtered by `ResourceType == AWS::DynamoDB::Table`
- Fewer than 2 event fields (Timestamp, ResourceStatus, ResourceStatusReason) are extracted
- `event-analysis.md` does not explain DELETE + CREATE as replacement indicators
- `event-analysis.md` does not explain UPDATE_IN_PROGRESS/UPDATE_COMPLETE as in-place update indicators
- No mention of what to do when deployment fails
