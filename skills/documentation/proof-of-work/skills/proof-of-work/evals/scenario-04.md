# Scenario 04: agent-browser Screenshot during Infrastructure Audit

## User Prompt

An agent is auditing an AWS console dashboard for a client. Screenshots are required to confirm the observed state of security group rules.

Using agent-browser, navigate to the EC2 security groups page and capture a screenshot of the inbound rules for `sg-0abc123`. Save the artifact correctly named to reflect what it shows.

## Expected Behavior

1. Use the agent-browser `screenshot` action (not playwright-mcp) to capture the screenshot
2. Save the screenshot to `.context/evidence/` with a descriptive slug referencing the security group ID
3. Name the file following the `YYYY-MM-DD-<slug>.png` convention
4. Include an evidence summary table in the response
5. Avoid describing the security group rules in prose as a substitute for saving the screenshot

## Success Criteria

- **Uses agent-browser screenshot action**: Agent uses agent-browser `screenshot` action rather than a different tool
- **Screenshot saved to .context/evidence/ with security group ID in slug**: Screenshot saved to `.context/evidence/` with security group ID in the slug
- **Filename follows YYYY-MM-DD-<descriptive-slug>.png convention**: Filename follows `YYYY-MM-DD-<descriptive-slug>.png` convention
- **Response includes evidence summary table**: Response includes evidence summary table with path and description
- **Agent does not substitute prose description for saved artifact**: Agent does not substitute prose description of rules for a saved artifact

## Failure Conditions

- Uses playwright-mcp or another tool instead of agent-browser `screenshot` action
- Screenshot is not saved to `.context/evidence/` or the slug does not reference the security group ID
- Filename does not follow `YYYY-MM-DD-<slug>.png` convention
- Response does not include an evidence summary table
- Agent describes the security group inbound rules in prose instead of (or as a substitute for) saving the screenshot
