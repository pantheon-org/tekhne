# Scenario 05: Comprehensive Validation Report

## User Prompt

You are given the following Terragrunt project structure:

**infrastructure/root.hcl**

```hcl
locals {
  aws_region  = "us-west-2"
  project     = "payments"
}

remote_state {
  backend = "s3"
  config = {
    bucket         = "payments-tfstate"
    key            = "${path_relative_to_include()}/terraform.tfstate"
    region         = local.aws_region
    encrypt        = true
    dynamodb_table = "payments-tfstate-lock"
  }
}

generate "provider" {
  path      = "provider.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
provider "aws" {
  region = "${local.aws_region}"
  assume_role { role_arn = "arn:aws:iam::111122223333:role/TerraformRole" }
}
EOF
}

terraform_version_constraint  = ">= 1.6.0"
terragrunt_version_constraint = ">= 0.50.0"
```

**infrastructure/prod/env.hcl**

```hcl
locals {
  environment  = "prod"
  aws_region   = "us-west-2"
  instance_type = "m5.xlarge"
}
```

**infrastructure/prod/rds/terragrunt.hcl**

```hcl
include "root" {
  path = find_in_parent_folders("root.hcl")
}

locals {
  env = read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

dependency "vpc" {
  config_path = "../vpc"
  mock_outputs = {
    vpc_id             = "vpc-mock"
    private_subnet_ids = ["subnet-mock-1", "subnet-mock-2"]
  }
  mock_outputs_allowed_terraform_commands = ["validate", "plan", "destroy"]
}

terraform {
  source = "tfr:///terraform-aws-modules/rds/aws?version=6.1.0"
}

inputs = {
  identifier = "${local.env.locals.environment}-database"
  instance_class = local.env.locals.instance_type
  vpc_security_group_ids = []
  db_subnet_group_name   = dependency.vpc.outputs.db_subnet_group_name
}
```

Produce a complete validation report by working through all four checklists from the terragrunt-validator skill.

For each checklist, mark every item as pass or fail with a brief explanation:

- **Checklist 1: Configuration Pattern**
- **Checklist 2: Dependency Management**
- **Checklist 3: Security**
- **Checklist 4: DRY Principle**

Then provide:
- A summary table of total pass/fail counts per checklist
- A prioritised list of issues (critical to low) with recommended fixes
- The validation commands to run next (format check, dependency graph, etc.)

## Expected Behavior

1. Complete all five Configuration Pattern items with correct assessments: named include (pass), root file naming (pass — root.hcl), env.hcl naming (pass), `find_in_parent_folders` with explicit filename (pass), `read_terragrunt_config` usage (pass)
2. Complete all four Dependency Management items with correct assessments: `mock_outputs` present (pass), `mock_outputs_allowed_terraform_commands` present (pass), relative `config_path` (pass), circular dependency check requires `terragrunt dag graph` (noted)
3. Complete the Security Checklist with correct findings: `encrypt = true` present (pass), DynamoDB locking present (pass), no hardcoded credentials (pass — uses `assume_role`), no sensitive variable issues (pass), version constraints present (pass)
4. Complete the DRY Principle Checklist with correct assessments: `generate` block present (pass), version constraints present (pass), shared locals in `env.hcl` (pass)
5. Include a summary table showing pass/fail counts for each of the four checklists
6. List at least `terragrunt hcl fmt --check`, `terragrunt dag graph`, and `terragrunt run --all validate` as the next validation commands to run

## Success Criteria

- **Configuration Pattern Checklist completed**: Agent assesses all five Configuration Pattern items: named include (pass), root file naming (pass), env.hcl naming (pass), `find_in_parent_folders` with explicit filename (pass), `read_terragrunt_config` usage (pass)
- **Dependency Management Checklist completed**: Agent assesses all four Dependency Management items: `mock_outputs` present (pass), `mock_outputs_allowed_terraform_commands` present (pass), relative `config_path` (pass), circular dependency check requires `dag graph` (noted)
- **Security Checklist completed with correct findings**: Agent correctly identifies `encrypt = true` present (pass), DynamoDB locking present (pass), no hardcoded credentials (pass — uses `assume_role`), no obvious sensitive variable issues (pass), and version constraints present (pass)
- **DRY Principle Checklist completed**: Agent assesses the `generate` block (pass), version constraints (pass), and shared locals in `env.hcl` (pass)
- **Summary table present**: Report includes a summary table showing pass/fail counts for each of the four checklists
- **Validation commands listed**: Report lists at least `terragrunt hcl fmt --check`, `terragrunt dag graph`, and `terragrunt run --all validate` as the next commands to run

## Failure Conditions

- Any Configuration Pattern checklist item is incorrectly assessed as fail when the configuration is compliant
- Any Dependency Management checklist item is incorrectly assessed as fail when the configuration is compliant
- Security Checklist incorrectly flags `assume_role` usage as a hardcoded credential violation
- DRY Principle Checklist is omitted or incomplete
- No summary table is provided for pass/fail counts per checklist
- Validation commands (`hcl fmt --check`, `dag graph`, `run --all validate`) are not listed
