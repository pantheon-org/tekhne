# Scenario 05: Validate Workflow in a Constrained CI Environment

## User Prompt

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

## Expected Behavior

1. Explicitly state that lint-only or static analysis mode was used because Docker is unavailable
2. Perform validation using the available tools (e.g., actionlint) rather than skipping validation entirely
3. List what checks were performed (YAML syntax, schema compliance, runner labels, expression syntax, security)
4. Identify at least one actual issue or observation about the workflow (e.g., action version currency, SHA pinning status)
5. Explicitly identify at least one category of check that requires Docker/act and cannot be done in lint-only mode
6. Mention actionlint by name as the tool used for static analysis

## Success Criteria

- **Lint-only approach documented**: validation-report.md explicitly states that lint-only or static analysis mode was used due to Docker being unavailable
- **Validation not skipped**: The report does NOT say that validation was skipped or deferred entirely because Docker was absent — it documents checks that were performed
- **Checks performed listed**: The report lists what was checked (e.g., YAML syntax, schema compliance, runner labels, expression syntax, security)
- **Docker limitations acknowledged**: The report explicitly identifies at least one category of check that requires Docker/act and cannot be done in lint-only mode
- **Issues identified**: The report identifies at least one actual issue or observation about the workflow (e.g., action version currency, SHA pinning status, or potential concerns)
- **actionlint referenced**: The report mentions actionlint (by name or as the tool used for static analysis)

## Failure Conditions

- The report does not state that lint-only or static analysis mode was used due to Docker being unavailable
- The report says validation was skipped because Docker was absent, without performing any checks
- The report does not list what checks were performed
- The report does not acknowledge any category of check that requires Docker/act
- The report identifies no issues or observations about the workflow
- actionlint is not mentioned by name in the report
