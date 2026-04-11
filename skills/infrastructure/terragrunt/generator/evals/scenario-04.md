# Scenario 04: Generate Feature Flags and Exclude Blocks

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

## Expected Behavior

1. Generate a `feature` block using `default = false` (a static boolean), not `default = local.env.locals.enable_monitoring`
2. Explain that feature flag defaults are evaluated before locals are resolved, so `local.*` references are not available at that evaluation phase
3. Generate an `exclude` block for `prod/rds` using `if = true` and `actions = ["destroy"]` to block only destroy operations while allowing plan and apply
4. Generate a replacement for `skip = true` using an `exclude` block with `if = true` and `actions = ["all"]` or `actions = ["all_except_output"]`
5. Show both `terragrunt apply --feature enable_monitoring=true` and the equivalent `TG_FEATURE` environment variable approach

## Success Criteria

- **Feature flag with static default generated**: Generated `feature` block uses `default = false` (a static boolean), not `default = local.env.locals.enable_monitoring`
- **Why local.* reference fails explained**: Agent explains that feature flag defaults are evaluated before locals are resolved, so `local.*` references are not available at that evaluation phase
- **Exclude block for destroy protection**: Generated `exclude` block for `prod/rds` uses `if = true` and `actions = ["destroy"]` to block only destroy operations while allowing plan and apply
- **skip = true replaced with exclude block**: Generated replacement for `skip = true` uses an `exclude` block with `if = true` and `actions = ["all"]` or `actions = ["all_except_output"]`
- **Feature flag activation commands shown**: Agent shows both `terragrunt apply --feature enable_monitoring=true` and the equivalent `TG_FEATURE` environment variable approach

## Failure Conditions

- Feature flag `default` uses `local.env.locals.enable_monitoring` instead of a static boolean value
- No explanation is provided for why the `local.*` reference fails in feature flag defaults
- `exclude` block for `prod/rds` does not use `actions = ["destroy"]` (blocks all actions instead of only destroy)
- `skip = true` is not replaced with an equivalent `exclude` block
- CLI (`--feature`) and environment variable (`TG_FEATURE`) activation approaches are not both shown
