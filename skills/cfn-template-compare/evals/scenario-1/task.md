# Pre-flight Checks for Template Comparison

## Problem Description

Before executing a CloudFormation template comparison for the `lct-monitoring-at-stack` in `eu-west-1` region using the `prod-ops` AWS profile, you need to verify that all prerequisites are met. Past comparison attempts have failed due to invalid credentials, non-existent stacks, and missing environment variables needed for CDK synthesis.

Your task is to create a comprehensive pre-flight check script that validates all prerequisites before attempting the actual comparison.

## Output Specification

Create a bash script `preflight-checks.sh` that:

1. Verifies AWS credentials are valid for the specified profile
2. Confirms the CloudFormation stack exists in the specified region
3. Validates that the local CDK project can synthesize successfully
4. Checks that retrieved templates contain valid JSON
5. Exits with appropriate error messages if any check fails

The script should include:
- Clear success/failure messages for each check
- Specific instructions on how to fix each type of failure
- Exit codes that indicate which check failed
- Comments explaining what each check validates

Also create a file `error-recovery.md` that documents:
- Common prerequisite failures and their solutions
- What to do if AWS credentials are invalid
- What to do if the stack doesn't exist
- What to do if CDK synthesis fails
- How to verify environment variables are properly set
