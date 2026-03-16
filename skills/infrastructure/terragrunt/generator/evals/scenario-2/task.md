# Task: Generate Child Module with Dependencies

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

## Your Task

Generate the complete `infrastructure/prod/eks/terragrunt.hcl` file.

Ensure:
1. Named include block referencing `root.hcl`
2. Both dependency blocks have `mock_outputs` with correct types matching the expected outputs
3. Both dependency blocks have `mock_outputs_allowed_terraform_commands`
4. `prevent_destroy = true` for production protection
5. Correct use of `local.env.locals.environment` for reading from env.hcl
