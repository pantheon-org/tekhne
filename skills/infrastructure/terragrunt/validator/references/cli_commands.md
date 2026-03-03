# CLI Commands Reference (Terragrunt 0.93+)

## Render Configuration

```bash
# Render configuration to JSON
terragrunt render --json

# Render and write to file
terragrunt render --json --write

# Output goes to terragrunt.rendered.json
```

## Info Print (replaces terragrunt-info)

```bash
# Get contextual information about current configuration
terragrunt info print

# Output includes:
# - config_path
# - download_dir
# - terraform_binary
# - working_dir
```

## Find and List Units

```bash
# Find all units/stacks in directory
terragrunt find

# Output as JSON
terragrunt find --json

# Include dependency information
terragrunt find --json --dag

# List units (simpler output)
terragrunt list
```

## Run Summary and Reports

```bash
# Run with summary output (default in newer versions)
terragrunt run --all plan

# Disable summary output
terragrunt run --all plan --summary-disable

# Generate detailed report file
terragrunt run --all plan --report-file=report.json

# CSV format report
terragrunt run --all plan --report-file=report.csv
```

## Terragrunt Stacks (GA in v0.78.0+)

Terragrunt Stacks provide declarative infrastructure generation using `terragrunt.stack.hcl` files.

### Stack File Structure

```hcl
# terragrunt.stack.hcl
locals {
  environment = "dev"
  aws_region  = "us-east-1"
}

# Define a unit (generates a single terragrunt.hcl)
unit "vpc" {
  source = "git::git@github.com:acme/infra-catalog.git//units/vpc?ref=v0.0.1"
  path   = "vpc"

  values = {
    environment = local.environment
    cidr        = "10.0.0.0/16"
  }
}

unit "database" {
  source = "git::git@github.com:acme/infra-catalog.git//units/database?ref=v0.0.1"
  path   = "database"

  values = {
    environment = local.environment
    vpc_path    = "../vpc"
  }
}

# Include reusable stacks
stack "monitoring" {
  source = "git::git@github.com:acme/infra-catalog.git//stacks/monitoring?ref=v0.0.1"
  path   = "monitoring"

  values = {
    environment = local.environment
  }
}
```

### Stack Commands

```bash
# Generate stack (creates .terragrunt-stack directory)
terragrunt stack generate

# Generate stack without validation
terragrunt stack generate --no-stack-validate

# Run command on all stack units
terragrunt stack run plan
terragrunt stack run apply

# Clean generated stack directories
terragrunt stack clean

# Get stack outputs
terragrunt stack output
```

### Stack Validation Control

Use `no_validation` attribute to skip validation for specific units:

```hcl
unit "experimental" {
  source = "git::git@github.com:acme/infra-catalog.git//units/experimental?ref=v0.0.1"
  path   = "experimental"

  # Skip validation for this unit (useful for incomplete/experimental units)
  no_validation = true

  values = {
    environment = local.environment
  }
}
```

### Benefits of Stacks

- **Clean working directory**: Generated code in hidden `.terragrunt-stack` directory
- **Reusable patterns**: Define infrastructure patterns once, deploy many times
- **Version pinning**: Different environments can pin different versions
- **Atomic updates**: Easy rollbacks of both modules and configurations

## Exec Command (Run Arbitrary Programs)

The `exec` command allows you to run arbitrary programs against units with Terragrunt context. This is useful for integrating other tools like tflint, checkov, or AWS CLI with Terragrunt's configuration.

```bash
# Run tflint with unit context (TF_VAR_ env vars available)
terragrunt exec -- tflint

# Run checkov against specific unit
terragrunt exec -- checkov -d .

# Run AWS CLI with unit's configuration
terragrunt exec -- aws s3 ls s3://my-bucket

# Run custom scripts with Terragrunt context
terragrunt exec -- ./scripts/validate.sh

# Run across all units
terragrunt run --all exec -- tflint
```

**Key Features:**

- Terragrunt loads the inputs for the unit and makes them available as `TF_VAR_` prefixed environment variables
- Works with any program that can use environment variables
- Integrates with Terragrunt's authentication context (e.g., AWS profiles)
- Can be combined with `run --all` for multi-unit operations

**Use Cases:**

- Running security scanners (checkov, trivy) with unit context
- Executing linters (tflint) per unit
- Running operational commands (AWS CLI) with correct credentials
- Custom validation scripts that need Terragrunt inputs

## Feature Flags (Production Feature)

Terragrunt supports first-class Feature Flags for safe infrastructure changes. Feature flags allow you to integrate incomplete work without risk, decouple release from deployment, and codify IaC evolution.

### Defining Feature Flags

```hcl
# terragrunt.hcl
feature "enable_monitoring" {
  default = false
}

feature "use_new_vpc" {
  default = true
}

inputs = {
  monitoring_enabled = feature.enable_monitoring.value
  vpc_version       = feature.use_new_vpc.value ? "v2" : "v1"
}
```

### Using Feature Flags via CLI

```bash
# Enable a feature flag
terragrunt plan --feature enable_monitoring=true

# Enable multiple feature flags
terragrunt plan --feature enable_monitoring=true --feature use_new_vpc=false

# Via environment variable
TG_FEATURE='enable_monitoring=true' terragrunt plan
```

### Feature Flags with run --all

```bash
# Apply feature flag across all units
terragrunt run --all plan --feature enable_monitoring=true
```

**Benefits:**

- **Safe rollouts**: Test changes on subset of infrastructure
- **Gradual migrations**: Enable new features incrementally
- **A/B testing**: Compare infrastructure configurations
- **Emergency rollbacks**: Quickly disable problematic features

## Experiments (Opt-in Unstable Features)

Terragrunt provides an experiments system for trying unstable features before they're GA:

```bash
# Enable all experiments (not recommended for production)
terragrunt --experiment-mode run --all plan

# Enable specific experiment
terragrunt --experiment symlinks run --all plan

# Enable CAS (Content Addressable Storage) for faster cloning
terragrunt --experiment cas run --all plan
```

**Available Experiments:**

- `symlinks` - Support symlink resolution for Terragrunt units
- `cas` - Content Addressable Storage for faster Git/module cloning
- `filter-flag` - Advanced filtering capabilities (coming in 1.0)
