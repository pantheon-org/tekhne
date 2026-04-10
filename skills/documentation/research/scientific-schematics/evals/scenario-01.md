# Scenario 01: Scientific Schematics — CONSORT Flowchart for Journal Paper

## User Prompt

The user is writing a clinical trial paper for submission to a peer-reviewed journal and needs a CONSORT flowchart showing participant recruitment, allocation, follow-up, and analysis.

Generate a publication-quality CONSORT flowchart using the `journal` document type. Confirm the output meets the quality threshold before delivering it.

## Expected Behavior

1. Load `ANTHROPIC_API_KEY` from the environment — do not hardcode any key
2. Pass `--doc-type journal` (threshold 8.5/10) to the generation script
3. Save the generated diagram under `figures/` with a descriptive filename
4. Confirm that the diagram meets the 8.5/10 journal threshold before reporting completion
5. Describe what was generated and provide the output file path to the user

## Success Criteria

- **ANTHROPIC_API_KEY loaded from environment**: The agent uses ANTHROPIC_API_KEY from the environment and does not hardcode any key.
- **--doc-type journal used**: The agent passes --doc-type journal (threshold 8.5/10) to the generation script.
- **Output stored in figures/**: The generated diagram is saved under figures/ with a descriptive filename.
- **Quality threshold verified**: The agent confirms the diagram meets the 8.5/10 journal threshold before reporting completion.
- **Diagram described to user**: The agent describes what was generated and provides the output file path.

## Failure Conditions

- Agent hardcodes an API key or fails to load it from the environment
- Agent uses the wrong `--doc-type` (e.g., `conference` or `poster`) instead of `journal`
- Agent saves the output outside `figures/` or uses a non-descriptive filename
- Agent reports completion without verifying the 8.5/10 quality threshold
- Agent does not describe the generated diagram or omits the output file path
