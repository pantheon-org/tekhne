# Task: Generate Feature Flags and Exclude Blocks

A team wants to add runtime feature control and production safety protections to their existing Terragrunt modules. Generate the following configurations:

## Requirement 1: Feature Flag for Optional Monitoring

Add a feature flag `enable_monitoring` to a module at `infrastructure/prod/app/terragrunt.hcl` that:
- Defaults to `false`
- When enabled via CLI (`--feature enable_monitoring=true`), adds monitoring resources to the inputs
- The flag default must be a static value

The team lead suggested writing: `default = local.env.locals.enable_monitoring` — **this is wrong**. Generate the correct approach.

## Requirement 2: Exclude Block for Production Safety

The production database module (`infrastructure/prod/rds/terragrunt.hcl`) should:
- Be excluded from `destroy` operations by default
- Still be included in plan and apply operations
- Complement `prevent_destroy = true`

## Requirement 3: Replace a Legacy skip Attribute

An existing module has `skip = true` that should be updated to the modern `exclude` block syntax. The original intent was to skip all operations on this module.

## Your Task

Generate the HCL snippets for:
1. The correct `feature` block with a static default (explain why the suggested local reference fails)
2. The `exclude` block for the prod/rds module
3. The modernised replacement for `skip = true`

Also show the CLI command and environment variable approaches to activate the feature flag at runtime.
