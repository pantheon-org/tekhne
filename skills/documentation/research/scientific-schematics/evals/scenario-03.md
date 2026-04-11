# Scenario 03: Scientific Schematics — Biological Pathway for Poster

## User Prompt

The user is preparing a conference poster and needs a diagram of the MAPK signalling pathway including key phosphorylation events.

Generate the diagram using the `poster` document type. The user is on a tight deadline, so use a maximum of 1 iteration.

## Expected Behavior

1. Use `--doc-type poster` (threshold 7.0/10) — the lowest appropriate threshold for poster output
2. Pass `--iterations 1` to limit generation to a single pass as requested
3. Include MAPK signalling pathway and key phosphorylation events in the diagram description
4. Save the generated poster diagram under `figures/`
5. Optionally note that the poster threshold (7.0) is lower than journal (8.5) to set appropriate quality expectations

## Success Criteria

- **--doc-type poster applied**: The agent uses --doc-type poster (threshold 7.0/10), the lowest appropriate threshold for poster output.
- **--iterations 1 passed**: The agent passes --iterations 1 to limit generation to a single pass as requested.
- **MAPK pathway and phosphorylation in prompt**: The diagram description includes MAPK signalling pathway and key phosphorylation events.
- **Output stored in figures/**: The generated poster diagram is saved under figures/.
- **Threshold difference explained**: The agent optionally notes that the poster threshold (7.0) is lower than journal (8.5), setting appropriate expectations.

## Failure Conditions

- Agent uses `--doc-type journal` or `--doc-type conference` instead of `poster`
- Agent omits `--iterations 1` or allows more than one generation pass
- Agent omits MAPK signalling pathway or key phosphorylation events from the diagram description
- Agent saves the output outside `figures/`
- Agent does not set appropriate expectations when using the lower poster quality threshold
