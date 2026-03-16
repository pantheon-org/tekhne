# Task: Generate Multi-Environment Infrastructure Structure

A team needs a new multi-environment AWS infrastructure setup for a web platform. Generate all the Terragrunt configuration files for:

**Environments:** `dev` and `prod`
**Modules per environment:** `vpc` and `app` (app depends on vpc)
**Architecture Pattern:** Pattern A (multi-env agnostic root)
**State bucket:** `webplatform-tfstate-{account_id}` (use a placeholder)
**Region:** `ap-southeast-1`

**Dev environment:**
- VPC CIDR: `10.0.0.0/16`
- App instance type: `t3.small`

**Prod environment:**
- VPC CIDR: `10.1.0.0/16`
- App instance type: `m5.large`
- App module should have `prevent_destroy = true`

## Your Task

Generate the complete file tree and contents for all files:

1. `infrastructure/root.hcl`
2. `infrastructure/dev/env.hcl`
3. `infrastructure/dev/vpc/terragrunt.hcl`
4. `infrastructure/dev/app/terragrunt.hcl`
5. `infrastructure/prod/env.hcl`
6. `infrastructure/prod/vpc/terragrunt.hcl`
7. `infrastructure/prod/app/terragrunt.hcl`

Show the directory tree first, then each file's content.
Follow Pattern A: root.hcl must NOT reference env.hcl. Child modules read env.hcl directly.
