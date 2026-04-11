# Scenario 02: Log Capture from a Failing Service

## User Prompt

A Kubernetes service is returning 500 errors intermittently. The agent must collect logs and save them as proof-of-work for the incident investigation.

Run `kubectl logs deploy/api-service --tail=200` and save the output as a structured log artifact. Include the evidence in your investigation summary.

## Expected Behavior

1. Identify log capture as the appropriate artifact type
2. Use `tee` or output redirection to save the log output to `.context/evidence/`
3. Save the artifact with a date-prefixed descriptive filename ending in `.txt`
4. Include an evidence summary table in the response referencing the saved log file
5. Avoid merely quoting log lines in prose without saving the artifact to disk

## Success Criteria

- **Identifies log capture as the correct artifact type**: Agent identifies log capture as the correct artifact type
- **Uses tee or output redirection to save to .context/evidence/**: Agent uses `tee` or output redirection to save logs to `.context/evidence/`
- **Artifact filename is date-prefixed and descriptive**: Artifact filename is date-prefixed and descriptive, ending in `.txt`
- **Response includes evidence summary table**: Response includes evidence summary table referencing the log artifact
- **Agent avoids quoting raw log output in prose**: Agent avoids quoting raw log output in prose without saving to disk

## Failure Conditions

- Misidentifies the artifact type (e.g., uses screenshot instead of log capture)
- Does not use `tee` or redirection — runs the kubectl command without saving output
- Artifact is not saved to `.context/evidence/` or filename is undescriptive
- Response does not include an evidence summary table referencing the artifact
- Agent pastes raw log content inline in prose rather than saving to disk
