# Multi-Region Notification Service Infrastructure

## Problem Description

A product team needs Terraform to manage identical infrastructure for three regional deployments of a `notification-service`: `us-east-1`, `eu-west-1`, and `ap-southeast-1`. Each region needs:

- An SQS queue (standard, not FIFO) for inbound notifications, named after the region
- An SNS topic that fans out to that queue

The team works in squads of 6 engineers, uses GitHub Actions for CI/CD, and all environments are managed from a shared Git repository. Currently there is no infrastructure state management beyond local files, which has caused several incidents where two engineers applied conflicting changes simultaneously. The lead engineer wants this fixed as part of this task.

The infrastructure should be easy to extend — a fourth region may be added in the future without restructuring the code.

## Output Specification

Generate the Terraform configuration for this setup. The solution should handle all three regions' resources within a single configuration. Include a backend configuration appropriate for a team workflow. After generating files, include usage instructions.
