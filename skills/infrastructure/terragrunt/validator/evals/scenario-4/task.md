# Task: Deprecated Syntax Migration Audit

A team is migrating their infrastructure to Terragrunt 0.93+. You are given several files from their existing setup:

**File: Makefile (CI commands)**

```makefile
validate:
    TERRAGRUNT_LOG_LEVEL=debug terragrunt run-all validate

plan:
    terragrunt run-all plan --parallelism 4

apply:
    terragrunt run-all apply
```

**File: infrastructure/root.hcl**

```hcl
locals {
  region = "eu-west-1"
}

remote_state {
  backend = "s3"
  config = {
    bucket         = "my-tf-state"
    key            = "${path_relative_to_include()}/terraform.tfstate"
    region         = local.region
    encrypt        = true
    dynamodb_table = "tf-state-lock"
  }
}

retryable_errors = [
  "(?s).*Error creating.*",
  "(?s).*dial tcp.*"
]
```

**File: infrastructure/dev/app/terragrunt.hcl**

```hcl
skip = true

include "root" {
  path = find_in_parent_folders("root.hcl")
}

terraform {
  source = "../../modules//app"
}
```

## Your Task

Identify all deprecated syntax and commands in the files above that need to be migrated for Terragrunt 0.93+ compatibility.

For each deprecated item:
1. State what is deprecated
2. State the modern replacement
3. Provide the corrected code snippet
4. Explain any behaviour differences between old and new syntax

Focus specifically on:
- Deprecated CLI commands (run-all vs run --all)
- Deprecated environment variables (TERRAGRUNT_* vs TG_*)
- Deprecated configuration attributes (retryable_errors, skip)
