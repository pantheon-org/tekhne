# Scenario 03: Security Checklist Validation

## User Prompt

You are given the following `root.hcl` file:

```hcl
locals {
  aws_region  = "us-east-1"
  aws_account = "123456789012"
  project     = "acme-platform"
}

remote_state {
  backend = "s3"
  config = {
    bucket  = "acme-terraform-state-${local.aws_account}"
    key     = "${path_relative_to_include()}/terraform.tfstate"
    region  = local.aws_region
  }
}

generate "provider" {
  path      = "provider.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
provider "aws" {
  region     = "${local.aws_region}"
  access_key = "AKIAIOSFODNN7EXAMPLE"
  secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
}
EOF
}
```

And the following child module excerpt:

```hcl
variable "db_password" {
  description = "Database password"
  type        = string
}
```

(No `sensitive = true` is set on the variable.)

Apply the **Security Checklist** from the terragrunt-validator skill to these files.

For each of the following checklist items, state whether it passes or fails:

1. State encryption: `remote_state` config has `encrypt = true`
2. State locking: DynamoDB table configured for S3 backend
3. No hardcoded credentials: Search for patterns like "AKIA", "password =", account IDs
4. Sensitive variables: Passwords/keys use `sensitive = true` in variable blocks
5. IAM roles: Provider uses `assume_role` instead of static credentials

For each failure, explain the security risk and provide the corrected HCL.

## Expected Behavior

1. Identify that the `remote_state` config block is missing `encrypt = true` and mark this as FAIL, explaining that S3 server-side encryption will not be enforced
2. Identify that the `remote_state` config has no `dynamodb_table` entry and mark this as FAIL, explaining the risk of concurrent state modifications
3. Identify the `AKIA*` `access_key` and `secret_key` in the provider `generate` block as hardcoded credentials, mark as FAIL (Critical), and recommend using `assume_role` instead
4. Identify the `db_password` variable lacks `sensitive = true` and mark as FAIL, explaining that Terraform will print the value in plan output
5. Provide corrected HCL for at least the `remote_state` block (adding `encrypt` and `dynamodb_table`) and the provider block (replacing static credentials with `assume_role`)

## Success Criteria

- **Missing encrypt=true identified**: Agent identifies that the `remote_state` config block is missing `encrypt = true` and marks this as FAIL, explaining that S3 server-side encryption will not be enforced
- **Missing DynamoDB state locking identified**: Agent identifies that the `remote_state` config has no `dynamodb_table` entry and marks this as FAIL, explaining the risk of concurrent state modifications
- **Hardcoded AWS credentials identified**: Agent identifies the `AKIA*` `access_key` and `secret_key` in the provider `generate` block as hardcoded credentials, marks as FAIL (Critical), and recommends using `assume_role` instead
- **Missing sensitive=true on password variable**: Agent identifies the `db_password` variable lacks `sensitive = true` and marks as FAIL, explaining that Terraform will print the value in plan output
- **Corrected HCL snippets provided for all failures**: Agent provides corrected HCL for at least the `remote_state` block (adding `encrypt` and `dynamodb_table`) and the provider block (replacing static credentials with `assume_role`)

## Failure Conditions

- Missing `encrypt = true` in `remote_state` is not identified as a FAIL
- Missing `dynamodb_table` for state locking is not flagged
- Hardcoded `AKIA*` access key and secret key are not identified as Critical security violations
- Missing `sensitive = true` on `db_password` variable is not flagged
- No corrected HCL is provided for the insecure configurations
