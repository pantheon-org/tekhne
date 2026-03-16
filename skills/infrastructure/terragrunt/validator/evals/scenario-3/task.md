# Task: Security Checklist Validation

You are given the following root.hcl file:

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

## Your Task

Apply the **Security Checklist** from the terragrunt-validator skill to these files.

For each of the following checklist items, state whether it passes or fails:

1. State encryption: remote_state config has `encrypt = true`
2. State locking: DynamoDB table configured for S3 backend
3. No hardcoded credentials: Search for patterns like "AKIA", "password =", account IDs
4. Sensitive variables: Passwords/keys use `sensitive = true` in variable blocks
5. IAM roles: Provider uses assume_role instead of static credentials

For each failure, explain the security risk and provide the corrected HCL.
