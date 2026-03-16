# Validate Workflow in a Constrained CI Environment

## Problem/Feature Description

A developer working in a lightweight cloud sandbox environment needs to validate a GitHub Actions workflow before pushing it to their repository. The environment does not have Docker installed and cannot install it (a common constraint in shared CI runners and remote dev environments). The developer wants to know: is the workflow syntactically valid, are there any security concerns, and are the runner labels and action inputs correct? They don't need to actually run the workflow steps locally.

Review the workflow file below, perform whatever validation is possible given the environment constraint, document your validation approach, and report any issues found.

## Output Specification

Produce:
- A `validation-report.md` documenting: (1) the validation approach taken given the Docker constraint, (2) what checks were performed, (3) any issues found, (4) what categories of issues cannot be checked without a container runtime

## Input Files

The following file is provided as input. Extract it before beginning.

=============== FILE: inputs/deploy.yml ===============
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@v4
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: ${{ secrets.AWS_ROLE_ARN }}
          aws-region: us-east-1
      - name: Deploy to ECS
        run: |
          aws ecs update-service \
            --cluster production \
            --service my-service \
            --force-new-deployment
      - name: Notify Slack
        uses: slackapi/slack-github-action@v2
        with:
          payload: |
            {"text": "Deployment complete for ${{ github.sha }}"}
        env:
          SLACK_BOT_TOKEN: ${{ secrets.SLACK_BOT_TOKEN }}
