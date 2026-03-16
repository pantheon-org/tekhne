# Task: Comprehensive Validation Report

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

## Your Task

Produce a complete validation report by working through all four checklists from the terragrunt-validator skill.

For each checklist, mark every item as ✅ pass or ❌ fail with a brief explanation:

**Checklist 1: Configuration Pattern**
**Checklist 2: Dependency Management**
**Checklist 3: Security**
**Checklist 4: DRY Principle**

Then provide:
- A summary table of total pass/fail counts per checklist
- A prioritised list of issues (critical → low) with recommended fixes
- The validation commands to run next (format check, dependency graph, etc.)
