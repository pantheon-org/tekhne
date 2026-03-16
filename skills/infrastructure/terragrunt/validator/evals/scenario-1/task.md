# Task: Configuration Pattern Checklist Verification

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

## Your Task

Apply the **Configuration Pattern Checklist** from the terragrunt-validator skill to these files.

For each of the following checklist items, state whether it passes or fails and explain why:

1. Include blocks: Child modules use `include "root" { path = find_in_parent_folders("root.hcl") }`
2. Named includes: All include blocks have names (not bare `include {}`)
3. Root file naming: Root config is named `root.hcl` (not `terragrunt.hcl`)
4. Environment configs: Environment-level configs named `env.hcl` (not `terragrunt.hcl`)
5. Common variables: Shared variables accessed correctly via `read_terragrunt_config()`

For each failure, provide the recommended fix.
