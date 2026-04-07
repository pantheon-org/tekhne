# Scenario 2: Log capture from a failing service

## Context

A Kubernetes service is returning 500 errors intermittently. The agent must collect logs and save them as proof-of-work for the incident investigation.

## Task

Run `kubectl logs deploy/api-service --tail=200` and save the output as a structured log artifact. Include the evidence in your investigation summary.

## Expected Behavior

1. Agent identifies log capture as the appropriate artifact type.
2. Agent uses `tee` or redirection to save output to `.context/evidence/`.
3. Artifact is saved with a date-prefixed descriptive filename ending in `.txt`.
4. Response includes an evidence summary table referencing the saved log file.
5. Agent does not merely quote log lines in prose without saving the artifact.
