# Execute DynamoDB Table Class Change Validation

## Problem Description

Your cost optimization team wants to change a DynamoDB table from STANDARD to STANDARD_INFREQUENT_ACCESS table class to reduce costs. However, they're concerned this might cause table replacement and data loss. The AWS documentation needs to be validated through actual testing.

You have a test environment with a simple DynamoDB table already defined in CDK. Your task is to document the exact command sequence and event inspection process needed to validate this change safely.

## Output Specification

Create a bash script `execute-validation.sh` that contains the complete command sequence for:

1. Deploying the initial state (with STANDARD table class)
2. Recording the table ARN/name after initial deployment
3. Making the change to STANDARD_INFREQUENT_ACCESS
4. Redeploying with the change
5. Inspecting CloudFormation events to determine what happened
6. Extracting the specific events related to the DynamoDB table

The script should include comments explaining each step and what to look for. Use placeholder stack names like `<test-stack-name>` where appropriate.

Also create a file `event-analysis.md` that explains:
- What specific CloudFormation events indicate replacement
- What events indicate in-place update
- How to extract and interpret the event data
- What to do if the deployment fails

The script doesn't need to execute (use echo for AWS commands if you prefer), but should demonstrate knowledge of the correct commands and flags.
