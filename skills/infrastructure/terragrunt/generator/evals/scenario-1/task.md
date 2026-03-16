# Task: Generate root.hcl for Multi-Environment Infrastructure (Pattern A)

You need to generate a root configuration file for a multi-environment AWS infrastructure project with the following requirements:

- **Project:** `data-platform`
- **Environments:** `dev`, `staging`, `prod` (each environment will have its own `env.hcl`)
- **State backend:** AWS S3, bucket name `data-platform-tfstate`, region `eu-west-1`, DynamoDB locking table `data-platform-tfstate-lock`
- **Provider:** AWS provider, region injected via generate block, authentication via IAM role assumption (`arn:aws:iam::ACCOUNT_ID:role/TerraformDeployRole`)
- **Terraform version constraint:** `>= 1.6.0`
- **Terragrunt version constraint:** `>= 0.93.0`
- **Error handling:** Retry on transient AWS API errors using the modern `errors` block (not deprecated `retryable_errors`)

## Your Task

1. First, output the **Architecture Pattern Selection checklist** for this project before writing any files.

2. Generate the `root.hcl` file content for this project following Pattern A (environment-agnostic root).

3. Verify your generated file against the Step 4 checklist items for root.hcl:
   - No `read_terragrunt_config(find_in_parent_folders("env.hcl"))` in root
   - `remote_state` block has `encrypt = true`
   - `errors` block used (not deprecated `retryable_errors`)

4. Show what the state key path would resolve to for a module at `prod/vpc/terragrunt.hcl`.
