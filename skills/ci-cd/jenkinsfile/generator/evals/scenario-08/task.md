# Scenario 08: Production Deployment Pipeline with Manual Approval Gate

## User Prompt

You are building a production deployment pipeline that includes a manual approval gate. The pipeline must build, test, and then deploy to production only on the `main` branch, requiring sign-off from a group called `release-managers` before the deployment proceeds. Artifacts should be archived with fingerprinting enabled.

The pipeline must include stages for: build, test, a conditional production deployment (main branch only) with a manual approval gate, and artifact archiving.
