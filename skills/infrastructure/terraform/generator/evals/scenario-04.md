# Scenario 04: Cross-Provider Infrastructure with AWS and Datadog

## User Prompt

A platform team manages their AWS infrastructure with Terraform and wants to add Datadog monitors as code in the same repository. They need a Terraform configuration that provisions:

- An AWS Lambda function (Python 3.12 runtime) triggered by an SQS queue
- A Datadog monitor that alerts when the Lambda's error rate exceeds a threshold

The team is using Datadog provider version 3.x. They have a Datadog API key and app key, and an AWS account. The engineering manager has stressed that the team's security policy forbids committing credentials of any kind to the repository.

The platform team has run into issues before where a new engineer joined, ran `terraform apply` without understanding what would change, and caused an outage. The manager wants clear documentation of the safe workflow for applying changes.

Generate the complete Terraform configuration. Include a README section (can be in a Terraform comment or a separate `.md` file) that documents the safe workflow for applying changes through CI/CD. Provide usage instructions after generating the files.

## Expected Behavior

1. The Datadog provider uses a version constraint (e.g., `~> 3.0`) in the `required_providers` block
2. The AWS provider block does NOT contain `access_key` or `secret_key` attributes with literal string values
3. The Datadog provider block does NOT contain `api_key` or `app_key` as literal string values — they use variables or environment variable references
4. The AWS provider `region` attribute uses `var.aws_region` or a variable reference, NOT a hardcoded string
5. The output includes documentation that shows `terraform plan -out=tfplan` followed by `terraform apply tfplan` as two separate steps
6. The `terraform` block includes `required_version` with both a lower and upper bound
7. Generated files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
8. Variables that hold credentials (`api_key`, `app_key`) have `sensitive = true`
9. All variable declarations include a `description` attribute

## Success Criteria

- **Datadog provider version constraint**: The Datadog provider uses a version constraint (e.g., `~> 3.0`) in the `required_providers` block
- **No hardcoded AWS credentials**: The AWS provider block does NOT contain `access_key` or `secret_key` attributes with literal string values
- **No hardcoded Datadog credentials**: The Datadog provider block does NOT contain `api_key` or `app_key` as literal string values — they use variables or environment variable references
- **AWS region via variable**: The AWS provider `region` attribute uses `var.aws_region` or a variable reference, NOT a hardcoded string
- **plan-out workflow documented**: The output includes documentation that shows `terraform plan -out=tfplan` followed by `terraform apply tfplan` as two separate steps
- **Terraform version constraint present**: The `terraform` block includes `required_version` with both a lower and upper bound
- **File organization correct**: Generated files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
- **Sensitive variables marked**: Variables that hold credentials (`api_key`, `app_key`) have `sensitive = true`
- **Variable descriptions present**: All variable declarations include a `description` attribute

## Failure Conditions

- Datadog provider version constraint is absent from `required_providers`
- AWS credentials (`access_key`, `secret_key`) are hardcoded in the provider block
- Datadog credentials (`api_key`, `app_key`) are hardcoded as literal string values
- AWS region is hardcoded instead of using a variable
- The `terraform plan -out=tfplan` / `terraform apply tfplan` safe workflow is not documented
- Credential variables are not marked with `sensitive = true`
- Required files (`main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`) are absent
