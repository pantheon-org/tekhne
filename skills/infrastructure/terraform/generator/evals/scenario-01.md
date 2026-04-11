# Scenario 01: New Microservice AWS Infrastructure Bootstrap

## User Prompt

A startup is launching a new `recommendation-engine` microservice on AWS. The engineering lead has asked you to create the initial Terraform configuration that will provision the following resources in `us-east-1`:

- An EC2 instance (type configurable, defaulting to `t3.micro`) running the service
- A security group that allows inbound HTTPS (port 443) from `10.0.0.0/8` only
- An IAM instance profile and role granting read access to a specific S3 bucket

The team is small but growing. The region, environment name, and project name should all be configurable. The security group's allowed CIDR ranges need to come from a list so they can be expanded without changing the module structure. There is no existing infrastructure — this is a green-field project.

The engineering lead wants the configuration to be immediately usable by another engineer who clones the repository, and wants to understand what files to look at and what commands to run.

Generate all the Terraform configuration files needed for this infrastructure. After generating the files, provide usage instructions explaining the generated files and the commands needed to deploy them.

The configuration should be production-quality: well-organized, documented, and safe to share in a team repository.

## Expected Behavior

1. Create a file named `main.tf` containing the primary resource definitions
2. Create a file named `variables.tf` containing input variable declarations
3. Create a file named `outputs.tf` containing output value declarations
4. Create a file named `versions.tf` (or equivalent) with the `terraform` block and `required_providers`
5. Every variable declaration in `variables.tf` includes a `description` attribute
6. Every variable declaration in `variables.tf` includes a `type` attribute
7. Every output declaration in `outputs.tf` includes a `description` attribute
8. The `terraform` block includes `required_version` with both a lower bound (`>= 1.x`) and an upper bound (`< 2.0`)
9. The AWS provider uses `version = "~> 6.0"` or an equivalent major-pinned `~>` constraint
10. The `provider "aws"` block uses `var.aws_region` or a variable reference — NOT a hardcoded string like `"us-east-1"`
11. Response includes a table or list of generated files plus commands to run (`terraform init`, `plan`, `apply`)
12. A `locals` block exists with common tags (at minimum `Environment` and `ManagedBy = "Terraform"`)

## Success Criteria

- **Separate main.tf**: A file named `main.tf` exists containing the primary resource definitions
- **Separate variables.tf**: A file named `variables.tf` exists containing input variable declarations
- **Separate outputs.tf**: A file named `outputs.tf` exists containing output value declarations
- **Separate versions.tf**: A file named `versions.tf` (or equivalent) exists with the `terraform` block and `required_providers`
- **Variables have descriptions**: Every variable declaration in `variables.tf` includes a `description` attribute
- **Variables have types**: Every variable declaration in `variables.tf` includes a `type` attribute
- **Output descriptions present**: Every output declaration in `outputs.tf` includes a `description` attribute
- **Terraform version constraint**: The `terraform` block includes `required_version` with both a lower bound (`>= 1.x`) and an upper bound (`< 2.0`)
- **AWS provider version pinned**: The AWS provider uses `version = "~> 6.0"` (or equivalent major-pinned `~>` constraint)
- **Variable reference in provider**: The `provider "aws"` block uses `var.aws_region` or a variable reference — NOT a hardcoded string like `"us-east-1"`
- **Usage instructions provided**: Response includes a table or list of generated files plus commands to run (`terraform init`, `plan`, `apply`)
- **locals for common tags**: A `locals` block exists with common tags (at minimum `Environment` and `ManagedBy="Terraform"`)

## Failure Conditions

- Any of the four required files (`main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`) is absent
- Variable declarations are missing `description` or `type` attributes
- Output declarations are missing `description` attributes
- `required_version` is absent or lacks an upper bound
- AWS provider version is not pinned with a `~>` constraint
- AWS region is hardcoded in the provider block instead of using a variable
- No usage instructions or deployment commands are provided
- No `locals` block with common tagging
