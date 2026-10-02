# Scenario 2: Log Capture from a Failing Service

## User Prompt

A Kubernetes service is returning 500 errors intermittently. The agent must collect logs and save them as proof-of-work for the incident investigation.

Run `kubectl logs deploy/api-service --tail=200` and save the output as a structured log artifact. Include the evidence in your investigation summary.
