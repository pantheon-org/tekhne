# Terragrunt Troubleshooting Guide

## Root.hcl Cannot Find env.hcl

**Symptom:** `Error: Attempt to get attribute from null value` in root.hcl

**Cause:** Root.hcl is trying to read `env.hcl` via `find_in_parent_folders("env.hcl")`, but env.hcl doesn't exist at the root level.

**Solution:** Make root.hcl environment-agnostic:

```hcl
# DON'T do this in root.hcl for multi-environment setups:
locals {
  env_vars = read_terragrunt_config(find_in_parent_folders("env.hcl"))  # FAILS
}

# DO use static values or get_env():
generate "provider" {
  path      = "provider.tf"
  if_exists = "overwrite_terragrunt"
  contents  = <<EOF
provider "aws" {
  region = "us-east-1"  # Or: get_env("AWS_REGION", "us-east-1")
}
EOF
}
```

## Provider Conflict with Registry Modules

**Symptom:** `Error: Duplicate required providers configuration`

**Solutions:**

1. **Remove conflicting generate block** — Only generate `provider "aws"`, not `required_providers`, when using registry modules:

   ```hcl
   generate "provider" {
     path      = "provider.tf"
     if_exists = "overwrite_terragrunt"
     contents  = <<EOF
   provider "aws" {
     region = "us-east-1"
   }
   EOF
   }
   ```

2. **Use `if_exists = "skip"`** — Skip generation if the file already exists:

   ```hcl
   generate "versions" {
     path      = "versions.tf"
     if_exists = "skip"
     contents  = "..."
   }
   ```

3. **Clear cache** if conflicts persist:

   ```bash
   rm -rf .terragrunt-cache && terragrunt init
   ```

## Feature Flag Validation Errors

**Symptom:** `Unknown variable; There is no variable named "local"` in feature blocks

**Cause:** Feature flag defaults must be static values, not references to locals or variables.

**Solution:** Ensure defaults are static values (see Feature Flags section in SKILL.md).

## Child Module Cannot Find env.hcl

**Symptom:** `Error: Attempt to get attribute from null value` in a child module

**Cause:** Child module is trying to read env.hcl but the file doesn't exist in the environment directory.

**Solution:** Ensure env.hcl exists in the environment directory:

```
dev/
├── env.hcl              # MUST exist
└── vpc/
    └── terragrunt.hcl   # Calls find_in_parent_folders("env.hcl")
```
