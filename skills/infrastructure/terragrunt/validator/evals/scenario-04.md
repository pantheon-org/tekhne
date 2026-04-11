# Scenario 04: Deprecated Syntax Migration Audit

## User Prompt

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

## Expected Behavior

1. Identify all three `run-all` usages in the Makefile as deprecated and provide the `run --all` replacement commands for each target
2. Identify `TERRAGRUNT_LOG_LEVEL` as a deprecated environment variable and state `TG_LOG=debug` (or `--log-level`) as the replacement
3. Identify `retryable_errors` as deprecated in `root.hcl` and provide an equivalent `errors { retry { ... } }` block as the 0.93+ replacement
4. Identify `skip = true` in `dev/app/terragrunt.hcl` as deprecated and provide an equivalent `exclude { if = true; actions = ["all"] }` block
5. Explain at least one behaviour difference, such as the `exclude` block supporting fine-grained action targeting (e.g., only blocking `destroy`) rather than the binary `skip` flag

## Success Criteria

- **run-all → run --all migration identified**: Agent identifies all three `run-all` usages in the Makefile as deprecated and provides the `run --all` replacement commands
- **TERRAGRUNT_LOG_LEVEL → TG_LOG migration identified**: Agent identifies `TERRAGRUNT_LOG_LEVEL` as a deprecated environment variable and states `TG_LOG=debug` (or `--log-level`) as the replacement
- **retryable_errors → errors block migration identified**: Agent identifies `retryable_errors` as deprecated and provides an equivalent `errors { retry { ... } }` block as the 0.93+ replacement
- **skip → exclude block migration identified**: Agent identifies `skip = true` as deprecated and provides an equivalent `exclude { if = true; actions = ["all"] }` block
- **Behaviour differences explained**: Agent explains at least one behaviour difference, such as the `exclude` block supporting fine-grained action targeting (e.g., only blocking `destroy`) rather than the binary `skip` flag

## Failure Conditions

- Not all three `run-all` usages in the Makefile are identified as deprecated
- `TERRAGRUNT_LOG_LEVEL` is not flagged as a deprecated environment variable
- `retryable_errors` is not identified as deprecated or no `errors { retry { ... } }` block is provided
- `skip = true` is not flagged as deprecated or no equivalent `exclude` block is provided
- No behaviour difference is explained between `skip` and `exclude`
