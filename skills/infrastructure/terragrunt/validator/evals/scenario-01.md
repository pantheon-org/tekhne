# Scenario 01: Configuration Pattern Checklist Verification

## User Prompt

You are given the following Terragrunt file structure and contents:

**File: infrastructure/terragrunt.hcl (root)**

```hcl
locals {
  aws_region = "us-east-1"
  project    = "myapp"
}

remote_state {
  backend = "s3"
  config = {
    bucket         = "myapp-terraform-state"
    key            = "${path_relative_to_include()}/terraform.tfstate"
    region         = local.aws_region
    dynamodb_table = "terraform-state-lock"
  }
}
```

**File: infrastructure/dev/vpc/terragrunt.hcl (child module)**

```hcl
include {
  path = find_in_parent_folders()
}

locals {
  env_vars = read_terragrunt_config(find_in_parent_folders("env.hcl"))
}

terraform {
  source = "git::https://github.com/myorg/modules.git//vpc?ref=v1.2.0"
}

inputs = {
  environment = local.env_vars.locals.environment
  vpc_cidr    = "10.0.0.0/16"
}
```

**File: infrastructure/dev/env.hcl**

```hcl
locals {
  environment = "dev"
  aws_region  = "us-east-1"
}
```

Apply the **Configuration Pattern Checklist** from the terragrunt-validator skill to these files.

For each of the following checklist items, state whether it passes or fails and explain why:

1. Include blocks: Child modules use `include "root" { path = find_in_parent_folders("root.hcl") }`
2. Named includes: All include blocks have names (not bare `include {}`)
3. Root file naming: Root config is named `root.hcl` (not `terragrunt.hcl`)
4. Environment configs: Environment-level configs named `env.hcl` (not `terragrunt.hcl`)
5. Common variables: Shared variables accessed correctly via `read_terragrunt_config()`

For each failure, provide the recommended fix.

## Expected Behavior

1. Flag that `infrastructure/terragrunt.hcl` should be named `root.hcl` per Terragrunt 0.93+ conventions and mark this item as FAIL
2. Identify that the child module uses a bare `include {}` without a name label and mark this as FAIL, recommending `include "root" { ... }` syntax
3. Identify that `find_in_parent_folders()` without an argument should be updated to `find_in_parent_folders("root.hcl")` once the root file is renamed
4. Correctly assess that `env.hcl` naming and use of `read_terragrunt_config(find_in_parent_folders("env.hcl"))` passes the checklist
5. Provide specific HCL code snippets showing the corrected include block syntax and the renamed root file reference

## Success Criteria

- **Root file naming violation identified**: Agent flags that `infrastructure/terragrunt.hcl` should be named `root.hcl` per Terragrunt 0.93+ conventions and marks this item as FAIL
- **Bare (unnamed) include block violation identified**: Agent identifies that the child module uses a bare `include {}` without a name label and marks this as FAIL, recommending `include "root" { ... }` syntax
- **find_in_parent_folders without explicit filename flagged**: Agent identifies that `find_in_parent_folders()` without an argument should be updated to `find_in_parent_folders("root.hcl")` once the root file is renamed
- **env.hcl naming and read_terragrunt_config usage assessed as pass**: Agent correctly assesses that `env.hcl` naming and use of `read_terragrunt_config(find_in_parent_folders("env.hcl"))` passes the checklist
- **Fix recommendations provided for each failure**: Agent provides specific HCL code snippets showing the corrected include block syntax and the renamed root file reference

## Failure Conditions

- Root file naming violation (`terragrunt.hcl` vs `root.hcl`) is not identified as a FAIL
- Bare unnamed `include {}` is not flagged as a violation
- `find_in_parent_folders()` without explicit filename argument is not identified
- `env.hcl` naming is incorrectly flagged as a violation
- No HCL code snippet is provided showing the corrected include block
