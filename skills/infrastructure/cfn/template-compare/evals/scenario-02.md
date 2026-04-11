# Scenario 02: Pre-flight Checks for Template Comparison

## User Prompt

Before executing a CloudFormation template comparison for the `lct-monitoring-at-stack` in `eu-west-1` region using the `prod-ops` AWS profile, you need to verify that all prerequisites are met. Past comparison attempts have failed due to invalid credentials, non-existent stacks, and missing environment variables needed for CDK synthesis.

Your task is to create a comprehensive pre-flight check script that validates all prerequisites before attempting the actual comparison.

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

## Expected Behavior

1. Use `aws sts get-caller-identity` to verify credentials
2. Include the `--profile` flag on the identity check command
3. Use `aws cloudformation describe-stacks` to confirm stack existence
4. Use a `--query` to extract `StackStatus` from describe-stacks
5. Run `make synth` to verify synthesis works
6. Use `jq` to validate JSON after template retrieval
7. Include error messages or echo statements for failures
8. Explain what to do if credentials are invalid (check AWS_PROFILE, verify profile name) in `error-recovery.md`
9. Explain what to do when `StackNotFoundException` occurs in `error-recovery.md`
10. Explain what to do when synthesis fails (check env vars in `env-local.mk` or `env.mk`) in `error-recovery.md`

## Success Criteria

- **AWS credential check**: preflight-checks.sh uses 'aws sts get-caller-identity' to verify credentials
- **Profile flag usage**: The get-caller-identity command includes --profile flag
- **Stack existence check**: preflight-checks.sh uses 'aws cloudformation describe-stacks' to confirm stack exists
- **StackStatus query**: The describe-stacks command uses --query to extract StackStatus
- **CDK synth validation**: preflight-checks.sh runs 'make synth' to verify synthesis works
- **JSON validation**: preflight-checks.sh uses jq to validate JSON after template retrieval
- **Error messages present**: Script includes error messages or echo statements for failures
- **Credential error fix**: error-recovery.md explains what to do if credentials are invalid (e.g., check AWS_PROFILE, verify profile name)
- **Stack not found fix**: error-recovery.md explains what to do if StackNotFoundException occurs (check stack name and region)
- **Synth failure fix**: error-recovery.md explains what to do if synthesis fails (check env vars in env-local.mk or env.mk)

## Failure Conditions

- Does not use `aws sts get-caller-identity` for credential verification
- The identity check command does not include `--profile`
- Does not use `aws cloudformation describe-stacks` for stack existence check
- `--query StackStatus` is absent from the describe-stacks command
- Does not run `make synth` for synthesis validation
- Does not use `jq` for JSON validation
- Script has no error messages for failures
- `error-recovery.md` does not explain credential failure recovery
- `error-recovery.md` does not explain stack not found recovery
- `error-recovery.md` does not explain synthesis failure recovery
