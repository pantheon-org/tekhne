# Scenario 4: agent-browser screenshot during infrastructure audit

## Context

An agent is auditing an AWS console dashboard for a client. Screenshots are required to confirm the observed state of security group rules.

## Task

Using agent-browser, navigate to the EC2 security groups page and capture a screenshot of the inbound rules for `sg-0abc123`. Save the artifact correctly named to reflect what it shows.

## Expected Behavior

1. Agent uses agent-browser `screenshot` action (not playwright-mcp).
2. Screenshot is saved to `.context/evidence/` with a descriptive slug referencing the security group.
3. Filename follows YYYY-MM-DD-<slug>.png convention.
4. Response includes evidence summary table.
5. Agent does not just describe the rules in prose without saving the screenshot.
