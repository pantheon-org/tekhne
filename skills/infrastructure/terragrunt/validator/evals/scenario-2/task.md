# Task: Dependency Management Audit

You are given the following Terragrunt configurations for an RDS module that depends on a VPC module:

**File: infrastructure/prod/rds/terragrunt.hcl**

```hcl
include "root" {
  path = find_in_parent_folders("root.hcl")
}

dependency "vpc" {
  config_path = "/infrastructure/prod/vpc"
}

dependency "sg" {
  config_path = "../security-groups"

  mock_outputs = {
    security_group_id = "sg-mock12345"
  }
}

terraform {
  source = "tfr:///terraform-aws-modules/rds/aws?version=6.1.0"
}

inputs = {
  db_name            = "appdb"
  vpc_security_group_ids = [dependency.sg.outputs.security_group_id]
  db_subnet_group_name   = dependency.vpc.outputs.db_subnet_group_name
}
```

## Your Task

Apply the **Dependency Management Checklist** from the terragrunt-validator skill to this file.

For each of the following checklist items, state whether it passes or fails:

1. Mock outputs: ALL dependency blocks have mock_outputs for validation
2. Mock allowed commands: mock_outputs_allowed_terraform_commands includes ["validate", "plan", "init"]
3. Explicit paths: Dependency config_path uses relative paths (not absolute)
4. No circular deps: State whether a check is possible from static analysis alone

For each failure, explain the risk and provide the corrected HCL.
