# Scenario 02: Generate Child Module with Dependencies

## User Prompt

Generate the Terragrunt configuration for an EKS cluster module that depends on a VPC module. Use the following requirements:

- **Module location:** `infrastructure/prod/eks/terragrunt.hcl`
- **Root config:** `root.hcl` at `infrastructure/`
- **Env config:** `env.hcl` at `infrastructure/prod/`
- **Terraform module source:** `tfr:///terraform-aws-modules/eks/aws?version=20.8.0`
- **Dependencies:**
  - VPC module at `../vpc` — needs outputs: `vpc_id` (string), `private_subnet_ids` (list of strings)
  - IAM module at `../iam` — needs output: `eks_role_arn` (string)
- **Inputs needed:**
  - `cluster_name`: `"${environment}-cluster"` (from env.hcl)
  - `cluster_version`: `"1.29"`
  - `vpc_id`: from vpc dependency
  - `subnet_ids`: from vpc dependency
  - `iam_role_arn`: from iam dependency
- **Protection:** This is a production module — add `prevent_destroy = true`

Generate the complete `infrastructure/prod/eks/terragrunt.hcl` file.

Ensure:
1. Named include block referencing `root.hcl`
2. Both dependency blocks have `mock_outputs` with correct types matching the expected outputs
3. Both dependency blocks have `mock_outputs_allowed_terraform_commands`
4. `prevent_destroy = true` for production protection
5. Correct use of `local.env.locals.environment` for reading from env.hcl

## Expected Behavior

1. Use `include "root" { path = find_in_parent_folders("root.hcl") }` with the named label and explicit filename
2. Include a `locals` block with `env = read_terragrunt_config(find_in_parent_folders("env.hcl"))` and access environment values via `local.env.locals.*`
3. The `vpc` dependency block has `mock_outputs` with `vpc_id` as a string (`"vpc-mock"`) and `private_subnet_ids` as a list of strings (`["subnet-mock-1"]`) — types must match expected output types
4. The `iam` dependency block has `mock_outputs` with `eks_role_arn` as a string mock value
5. Both dependency blocks include `mock_outputs_allowed_terraform_commands = ["validate", "plan", "destroy"]` or equivalent
6. Generated file includes `prevent_destroy = true` to protect the production EKS cluster from accidental deletion

## Success Criteria

- **Named include block with find_in_parent_folders(root.hcl)**: Generated file uses `include "root" { path = find_in_parent_folders("root.hcl") }` with the named label and explicit filename
- **env.hcl read via locals block**: Generated file contains a `locals` block with `env = read_terragrunt_config(find_in_parent_folders("env.hcl"))` and accesses environment values via `local.env.locals.*`
- **vpc dependency with correctly typed mock_outputs**: `vpc` dependency block has `mock_outputs` with `vpc_id` as a string (`"vpc-mock"`) and `private_subnet_ids` as a list of strings (`["subnet-mock-1"]`) — types must match expected output types
- **iam dependency with mock_outputs**: `iam` dependency block has `mock_outputs` with `eks_role_arn` as a string mock value
- **mock_outputs_allowed_terraform_commands on all dependencies**: Both dependency blocks include `mock_outputs_allowed_terraform_commands = ["validate", "plan", "destroy"]` or equivalent
- **prevent_destroy = true present**: Generated file includes `prevent_destroy = true` to protect the production EKS cluster from accidental deletion

## Failure Conditions

- `include` block is unnamed or does not use `find_in_parent_folders("root.hcl")` with the explicit filename
- `env.hcl` is not read via `read_terragrunt_config` in a `locals` block
- `vpc_id` mock output is not a string type, or `private_subnet_ids` mock output is not a list of strings
- `iam` dependency block is missing `mock_outputs`
- Either dependency block is missing `mock_outputs_allowed_terraform_commands`
- `prevent_destroy = true` is absent from the production module configuration
