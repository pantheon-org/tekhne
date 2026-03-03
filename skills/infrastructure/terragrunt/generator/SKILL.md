---
name: terragrunt-generator
description: Comprehensive toolkit for generating best-practice Terragrunt configurations (HCL files) following current standards and conventions. Generates terragrunt.hcl files, root configurations, child modules, stacks, and environment setups; configures remote state backends, dependency blocks, include blocks, feature flags, exclude blocks, and errors blocks; supports DRY Terraform patterns, multi-environment layouts (dev/staging/prod), and OpenTofu engine integration. Use when creating new Terragrunt projects or resources, scaffolding multi-environment infrastructure, implementing DRY Terraform wrapper configurations, setting up terragrunt.hcl files with remote state or provider config, managing module dependencies, or building infrastructure modules with Terragrunt stacks.
---

# Terragrunt Generator

## Overview

Generate production-ready Terragrunt configurations following current best practices, naming conventions, and security standards. All generated configurations are automatically validated.

**Terragrunt 2025 Features Supported:**
- [Stacks](https://terragrunt.gruntwork.io/docs/features/stacks/) - Infrastructure blueprints with `terragrunt.stack.hcl` (GA since v0.78.0)
- [Feature Flags](https://terragrunt.gruntwork.io/docs/features/feature-flags/) - Runtime control via `feature` blocks
- [Exclude Blocks](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#exclude) - Fine-grained execution control (replaces deprecated `skip`)
- [Errors Blocks](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#errors) - Advanced error handling (replaces deprecated `retryable_errors`)
- [OpenTofu Engine](https://terragrunt.gruntwork.io/docs/features/engine/) - Alternative IaC engine support

## Root Configuration Naming

> **RECOMMENDED**: Use `root.hcl` instead of `terragrunt.hcl` for root files per [migration guide](https://terragrunt.gruntwork.io/docs/migrate/migrating-from-root-terragrunt-hcl).

| Approach | Root File | Include Syntax |
|----------|-----------|----------------|
| **Modern** | `root.hcl` | `find_in_parent_folders("root.hcl")` |
| **Legacy** | `terragrunt.hcl` | `find_in_parent_folders()` |

## Architecture Patterns

> **CRITICAL:** Before generating ANY configuration, determine the architecture pattern and understand its constraints.

### Pattern Selection

| Pattern | Use When | Root Behavior | Structure |
|---------|----------|---------------|-----------|
| **A: Multi-Env Agnostic** | Multiple environments with shared root | Root reads NO env files; uses static values or `get_env()` | `root.hcl` + `{env}/env.hcl` per environment |
| **B: Single/Env-Aware** | Single environment OR environment detection needed | Root can parse path or read `get_env()` | `root.hcl` with optional `account.hcl`/`region.hcl` |
| **C: Centralized Vars** | Shared environment definitions | Root is agnostic; `env.hcl` reads from `_env/` | `root.hcl` + `_env/{env}.hcl` + `{env}/env.hcl` |

### Pattern A: Multi-Environment Agnostic Root

**Key principle:** `root.hcl` does NOT read `env.hcl`. Child modules read `env.hcl` directly.

```
infrastructure/
├── root.hcl              # Environment-AGNOSTIC
├── dev/env.hcl           # locals { environment = "dev" }
│   └── vpc/terragrunt.hcl
└── prod/env.hcl          # locals { environment = "prod" }
    └── vpc/terragrunt.hcl
```

**Child module pattern:**
```hcl
include "root" { path = find_in_parent_folders("root.hcl") }
locals { env = read_terragrunt_config(find_in_parent_folders("env.hcl")) }
inputs = { name = "${local.env.locals.environment}-vpc" }
```

### Pattern B: Environment-Aware Root

**Key principle:** Root detects environment from path or environment variable.

```hcl
# root.hcl
locals {
  path_parts  = split("/", path_relative_to_include())
  environment = local.path_parts[0]  # OR: get_env("TG_ENVIRONMENT", "dev")
}
```

### Pattern C: Centralized Variables

**Key principle:** Each `env.hcl` reads from centralized `_env/{env}.hcl`.

```hcl
# prod/env.hcl
locals {
  env_vars    = read_terragrunt_config("${get_repo_root()}/_env/prod.hcl")
  environment = local.env_vars.locals.environment
  aws_region  = local.env_vars.locals.aws_region
}
```

**Decision:** Multi-env + shared root → **A** | Single env / env detection → **B** | Centralized vars → **C**

## Core Capabilities

### 1. Generate Root Configuration
Create root-level `root.hcl` or `terragrunt.hcl` with remote state, provider config, and common variables.

**Read before generating:** `assets/templates/root/terragrunt.hcl`
**Patterns:** `references/common-patterns.md` → Root Configuration Patterns

**Key placeholders to replace:**
- `[BUCKET_NAME]`, `[AWS_REGION]`, `[DYNAMODB_TABLE]`
- `[TERRAFORM_VERSION]`, `[PROVIDER_NAME]`, `[PROVIDER_VERSION]`
- `[ENVIRONMENT]`, `[PROJECT_NAME]`

**Root.hcl Design Principles:**
1. **Environment-agnostic by default** — Don't assume env.hcl exists at root level
2. **Use static values for provider/backend region** — Or `get_env()` for runtime config
3. **State key uses `path_relative_to_include()`** — Automatically includes environment path
4. **Provider tags can be static** — Environment-specific tags go in child modules

### 2. Generate Child Module Configuration
Create child modules with dependencies, mock outputs, and proper includes.

**Read before generating:** `assets/templates/child/terragrunt.hcl`
**Patterns:** `references/common-patterns.md` → Child Module Patterns

**Module source options:**
- Local: `"../../modules/vpc"`
- Git: `"git::https://github.com/org/repo.git//path?ref=v1.0.0"`
- Registry: `"tfr:///terraform-aws-modules/vpc/aws?version=5.1.0"`

### 3. Generate Standalone Module
Self-contained modules without root dependency.

**Read before generating:** `assets/templates/module/terragrunt.hcl`

### 4. Generate Multi-Environment Infrastructure
Complete directory structures for dev/staging/prod.

Before generating:
1. Determine architecture pattern (see Architecture Patterns section)
2. Read relevant templates for root and child modules
3. Verify env.hcl placement and access patterns

**Patterns:** `references/common-patterns.md` → Environment-Specific Patterns

**Typical structure (Pattern A):**
```
infrastructure/
├── root.hcl              # Environment-AGNOSTIC root config
├── dev/
│   ├── env.hcl           # Dev environment variables
│   └── vpc/terragrunt.hcl
└── prod/
    ├── env.hcl           # Prod environment variables
    └── vpc/terragrunt.hcl
```

### 5. Generate Terragrunt Stacks (2025)
Infrastructure blueprints using `terragrunt.stack.hcl`.

**Read before generating:** `assets/templates/stack/terragrunt.stack.hcl` and `assets/templates/catalog/terragrunt.hcl`
**Docs:** [Stacks Documentation](https://terragrunt.gruntwork.io/docs/features/stacks/)
**Patterns:** `references/common-patterns.md` → Stacks Patterns

**Commands:**
```bash
terragrunt stack generate    # Generate unit configurations
terragrunt stack run plan    # Plan all units
terragrunt stack run apply   # Apply all units
terragrunt stack output      # Get aggregated outputs
terragrunt stack clean       # Clean generated directories
```

### 6. Generate Feature Flags (2025)
Runtime control without code changes.

**Docs:** [Feature Flags Documentation](https://terragrunt.gruntwork.io/docs/features/feature-flags/)
**Patterns:** `references/common-patterns.md` → Feature Flags Patterns

> **CRITICAL:** Feature flag `default` values MUST be static (boolean, string, number) — they CANNOT reference `local.*` values.

```hcl
# Correct: static default
feature "enable_monitoring" {
  default = false
}

# Incorrect: dynamic reference — FAILS
feature "enable_monitoring" {
  default = local.env.locals.enable_monitoring
}
```

**Usage:**
```bash
terragrunt apply --feature enable_monitoring=true
# or
export TG_FEATURE="enable_monitoring=true"
```

### 7. Generate Exclude Blocks (2025)
Fine-grained execution control (replaces deprecated `skip`).

**Docs:** [Exclude Block Reference](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#exclude)
**Patterns:** `references/common-patterns.md` → Exclude Block Patterns

**Actions:** `"plan"`, `"apply"`, `"destroy"`, `"all"`, `"all_except_output"`

**Production recommendation:** Protect critical resources from accidental destruction:
```hcl
exclude {
  if      = true
  actions = ["destroy"]
  exclude_dependencies = false
}

prevent_destroy = true
```

### 8. Generate Errors Blocks (2025)
Advanced error handling (replaces deprecated `retryable_errors`).

**Docs:** [Errors Block Reference](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#errors)
**Patterns:** `references/common-patterns.md` → Errors Block Patterns

### 9. Generate OpenTofu Engine Configuration (2025)
Use OpenTofu as the IaC engine.

**Docs:** [Engine Documentation](https://terragrunt.gruntwork.io/docs/features/engine/)
**Patterns:** `references/common-patterns.md` → OpenTofu Engine Patterns

### 10. Handling Custom Providers/Modules
When generating configs with custom providers:

1. **Identify** the provider name, source, and version
2. **Search** using WebSearch: `"[provider] terraform provider [version] documentation"`
3. **Or use Context7 MCP** if available for structured docs
4. **Generate** with proper `required_providers` block
5. **Document** authentication requirements in comments

## Generation Workflow

> **CRITICAL:** Follow this workflow for EVERY generation task. Skipping steps leads to validation errors.

### Step 1: Understand Requirements
- What type of configuration? (root, child, standalone, stack)
- Single or multi-environment?
- What dependencies exist between modules?
- What providers/modules will be used?

### Step 2: Determine Architecture Pattern and Complete Checklist

| Scenario | Pattern | Root.hcl Scope |
|----------|---------|----------------|
| Multi-env with shared root | Pattern A | Environment-agnostic |
| Single environment | Pattern B | Environment-aware |
| Centralized env vars | Pattern C | Environment-agnostic |

> **MANDATORY:** Before writing any files, complete and output this checklist to the user.

```
## Architecture Pattern Selection

[x] Identified architecture pattern: Pattern ___ (A/B/C)
[x] Root.hcl scope: [ ] environment-agnostic  OR  [ ] environment-aware
[x] env.hcl location: ___________________
[x] Child modules access env via: ___________________
[x] Verified: No file references a path that doesn't exist from its location
```

### Step 3: Read Required Templates

| Configuration Type | Template to Read |
|-------------------|------------------|
| Root configuration | `assets/templates/root/terragrunt.hcl` |
| Child module | `assets/templates/child/terragrunt.hcl` |
| Standalone module | `assets/templates/module/terragrunt.hcl` |
| Stack file | `assets/templates/stack/terragrunt.stack.hcl` |
| Catalog unit | `assets/templates/catalog/terragrunt.hcl` |

Also read: `references/common-patterns.md` — primary source for all generation patterns.

### Step 4: Generate with Validation

**Generation order for multi-environment projects:**

1. **Generate root.hcl first**
   - [ ] No `read_terragrunt_config(find_in_parent_folders("env.hcl"))` if environment-agnostic
   - [ ] `remote_state` block has `encrypt = true`
   - [ ] `errors` block used (not deprecated `retryable_errors`)

2. **Generate env.hcl files for each environment**
   - [ ] `locals` block contains environment, aws_region, and module-specific vars
   - [ ] No references to files that don't exist at that directory level

3. **Generate child modules — modules with NO dependencies first**
   - [ ] `include` block uses `find_in_parent_folders("root.hcl")`
   - [ ] `read_terragrunt_config(find_in_parent_folders("env.hcl"))` present
   - [ ] `terraform.source` uses valid syntax (tfr:///, git::, or relative path)

4. **Generate dependent modules (RDS, EKS, etc.)**
   - [ ] `dependency` blocks have `mock_outputs`
   - [ ] `mock_outputs_allowed_terraform_commands` includes `["validate", "plan", "destroy"]`
   - [ ] Production modules have `prevent_destroy = true` and/or `exclude` block

5. **Run batch validation after ALL files are generated:**
   ```bash
   terragrunt hcl fmt --check          # Format validation
   terragrunt dag graph                 # Dependency graph validation
   ```
   Invoke `devops-skills:terragrunt-validator` for comprehensive validation.

### Step 5: Fix and Re-Validate
If validation fails:
1. Analyze errors (path resolution, missing variables, syntax errors)
2. Fix issues in the specific file(s)
3. Re-validate the fixed file(s)
4. Repeat until ALL errors are resolved

### Step 6: Present Results
Follow the Presentation Requirements section below.

## Validation Workflow

Every generated configuration MUST be validated.

### Incremental Validation

**After generating root.hcl:**
```bash
cd <infrastructure-directory>
terragrunt hcl fmt --check
```

**After generating each child module:**
```bash
cd <module-directory>
terragrunt hcl fmt --check
# If no dependencies on other modules:
terragrunt hcl validate --inputs
```

### Full Validation

After all files are generated:

1. Invoke `devops-skills:terragrunt-validator` skill
2. If validation fails: analyze errors, fix, and re-validate until all pass
3. If validation succeeds: present configurations with usage instructions

**Skip validation only for:** Partial snippets, documentation examples, or explicit user request.

## Presentation Requirements

After successful validation, present ALL of the following sections.

### 1. Directory Structure Summary
```bash
tree <infrastructure-directory>
```

### 2. Files Generated

```markdown
| File | Purpose |
|------|---------|
| root.hcl | Shared configuration for all child modules (state backend, provider) |
| dev/env.hcl | Development environment variables |
| prod/env.hcl | Production environment variables |
| dev/vpc/terragrunt.hcl | VPC module for development |
| ... | ... |
```

### 3. Usage Instructions

```markdown
## Usage Instructions

### Prerequisites
1. AWS credentials configured (`aws configure` or environment variables)
2. S3 bucket `<BUCKET_NAME>` exists for state storage
3. DynamoDB table `<TABLE_NAME>` exists for state locking

### Commands
cd <INFRASTRUCTURE_DIR>

terragrunt run --all init          # Initialize all modules
cd <ENV>/vpc && terragrunt plan    # Preview a specific module
terragrunt run --all plan          # Preview all changes
terragrunt run --all apply         # Apply changes (requires approval)
terragrunt run --all destroy       # Destroy (use with extreme caution)
```

### 4. Environment-Specific Notes

```markdown
## Environment Notes

### Required Environment Variables
| Variable | Description | Example |
|----------|-------------|---------|
| AWS_PROFILE | AWS CLI profile to use | `my-profile` |
| AWS_REGION | AWS region (or set in provider) | `us-east-1` |

### Prerequisites
- [ ] S3 bucket `<BUCKET_NAME>` must exist before first run
- [ ] DynamoDB table `<TABLE_NAME>` must exist for state locking
- [ ] IAM permissions for Terraform state management

### Production-Specific Protections
| Module | Protection | Description |
|--------|------------|-------------|
| prod/rds | `prevent_destroy = true` | Prevents accidental database deletion |
| prod/rds | `exclude { actions = ["destroy"] }` | Blocks destroy commands |
```

### 5. Next Steps (Optional)
Suggest what the user might want to do next (add more modules, customize configurations, etc.)

## Best Practices

Reference `../devops-skills:terragrunt-validator/references/best_practices.md` for comprehensive guidelines.

**Key principles:**
- Use `include` blocks to inherit root configuration (DRY)
- Always provide mock outputs for dependencies
- Enable state encryption (`encrypt = true`)
- Use `generate` blocks for provider configuration
- Specify bounded version constraints (`~> 5.0`, not `>= 5.0`) for local/Git modules
- Never hardcode credentials or secrets
- Configure retry logic for transient errors

> **Note on Version Constraints with Registry Modules:** When using Terraform Registry modules (e.g., `tfr:///terraform-aws-modules/vpc/aws?version=5.1.0`), they typically define their own `required_providers`. Omit generating `required_providers` in `root.hcl` to avoid conflicts — the module's pinned version provides the constraint.

**Anti-patterns to avoid:**
- Hardcoded account IDs, regions, or environment names
- Missing mock outputs for dependencies
- Duplicated configuration across modules
- Unencrypted state storage
- Missing or loose version constraints (except when using registry modules that define their own)
- Root.hcl trying to read env.hcl that doesn't exist at root level

## Deprecated Attributes

| Deprecated | Replacement | Reference |
|------------|-------------|-----------|
| `skip` | `exclude` block | [Docs](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#exclude) |
| `retryable_errors` | `errors.retry` block | [Docs](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/#errors) |
| `run-all` | `run --all` | [Migration](https://terragrunt.gruntwork.io/docs/migrate/migrating-from-run-all/) |
| `--terragrunt-*` flags | Unprefixed flags | [CLI Reference](https://terragrunt.gruntwork.io/docs/reference/cli-options/) |
| `TERRAGRUNT_*` env vars | `TG_*` env vars | [CLI Reference](https://terragrunt.gruntwork.io/docs/reference/cli-options/) |

## Resources

### Templates — Read Before Generating

| Configuration Type | Template File | When to Read |
|-------------------|---------------|--------------|
| Root configuration | `assets/templates/root/terragrunt.hcl` | Before generating any root.hcl |
| Child module | `assets/templates/child/terragrunt.hcl` | Before generating any child module |
| Standalone module | `assets/templates/module/terragrunt.hcl` | Before generating standalone modules |
| Stack file | `assets/templates/stack/terragrunt.stack.hcl` | Before generating stacks |
| Catalog unit | `assets/templates/catalog/terragrunt.hcl` | Before generating catalog units |

### References

| Reference | Content | When to Read |
|-----------|---------|--------------|
| `references/common-patterns.md` | All generation patterns with examples | Always, before generating |
| `references/troubleshooting.md` | Common issues and fixes | When encountering errors |
| `../devops-skills:terragrunt-validator/references/best_practices.md` | Comprehensive best practices | Always, before generating |

### Official Documentation
- [Terragrunt Docs](https://terragrunt.gruntwork.io/docs/)
- [Configuration Reference](https://terragrunt.gruntwork.io/docs/reference/config-blocks-and-attributes/)
- [CLI Reference](https://terragrunt.gruntwork.io/docs/reference/cli-options/)
- [Stacks](https://terragrunt.gruntwork.io/docs/features/stacks/)
- [Feature Flags](https://terragrunt.gruntwork.io/docs/features/feature-flags/)
- [Engine](https://terragrunt.gruntwork.io/docs/features/engine/)
- [Migration Guides](https://terragrunt.gruntwork.io/docs/migrate/)

## Common Issues

For troubleshooting guidance, see [`references/troubleshooting.md`](references/troubleshooting.md), which covers:

- Root.hcl cannot find env.hcl errors
- Provider conflicts with registry modules
- Feature flag validation errors
- Child module env.hcl resolution issues
