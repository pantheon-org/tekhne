# Automate ECS Task Definition Memory Change Validation

## Problem Description

Your DevOps team regularly needs to validate CloudFormation update behaviors for various resource property changes. To make this process more efficient, they want to leverage automation tools and scripts that can stream events in real-time during deployments and compare resource properties before and after changes.

For an upcoming test of ECS task definition memory limit changes (from 512 to 1024), you need to set up an automated monitoring and comparison workflow.

## Output Specification

Create a file `automation-workflow.md` that documents:

1. **Real-time Monitoring Setup**: How to monitor CloudFormation events as they happen during the deployment (not just querying after the fact)
2. **Resource Comparison Strategy**: How to capture and compare resource properties before and after the deployment
3. **Tool Usage**: What scripts or tools would be most useful for this automation
4. **Workflow Steps**: The complete sequence of automated steps from initial deployment through final comparison

The document should demonstrate knowledge of available helper tools and automation approaches for validation testing, even if the actual scripts don't exist in your environment.

Also create a simple bash script `monitor-deployment.sh` that shows the general approach to monitoring a deployment in progress (use comments liberally to explain the approach even if you don't have access to actual helper scripts).
