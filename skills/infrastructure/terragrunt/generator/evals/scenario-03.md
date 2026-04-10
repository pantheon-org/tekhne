# Scenario 03: Generate Multi-Environment Infrastructure Structure

## User Prompt

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

## Expected Behavior

1. Present the complete directory tree (`infrastructure/` with `root.hcl`, `dev/`, `prod/` subdirectories) before showing file contents
2. Generate `root.hcl` that does not reference `env.hcl`, does not hard-code `dev` or `prod`, and uses `path_relative_to_include()` for the state key
3. `dev/env.hcl` has `environment=dev`, `vpc_cidr=10.0.0.0/16`, `instance_type=t3.small`; `prod/env.hcl` has `environment=prod`, `vpc_cidr=10.1.0.0/16`, `instance_type=m5.large`
4. All four child `terragrunt.hcl` files include a `locals` block reading `env.hcl` and use `local.env.locals.*` for environment-specific values
5. Both `app/terragrunt.hcl` files (dev and prod) have a dependency on `../vpc` with at least one `mock_output` and `mock_outputs_allowed_terraform_commands`
6. `infrastructure/prod/app/terragrunt.hcl` includes `prevent_destroy = true` (dev/app does not require it)

## Success Criteria

- **Directory tree shown before files**: Agent presents the complete directory tree (`infrastructure/` with `root.hcl`, `dev/`, `prod/` subdirectories) before showing file contents
- **root.hcl is environment-agnostic**: `root.hcl` does not reference `env.hcl`, does not hard-code `dev` or `prod`, and uses `path_relative_to_include()` for the state key
- **env.hcl files contain environment-specific values**: `dev/env.hcl` has `environment=dev`, `vpc_cidr=10.0.0.0/16`, `instance_type=t3.small`; `prod/env.hcl` has `environment=prod`, `vpc_cidr=10.1.0.0/16`, `instance_type=m5.large`
- **Child modules read env.hcl via read_terragrunt_config**: All four child `terragrunt.hcl` files include a `locals` block reading `env.hcl` and use `local.env.locals.*` for environment-specific values
- **app dependency on vpc has mock_outputs**: Both `app/terragrunt.hcl` files (dev and prod) have a dependency on `../vpc` with at least one `mock_output` and `mock_outputs_allowed_terraform_commands`
- **prod/app has prevent_destroy=true**: `infrastructure/prod/app/terragrunt.hcl` includes `prevent_destroy = true`; `dev/app` does not require it

## Failure Conditions

- Directory tree is not shown before file contents
- `root.hcl` references `env.hcl` or hard-codes environment names (violates Pattern A)
- `env.hcl` files contain incorrect CIDR blocks or instance types
- Child modules do not read `env.hcl` via `read_terragrunt_config` or do not access locals via `local.env.locals.*`
- `app/terragrunt.hcl` dependency on `../vpc` is missing `mock_outputs` or `mock_outputs_allowed_terraform_commands`
- `prod/app/terragrunt.hcl` does not include `prevent_destroy = true`
