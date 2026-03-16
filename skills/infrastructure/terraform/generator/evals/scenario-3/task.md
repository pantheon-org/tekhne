# Cross-Provider Infrastructure: AWS + Datadog Monitoring

## Problem Description

A platform team manages their AWS infrastructure with Terraform and wants to add Datadog monitors as code in the same repository. They need a Terraform configuration that provisions:

- An AWS Lambda function (Python 3.12 runtime) triggered by an SQS queue
- A Datadog monitor that alerts when the Lambda's error rate exceeds a threshold

The team is using Datadog provider version 3.x. They have a Datadog API key and app key, and an AWS account. The engineering manager has stressed that the team's security policy forbids committing credentials of any kind to the repository.

The platform team has run into issues before where a new engineer joined, ran `terraform apply` without understanding what would change, and caused an outage. The manager wants clear documentation of the safe workflow for applying changes.

## Output Specification

Generate the complete Terraform configuration. Include a `README` section (can be in a terraform comment or a separate .md file) that documents the safe workflow for applying changes through CI/CD. Provide usage instructions after generating the files.
