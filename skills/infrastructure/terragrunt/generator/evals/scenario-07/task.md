# Scenario 07: Generate Feature Flags and Exclude Blocks

## User Prompt

A team wants to add runtime feature control and production safety protections to their existing Terragrunt modules. Generate the following configurations:

### Requirement 1: Feature Flag for Optional Monitoring

Add a feature flag `enable_monitoring` to a module at `infrastructure/prod/app/terragrunt.hcl` that:
- Defaults to `false`
- When enabled via CLI (`--feature enable_monitoring=true`), adds monitoring resources to the inputs
- The flag default must be a static value

The team lead suggested writing: `default = local.env.locals.enable_monitoring` — **this is wrong**. Generate the correct approach.

### Requirement 2: Exclude Block for Production Safety

Add an exclude block to `infrastructure/prod/rds/terragrunt.hcl` that:
- Prevents all `destroy` operations on the production RDS module
- Still allows `plan` and `apply` operations

### Requirement 3: Replace a Legacy `skip` Attribute

A legacy module uses `skip = true` to disable itself. Replace this with the modern `exclude` block equivalent.

Generate the HCL snippets for:
1. The correct `feature` block with a static default (explain why the suggested local reference fails)
2. The `exclude` block for the prod/rds module
3. The modernised replacement for `skip = true`

Also show the CLI command and environment variable approaches to activate the feature flag at runtime.
