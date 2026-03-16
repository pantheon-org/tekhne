# New Microservice AWS Infrastructure Bootstrap

## Problem Description

A startup is launching a new `recommendation-engine` microservice on AWS. The engineering lead has asked you to create the initial Terraform configuration that will provision the following resources in `us-east-1`:

- An EC2 instance (type configurable, defaulting to `t3.micro`) running the service
- A security group that allows inbound HTTPS (port 443) from `10.0.0.0/8` only
- An IAM instance profile and role granting read access to a specific S3 bucket

The team is small but growing. The region, environment name, and project name should all be configurable. The security group's allowed CIDR ranges need to come from a list so they can be expanded without changing the module structure. There is no existing infrastructure — this is a green-field project.

The engineering lead wants the configuration to be immediately usable by another engineer who clones the repository, and wants to understand what files to look at and what commands to run.

## Output Specification

Generate all the Terraform configuration files needed for this infrastructure. After generating the files, provide usage instructions explaining the generated files and the commands needed to deploy them.

The configuration should be production-quality: well-organized, documented, and safe to share in a team repository.
