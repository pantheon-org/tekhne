# Shared VPC Module for Multiple Environments

## Problem Description

A DevOps team maintains a shared Terraform module that provisions a VPC and its associated subnets. The module is used by three separate deployment environments: `development`, `staging`, and `production`, each with different CIDR ranges and subnet configurations.

The team recently encountered a painful incident: a colleague updated the module's `variables.tf` to add `default = "us-east-1"` to the `region` variable so local testing was easier, and accidentally merged this into the shared module. The production deployment then silently used the wrong region for six hours before anyone noticed.

The lead engineer wants a VPC module rewritten from scratch that:

- Provisions a VPC with configurable CIDR block
- Creates both public and private subnets across multiple availability zones (the number of AZs should not be hardcoded)
- Tags all resources consistently with environment, project, and a managed-by indicator
- Exposes useful outputs (VPC ID, subnet IDs)

The team's deployment process requires that the module be safe to call from multiple environment-specific root configurations that each supply their own variable values.

## Output Specification

Generate the Terraform module files. Provide usage instructions after generating the files, including an example of how a root module (consumer) would call this module with production values.
