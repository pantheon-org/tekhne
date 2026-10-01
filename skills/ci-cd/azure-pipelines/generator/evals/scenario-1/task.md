# Scenario 1: Create a Node.js CI Pipeline for a New Service

## User Prompt

A startup's backend team has just created a new Node.js microservice and needs a CI pipeline set up in Azure DevOps. The service uses npm for package management and Jest for testing. The team wants the pipeline to install dependencies, build the service, run tests, and publish test results so they show up in the ADO build summary. The pipeline should run on every push to the main and develop branches.

The DevOps engineer who normally handles this is on leave, so you've been asked to generate the pipeline YAML. The team has had problems in the past with pipelines that suddenly broke when agent software updated, so reliability is a priority.
