# Scenario 02: Dependency Management Audit

## User Prompt

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
  db_name                = "appdb"
  vpc_security_group_ids = [dependency.sg.outputs.security_group_id]
  db_subnet_group_name   = dependency.vpc.outputs.db_subnet_group_name
}
```

Apply the **Dependency Management Checklist** from the terragrunt-validator skill to this file.

For each of the following checklist items, state whether it passes or fails:

1. Mock outputs: ALL dependency blocks have `mock_outputs` for validation
2. Mock allowed commands: `mock_outputs_allowed_terraform_commands` includes `["validate", "plan", "init"]`
3. Explicit paths: Dependency `config_path` uses relative paths (not absolute)
4. No circular deps: State whether a check is possible from static analysis alone

For each failure, explain the risk and provide the corrected HCL.

## Expected Behavior

1. Identify that the `vpc` dependency block has no `mock_outputs` defined and mark this as FAIL, explaining that `terragrunt plan` will fail if vpc has not yet been applied
2. Identify that neither dependency block specifies `mock_outputs_allowed_terraform_commands` and recommend adding it with `["validate", "plan", "destroy"]` or similar
3. Identify that `config_path = "/infrastructure/prod/vpc"` is an absolute path and recommend using a relative path like `"../vpc"`
4. State that circular dependency detection requires running `terragrunt dag graph` and cannot be definitively confirmed from static file analysis alone
5. Provide corrected HCL for the `vpc` dependency block including `mock_outputs`, `mock_outputs_allowed_terraform_commands`, and a relative `config_path`

## Success Criteria

- **Missing mock_outputs on vpc dependency identified**: Agent identifies that the `vpc` dependency block has no `mock_outputs` defined and marks this as FAIL, explaining that `terragrunt plan` will fail if vpc has not yet been applied
- **Missing mock_outputs_allowed_terraform_commands identified**: Agent identifies that neither dependency block specifies `mock_outputs_allowed_terraform_commands` and recommends adding it with `["validate", "plan", "destroy"]` or similar
- **Absolute config_path on vpc dependency flagged**: Agent identifies that `config_path = "/infrastructure/prod/vpc"` is an absolute path and recommends using a relative path like `"../vpc"`
- **Circular dependency check limitation explained**: Agent states that circular dependency detection requires running `terragrunt dag graph` and cannot be definitively confirmed from static file analysis alone
- **Corrected HCL snippets provided**: Agent provides corrected HCL for the `vpc` dependency block including `mock_outputs`, `mock_outputs_allowed_terraform_commands`, and a relative `config_path`

## Failure Conditions

- Missing `mock_outputs` on the `vpc` dependency is not identified as a FAIL
- Missing `mock_outputs_allowed_terraform_commands` on both dependencies is not flagged
- Absolute `config_path` on the `vpc` dependency is not flagged
- Circular dependency check limitation (requires `terragrunt dag graph`) is not mentioned
- No corrected HCL snippet is provided for the `vpc` dependency block
