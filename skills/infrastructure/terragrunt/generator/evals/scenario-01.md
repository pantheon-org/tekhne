# Scenario 01: Generate root.hcl for Multi-Environment Infrastructure (Pattern A)

## User Prompt

You need to generate a root configuration file for a multi-environment AWS infrastructure project with the following requirements:

- **Project:** `data-platform`
- **Environments:** `dev`, `staging`, `prod` (each environment will have its own `env.hcl`)
- **State backend:** AWS S3, bucket name `data-platform-tfstate`, region `eu-west-1`, DynamoDB locking table `data-platform-tfstate-lock`
- **Provider:** AWS provider, region injected via generate block, authentication via IAM role assumption (`arn:aws:iam::ACCOUNT_ID:role/TerraformDeployRole`)
- **Terraform version constraint:** `>= 1.6.0`
- **Terragrunt version constraint:** `>= 0.93.0`
- **Error handling:** Retry on transient AWS API errors using the modern `errors` block (not deprecated `retryable_errors`)

1. First, output the **Architecture Pattern Selection checklist** for this project before writing any files.

2. Generate the `root.hcl` file content for this project following Pattern A (environment-agnostic root).

3. Verify your generated file against the Step 4 checklist items for root.hcl:
   - No `read_terragrunt_config(find_in_parent_folders("env.hcl"))` in root
   - `remote_state` block has `encrypt = true`
   - `errors` block used (not deprecated `retryable_errors`)

4. Show what the state key path would resolve to for a module at `prod/vpc/terragrunt.hcl`.

## Expected Behavior

1. Output the Architecture Pattern Selection checklist before writing any files, identifying Pattern A, environment-agnostic root scope, `env.hcl` location in each environment directory, and child module access pattern
2. Generate `root.hcl` that does NOT contain `read_terragrunt_config(find_in_parent_folders("env.hcl"))` or any reference to `env.hcl` at the root level
3. Include a `remote_state` block with `encrypt = true`, the correct bucket name, `dynamodb_table = "data-platform-tfstate-lock"`, and `path_relative_to_include()` for the state key
4. Include a `generate` block that produces a `provider "aws"` block using `assume_role` with the specified role ARN rather than hardcoded `access_key`/`secret_key`
5. Use an `errors { retry { ... } }` block for transient error handling, not the deprecated `retryable_errors` attribute
6. Include `terraform_version_constraint` and `terragrunt_version_constraint` with the specified versions

## Success Criteria

- **Architecture Pattern Selection checklist output**: Agent outputs the mandatory checklist before writing any files, identifying Pattern A, environment-agnostic root scope, `env.hcl` location in each environment directory, and child module access pattern
- **Environment-agnostic root: no env.hcl read**: Generated `root.hcl` does NOT contain `read_terragrunt_config(find_in_parent_folders("env.hcl"))` or any reference to `env.hcl` at the root level
- **Remote state with encrypt=true and DynamoDB locking**: `remote_state` block includes `encrypt = true`, the correct bucket name, `dynamodb_table = "data-platform-tfstate-lock"`, and uses `path_relative_to_include()` for the state key
- **Provider with assume_role in generate block**: `generate` block produces a `provider "aws"` block using `assume_role` with the specified role ARN rather than hardcoded `access_key`/`secret_key`
- **errors block used instead of retryable_errors**: Generated file uses an `errors { retry { ... } }` block for transient error handling, not the deprecated `retryable_errors` attribute
- **Version constraints present**: Generated file includes `terraform_version_constraint` and `terragrunt_version_constraint` with the specified versions

## Failure Conditions

- Architecture Pattern Selection checklist is not output before generating files
- `root.hcl` contains `read_terragrunt_config(find_in_parent_folders("env.hcl"))` (violates Pattern A)
- `remote_state` block is missing `encrypt = true` or `dynamodb_table` for state locking
- Provider uses hardcoded credentials instead of `assume_role` in a `generate` block
- Deprecated `retryable_errors` attribute is used instead of the modern `errors { retry { ... } }` block
- Version constraints are absent or use incorrect syntax
