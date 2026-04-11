# Scenario 05: Shared VPC Module for Multiple Environments

## User Prompt

A DevOps team maintains a shared Terraform module that provisions a VPC and its associated subnets. The module is used by three separate deployment environments: `development`, `staging`, and `production`, each with different CIDR ranges and subnet configurations.

The team recently encountered a painful incident: a colleague updated the module's `variables.tf` to add `default = "us-east-1"` to the `region` variable so local testing was easier, and accidentally merged this into the shared module. The production deployment then silently used the wrong region for six hours before anyone noticed.

The lead engineer wants a VPC module rewritten from scratch that:

- Provisions a VPC with configurable CIDR block
- Creates both public and private subnets across multiple availability zones (the number of AZs should not be hardcoded)
- Tags all resources consistently with environment, project, and a managed-by indicator
- Exposes useful outputs (VPC ID, subnet IDs)

The team's deployment process requires that the module be safe to call from multiple environment-specific root configurations that each supply their own variable values.

Generate the Terraform module files. Provide usage instructions after generating the files, including an example of how a root module (consumer) would call this module with production values.

## Expected Behavior

1. The region (or `aws_region`) variable in the module's `variables.tf` does NOT have a `default` attribute
2. Any variable that is environment-specific (`environment`, `env`, or similar) does NOT have a `default` value in the module
3. Availability zones are retrieved using `data "aws_availability_zones" "available" {}` rather than hardcoded strings
4. A `locals` block defines `common_tags` (or equivalent) including at least `Environment` and `ManagedBy = "Terraform"`
5. Resource blocks reference the `locals` common_tags rather than duplicating tag definitions inline
6. All outputs include a `description` attribute
7. All variable declarations include both `description` and `type` attributes
8. Module files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
9. Usage instructions include an example `module` block showing how a root module would call this module with production values

## Success Criteria

- **No default on region variable**: The region (or `aws_region`) variable in the module's `variables.tf` does NOT have a `default` attribute
- **No default on environment variable**: Any variable that is environment-specific (`environment`, `env`, or similar) does NOT have a `default` value in the module
- **Data source for AZs**: Availability zones are retrieved using `data "aws_availability_zones" "available" {}` rather than hardcoded strings
- **locals for common tags**: A `locals` block defines `common_tags` (or equivalent) including at least `Environment` and `ManagedBy = "Terraform"`
- **Tags applied via locals**: Resource blocks reference the `locals` common_tags rather than duplicating tag definitions inline
- **Output descriptions present**: All outputs include a `description` attribute
- **Variables have descriptions and types**: All variable declarations include both `description` and `type` attributes
- **File organization correct**: Module files include at minimum: `main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`
- **Consumer example provided**: Usage instructions include an example `module` block showing how a root module would call this module with production values

## Failure Conditions

- The `region` variable has a `default` value in the module (defeats the protection against the incident described)
- Any environment-specific variable has a `default` value in the module
- Availability zones are hardcoded as strings instead of using a `data "aws_availability_zones"` source
- No `locals` block for common tags, or tags are duplicated inline on resources
- Variables are missing `description` or `type` attributes
- Outputs are missing `description` attributes
- Required module files (`main.tf`, `variables.tf`, `outputs.tf`, `versions.tf`) are absent
